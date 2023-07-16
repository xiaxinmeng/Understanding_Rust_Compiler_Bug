
   Compiling playground v0.0.1 (/playground)
error[E0277]: `&Cow<'_, [u8]>` is not an iterator
 --> src/lib.rs:6:17
  |
6 |     for elem in &c {
  |                 ^^ `&Cow<'_, [u8]>` is not an iterator
  |
  = help: the trait `Iterator` is not implemented for `&Cow<'_, [u8]>`
  = note: required because of the requirements on the impl of `IntoIterator` for `&Cow<'_, [u8]>`
  = note: required by `into_iter`

error[E0277]: `&Cow<'_, [u8]>` is not an iterator
  --> src/lib.rs:14:17
   |
14 |     for elem in &c {
   |                 ^^ `&Cow<'_, [u8]>` is not an iterator
   |
   = help: the trait `Iterator` is not implemented for `&Cow<'_, [u8]>`
   = note: required because of the requirements on the impl of `IntoIterator` for `&Cow<'_, [u8]>`
   = note: required by `into_iter`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
