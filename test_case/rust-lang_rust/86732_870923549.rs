rust
#![feature(type_alias_impl_trait)]

type Foo = impl Copy;

enum Wrapper {
    First(Foo),
    Second
}

