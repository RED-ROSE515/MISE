use color_eyre::eyre::{Context, Result};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run_with_timeout<F, T>(f: F, timeout: Duration) -> Result<T>
where
    F: FnOnce() -> Result<T> + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let result = f();
        tx.send(result).unwrap();
    });
    rx.recv_timeout(timeout).context("timed out")?
}
