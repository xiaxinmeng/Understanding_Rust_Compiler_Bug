 rust
trait MagicEnumTrait {
    type Repr;
    fn value(&self) -> Self::Repr;
}
