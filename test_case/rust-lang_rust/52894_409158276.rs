rust
#![feature(specialization)]
#![allow(dead_code)]

use std::fmt::Debug;

trait MaybeDebug {
    fn maybe_debug(&self);
}

impl<T> MaybeDebug for T {
    default fn maybe_debug(&self) {
        println!("Initialized Foo with unprintable value");
    }
}

impl<T: Debug> MaybeDebug for T {
    fn maybe_debug(&self) {
        println!("Initialized Foo with value: {:?}", self);
    }
}

struct Foo<T> {
    t: T
}

impl<T> Foo<T> {
    fn new(t: T) -> Foo<T> {
        // (... schtuff ...)
        t.maybe_debug();
        Foo {
            t
        }
    }
}

struct NoDebug;

fn main() {
    Foo::new(());
    Foo::new(0);
    Foo::new(NoDebug);
}
