 rust
pub trait IterFunction2<I,T> 
    where I: Iterator<Item = T>
{
    fn apply(self, iter: I) -> T;
}

impl<I,S,T> IterFunction2<I,T> for Const<S> 
    where I: Iterator<Item = T>, S: Cast<T>
{
    fn apply(self, _: I) -> T { self.0.cast() }
}

fn expects_polymorphic_function2<F>(f: F)
    where F: for<'a> IterFunction2<std::vec::Drain<'a,&'a str>, &'a str> 
{
    let mut vec = vec!["abc"];
    f.apply(vec.drain(..));
}

fn main() {
    expects_polymorphic_function2(Const("hi"));
}
