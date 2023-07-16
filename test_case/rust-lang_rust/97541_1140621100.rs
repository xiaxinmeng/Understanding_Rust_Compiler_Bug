
error[E0053]: method `crunch` has an incompatible generic parameter for trait `CrunchICE`
 --> 97541.rs:9:15
  |
2 | trait CrunchICE {
  |       ---------
3 |     fn crunch<const N: usize, T>(ice: T);
  |               -------------- expected const parameter of type `usize`
...
7 | impl CrunchICE for () {
  | ---------------------
8 |     // Note that the const generic and generic have swapped places here.
9 |     fn crunch<T, const N: usize>(ice: T) {}
  |               ^ found type parameter

error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.
