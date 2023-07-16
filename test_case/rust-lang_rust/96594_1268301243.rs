rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub enum SomeByte<const VALUE: isize>
where
    [(); VALUE as usize]:,
{
    Value = VALUE,
}
