
error[E0119]: conflicting implementations of trait `core::iter::IntoIterator` for type `&boxed::Box<[_], _>`:
    --> library/alloc/src/boxed.rs:1578:1
     |
1578 | impl<'a, T, A> IntoIterator for &'a Box<[T], A> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: conflicting implementation in crate `core`:
             - impl<I> IntoIterator for I
               where I: Iterator;
     = note: downstream crates may implement trait `core::iter::Iterator` for type `&boxed::Box<[_], _>`
