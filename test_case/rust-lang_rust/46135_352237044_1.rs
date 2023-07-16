
error[E0261]: use of undeclared lifetime name `'a`
 --> foo.rs:5:10
  |
5 | impl Foo<'a> {
  |          ^^ undeclared lifetime

error[E0261]: use of undeclared lifetime name `'a`
 --> foo.rs:6:21
  |
6 |     fn x(&self) -> &'a str { self.x }
  |                     ^^ undeclared lifetime

error: aborting due to 2 previous errors
