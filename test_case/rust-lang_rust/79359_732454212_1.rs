
   Compiling playground v0.0.1 (/playground)
error[E0277]: expected a `Fn<(char,)>` closure, found `char`
 --> src/main.rs:3:45
  |
3 |     s.chars().filter(|c| !(" \n\t".contains(c))).collect::<String>();
  |                                             ^ expected an `Fn<(char,)>` closure, found `char`
  |
  = help: the trait `Fn<(char,)>` is not implemented for `char`
  = note: required because of the requirements on the impl of `FnOnce<(char,)>` for `&char`
  = note: required because of the requirements on the impl of `Pattern<'_>` for `&char`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
