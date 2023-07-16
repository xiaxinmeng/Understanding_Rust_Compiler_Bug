
error[E0308]: mismatched types
   --> src/libcore/iter/mod.rs:585:9
    |
584 |     unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item {
    |                                                     ---------- expected `<iter::Cloned<I> as iter::iterator::Iterator>::Item` because of return type
585 |         self.it.get_unchecked(i).clone()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected associated type, found type parameter
    |
    = note: expected type `<iter::Cloned<I> as iter::iterator::Iterator>::Item`
               found type `T`
