rust
// lib.rs
#![feature(const_generics)]

pub struct Num<const N: usize>;

impl<const N: usize> Num<N> {
    pub fn foo(&self) {
        println!("any N");
    }

    pub fn new() -> Self {
        Self
    }
}

// Calling this method in separate crate now works 
// (verified that it crashes in earlier nightlies)
impl Num<3> {
    pub fn three(&self) {
        println!("I'm a 3");
    }
}

// Braces around const expression causes crash
impl Num<{4}> {
    pub fn four(&self) {
        println!("I'm a 4");
    }
}

// Another impl this time without the braces is fine
impl Num<4> {
    pub fn also_four(&self) {
        println!("still a 4");
    }
}

const FIVE: usize = 5;

// Using a const even without braces crashes
impl Num<FIVE> {
    pub fn five(&self) {
        println!("I'm a 5");
    }
}

// Using a type alias with braces then an impl on that is fine
pub type Num6 = Num<{6}>;

impl Num6 {
    pub fn six(&self) {
        println!("Type-aliased 6");
    }
}
