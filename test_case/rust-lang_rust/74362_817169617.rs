rust 
#![feature(specialization)]

use std::marker::PhantomData;

struct IsSameType<T0: 'static, T1: 'static>(PhantomData<T0>, PhantomData<T1>);

trait IsSameTypeTrait{
    const IS_SAME: bool;
}

impl<T0: 'static, T1: 'static> IsSameTypeTrait for IsSameType<T0, T1>{
    default const IS_SAME: bool = false;
}

impl<T> IsSameTypeTrait for IsSameType<T, T>{
    const IS_SAME: bool = true;
}
