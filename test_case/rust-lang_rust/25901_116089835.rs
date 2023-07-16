 rust
struct A;
struct B;
static C: B = B;

use std::ops::Deref;

impl Deref for A {
    type Target = B;
    fn deref(&self)->&B { &C }
}

impl B {
    fn foo(&self){}
}

fn main(){
    A.foo()
}
