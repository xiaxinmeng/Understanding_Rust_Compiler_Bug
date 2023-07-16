
error: unconstrained generic constant
  --> src/main.rs:20:5
   |
20 |     foo::<N>([0])
   |     ^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); simple_log_2(N)]:`
note: required by a bound in `foo`
  --> src/main.rs:14:37
   |
14 | fn foo<const N: usize>(arr: [usize; simple_log_2(N)]) {
   |                                     ^^^^^^^^^^^^^^^ required by this bound in `foo`

error: could not compile `tmp` due to previous error
[Finished running. Exit status: 101]
