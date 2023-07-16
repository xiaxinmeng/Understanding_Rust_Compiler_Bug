
error[E0261]: use of undeclared lifetime name `'a`
 --> src/main.rs:4:22
  |
3 | impl X {
  | ------ if you mean to declare lifetime `'a` in the `impl`, use `impl<'a> X<'a>`
4 |     fn bar(bar_arg: &'a usize) {
  |     ------           ^^ undeclared lifetime
  |     |
  |     if you meant to declare lifetime `'a` in the `fn`, use `fn bar<'a>`

error[E0261]: use of undeclared lifetime name `'a`
 --> src/main.rs:5:26
  |
5 |         fn foo(foo_arg: &'a usize) {
  |            ---              ^^ undeclared lifetime
  |            |
  |            help: declare lifetime `'a`: `foo<'a>`

error[E0261]: use of undeclared lifetime name `'a`
 --> src/main.rs:7:21
  |
6 |            struct Y {
  |                   - help: declare lifetime `'a`: `Y<'a>`
7 |                 y: &'a usize,
  |                     ^^ undeclared lifetime

error: aborting due to 3 previous errors
