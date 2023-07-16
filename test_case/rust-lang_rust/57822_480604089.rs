rust
extern crate rexpect;
use rexpect::session::spawn_command;
use std::process::Command;
use tempfile::*;

type R<T> = Result<T, Box<std::error::Error>>;
type Session = rexpect::session::PtySession;


#[test]
fn test() -> R<()> {
    let mut p = run(&["example.txt"])?;
    p.exp_regex("Deleting example.txt")?;
    p.exp_eof()?;
    Ok(())
}


fn run(args: &[&str]) -> R<Session> {
    let directory = tempdir()?;
    let directory_path = directory.path();
    let mut command = Command::new("./target/debug/rust-rm");
    command.args(args);
    command.current_dir(directory_path);
    println!("Tmp folder: {:?}", directory_path);
    let mut p = spawn_command(command, Some(2000))?;
    Ok(p)
}
