
trait ActuallyAFn {
  type Param;
  type Return;
  fn call(&self, param: Self::Param) -> Return;
}
