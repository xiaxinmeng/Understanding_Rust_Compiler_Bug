rust
// echo.rs
use std::io::{self, Write};
use std::env;

fn main() -> io::Result<()> {
    let arg = env::args().nth(1).unwrap_or_default();
    writeln!(io::stdout(), "{}", arg)?;
    Ok(())
}
