text
error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
 --> test.rs:6:5
  |
2 |     fn bar() {}
  |     ----------- `Foo::bar` defined here
...
6 |     Foo::bar();
  |     ^^^^^^^^ cannot call associated function of trait
