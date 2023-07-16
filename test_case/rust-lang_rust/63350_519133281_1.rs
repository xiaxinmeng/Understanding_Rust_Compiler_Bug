
error[E0221]: ambiguous associated type `Item` in bounds of `U`
   --> src/libcore/iter/adapters/flatten.rs:77:39
    |
77  |     fn next_back(&mut self) -> Option<U::Item> { self.inner.next_back() }
    |                                       ^^^^^^^ ambiguous associated type `Item`
    |
   ::: src/libcore/iter/traits/iterator.rs:94:5
    |
94  |     type Item;
    |     ---------- ambiguous `Item` from `iter::traits::iterator::Iterator`
    |
   ::: src/libcore/iter/traits/collect.rs:211:5
    |
211 |     type Item;
    |     ---------- ambiguous `Item` from `iter::traits::collect::IntoIterator`
