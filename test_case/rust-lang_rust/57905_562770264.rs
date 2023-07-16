rust
trait Example {
    type Foo;
    
    fn foo<T: AsRef<()>>(&self, t: T);
}

impl <A, B> Example for (A, B) where
    A: Iterator<Item = u32>,
    B: AsRef<A::Item>,
{
    type Foo = ();
    
    fn foo<T: AsRef<Self::Foo>>(&self, t: T) {}
}
