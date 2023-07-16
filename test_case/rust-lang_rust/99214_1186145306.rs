rs
#![allow(unused)]
use std::io::{self, Stdout, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    // Note that `output.stdout` is a slice of bytes
    assert_eq!(&output.stdout, b"hello\n");

    // It can be written to stdout as a string either via stdout.write_all:
    let mut stdout = io::stdout().lock();
    stdout.write_all(&output.stdout)?;

    // Or via a lossy conversion:
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\n");

    Ok(())
}
