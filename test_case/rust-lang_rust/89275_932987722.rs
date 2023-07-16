rust
#![recursion_limit = "5"] // To reduce noise

struct Ratio<T>(T);

pub trait Pow {
    fn pow(self) -> Self;
}

impl<'a, T> Pow for &'a Ratio<T>
where
    &'a T: Pow,
{
    fn pow(self) -> Self {
        self
    }
}

fn downcast<'a, W: ?Sized>() -> &'a W {
    todo!()
}

struct Other;

fn main() {
    let other: &mut Other = downcast();
}
