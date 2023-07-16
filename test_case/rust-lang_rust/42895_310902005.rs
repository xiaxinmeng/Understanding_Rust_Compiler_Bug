rust
#![feature(decl_macro)]
#![allow(unused)]

trait X<A> {}
struct Y<A, B>(A, B);
struct Z<A, B>(A, B);

macro x($S:ident, $A:ident) {
    impl<$A, B> X<B> for $S<$A, B> {}
}

x!(Y, A); // works
x!(Z, B); // works

fn main() {}
