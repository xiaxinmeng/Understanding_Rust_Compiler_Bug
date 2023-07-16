rust
trait Child: Parent {
    #[default_method_body_is_const]
    fn childing(self) -> Self where Self: ~const Parent {
        self.parenting()
    }
}
