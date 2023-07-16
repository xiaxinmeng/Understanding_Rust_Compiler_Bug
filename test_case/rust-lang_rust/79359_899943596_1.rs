
error[E0277]: expected a `Fn<()>` closure, found `dyn Foo`
  --> src/lib.rs:16:9
   |
11 | fn foo(foo: impl Foo) -> String {
   |                  --- required by this bound in `foo`
...
16 |     foo(x);
   |         ^ expected an `Fn<()>` closure, found `dyn Foo`
   |
   = help: the trait `Fn<()>` is not implemented for `dyn Foo`
   = note: wrap the `dyn Foo` in a closure with no arguments: `|| { /* code */ }`
   = note: required because of the requirements on the impl of `Fn<()>` for `Box<dyn Foo>`
note: required because of the requirements on the impl of `Foo` for `Box<dyn Foo>`
  --> src/lib.rs:5:9
   |
5  | impl<F> Foo for F where F: Fn() -> String {
   |         ^^^     ^
