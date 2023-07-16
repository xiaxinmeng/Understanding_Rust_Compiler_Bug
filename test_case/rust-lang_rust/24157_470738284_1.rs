
error[E0308]: match arms have incompatible types
 --> src/main.rs:8:14
  |
6 | /     match a {
7 | |         1 => true,
  | |              ---- this is found to be of type `bool`
8 | |         _ => panic!("It's not one!"),
  | |              ^^^^^^^^^^^^^^^^^^^^^^^ expected bool, found ()
9 | |     }
  | |_____- `match` arms have incompatible types
  |
  = note: expected type `bool`
             found type `()`
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
