rust
#![feature(generators)]

struct Foo<T: Fn()>(T, T::Output);

fn ice<'a>() -> impl Send + 'a {
    Foo(|| {}, ())
}

fn main() {
    || {
        let _f = ice();
        yield;
    };
}
