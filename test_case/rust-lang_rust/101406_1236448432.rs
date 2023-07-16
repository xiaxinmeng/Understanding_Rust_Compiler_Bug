rust
trait Bar<'a> {
    type Ctx;

    fn run(
        &mut self,
        ctx: &Self::Ctx,
    );
}

struct Foo<F>(F);

impl<'a, F> Bar<'a> for Foo<F>
    where F: FnMut(&<Self as Bar<'_>>::Ctx)
{
    type Ctx = ();

    fn run(
        &mut self,
        ctx: &Self::Ctx,
    ) {
        (self.0)(ctx)
    }
}
