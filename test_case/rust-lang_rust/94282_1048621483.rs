rust
#![feature(generic_const_exprs)]

pub struct If<const CONDITION: bool>;
pub trait True {}
impl True for If<true> {}

pub struct FixedI8<const FRAC: u32> {
    pub bits: i8,
}

impl<const FRAC: u32> PartialEq<FixedI8<FRAC>> for u32
where
    If<{ FRAC <= 8 }>: True,
{
    fn eq(&self, _rhs: &FixedI8<FRAC>) -> bool {
        unimplemented!()
    }
}
