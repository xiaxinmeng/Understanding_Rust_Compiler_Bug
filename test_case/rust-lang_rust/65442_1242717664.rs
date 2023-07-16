rust
#![feature(type_alias_impl_trait)]

type Ty = impl Fn();

fn run<E>(_: E) -> Ty {
    || ()
}
