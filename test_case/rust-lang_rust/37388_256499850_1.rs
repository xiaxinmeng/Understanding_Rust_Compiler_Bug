
error[E0308]: mismatched types
 --> t.rs:2:20
  |
2 |     let x: usize = String::new();
  |                    ^^^^^^^^^^^^^ expected usize, found struct `std::string::String`
  |
  = note: expected type `usize`
  = note:    found type `std::string::String`
  = help: no safe suggestion found, here are functions which match your needs but be careful:
 - capacity
 - len

error: aborting due to previous error
