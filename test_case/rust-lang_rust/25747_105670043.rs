 rust
#![feature(core)]
use std::cell::*;

// So that I don’t have to re-bootrap...
fn simplified_map_ref<'b, T, U, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
where F: FnOnce(&T) -> &U {
    map_ref(orig, move |v| Some(f(v))).unwrap()
}

fn main() {
    let x: RefCell<Result<u32, ()>> = RefCell::new(Ok(5));
    simplified_map_ref(x.borrow(), |r| &r.ok());
}
