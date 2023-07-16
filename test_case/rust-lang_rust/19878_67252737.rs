 rust
#![feature(default_type_params, unboxed_closures)]

trait FnBox<A, R = ()> {
    fn call_box(self: Box<Self>, arg: A) -> R;
}

impl<A, R, F> FnBox<A, R> for F
    where F : FnOnce(A) -> R
{
    fn call_box(self: Box<F>, arg: A) -> R {
        (*self)(arg)
    }
}

struct Foo<'a>;

fn put_in_box<F>(f: F) -> Box<for<'a> FnBox<Foo<'a>, ()> + 'static> where
    F: FnOnce(Foo) + 'static
{
    box f
}

fn main() {
    put_in_box(|_| {}).call_box(Foo);
}
