rust
#![feature(const_mut_refs)]
use core::marker::PhantomData;


struct DemandSet<D> {
    // const-set to store a typeid -> closure map goes here
    demanded: PhantomData<D>
}

impl<D: Default> DemandSet<D> {
    
    const fn new() -> Self {
        DemandSet {
            demanded: PhantomData
        }
    }
    
    const fn add_demand<T>(&mut self, store: fn(&mut D, T)) {
        // TODO
    }
}

struct Foo {}

struct FulfilledDemands {
    foo: Option<Foo>
}

impl Default for FulfilledDemands {
    fn default() -> Self { FulfilledDemands {foo: None} }
}

const MY_DEMANDS: DemandSet<FulfilledDemands> = {
    let mut d = DemandSet::<FulfilledDemands>::new();
    d.add_demand::<Foo>(|to_fulfill, foo| to_fulfill.foo = Some(foo));
    d
};
