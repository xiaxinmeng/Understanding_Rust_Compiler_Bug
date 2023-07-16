rust
pub struct Unit;
trait Obj {}

trait Bound {}
impl Bound for Unit {}

pub trait HasProj {
    type Proj;
}

impl<T> HasProj for T {
    type Proj = Unit;
}

trait HasProjFn {
    type Proj;
    fn the_fn(_: Self::Proj);
}

impl HasProjFn for Unit
where
    Box<dyn Obj>: HasProj,
    <Box<dyn Obj> as HasProj>::Proj: Bound,
{
    type Proj = Unit;
    fn the_fn(_: Self::Proj) {}
}
