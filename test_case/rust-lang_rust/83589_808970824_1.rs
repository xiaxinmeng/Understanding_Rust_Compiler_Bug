
error[E0119]: conflicting implementations of trait `core::iter::IntoIterator` for type `&mut boxed::Box<[_], _>`:
    --> library/alloc/src/boxed.rs:1578:1
     |
1578 | impl<'a, T, A> IntoIterator for &'a mut Box<[T], A> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: conflicting implementation in crate `core`:
             - impl<I> IntoIterator for I
               where I: Iterator;
     = note: upstream crates may add a new impl of trait `core::iter::Iterator` for type `[_]` in future versions
