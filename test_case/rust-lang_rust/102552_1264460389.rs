
error[E0310]: the parameter type `Item` may not live long enough
  --> /home/gh-compiler-errors/test.rs:11:5
   |
11 |     fn foo<'a>(&'a self) -> impl Debug where Item: 'a {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `Item` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
  --> /home/gh-compiler-errors/test.rs:11:52
   |
11 |     fn foo<'a>(&'a self) -> impl Debug where Item: 'a {
   |                                                    ^^
help: consider adding an explicit lifetime bound...
   |
10 | impl<Item: 'static, D: Debug + Clone> Foo<Item> for D {
   |          +++++++++
