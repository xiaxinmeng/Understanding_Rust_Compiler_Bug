rust
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

type Foo = impl Copy;

enum Wrapper<T> {
    First(T),
    Second
}

fn produce() -> Wrapper<Foo> {
    Wrapper::Second
}
