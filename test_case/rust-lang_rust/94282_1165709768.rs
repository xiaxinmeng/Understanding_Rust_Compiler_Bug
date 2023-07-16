rust
#![feature(generic_const_exprs)]

pub struct If<const CONDITION: bool>;
pub trait True {}
impl True for If<true> {}

pub struct Const<const FRAC: u32>;

impl<const VALUE: u32> PartialEq<Const<VALUE>> for u32
where
    If<{ VALUE == 8 }>: True,
{
    fn eq(&self, _rhs: &FixedI8<{ VALUE }>) -> bool {
        true
    }
}
