 rust
#![feature(unboxed_closures)]

use std::cell::UnsafeCell;
use std::sync::Arc;

pub trait Foo<M: Send> : Send {
    fn foo(&mut self, msg: M);
}

impl<M: Send, F: Send + FnMut(M)> Foo<M> for F {
    fn foo(&mut self, msg: M) {
        self.call_mut((msg,));
    }
}

pub struct Both<M, F> {
    inner: Arc<UnsafeCell<(M, F)>>,
}

impl<M: Send, F: Foo<M>> Clone for Both<M, F> {
    fn clone(&self) -> Both<M, F> {
        Both { inner: self.inner.clone() }
    }
}

fn repro1<M: Send, F: Foo<M>>(_both: Both<M, F>) {
}

fn repro2<M: Send, F: Foo<M>>(msg: M, foo: F) {
    let both = Both { inner: Arc::new(UnsafeCell::new((msg, foo))) };
    repro1(both.clone()); // <--- This clone causes problem
}

pub fn main() {
    println!("hello");
}
