
fn selector<'a>(ports: &'a [&Port]) -> Selector<'a> { ... }
impl Selector {
  fn select(&mut self) -> int;
}
