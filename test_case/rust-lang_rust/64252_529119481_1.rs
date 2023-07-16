
error: expected one of `:` or `@`, found `<`
 --> src/lib.rs:2:21
  |
2 |     pub fn hello(Box<Self>) {}
  |                     ^ expected one of `:` or `@` here
  |
  = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: if this was a parameter name, give it a type
  |
2 |     pub fn hello(Box: TypeName<Self>) {}
  |                  ^^^^^^^^^^^^^
help: if this is a type, explicitly ignore the parameter name
  |
2 |     pub fn hello(_: Box<Self>) {}
  |                  ^^^^^^

error: aborting due to previous error
