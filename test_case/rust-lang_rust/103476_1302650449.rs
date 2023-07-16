rust
#![feature(let_chains)]
#![allow(irrefutable_let_patterns)]

struct B<'a>(&'a ());

impl<'a> std::ops::Drop for B<'a> {
    fn drop(&mut self) {}
}

fn f() {
    if let n = () && let _ = B(&n) {};
}
