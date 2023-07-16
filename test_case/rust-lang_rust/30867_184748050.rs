 rust
trait Cast<T> {
    fn cast(self) -> T;
}

impl<'a> Cast<&'a str> for &'static str {
    fn cast(self) -> &'a str { self }
}

struct Const<S>(S);

pub trait IterFunction1<I> 
    where I: Iterator
{
    fn apply(self, iter: I) -> I::Item;
}

impl<I,S,T> IterFunction1<I> for Const<S> 
    where I: Iterator<Item = T>, S: Cast<T>
{
    fn apply(self, _: I) -> T { self.0.cast() }
}
