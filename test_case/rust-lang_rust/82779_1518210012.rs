
error[[E0275]](https://doc.rust-lang.org/stable/error_codes/E0275.html): overflow evaluating the requirement `for<'r> &'r Simd<_, _>: Add`
  --> src/main.rs:26:5
   |
26 |     addition(&Mat { a: 2 }, &Mat{ a: 3 }); // also works if we comment this line
   |     ^^^^^^^^
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`playground`)
note: required for `&'r Mat<Simd<_, _>>` to implement `for<'r> Add`
  --> src/main.rs:8:17
   |
8  | impl<'a, N: 'a> Add<&'a Mat<N>> for &'a Mat<N>
   |                 ^^^^^^^^^^^^^^^     ^^^^^^^^^^
9  | where for<'r> &'r N: Add<&'r N, Output=N>,
   |                                 -------- unsatisfied trait bound introduced here
   = note: 127 redundant requirements hidden
   = note: required for `&Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<Mat<...>>>>>>>>>>>>>>>>>>>>>` to implement `for<'r> Add`
   = note: the full type name has been written to '/playground/target/debug/deps/playground-a70d03cb77de99e7.long-type-4444812125513997297.txt'
note: required by a bound in `addition`
  --> src/main.rs:19:22
   |
18 | fn addition<'a, N: 'a>(lhs: &'a Mat<N>, rhs: &'a Mat<N>) -> Mat<N>
   |    -------- required by a bound in this function
19 | where for<'r> &'r N: Add<&'r N, Output=N>,
   |                      ^^^^^^^^^^^^^^^^^^^^ required by this bound in `addition`

For more information about this error, try `rustc --explain E0275`.
error: could not compile `playground` due to previous error
