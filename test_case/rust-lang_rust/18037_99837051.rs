
#![feature(rustc_private)]

extern crate arena;

use arena::TypedArena;

fn main() {
    let arena = TypedArena::new();
    arena.alloc(());
}

