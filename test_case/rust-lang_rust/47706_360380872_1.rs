
error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
  --> src/main.rs:11:18
   |
6  |     pub fn new(foo: Option<i32>, _: ()) -> Foo {
   |     ------------------------------------------ takes 2 arguments
...
11 |         self.foo.map(Foo::new)
   |                  ^^^ expected function that takes 1 argument
