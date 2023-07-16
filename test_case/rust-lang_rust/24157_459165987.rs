
error[E0308]: match arms have incompatible types
 --> src/main.rs:6:5
  |
6 | /     match a {
7 | |         1 => true,
8 | |         _ => panic!("It's not one!"),
9 | |     }
  | |_____^ expected bool, found ()
  |
  = note: expected type `bool`
             found type `()`
note: match arm with an incompatible type
 --> src/main.rs:8:14
  |
8 |         _ => panic!("It's not one!"),
  |              ^^^^^^^^^^^^^^^^^^^^^^^
  = note: this error originates in a macro outside of the current crate
