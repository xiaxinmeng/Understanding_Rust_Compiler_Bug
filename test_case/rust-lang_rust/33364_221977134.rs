 rust
use std::marker::PhantomData;

trait Foo<'a> {
    type Item;
    fn consume<F>(self, f: F) where F: Fn(Self::Item);
}
struct Consume<A>(PhantomData<A>);

impl<'a, A:'a> Foo<'a> for Consume<A> {
    type Item = &'a A;

    fn consume<F>(self, _f: F) where F: Fn(Self::Item) -> Self::Item {
        if blackbox() {
            _f(any());
        }
    }
}

#[derive(Clone)]
struct Wrap<T> { foo: T }

impl<T: for <'a> Foo<'a>> Wrap<T> {
    fn consume<F>(self, f: F) where F: for <'b> Fn(<T as Foo<'b>>::Item) -> <T as Foo<'b>>::Item {
        self.foo.consume(f);
    }
}

fn main() {
    // This does not (but is only noticed if you call the closure).
    let _wrap = Wrap { foo: Consume(PhantomData::<u32>,) };
    _wrap.consume(|xxxxx| xxxxx);
}

pub static mut FLAG: bool = false;
fn blackbox() -> bool { unsafe { FLAG } }
fn any<T>() -> T { loop { } }
