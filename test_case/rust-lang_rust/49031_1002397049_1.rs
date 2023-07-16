
error[E0425]: cannot find value `Foo` in this scope
 --> src/main.rs:3:20
  |
3 |     println!("{}", Foo);
  |                    ^^^ not found in this scope
  |
help: consider importing this unit variant
  |
1 | use E::Foo;
  |
