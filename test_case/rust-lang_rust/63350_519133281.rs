
error[E0221]: ambiguous associated type `Item` in bounds of `I`
   --> src/libcore/iter/adapters/flatten.rs:112:30
    |
112 |     inner: FlattenCompat<I, <I::Item as IntoIterator>::IntoIter>,
    |                              ^^^^^^^ ambiguous associated type `Item`
    |
   ::: src/libcore/iter/traits/collect.rs:211:5
    |
211 |     type Item;
    |     ---------- ambiguous `Item` from `iter::traits::collect::IntoIterator`
    |
   ::: src/libcore/iter/traits/iterator.rs:94:5
    |
94  |     type Item;
    |     ---------- ambiguous `Item` from `iter::traits::iterator::Iterator`
