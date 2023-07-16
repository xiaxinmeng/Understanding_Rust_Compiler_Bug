 rust
/// "Callbacks" for a push-based parser
trait Sink {
    fn handle_foo(&mut self, ...);
    // ...

    type Output = Self;
    fn finish(self) -> Self::Output { self }
}
