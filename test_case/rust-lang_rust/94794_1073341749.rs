plain
---- src/string.rs - string::String (line 125) stdout ----
error[E0308]: mismatched types
  --> src/string.rs:135:58
   |
13 | let size: usize = s.into_iter().map(|c| mem::size_of_val(c)).sum();
   |                                                          |
   |                                                          expected reference, found `char`
   |                                                          help: consider borrowing here: `&c`
   |
   |
   = note: expected reference `&_`
                   found type `char`

error[E0308]: mismatched types
  --> src/string.rs:144:58
   |
22 | let size: usize = s.into_iter().map(|c| mem::size_of_val(c)).sum();
   |                                                          |
   |                                                          expected reference, found `char`
   |                                                          help: consider borrowing here: `&c`
   |
