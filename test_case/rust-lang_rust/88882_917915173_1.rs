rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub trait TraitWithAssocConst {
    const SIZE: usize;
}

impl TraitWithAssocConst for () {
    const SIZE: usize = 8;
}

pub fn dummy<T>(_: T)
where
    T: TraitWithAssocConst,
    [(); T::SIZE]: ,
{
}
