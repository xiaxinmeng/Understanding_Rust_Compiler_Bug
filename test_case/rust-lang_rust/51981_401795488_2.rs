
error[E0277]: cannot multiply `{integer}` to `{float}`
  --> src/lib.rs:11:32
   |
1  | / macro_rules! foo {
2  | |     ($e:expr) => {
3  | |         bar!($e);
4  | |         baz!($e);
   | |         --------- in this macro invocation
5  | |     }
6  | | }
   | |_- in this expansion of `foo!`
...
10 | / macro_rules! baz {
11 | |     ($e:expr) => { if $e + 0.1 * 2 { panic!(); }  }
   | |                                ^ no implementation for `{float} * {integer}`
12 | | }
   | |_- in this expansion of `baz!`
...
15 |       foo!(true);
   |       ----------- in this macro invocation
   |
   = help: the trait `std::ops::Mul<{integer}>` is not implemented for `{float}`
