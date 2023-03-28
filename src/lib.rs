//! Read from stdin over a Tokio channel
//!
//! This is useful for interactive programs that read from
//! stdin while waiting for other events to occur.
//!
//! # Examples
//!
//! ```no_run
//! use async_stdin::recv_from_stdin;
//!
//! #[tokio::main]
//! async fn main() {
//!    let mut rx = recv_from_stdin(10);
//!    while let Some(s) = rx.recv().await {
//!       println!("Received: {}", s);
//!   }
//! }
//! ```
use std::io::{stdin, BufRead, BufReader};
use tokio::sync::mpsc;

/// Returns a [`mpsc::Receiver`] that contains the input from [`stdin`]
///
/// This is accomplished by spawning a thread which continuously
/// blocks on reading from [`stdin`] and sends input via [`mpsc::Sender`]
///
/// # Examples
///
/// ```no_run
/// use async_stdin::recv_from_stdin;
///
/// #[tokio::main]
/// async fn main() {
///    let mut rx = recv_from_stdin(10);
///    while let Some(s) = rx.recv().await {
///       println!("Received: {}", s);
///   }
/// }
/// ```
pub fn recv_from_stdin(buffer_size: usize) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>(buffer_size);
    let stdin = BufReader::new(stdin());
    std::thread::spawn(move || read_loop(stdin, tx));
    rx
}

fn read_loop<R>(reader: R, tx: mpsc::Sender<String>)
where
    R: BufRead,
{
    let mut lines = reader.lines();
    loop {
        if let Some(Ok(line)) = lines.next() {
            let _ = tx.blocking_send(line);
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_blocking_read() {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);

        // Send input to
        let reader = std::io::BufReader::new("hello".as_bytes());
        std::thread::spawn(move || super::read_loop(reader, tx));

        let s = rx.recv().await.unwrap();
        assert_eq!(s, "hello");
    }
}
