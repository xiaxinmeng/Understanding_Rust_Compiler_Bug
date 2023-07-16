 rust
trait T {
    fn t(&self) -> &T {
        &self
    }
}
