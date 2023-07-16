
error[E0391]: unsupported cyclic reference between types/traits detected
  --> /path/to/bug/src/lib.rs:17:1
   |
17 | / impl<T, R> Reduce<R> for T
18 | | where
19 | |     R: Reducible,
20 | |     T: Reduce<R::Reduced>,
21 | | {
22 | | }
   | |_^ cyclic reference
   |
note: the cycle begins when computing whether impls specialize one another...
  --> /path/to/bug/src/lib.rs:17:1
   |
17 | / impl<T, R> Reduce<R> for T
18 | | where
19 | |     R: Reducible,
20 | |     T: Reduce<R::Reduced>,
21 | | {
22 | | }
   | |_^
   = note: ...which then again requires computing whether impls specialize one another, completing the cycle.
