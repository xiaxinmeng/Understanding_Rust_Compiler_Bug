rust
trait MyFn<Arg1, Arg2> {
    type Output;
    fn call(&self, a1: Arg1, a2: Arg2) -> Self::Output;
}

struct Fun;
impl<'b, 'a> MyFn<&'b (), &'a ()> for Fun {
    type Output = PhantomData<&'b &'a ()>;
    fn call(&self, a1: &'b (), a2: &'a ()) -> PhantomData<&'b &'a ()> {
        PhantomData
    }
}
