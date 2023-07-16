
Compiling playground v0.0.1 (/playground)
error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): `match` arms have incompatible types
 [--> src/main.rs:5:21
](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018#)  |
3 | /     match std::io::stdin().read_line(&mut input) {
4 | |         Ok(bytes_read) => bytes_read,
  | |                           ---------- this is found to be of type `usize`
5 | |         Err(why) => panic!("{}", why)
  | |                     ^^^^^^^^^^^^^^^^^ expected `usize`, found `()`
6 | |     }
  | |_____- `match` arms have incompatible types
  |
  = note: this error originates in the macro `$crate::panic::panic_2015`
  (in Nightly builds, run with -Z macro-backtrace for more info)
