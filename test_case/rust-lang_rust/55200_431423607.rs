rust
use std::process::{Command, Stdio};

let reverse = Command::new("rev")
    .arg("non_existing_file.txt")
    .stderr(Stdio::piped())
    .spawn()
    .expect("failed reverse command");

let cat = Command::new("cat")
    .arg("-")
    .stdin(reverse.stderr.unwrap()) // Converted into a Stdio here
    .output()
    .expect("failed echo command");

assert_eq!(
    String::from_utf8_lossy(&cat.stdout),
    "rev: cannot open non_existing_file.txt: No such file or directory\n"
);
