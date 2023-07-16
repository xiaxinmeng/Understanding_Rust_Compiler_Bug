rust
use std::pin::Pin;

fn foo(_: &mut ()) {}

fn main() {
    let r = Pin::new(&mut ());
    foo(r.get_mut());
    foo(r.get_mut());
}
