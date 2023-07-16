
error[E0283]: type annotations needed
  --> src/main.rs:12:14
   |
4  |   const BAR: u8 = 32;
   |   ------------------- required by `Foo::BAR`
...
12 |   take_debug(Foo::BAR);
   |              ^^^^^^^^ cannot infer type
   |
   = note: cannot resolve `_: Foo`
