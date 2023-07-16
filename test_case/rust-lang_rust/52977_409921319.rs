Rust
#![feature(existential_type)]

pub trait Bar<T> {
    type Item;
}

existential type Foo: Bar<Foo, Item = Foo>;

fn crash(x: Foo) -> Foo {
    x
}

fn main() {
    
}
