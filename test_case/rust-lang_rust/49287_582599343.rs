
error[E0261]: use of undeclared lifetime name `'a`
 --> file4.rs:1:27
  |
1 | fn foo(_compare: impl Fn(&'a u8)) {
  |                           ^^ undeclared lifetime
  |
  = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
help: consider introducing lifetime `'a` here
  |
1 | fn foo<'a>(_compare: impl Fn(&'a u8)) {
  |       ^^^^
help: consider making the bound lifetime-generic with a new `'a` lifetime
  |
1 | fn foo(_compare: impl for<'a> Fn(&'a u8)) {
  |                       ^^^^^^^
