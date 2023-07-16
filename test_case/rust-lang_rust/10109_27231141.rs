 rust
match term_signal {
  0 => match exit_status {
    n if n < 0 => ExitExecError(UvError(n)),
    n => ExitStatus(n),
  },
  n => ExitSignal(n),
}
