
trait T {}
impl<U: T> (Vec<U>) {}
fn main() { let r = Vec::from_slice(&[1u]); }
