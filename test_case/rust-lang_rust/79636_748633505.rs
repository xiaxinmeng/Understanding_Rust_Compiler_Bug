rust
#![feature(generic_associated_types)]

pub trait SomeTrait {
    type Wrapped<A>: SomeTrait;

    fn f() -> ();
}

fn program<W>() -> ()
where
    W: SomeTrait<Wrapped = W>,
{
    return W::f();
}

fn main() {}

