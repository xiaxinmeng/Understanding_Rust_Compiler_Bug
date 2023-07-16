rust
pub enum MaybeOwned<'a> {
    Borrowed(&'a isize),
}

pub struct Inv<'a> {
    x: &'a mut &'a isize,
}

pub trait IntoMaybeOwned<'a> {
    fn into_maybe_owned(self) -> MaybeOwned<'a>;
}

impl<'a> IntoMaybeOwned<'a> for Inv<'a> {
    fn into_maybe_owned(self) -> MaybeOwned<'a> {
        panic!()
    }
}

