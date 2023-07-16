rust
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};

/// Runs POSIX shell command `shell` in a context with its stdout closed (probably; technically
/// racy), and reports on the exit status's code or signal.
fn check(shell: &str) -> std::io::Result<()> {
    let mut child = Command::new("sh")
        .args(&["-c", "sleep 0.1; exec sh -c \"$@\"", ":", shell])
        .stdout(Stdio::piped())
        .spawn()?;
    drop(child.stdout.take().unwrap());
    let status = child.wait()?;
    println!(
        "{:?} exited with code {:?}; signal: {:?}",
        shell,
        status.code(),
        status.signal(),
    );
    Ok(())
}

fn main() -> std::io::Result<()> {
    check("exit 141")?;
    check("echo sigpipe")?;
    check("kill -13 $$")?;
    Ok(())
}
