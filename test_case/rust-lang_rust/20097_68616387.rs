 rust
#![allow(dead_code)]

pub trait Foo : Send {
    fn foo<'a>(me: Managed<'a, Self>);
}

pub trait BoxedFoo : Send {
    fn foo_boxed(self: Box<Self>);
}

pub trait ToFoo<F: Foo> {
    fn to_box(self) -> Box<BoxedFoo>;
}

/*
 *
 * ===== impl Foo for String =====
 *
 */

impl Foo for String {
    fn foo<'a>(me: Managed<'a, Self>) {
        println!("foo: {}", me.take());
    }
}

impl<F: Foo> BoxedFoo for F {
    fn foo_boxed(self: Box<Self>) {
        Foo::foo(Managed::heap(self));
    }
}

impl<F: Foo> ToFoo<F> for F {
    fn to_box(self) -> Box<BoxedFoo> {
        box self
    }
}

impl<'a, F: Foo> ToFoo<F> for Managed<'a, F> {
    fn to_box(self) -> Box<BoxedFoo> {
        match self.store {
            Store::Heap(boxed) => boxed,
            Store::Stack(opt) => box opt.take().unwrap(),
        }
    }
}

/*
 *
 * ===== Managed =====
 *
 */

pub struct Managed<'a, T:'a> {
    store: Store<'a, T>,
}

impl<'a, T> Managed<'a, T> {
    fn heap(val: Box<T>) -> Managed<'a, T> {
        Managed { store: Store::Heap(val) }
    }

    fn take(self) -> T {
        match self.store {
            Store::Heap(boxed) => *boxed,
            Store::Stack(opt) => opt.take().expect("option is none"),
        }
    }
}

enum Store<'a, T:'a> {
    Heap(Box<T>), // Callback is already heap allocated
    Stack(&'a mut Option<T>), // Callback is stack allocated
}

fn do_foo<T: ToFoo<F>, F: Foo>(f: T) {
    f.to_box().foo_boxed();
}

pub fn main() {
    do_foo(Managed::heap(box "foo".to_string()));
}
