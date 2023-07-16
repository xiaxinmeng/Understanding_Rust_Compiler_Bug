
#![no_std]

#![feature(generic_associated_types)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(const_fn)]

#![allow(dead_code)]
#![allow(incomplete_features)]

use core::marker::PhantomData;
use core::mem::{size_of, align_of};

trait BoolConst {
    const VALUE : bool;
}

struct AlwaysTrueAsserter {}

impl BoolConst for AlwaysTrueAsserter {
    const VALUE : bool = true;
}

struct SizeAsserter<L,R>(PhantomData<(L,R)>);

impl<S,T> BoolConst for SizeAsserter<S, T> {
    const VALUE : bool = size_of::<S>() >= size_of::<T>() && 
                         align_of::<S>() % align_of::<T>() == 0;
}

trait Storage {
    type Asserter<T> : BoolConst;
}

struct HeapStorage {}

impl Storage for HeapStorage {
    type Asserter<T> = AlwaysTrueAsserter;
}

struct RawBox<T, S: Storage>(PhantomData<(T,S)>);

const fn size_assertion<S : Storage, T>() -> usize {
    S::Asserter::<T>::VALUE as usize - 1
}

impl<T, S : Storage> RawBox<T, S> {

    fn create(_val : T) -> Self where
        [u8 ; size_assertion::<S, T>()] : Sized
    {
        Self(PhantomData)
    }

}

fn create<T>(val : T) -> RawBox::<T, HeapStorage> 
    // this here is my issue
    where  [u8 ; size_assertion::<HeapStorage, T>()] : Sized
{
    RawBox::<T, HeapStorage>::create(val)
}
