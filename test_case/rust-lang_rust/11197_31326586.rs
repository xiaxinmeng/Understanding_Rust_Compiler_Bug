 rust
use std::io;

fn foo(mut a: ~Writer) {
    a.write(bytes!("Hello\n"));
}

fn main() {
    foo(~io::stdout());
}
