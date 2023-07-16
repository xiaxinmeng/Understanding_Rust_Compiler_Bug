Rust
use std::cell::Cell;

// no destructor
struct S<'a>(Cell<Option<&'a S<'a>>>);
fn main() {
    #[cfg(break)] let _d = Box::new(0);
    let x = S(Cell::new(None));
    let y = S(Cell::new(None));
    x.0.set(Some(&y));
    y.0.set(Some(&x));
    panic!()
}
