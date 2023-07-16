 rust
trait Idx {
    type Data;
    const INVALID: Self;
    fn idx(&self) -> usize;
}
