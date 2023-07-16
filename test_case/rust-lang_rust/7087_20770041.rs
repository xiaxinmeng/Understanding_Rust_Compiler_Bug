 rust
trait MoveConvert<T> {
  fn be(self)->T;
}
trait RefConvert<T> {
  fn as(&self)->&T;
}
trait CopyConvert<T> {
  fn to(&self)->T;
}
