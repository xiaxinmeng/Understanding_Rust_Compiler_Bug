
error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
 --> src/lib.rs:1:19
  |
1 | fn foo() -> Result((&str, i32), i32) {}
  |                   ^^^^^^^^^^^^^^^^^^
  |                   |
  |                   only `Fn` traits may use parentheses
  |                   help: use angle brackets instead: `<(&str, i32), i32>`

error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:21
  |
1 | fn foo() -> Result((&str, i32), i32) {}
  |                     ^ help: consider giving it a 'static lifetime: `&'static`
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
