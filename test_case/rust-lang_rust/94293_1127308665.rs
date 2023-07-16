rust
#![feature(generic_const_exprs)]

trait Foo<const N: usize>
where
    [(); check_empty_array_size(N)]: Sized,
{
    type Narr: Foo<0>;
}

#[allow(unused_variables)]
pub const fn check_empty_array_size(array_size: usize) -> usize {
    0
}
