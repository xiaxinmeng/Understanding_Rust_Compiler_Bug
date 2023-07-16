rust
#![feature(rustc_attrs)]

#[rustc_must_implement_one_of(eq, neq)]
trait Equal {
    fn eq(&self, other: &Self) -> bool { !self.neq(other) }
    fn neq(&self, other: &Self) -> bool { !self.eq(other) }
}

struct S(u32);

impl Equal for S {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
    fn neq(&self, other: &Self) -> bool { self.0 != other.0 }
}

fn main() {}
