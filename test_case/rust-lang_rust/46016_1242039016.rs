rust
// These macros are needed because the normal ones panic when there's a broken pipe.
// This is especially problematic for CLI tools that are frequently piped into `head` or `grep -q`
macro_rules! println {
  () => (print!("\n"));
  ($fmt:expr) => ({
    writeln!(std::io::stdout(), $fmt)
  });
  ($fmt:expr, $($arg:tt)*) => ({
    writeln!(std::io::stdout(), $fmt, $($arg)*)
  })
}

macro_rules! print {
  () => (print!("\n"));
  ($fmt:expr) => ({
    write!(std::io::stdout(), $fmt)
  });
  ($fmt:expr, $($arg:tt)*) => ({
    write!(std::io::stdout(), $fmt, $($arg)*)
  })
}
