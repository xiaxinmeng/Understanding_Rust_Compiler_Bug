
error[E0716]: temporary value dropped while borrowed
 --> src/main.rs:5:59
  |
5 |     let _res = is_not(obfstr::obfstr!("linux")) && is_not(obfstr::obfstr!("windows")) && is_not(obfstr::obfstr!("macos"));
  |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^-    ------ borrow later used here
  |                                                           |                         |
  |                                                           |                         temporary value is freed at the end of this statement
  |                                                           creates a temporary which is freed while still in use
  |
  = note: consider using a `let` binding to create a longer lived value
  = note: this error originates in the macro `$crate::__obfbytes` which comes from the expansion of the macro `obfstr::obfstr` (in Nightly builds, run with -Z macro-backtrace for more info)
