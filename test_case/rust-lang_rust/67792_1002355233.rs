rust
trait Parent {
    fn parenting(self) -> Self;
}
trait Child: Parent {
    fn childing(self) -> Self {
        self.parenting()
    }
}
