 rust
use std::ascii::StrAsciiExt;

static NAME: &'static str = "hello world";

#[cfg(not(with_local))]
fn main() {
    match NAME.to_ascii_lower().as_slice() {
        "foo" => {}
        _ => {}
    }
}
#[cfg(with_local)]
fn main() {
    let name = NAME.to_ascii_lower();
    match name.as_slice() {
        "foo" => {}
        _ => {}
    }
}
