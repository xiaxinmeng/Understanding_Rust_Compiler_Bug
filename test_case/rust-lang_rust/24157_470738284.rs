
error[E0308]: match arms have incompatible types
 --> src/main.rs:5:21
  |
3 | /     match std::io::stdin().read_line(&mut input) {
4 | |         Ok(bytes_read) => bytes_read,
  | |                           ---------- this is found to be of type `usize`
5 | |         Err(why) => panic!("{}", why)
  | |                     ^^^^^^^^^^^^^^^^^ expected usize, found ()
6 | |     }
  | |_____- `match` arms have incompatible types
  |
  = note: expected type `usize`
             found type `()`
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
