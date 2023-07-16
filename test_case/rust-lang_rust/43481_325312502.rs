rust
use std::cell::Cell;

struct S<'a> {
    r: Cell<Option<&'a S<'a>>>,
}

fn main() {
    let x = S { r: Cell::new(None) };
    x.r.set(Some(&x));
}
