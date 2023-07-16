
error[E0599]: no function or associated item named `new` found for type `Foo` in the current scope
 --> src/main.rs:4:5
  |
1 | struct Foo;
  | ----------- function or associated item `new` not found for this
...
4 |     Foo::new();
  |     ^^^^^^^^ function or associated item not found in `Foo`
