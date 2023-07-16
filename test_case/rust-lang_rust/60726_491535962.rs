
use std::marker::PhantomData;

pub struct True;
pub struct False;

pub trait InterfaceType{
    type Send;
}


pub struct FooInterface<T>(PhantomData<fn()->T>);

impl<T> InterfaceType for FooInterface<T> {
    type Send=False;
}


pub struct DynTrait<I>{
    _interface:PhantomData<fn()->I>,
    _unsync_unsend:PhantomData<::std::rc::Rc<()>>,
}

unsafe impl<I> Send for DynTrait<I>
where
    I:InterfaceType<Send=True>
{}

pub struct IntoIter<T>{
    hello:DynTrait<FooInterface<T>>,
}
