rust
#[non_exhaustive]
enum Foo {
    FooVariant1,
    FooVariant2(Bar),
}

#[non_exhaustive]
enum Bar {
    BarVariant1,
    BarVariant2,
}

// ...

match foo {
    Foo::FooVariant1 => ...,
    Foo::FooVariant2(Bar::BarVariant1) => ...
    Foo::FooVariant2(Bar::BarVariant2) => ...
    Foo::FooVariant2(_) => ...
    _ => ...
}    
