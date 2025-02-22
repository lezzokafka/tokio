//! Thread pool for blocking operations

use tokio_sync::oneshot;

use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Condvar, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

struct Pool {
    shared: Mutex<Shared>,
    condvar: Condvar,
}

struct Shared {
    queue: VecDeque<Box<dyn FnOnce() + Send>>,
    num_th: u32,
    num_idle: u32,
    num_notify: u32,
}

lazy_static! {
    static ref POOL: Pool = Pool::new();
}

const MAX_THREADS: u32 = 1_000;
const KEEP_ALIVE: Duration = Duration::from_secs(10);

/// Result of a blocking operation running on the blocking thread pool.
#[derive(Debug)]
pub struct Blocking<T> {
    rx: oneshot::Receiver<T>,
}

/// Run the provided function on a threadpool dedicated to blocking operations.
pub fn run<F, R>(f: F) -> Blocking<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    let should_spawn = {
        let mut shared = POOL.shared.lock().unwrap();

        shared.queue.push_back(Box::new(move || {
            // The receiver may have been dropped.
            let _ = tx.send(f());
        }));

        if shared.num_idle == 0 {
            // No threads are able to process the task.

            if shared.num_th == MAX_THREADS {
                // At max number of threads
                false
            } else {
                shared.num_th += 1;
                true
            }
        } else {
            // Notify an idle worker thread. The notification counter
            // is used to count the needed amount of notifications
            // exactly. Thread libraries may generate spurious
            // wakeups, this counter is used to keep us in a
            // consistent state.
            shared.num_idle -= 1;
            shared.num_notify += 1;
            POOL.condvar.notify_one();
            false
        }
    };

    if should_spawn {
        spawn_thread();
    }

    Blocking { rx }
}

impl<T> Future for Blocking<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use std::task::Poll::*;

        match Pin::new(&mut self.rx).poll(cx) {
            Ready(Ok(v)) => Ready(v),
            Ready(Err(_)) => panic!(
                "the blocking operation has been dropped before completing. \
                 This should not happen and is a bug."
            ),
            Pending => Pending,
        }
    }
}

fn spawn_thread() {
    thread::Builder::new()
        .name("tokio-blocking-driver".to_string())
        .spawn(|| {
            let mut shared = POOL.shared.lock().unwrap();
            loop {
                // BUSY
                while let Some(task) = shared.queue.pop_front() {
                    drop(shared);
                    run_task(task);
                    shared = POOL.shared.lock().unwrap();
                }

                // IDLE
                shared.num_idle += 1;

                loop {
                    let lock_result = POOL.condvar.wait_timeout(shared, KEEP_ALIVE).unwrap();
                    shared = lock_result.0;
                    let timeout_result = lock_result.1;

                    if shared.num_notify != 0 {
                        // We have received a legitimate wakeup,
                        // acknowledge it by decrementing the counter
                        // and transition to the BUSY state.
                        shared.num_notify -= 1;
                        break;
                    }

                    if timeout_result.timed_out() {
                        // Thread exit
                        shared.num_th -= 1;

                        // num_idle should now be tracked exactly, panic
                        // with a descriptive message if it is not the
                        // case.
                        shared.num_idle = shared
                            .num_idle
                            .checked_sub(1)
                            .expect("num_idle underflowed on thread exit");
                        return;
                    }

                    // Spurious wakeup detected, go back to sleep.
                }
            }
        })
        .unwrap();
}

fn run_task(f: Box<dyn FnOnce() + Send>) {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let _ = catch_unwind(AssertUnwindSafe(|| f()));
}

impl Pool {
    fn new() -> Pool {
        Pool {
            shared: Mutex::new(Shared {
                queue: VecDeque::new(),
                num_th: 0,
                num_idle: 0,
                num_notify: 0,
            }),
            condvar: Condvar::new(),
        }
    }
}
