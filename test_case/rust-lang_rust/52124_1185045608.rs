
error[E0412]: cannot find type `Foo` in this scope
 --> test.rs:5:14
  |
5 |     let foo: Foo;
  |              ^^^ not found in this scope
  |
note: struct `foo::Foo` exists but is inaccessible
 --> foo.rs:1:1
  |
1 | struct Foo;
  | ^^^^^^^^^^^ not accessible

warning: unused import: `foo::*`
 --> test.rs:2:5
  |
2 | use foo::*;
  |     ^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error: the generated executable for the input file "test.rs" conflicts with the existing directory "test"

error: aborting due to 2 previous errors; 1 warning emitted
