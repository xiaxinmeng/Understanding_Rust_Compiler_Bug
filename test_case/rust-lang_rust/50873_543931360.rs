
error[E0038]: the trait `ObjectSafe` cannot be made into an object
 --> src/main.rs:9:1
  |
4 |     const Const: i32;
  |           ----- the trait cannot contain associated consts like `Const`
...
9 | fn takes_object_safe(os: dyn ObjectSafe) {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ObjectSafe` cannot be made into an object
