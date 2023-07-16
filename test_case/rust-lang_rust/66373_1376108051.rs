rs
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhantomNonExhaustive<PD = ::core::marker::PhantomData<u8>>(
    pub(crate) PD,
);
