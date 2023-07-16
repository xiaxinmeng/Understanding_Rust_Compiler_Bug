rust
#![feature(type_alias_impl_trait)]

type A = impl Drop;

struct S {
    a: A,
}

fn foo() -> S {
    S { a: Box::new(1) }
}
