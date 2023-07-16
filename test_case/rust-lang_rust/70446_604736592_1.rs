
   Compiling playground v0.0.1 (/playground)
error: local ambiguity: multiple parsing options: built-in NTs lifetime ('l') or path ('p').
 --> src/main.rs:3:38
  |
3 |     => {m!{@internal $(: $p     )? : $l         }};
  |                                      ^^
...
8 | m! {: 'static}
  | -------------- in this macro invocation
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

error: could not compile `playground`.

To learn more, run the command again with --verbose.

