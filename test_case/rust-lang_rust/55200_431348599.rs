rust
use std::process::{Command, Stdio};
    
let hello = Command::new("echo")
    .arg("Hello, world!")
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed echo command");

let reverse = Command::new("rev")
    .stdin(Stdio::from(hello.stdout.unwrap()))  // convert the standard input into a Stdio
    .output()
    .expect("failed reverse command");

assert_eq!(reverse.stdout, b"!dlrow ,olleH\n");
