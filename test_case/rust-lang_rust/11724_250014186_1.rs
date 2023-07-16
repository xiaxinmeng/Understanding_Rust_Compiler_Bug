
error: invalid format string: expected `'}'`, found `','`
 --> /Users/jturner/Source/bad_error.rs:9:22
  |
9 |     let s1 = format!("This page has been visited {:d} {0, plural, one{time} other{times}}.", count);
  |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
