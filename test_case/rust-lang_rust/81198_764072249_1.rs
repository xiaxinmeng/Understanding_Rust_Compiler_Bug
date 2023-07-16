
error[E0277]: can't compare `B` with `A`
  --> src/main.rs:71:33
   |
17 | fn needs_partial_eq(a: A, lhs: impl PartialEq<A>) -> bool {
   |                                     ------------ required by this bound in `needs_partial_eq`
...
71 |     let _ = needs_partial_eq(A, B);
   |                                 ^ no implementation for `B == A`
   |
   = help: the trait `PartialEq<A>` is not implemented for `B`, but `PartialEq<B>` is implemented for `A`
   = help: try `let _ = needs_partial_eq(A, std::cmp::symmetric(B));`
