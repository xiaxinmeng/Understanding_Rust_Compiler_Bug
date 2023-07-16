 rust
#[phase(syntax)] // don't link to this crate, just get some macros
extern crate probe;

fn main() {
    probe!(...)
}
