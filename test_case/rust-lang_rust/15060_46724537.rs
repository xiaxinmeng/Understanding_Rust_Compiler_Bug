 rust
trait T {}
impl<U: T> Vec<U> {
    fn from_slice<'a>(x: &'a [uint]) -> Vec<uint> {
        fail!()
    }
}
fn main() { let r = Vec::from_slice(&[1u]); }
