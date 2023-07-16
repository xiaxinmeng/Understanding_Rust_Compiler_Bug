rs
use std::any::Any;

trait Lt<'a>: 'a {}
impl<'a, T: ?Sized + 'a> Lt<'a> for T {}

trait Tr {}
impl<T: ?Sized> Tr for T {}

fn test<T, R: Any>(x: T) -> R {
    // works despite non-'static
    step1::<T, R>(x)
}

// remark: starting in `test` with `step2` directly works, too.
fn step1<T, R: Any>(x: T) -> R
where
    for<'a> T: 'a,
{
    step2::<T, R>(x)
}

fn step2<T, R: Any>(x: T) -> R
where
    for<'a> T: Lt<'a>,
{
    step3::<T, R>(x)
}

fn step3<T, R: Any>(x: T) -> R
where
    T: Lt<'static>,
{
    step4::<T, R>(x)
}

fn step4<T, R: Any>(x: T) -> R
where
    T: 'static,
{
    *(Box::new(x) as Box<dyn Any>).downcast::<R>().unwrap()
}

fn main() {
    let x = String::from("Hello World");
    let r: &'static str = test(&x[..]);
    drop(x);
    println!("{r}");
}
