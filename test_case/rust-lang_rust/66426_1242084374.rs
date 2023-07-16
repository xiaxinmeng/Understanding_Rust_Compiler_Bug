rust
#![feature(type_alias_impl_trait)]

type Foo = impl Copy;

enum Wrapper<T> {
    First(T),
    Second
}

fn produce() -> Wrapper<Foo> {
    Wrapper::Second
}
