rust
#![feature(specialization)]

struct X;
struct Y;
impl From<X> for Y {
    fn from(_: X) -> Y { Y }
}

// impl Into<Y> for X {}

fn main() {
    let x = X.into();
}
