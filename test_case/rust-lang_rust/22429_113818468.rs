 rust
use std::rt::backtrace;
use std::io::stdout;

fn main() {
    backtrace::write(&mut stdout());
}
