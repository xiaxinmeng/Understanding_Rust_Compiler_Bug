
error: unexpected end of macro invocation
 --> ./test.rs:3:31
  |
3 |         $(assert_eq!(0 == $e::$var);)*
  |                               ^^^^
...
8 |     check_enum!(Foo, Bar, Baz);
  |     --------------------------- in this macro invocation


