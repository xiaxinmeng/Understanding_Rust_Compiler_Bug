
error[E0621]: explicit lifetime required in the type of `foo`
 --> src/lib.rs:8:9
  |
7 | fn bar<'a>(foo: &mut Foo<'a>) {
  |                 ------------ help: add explicit lifetime `'a` to the type of `foo`: `&'a mut Foo<'a>`
8 |     foo.modify();
  |         ^^^^^^ lifetime `'a` required
