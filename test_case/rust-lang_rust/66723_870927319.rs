rust
#![feature(min_type_alias_impl_trait)]

type Foo = impl Copy;

enum Wrapper<T> {
    First(T),
    Second
}

fn produce() -> Wrapper<Foo> {
    Wrapper::First((22, Default::default()))
}
