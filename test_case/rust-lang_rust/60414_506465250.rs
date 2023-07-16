rust
#![feature(existential_type)]

trait Implemented {
    type Assoc;
}
impl<T> Implemented for T {
    type Assoc = u8;
}

trait Trait {
    type Out;
}

impl Trait for () {
    type Out = u8;
}

existential type Ex: Trait<Out = <() as Implemented>::Assoc>;

fn define() -> Ex {
    ()
}
