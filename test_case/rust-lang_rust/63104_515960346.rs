rust
error[E0119]: conflicting implementations of trait `std::cmp::PartialOrd<&_>` for type `&_`:
 --> src/lib.rs:3:1
  |
3 | impl<T: Ord> PartialOrd for T {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: conflicting implementation in crate `core`:
          - impl<A, B> std::cmp::PartialOrd<&B> for &A
            where A: std::cmp::PartialOrd<B>, A: ?Sized, B: ?Sized;
