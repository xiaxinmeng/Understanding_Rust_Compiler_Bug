
use std::marker::PhantomData;

pub trait Precision {}

pub struct Satoshi;
impl Precision for Satoshi {}

impl<T:Precision> Precision for PhantomData<T>{}

type Inner = i64;

pub type Ammount<P=Satoshi>
    __Ammount<PhantomData<P>>;

#[derive(Copy, Clone)]
pub struct __Amount<P: Precision>(
    Inner,
    PhantomData<P>
);
