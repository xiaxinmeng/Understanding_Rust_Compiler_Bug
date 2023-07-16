 rust
use std::sync::mpsc::{channel, SendError};

fn main() {
    let (tx, _) = channel();

    let _ = tx.send(123);
    assert_eq!(tx.send(123), Err(SendError(123)));
}
