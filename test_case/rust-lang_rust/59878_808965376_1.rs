
error[E0119]: conflicting implementations of trait `core::iter::IntoIterator` for type `boxed::Box<[_], _>`:
    --> library/alloc/src/boxed.rs:1578:1
     |
1578 | impl<T, A> IntoIterator for Box<[T], A> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: conflicting implementation in crate `core`:
             - impl<I> IntoIterator for I
               where I: Iterator;
     = note: upstream crates may add a new impl of trait `core::iter::Iterator` for type `[_]` in future versions

error[E0119]: conflicting implementations of trait `core::iter::IntoIterator` for type `boxed::Box<[_; _], _>`:
    --> library/alloc/src/boxed.rs:1589:1
     |
1589 | impl<T, A, const N: usize> IntoIterator for Box<[T; N], A> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: conflicting implementation in crate `core`:
             - impl<I> IntoIterator for I
               where I: Iterator;
     = note: upstream crates may add a new impl of trait `core::iter::Iterator` for type `[_; _]` in future versions
