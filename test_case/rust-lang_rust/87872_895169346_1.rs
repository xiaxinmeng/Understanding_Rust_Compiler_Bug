
error[E0027]: pattern does not mention fields `x`, `y`
  --> src/main.rs:10:9
   |
10 |     let foo::Foo {} = foo::Foo::default();
   |         ^^^^^^^^^^^ missing fields `x`, `y`
   |
help: include the missing fields in the pattern
   |
10 |     let foo::Foo { x, y } = foo::Foo::default();
   |                  ^^^^^^^^
help: if you don't care about these missing fields, you can explicitly ignore them
   |
10 |     let foo::Foo { .. } = foo::Foo::default();
   |                  ^^^^^^

For more information about this error, try `rustc --explain E0027`.
