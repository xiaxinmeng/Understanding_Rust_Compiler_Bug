
error[E0308]: mismatched types
 --> test.rs:1:47
  |
1 | fn foo() -> bool { if let Some(()) = Some(()) { } else { } }
  |                                               ^^^ expected bool, found ()
  |
  = note: expected type `bool`
             found type `()`

error[E0308]: mismatched types
 --> test.rs:1:56
  |
1 | fn foo() -> bool { if let Some(()) = Some(()) { } else { } }
  |                                                        ^^^ expected bool, found ()
  |
  = note: expected type `bool`
             found type `()`

error: aborting due to 2 previous errors
