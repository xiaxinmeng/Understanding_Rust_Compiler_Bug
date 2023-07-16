rust
use core::marker::PhantomData;

// API:

pub struct Rc<Dyn: ?Sized + Trace<Dyn>>{
    ptr_and_stuff: PhantomData<Dyn>,
}

pub trait Trace<Dyn: ?Sized> {
    fn trace(&self, tracer: fn(&Rc<Dyn>));
}

// user code:

trait MyTrait: Trace<dyn MyTrait>{}

struct MyStruct{
    field: Rc<dyn MyTrait>,
}

impl Trace<dyn MyTrait> for MyStruct {
    fn trace(&self, tracer: fn(&Rc<dyn MyTrait>)){
        tracer(&self.field);
    }
}
