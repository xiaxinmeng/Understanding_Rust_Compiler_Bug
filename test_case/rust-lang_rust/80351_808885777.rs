rust
pub fn main() {
    f::<(), _>(|_| {});
}
fn f<S, P>(p: P)
where
    P: for<'r> FnOnce(<S as Trait<'r>>::Item),
    S: for<'r> Trait<'r>,
{
    p(None.unwrap())
}

pub trait Trait<'a> {
    type Item;
}
impl<'a, S> Trait<'a> for S {
    type Item = ();
}
