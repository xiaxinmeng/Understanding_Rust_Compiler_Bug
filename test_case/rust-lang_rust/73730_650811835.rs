rust
#![feature(const_generics, in_band_lifetimes)]
#![allow(incomplete_features)]

trait Foo<'a, A>: Iterator<Item=A> {
    fn bar<const N: usize>(&mut self) -> *const [A; N];
}

impl<A, I: ?Sized> Foo<'a, A> for I where I: Iterator<Item=A>  {
    fn bar<const N: usize>(&mut self) -> *const [A; N] {
        std::ptr::null()
    }
}

fn main() {
    (0_u8 .. 10).bar::<10_usize>();
}
