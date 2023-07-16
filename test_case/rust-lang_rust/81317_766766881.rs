rust
use core::ops::Shl;

struct A;

impl A {
    fn a(&self) {}
}

impl Shl<i8> for A {
    type Output = A;
    
    fn shl(self, _: i8) -> A {
        self
    }
}

impl Shl<i32> for A {
    type Output = A;
    
    fn shl(self, _: i32) -> A {
        self
    }
}

pub fn decrypt_portion_no_worky() {
    let iv = A << 16;
    iv.a();
}

pub fn decrypt_portion_works() {
    let iv = A << 16;
    // Calling no methods works?!
}
