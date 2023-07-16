 rust
// `?Sized` bounds omitted for brevity
trait Index<Index> {
    type Output;
    fn index(&self, &Index) -> &Self::Output;
}

trait IndexMut<Index>: Index<Index> {
    fn index_mut(&mut self, &Index) -> &mut <Self as Index<Index>>::Output;
}
