rust
use std::marker::PhantomData;

fn unsoundness_demonstration<'b, 'a, F, T>(x: &'a T) -> &'b T
where
    for<'b_, 'a_> F: Fn(&'b_ (), &'a_ ()) -> PhantomData<&'b_ &'a_ ()>,
{
    conversion::<'b, 'a, F, _>(x)
}

fn conversion<'b, 'a, F, T>(x: &'a T) -> &'b T
where
    F: HelperTrait<'b, 'a>,
{
    F::convert(x)
}

trait HelperTrait<'b, 'a> {
    fn convert<T>(x: &'a T) -> &'b T;
}
impl<'b, 'a, F, R> HelperTrait<'b, 'a> for F
where
    F: Fn(&'b (), &'a ()) -> PhantomData<R>,
    PhantomData<R>: Proof<'b, 'a>,
{
    fn convert<T>(x: &'a T) -> &'b T {
        PhantomData.convert(x)
    }
}

trait Proof<'b, 'a> {
    fn convert<T>(self, x: &'a T) -> &'b T;
}

impl<'b, 'a> Proof<'b, 'a> for PhantomData<&'b &'a ()> {
    fn convert<T>(self, x: &'a T) -> &'b T {
        x
    }
}

fn extend_lifetime<'a, 'b, T>(x: &'a T) -> &'b T {
    unsoundness_demonstration::<
        Box<dyn for<'b_, 'a_> Fn(&'b_ (), &'a_ ()) -> PhantomData<&'b_ &'a_ ()>>,
        _,
    >(x)
}

fn main() {
    let d;
    {
        let x = "Hello World".to_string();
        d = extend_lifetime(&x);
    }
    println!("{}", d);
}
