rust
#![feature(destructuring_assignment)]
#![allow(unused_imports, unused_assignments)]

use std::mem::swap;

fn foo1() {
    let a = &mut [0_u32; 10];
    let b = &mut [0_u32; 10];
    swap(a, b); // OK
    (a, b) = (b, a); // cannot assign twice to immutable variable error
}

fn foo2(va: &mut Vec<u32>, vb: &mut Vec<u32>) {
    swap(vb, va); // OK
    (vb, va) = (va, vb); // Lifetime mismatch error
}

struct Bar {
    vb: Vec<u32>,
}

impl Bar {
    fn spam(&mut self) {
        let mut va = vec![0_u32; 10];
        swap(&mut self.vb, &mut va); // OK
        (self.vb, va) = (va, self.vb); // Error cannot move out of `self.vb` which is behind a mutable reference
    }
}

fn main() {}
