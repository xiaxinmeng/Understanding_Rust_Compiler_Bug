
struct VecA { x: u32, y: u32 }
impl VecA {
    fn new() -> Self {
        Self { x: 0, y: 0 } // OK
    }
}
struct VecB(u32, u32);
impl VecB {
    fn new() -> Self {
        Self(0, 0) // Error
    }
}
fn main() {}
