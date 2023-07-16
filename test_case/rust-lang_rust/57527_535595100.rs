
error[E0308]: mismatched types
 --> src/main.rs:4:17
  |
1 | const fn foo() {}
  | -------------- fn() {foo} defined here
...
4 |     let _: () = foo;
  |                 ^^^
  |                 |
  |                 expected (), found fn item
  |                 help: use parentheses to call this function: `foo()`
  |
  = note: expected type `()`
             found type `fn() {foo}`
