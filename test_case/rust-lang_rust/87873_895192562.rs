rust
// main.rs
use std::io::{self, Write};
use std::{thread, time};

fn main() -> io::Result<()> {
    thread::sleep(time::Duration::from_millis(10));
    io::stdout().write(b"success?\n")?;
    Ok(())
}
