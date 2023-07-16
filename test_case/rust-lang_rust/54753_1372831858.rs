
error[E0310]: the parameter type `T` may not live long enough
  --> src/main.rs:14:9
   |
14 |         Box::new(Bazzzz(self.0, self.1))
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
   |
help: consider adding an explicit lifetime bound...
   |
12 | impl<T: 'static> Bar for Foo<T> {
   |       +++++++++
