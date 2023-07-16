
struct Foo<T>([T]);

impl<I: core::slice::SliceIndex<[T]>, T> core::ops::Index<I> for Foo<T>
where
    I::Output: AsRef<[u8]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}
