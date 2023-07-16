 rust
#![feature(core)]

use std::cell::{RefCell, Ref, map_ref};

struct Foo {
    destroyed: bool
}

impl Drop for Foo {
    fn drop(&mut self) {
        self.destroyed = true;
    }
}

fn main() {
    let a = RefCell::new(Box::new(Foo { destroyed: false }));
    let mut b = None;
    let _: Option<Ref<()>> = map_ref(a.borrow(), |s| {
        b = Some(&**s);
        None
    });
    println!("{}", b.unwrap().destroyed);
    *a.borrow_mut() = Box::new(Foo { destroyed: false });
    println!("{}", b.unwrap().destroyed);  // Use after free
}
