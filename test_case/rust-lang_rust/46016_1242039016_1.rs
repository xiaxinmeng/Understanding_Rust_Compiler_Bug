rust
fn main() -> ExitCode {
  match run() {
    Err(Error::IOError(err)) if err.kind() == io::ErrorKind::BrokenPipe => {
      // Okay, this happens when the output is piped to a program like `head`
      ExitCode::SUCCESS
    }
    Err(err) => {
      eprintln!("{}", err).ok();
      ExitCode::FAILURE
    }
    Ok(_) => ExitCode::SUCCESS,
  }
}
