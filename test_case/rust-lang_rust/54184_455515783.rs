rust
existential type Foo: Bar;

fn foo<T: Bar>(t: T) -> Foo {
    t
}
