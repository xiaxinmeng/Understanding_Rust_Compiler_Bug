rust
trait Child: ~const Parent {
    #[default_method_body_is_const]
    fn childing(self) -> Self {
        self.parenting()
    }
}
