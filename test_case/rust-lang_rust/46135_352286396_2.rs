
error[E0261]: use of undeclared lifetime name `'a`
 --> foo.rs:5:10
  |
5 | impl Foo<'a> {
  | ----     ^^ undeclared lifetime
  | |
  | help: declare lifetime `'a` here: `impl<'a>`
