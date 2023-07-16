rust
#![feature(existential_type)]

existential type Foo<R>: Fn(&R) -> ();

fn bar<R>() -> Foo<R> {
    |r| ()
}

