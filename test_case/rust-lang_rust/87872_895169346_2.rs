
error[E0027]: pattern does not mention field `x` and inaccessible fields
  --> src/main.rs:10:9
   |
10 |     let foo::Foo {} = foo::Foo::default();
   |         ^^^^^^^^^^^ missing field `x` and inaccessible fields
   |
help: include the missing fields and ignore the inaccessible fields in the pattern
   |
10 |     let foo::Foo { x, .. } = foo::Foo::default();
   |                  ^^^^^^^^^
help: if you don't care about these missing fields, you can explicitly ignore them
   |
10 |     let foo::Foo { .. } = foo::Foo::default();
   |                  ^^^^^^

For more information about this error, try `rustc --explain E0027`.
