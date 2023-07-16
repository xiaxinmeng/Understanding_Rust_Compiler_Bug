rust
use num_traits as _;

struct Ratio<T>(T);

pub trait Pow {
    fn pow(self);
}

impl<'a, T> Pow for &'a Ratio<T>
where
    &'a T: Pow,
{
    fn pow(self) {
        unimplemented!()
    }
}

struct Foo;

impl Foo {
    fn downcast<W>(&self) -> &W {
        todo!()
    }
}

struct Other;

fn main() {
    let other: &mut Other = Foo.downcast();
}
