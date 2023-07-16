 rust
use std::sync::mpsc::channel;

fn main() {
    // compiles
    channel::<Box<u8>>();
    // ICEs
    channel::<Box<_>>();
}
