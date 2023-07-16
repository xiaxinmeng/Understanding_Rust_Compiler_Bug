 rust
#![feature(rustc_private)]
extern crate arena;

use std::cell::Cell;

struct Foo<'a> {
    data: Box<u32>,
    other: Cell<Option<&'a Foo<'a>>>
}

impl<'a> Drop for Foo<'a> {
    fn drop(&mut self) {
        let other = self.other.get().unwrap();
        println!("Accessing other item’s data = {}", *other.data);  // Use after free?
        println!("Dropping with data = {}", *self.data);
    }
}

fn main() {
    let arena = arena::TypedArena::new();
    // Create a reference cycle within the arena:
    let a = arena.alloc(Foo {
        data: Box::new(42),
        other: Cell::new(None),
    });
    let b = arena.alloc(Foo {
        data: Box::new(6),
        other: Cell::new(None)
    });
    a.other.set(Some(b));
    b.other.set(Some(a));
}
