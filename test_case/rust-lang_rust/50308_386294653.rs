rust
use std::marker::PhantomData;

pub trait AsIndex: Copy {

    const MAX: usize;

    fn cast_to(self) -> usize;

    fn cast_from(idx: usize) -> Self;

}

impl AsIndex for usize {

    const MAX: usize = !0usize >> 1;

    fn cast_to(self) -> usize {self}

    fn cast_from(idx: usize) -> usize {idx}

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BiMap<L, R>
where
    L: AsIndex,
    R: AsIndex,
{
    left: [Option<usize>; <L as AsIndex>::MAX + 1],
    right: [Option<usize>; <R as AsIndex>::MAX + 1],
    _marker: PhantomData<(L, R)>,
}
