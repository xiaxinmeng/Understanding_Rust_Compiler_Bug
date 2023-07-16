rust
trait Foo {
    fn foo();
}

impl<T> Foo for T {
    default fn foo() { }
}

fn same_type<A>(a: A, b: A) { }

fn main() {
    same_type(<u8 as Foo>::foo, <u16 as Foo>::foo);
}
