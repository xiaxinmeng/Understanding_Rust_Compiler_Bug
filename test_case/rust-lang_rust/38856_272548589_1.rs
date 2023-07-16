
error[E0271]: type mismatch resolving `<&std::vec::Vec<(&str, &str)> 
    as std::iter::IntoIterator>::Item == (_, _)`
  --> test.rs:18:5
   |
18 |     test(&vc);
   |     ^^^^ expected reference, found tuple
   |
   = note: expected type `&(&str, &str)`
   = note:    found type `(_, _)`
   = note: required by `test`
