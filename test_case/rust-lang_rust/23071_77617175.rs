
trait Drain<Input> {
  type Drain<'*>; // HKT
  fn drain<'a>(&'a mut self, input: Input) -> Drain<'a>;
}
