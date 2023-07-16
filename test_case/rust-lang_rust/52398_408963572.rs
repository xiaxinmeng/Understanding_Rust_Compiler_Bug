rust
#![feature(generators, generator_trait)]
use std::ops::Generator;
use std::cell::RefCell;

fn foo<'a>(y: &'a RefCell<u32>) -> impl Generator + 'a {
    return move || {
        let a = y.borrow();
        yield *a;
        return "Done";
    };
}

fn main() {
    let y = 10;
    let y_cell = RefCell::new(y);
    foo(&y_cell);
}
