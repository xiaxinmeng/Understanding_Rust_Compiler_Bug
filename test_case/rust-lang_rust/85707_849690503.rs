
after fixes were automatically applied the compiler reported errors within these files:

  * src/char.rs
  * src/int.rs
  * src/lib.rs

This likely indicates a bug in either rustc or cargo itself,
and we would appreciate a bug report! You're likely to see 
a number of compiler warnings after this message which cargo
attempted to fix but failed. If you could open an issue at
https://github.com/rust-lang/rust/issues
quoting the full output of this command we'd be very appreciative!
Note that you may be able to make some more progress in the near-term
fixing code with the `--broken-code` flag

The following errors were reported:
error[E0107]: missing generics for trait `TryFrom`
  --> src/lib.rs:55:30
   |
55 |         let result = <u32 as TryFrom>::try_from("3");
   |                              ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
55 |         let result = <u32 as TryFrom<T>>::try_from("3");
   |                              ^^^^^^^^^^
