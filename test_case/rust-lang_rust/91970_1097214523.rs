plain

   Doc-tests core

running 3814 tests
...................................i..i......iiiiii............F...............F....F... 88/3814
........................................................................................ 264/3814
............ii.......................................................................... 352/3814
..................................................................................i..... 440/3814
........................................................................................ 528/3814
---
.......................i................................................................ 3784/3814
..............................
failures:

---- src/any.rs - any::Demand::provide_value (line 879) stdout ----
error[E0107]: this associated function takes 2 generic arguments but 1 generic argument was supplied
    |
    |
10  |         demand.provide_value::<String>(|| self.field.clone());
    |                ^^^^^^^^^^^^^   ------ supplied 1 generic argument
    |                expected 2 generic arguments
    |
    |
note: associated function defined here, with 2 generic parameters: `T`, `F`
    |
    |
891 |     pub fn provide_value<T, F>(&mut self, fulfil: F) -> &mut Self
    |            ^^^^^^^^^^^^^ -  -
help: add missing generic argument
    |
10  |         demand.provide_value::<String, F>(|| self.field.clone());

error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
For more information about this error, try `rustc --explain E0107`.
Couldn't compile the test.
---- src/any.rs - any::request_ref (line 832) stdout ----
error[E0261]: use of undeclared lifetime name `'a`
 --> src/any.rs:836:36
  |
7 | fn get_str<P: Provider>(provider: &'a P) -> &'a str {
  |            -                       ^^ undeclared lifetime
  |            |
  |            help: consider introducing lifetime `'a` here: `'a,`
error[E0261]: use of undeclared lifetime name `'a`
 --> src/any.rs:836:46
  |
  |
7 | fn get_str<P: Provider>(provider: &'a P) -> &'a str {
  |            -                                 ^^ undeclared lifetime
  |            |
  |            help: consider introducing lifetime `'a` here: `'a,`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0261`.
Couldn't compile the test.
Couldn't compile the test.
---- src/any.rs - any::request_value (line 809) stdout ----
error[E0261]: use of undeclared lifetime name `'a`
 --> src/any.rs:813:39
  |
7 | fn get_string<P: Provider>(provider: &'a P) -> String {
  |               -                       ^^ undeclared lifetime
  |               |
  |               help: consider introducing lifetime `'a` here: `'a,`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0261`.
Couldn't compile the test.
