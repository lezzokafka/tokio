//! Asynchronous I/O.
//!
//! This module is the asynchronous version of `std::io`. Primarily, it
//! defines two traits, [`AsyncRead`] and [`AsyncWrite`], which extend the
//! `Read` and `Write` traits of the standard library.
//!
//! # AsyncRead and AsyncWrite
//!
//! [`AsyncRead`] and [`AsyncWrite`] must only be implemented for
//! non-blocking I/O types that integrate with the futures type system. In
//! other words, these types must never block the thread, and instead the
//! current task is notified when the I/O resource is ready.
//!
//! # Standard input and output
//!
//! Tokio provides asynchronous APIs to standard [input], [output], and [error].
//! These APIs are very similar to the ones provided by `std`, but they also
//! implement [`AsyncRead`] and [`AsyncWrite`].
//!
//! Unlike *most* other Tokio APIs, the standard input / output APIs
//! **must** be used from the context of the Tokio runtime as they require
//! Tokio specific features to function.
//!
//! [input]: fn.stdin.html
//! [output]: fn.stdout.html
//! [error]: fn.stderr.html
//!
//! # `std` re-exports
//!
//! Additionally, [`Error`], [`ErrorKind`], and [`Result`] are re-exported
//! from `std::io` for ease of use.
//!
//! [`AsyncRead`]: trait.AsyncRead.html
//! [`AsyncWrite`]: trait.AsyncWrite.html
//! [`Error`]: struct.Error.html
//! [`ErrorKind`]: enum.ErrorKind.html
//! [`Result`]: type.Result.html

// standard input, output, and error
#[cfg(feature = "fs")]
pub use tokio_fs::{stderr, stdin, stdout, Stderr, Stdin, Stdout};
pub use tokio_io::split::split;
pub use tokio_io::{
    empty, repeat, sink, AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite,
    AsyncWriteExt, BufReader, BufWriter, Empty, Repeat, Sink,
};

// Re-export io::Error so that users don't have to deal
// with conflicts when `use`ing `tokio::io` and `std::io`.
pub use std::io::{Error, ErrorKind, Result};
