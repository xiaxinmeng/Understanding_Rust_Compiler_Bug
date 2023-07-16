rust
pub struct True;
pub struct False;

pub trait InterfaceType {
    type Send;
}

pub struct FooInterface;
pub struct DynTrait<I>(I);
pub struct IntoIter(DynTrait<FooInterface>);

impl InterfaceType for FooInterface {
    type Send = False;
}

unsafe impl<I> Send for DynTrait<I> where I: InterfaceType<Send = True> {}
