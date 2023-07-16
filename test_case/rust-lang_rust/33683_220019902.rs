 rust
macro_rules! paren {
    ($e:expr) => (($e))
}

mod m {
    pub struct S {
        x: i32
    }
    pub fn make() -> S {
        S { x: 0 }
    }
}

fn main() {
    let s = m::make();
    paren!(s.x);
}
