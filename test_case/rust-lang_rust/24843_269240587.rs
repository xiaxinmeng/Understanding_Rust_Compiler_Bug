
error[E0394]: cannot refer to other statics by value, use the address-of operator or a constant instead
 --> src/main.rs:3:35
  |
3 | static TEST_STR_2: &'static str = &test::TEST_STR;
  |                                   ^^^^^^^^^^^^^^^ referring to another static by value
  |
  = note: use the address-of operator or a constant instead
