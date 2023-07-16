
error: expected identifier, found `,`
 --> src/main.rs:3:31
  |
3 |         $(assert_eq!(0 == $e::$var);)*
  |                               ^
  |                               |
  |                               expected identifier
  |                               help: remove this comma
...
8 |     check_enum!(Foo, Bar, Baz);
  |     --------------------------- in this macro invocation
