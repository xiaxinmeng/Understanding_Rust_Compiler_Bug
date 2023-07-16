
error[E0282]: type annotations needed
  --> src/main.rs:24:20
   |
24 |     println!("{}", 23u64.test(xs.iter().sum()));
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the associated function `new_display`
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider specifying the generic argument
   |
24 |     println!("{}", 23u64.test(xs.iter().sum())::<T>);
   |                                               +++++

error[E0283]: type annotations needed
  --> src/main.rs:24:41
   |
24 |     println!("{}", 23u64.test(xs.iter().sum()));
   |                          ----           ^^^ cannot infer type of the type parameter `S` declared on the associated function `sum`
   |                          |
   |                          required by a bound introduced by this call
   |
note: multiple `impl`s satisfying `u64: Test<_>` found
  --> src/main.rs:7:1
   |
7  | impl Test<u32> for u64 {
   | ^^^^^^^^^^^^^^^^^^^^^^
...
13 | impl Test<u64> for u64 {
   | ^^^^^^^^^^^^^^^^^^^^^^
help: consider specifying the generic argument
   |
24 |     println!("{}", 23u64.test(xs.iter().sum::<S>()));
   |                                            +++++
