 rust
use std::cell::RefCell;

enum Wrap<A> {
    WrapSome(A),
    WrapNone
}

use Wrap::*;

struct T;
struct U;

trait Get<T: ?Sized> {
    fn get(&self) -> &T;
}

impl Get<MyShow + 'static> for Wrap<T> {
    fn get(&self) -> &(MyShow + 'static) {
        static x: usize = 42;
        &x
    }
}

impl Get<usize> for Wrap<U> {
    fn get(&self) -> &usize {
        static x: usize = 55;
        &x
    }
}

trait MyShow { fn dummy(&self) { } }
impl<'a> MyShow for &'a (MyShow + 'a) { }
impl MyShow for usize { }
fn constrain<'a>(rc: RefCell<&'a (MyShow + 'a)>) { }

fn main() {
    let mut collection: Wrap<_> = WrapNone;

    {
        let __arg0 = Get::get(&collection);
        let __args_cell = RefCell::new(__arg0);
        constrain(__args_cell);
    }
    collection = WrapSome(T);
}fn constrain<'a>(rc: RefCell<&'a (MyShow + 'a)>) { }

fn main() {
    let mut collection: Wrap<_> = WrapNone;

    {
        let __arg0 = Get::get(&collection);
        let __args_cell = RefCell::new(__arg0);
        constrain(__args_cell);
    }
    collection = WrapSome(T);
}
