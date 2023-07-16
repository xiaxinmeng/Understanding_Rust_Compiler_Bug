rust
use std::marker::PhantomData;
trait Foo<T: ?Sized> {
    fn fun(f: &dyn Fn(PhantomData<&Self>) -> &Self) -> &dyn Fn(PhantomData<&T>) -> &T;
}

impl<T: ?Sized> Foo<T> for T {
    fn fun(f: &dyn Fn(PhantomData<&Self>) -> &Self) -> &dyn Fn(PhantomData<&T>) -> &T {
        f
    }
}

fn extend_lifetime<'a, T: ?Sized>(x: &T) -> &'a T {
    Foo::fun(&|_| impl_foo(x))(PhantomData)
}

fn impl_foo<T: ?Sized>(x: &T) -> &(impl ?Sized + Foo<T> + '_) {
    x
}

fn main() {
    let r;
    {
        let x = String::from("Hello World?");
        r = extend_lifetime(&x);
    }
    println!("{}", r);
}
