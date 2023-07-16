 Rust
pub enum Enum<T> {
    A(T),
}

pub trait X {}
impl X for int {}

pub struct Z<'self>(Enum<&'self X>);
pub fn main() { let x = 42; let z = Z(A(&x as &X)); let _ = z; }
