rust
trait Blah<T> {
    fn foo(t: T);
}

impl<B, F> Blah<Vec<F>> for B
where
    B: Blah<F>,
{
    fn foo(t: Vec<F>) {
        t.into_iter(F::foo);
    }
}
