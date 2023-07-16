rust
type Factory {
    type Item = impl Debug;
    fn produce(&mut self) -> Self::Item { () }
}
