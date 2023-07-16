plain
i......................i......................i......................i.............................. 3700/3763
...............................................................
failures:

---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::map_windows (line 1566) stdout ----
error: expected `,`, found `*`
   |
   |
40 |     .map_windows(|arr: &[_; 2] *arr);
   |                                ^ expected `,`

error: expected one of `#`, `&`, `(`, `-`, `...`, `..=`, `..`, `[`, `_`, `box`, `mut`, `ref`, `|`, identifier, or path, found `*`
   |
   |
40 |     .map_windows(|arr: &[_; 2] *arr);
   |                                ^ expected one of 15 possible tokens
error[E0433]: failed to resolve: use of undeclared type `NonFusedIterator`
  --> src/iter/traits/iterator.rs:1588:16
   |
   |
25 | let mut iter = NonFusedIterator::default();
   |                ^^^^^^^^^^^^^^^^ use of undeclared type `NonFusedIterator`
error[E0412]: cannot find type `NonFusedIterator` in this scope
  --> src/iter/traits/iterator.rs:1574:19
   |
   |
7  | struct NoneFusedIterator {
   | ------------------------ similarly named struct `NoneFusedIterator` defined here
11 | impl Iterator for NonFusedIterator {
   |                   ^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `NoneFusedIterator`

error[E0425]: cannot find value `state` in this scope
error[E0425]: cannot find value `state` in this scope
  --> src/iter/traits/iterator.rs:1579:12
   |
16 |         if state < 5 || state % 2 == 0 {

error[E0425]: cannot find value `state` in this scope
  --> src/iter/traits/iterator.rs:1579:25
   |
   |
16 |         if state < 5 || state % 2 == 0 {

error[E0425]: cannot find value `state` in this scope
  --> src/iter/traits/iterator.rs:1580:18
   |
