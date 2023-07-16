 Rust
pub trait Data { fn doit(&self) {} }
impl<T> Data for T { }
pub trait UnaryLogic { type D: Data; }

pub fn ice<T: UnaryLogic>(t: T::D) {
    t.doit();
}

fn main() {}
