 rust
trait T {
  fn t<'a>(&'a self) -> SomeStruct<'a> { SomeStruct { t: self._self_ref_hack() } }
  fn _self_ref_hack(&self) -> &T;
}
