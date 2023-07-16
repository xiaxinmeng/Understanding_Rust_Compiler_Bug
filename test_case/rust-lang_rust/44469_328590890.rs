rust
#[repr(C)]
struct Foo {
    x: u8,
    tail: Opaque, // OK
}

fn use_foo(foo: &Foo) -> &Opaque {
    &foo.tail // ERROR: Cannot access field `foo`, type `Opaque` doesn't implement `DynSized`
}
