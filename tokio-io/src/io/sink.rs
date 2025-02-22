use crate::AsyncWrite;
use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

/// An async writer which will move data into the void.
///
/// This struct is generally created by calling [`sink`][sink]. Please
/// see the documentation of `sink()` for more details.
///
/// This is an asynchronous version of `std::io::Sink`.
///
/// [sink]: fn.sink.html
pub struct Sink {
    _p: (),
}

/// Creates an instance of an async writer which will successfully consume all
/// data.
///
/// All calls to `poll_write` on the returned instance will return
/// `Poll::Ready(Ok(buf.len()))` and the contents of the buffer will not be
/// inspected.
///
/// This is an asynchronous version of `std::io::sink`.
///
/// # Examples
///
/// ```rust
/// # use tokio_io::{self as io, AsyncWriteExt};
/// # async fn dox() {
/// let buffer = vec![1, 2, 3, 5, 8];
/// let num_bytes = io::sink().write(&buffer).await.unwrap();
/// assert_eq!(num_bytes, 5);
/// # }
/// ```
pub fn sink() -> Sink {
    Sink { _p: () }
}

impl AsyncWrite for Sink {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Poll::Ready(Ok(buf.len()))
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl fmt::Debug for Sink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Sink { .. }")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_unpin() {
        crate::is_unpin::<Sink>();
    }
}
