rust
use std::marker::PhantomData;

trait FancyTrait<'b, T: ?Sized> {
    type Associated;
    fn convert(self, x: Self::Associated, proof: PhantomData<&'b Self::Associated>) -> &'b T;
}

impl<'a, 'b, T: ?Sized + 'a, F> FancyTrait<'b, T> for F
where
    F: Fn() -> &'a str,
{
    type Associated = &'a T;
    fn convert(self, x: &'a T, _: PhantomData<&'b &'a T>) -> &'b T {
        x
    }
}

fn extend_lifetime<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    // closure has inferred return type `&'a str`
    static_transfers_to_associated(|| "", x)
}

fn static_transfers_to_associated<'a, 'b, F, T: ?Sized>(f: F, x: F::Associated) -> &'b T
where
    F: FancyTrait<'b, T> + 'b,
{
    // `F: 'b` implies `F::Assocaited: 'b`
    f.convert(x, PhantomData) // callable since `&'b F::Associated` is a valid type
}

fn main() {
    let r;
    {
        let x = String::from("Hello World?");
        r = extend_lifetime(&x);
    }
    println!("{}", r);
}
