
trait Thing {
    fn f<'a>(self, ctx: &'a Ctx) -> Self<'a>;
}

// Desugars to something like:
impl<'x> Thing for A<'x> {
  type Assoc<'a> = A<'a>;
  fn f<'a>(self, ctx: &'a Ctx) -> Self::Assoc<'a> { .. }
}
