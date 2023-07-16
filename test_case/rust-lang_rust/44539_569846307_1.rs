
error[E0283]: type annotations needed
  --> src/main.rs:20:12
   |
2  |   const BAR: BAR = BAR;
   |   --------------------- required by `Foo::BAR`
...
20 |   take_foo(Foo::BAR);
   |            ^^^^^^^^ cannot infer type
   |
   = note: cannot resolve `_: Foo`
