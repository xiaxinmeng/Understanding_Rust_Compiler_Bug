rust
use std::marker::PhantomData;

trait Trait<'a, 'b> {
    type Associated: ConversionProof<'a, 'b>;
}

fn convert<'a, 'b, T: ?Sized, O: ?Sized>(x: &'a T) -> &'b T
where
    O: Trait<'a, 'b>,
{
    O::Associated::convert(PhantomData, x)
}

trait ConversionProof<'a, 'b> {
    fn convert<T: ?Sized>(proof: PhantomData<Self>, x: &'a T) -> &'b T;
}

impl<'a, 'b> ConversionProof<'a, 'b> for &'b &'a () {
    fn convert<T: ?Sized>(_proof: PhantomData<Self>, x: &'a T) -> &'b T {
        x
    }
}

fn extend_lifetime<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    convert::<_, dyn for<'a_, 'b_> Trait<'a_, 'b_, Associated = &'b_ &'a_ ()>>(x)
}

fn main() {
    let d;
    {
        let x = "Hello World".to_string();
        d = extend_lifetime(&x);
    }
    println!("{}", d);
}
