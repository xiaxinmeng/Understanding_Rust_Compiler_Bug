
struct B<'a: 'b, 'b>(&'b &'a ());

trait Foo {
    type Bar<'a, 'b>
    where
        'a: 'b;

    fn suceeds1<F, T>(&self, f: F) -> T
    where
        F: for<'a, 'b> FnOnce(&'b B<'a, 'b>) -> T;

    fn succeeds2(&self)
    where
        for<'a, 'b> B<'a, 'b>: Default;

    fn fails1<F, T>(&self, f: F) -> T
    where
        F: for<'a, 'b> FnOnce(&'b Self::Bar<'a, 'b>) -> T;

    fn fails2(&self)
    where
        for<'a, 'b> Self::Bar<'a, 'b>: Default;
}

impl Foo for () {
    type Bar<'a, 'b> = B<'a, 'b> where 'a: 'b;

    fn suceeds1<F, T>(&self, f: F) -> T
    where
        F: for<'a, 'b> FnOnce(&'b B<'a, 'b>) -> T,
    {
        todo!()
    }

    fn succeeds2(&self)
    where
        for<'a, 'b> B<'a, 'b>: Default,
    {
        todo!()
    }

    fn fails1<F, T>(&self, f: F) -> T
    where
        F: for<'a, 'b> FnOnce(&'b Self::Bar<'a, 'b>) -> T,
    {
        todo!()
    }

    fn fails2(&self)
    where
        for<'a, 'b> Self::Bar<'a, 'b>: Default,
    {
        todo!()
    }
}
