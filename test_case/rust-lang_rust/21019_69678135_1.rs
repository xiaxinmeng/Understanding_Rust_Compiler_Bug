 rust
struct Map<I,F>
    where I : Iterator, F : FnMut<(I::Item,)>
{
    iter: I,
    fn: F
}

impl<I,F> Iterator<F::Output> for Map<I,F>
    where I : Iterator, F : FnMut<(I::Item,)>
{
}
