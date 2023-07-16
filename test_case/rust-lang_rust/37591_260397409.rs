 rust
pub trait X {}

impl X {
    fn f() -> u32 {
        10
    }
}

fn main() {
    let a = X::f();
}
