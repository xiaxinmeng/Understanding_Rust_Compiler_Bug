rust
use std::fmt::{self, Debug};

trait Trait {
    type A: Debug;
}

struct S1 {}

impl Trait for S1 {
    type A = u64;
}

struct S2<T: Trait> {
    a: T::A,
}

impl<T: Trait> Debug for S2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let s = S2::<S1> { a: 10 };
    format!("{:?}", s);
}
