 rust
fn set_process_stdout<W: Write + IntoRawFd>(write: W) -> io::Result<()> { ... }
fn set_thread_stdout<W: Write>(write: W) -> io::Result<()> { ... }
