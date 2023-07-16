
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
1 | struct Foo();
  | ------------- `Foo` defined here
...
4 |     let _: Foo = Foo;
  |                  ^^^
  |                  |
  |                  expected struct `Foo`, found struct `Foo`'s constructor
  |                  help: use parentheses to instantiate this tuple struct: `Foo()`
  |
  = note: expected type `Foo`
             found constructor for type `Foo`
