rust
trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    #[default_method_body_is_const]
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other) // call to self.eq is allowed in const context because `eq` has a const implementation
    }
}
