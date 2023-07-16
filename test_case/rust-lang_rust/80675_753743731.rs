
error[E0310]: the parameter type `impl Foo` may not live long enough
  --> src/lib.rs:24:5
   |
23 | fn box_foo(foo: impl Foo) -> Box<dyn Foo> {
   |                 -------- help: consider adding an explicit lifetime bound...: `impl Foo + 'static`
24 |     Box::new(foo)
   |     ^^^^^^^^^^^^^ ...so that the type `impl Foo` will meet its required lifetime bounds
