
error[E0412]: cannot find type `MissingType` in this scope
 --> src/lib.rs:2:10
  |
2 |     val: MissingType
  |          ^^^^^^^^^^^ not found in this scope

error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
 --> src/lib.rs:6:16
  |
6 |     Foo { val: Default::default() };
  |                ^^^^^^^^^^^^^^^^ cannot call associated function of trait
  |
help: use a fully-qualified path to a specific available implementation (1057 found)
  |
6 |     Foo { val: <std::io::Empty as Default>::default() };
  |                ++++++++++++++++++        +
