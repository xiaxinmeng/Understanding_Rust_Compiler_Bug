rust
#![feature(optin_builtin_traits)]

auto trait NotSame { }
impl<A> !NotSame for (A, A) { }

fn f<T, U>(_: T, _: U) where (T, U): NotSame {}

struct S;
struct Z;

fn main() {
    f(S, Z); // error: the trait `NotSame` is not implemented for the type `(_, _)`
    f(S, S); // error: the trait `NotSame` is not implemented for the type `(_, _)`
}
