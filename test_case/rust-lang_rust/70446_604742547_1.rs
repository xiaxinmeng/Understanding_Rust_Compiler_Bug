
error: local ambiguity: multiple parsing options: built-in NTs lifetime ('l') or path ('p').
  --> src/lib.rs:2:66
   |
2  |     ($(: $p:path)?  $(: $l:lifetime)? ) => { bar! {$(: $p)?  $(: $l)?  } };
   |                                                                  ^^
...
10 | foo! {: 'a }
   | ------------ in this macro invocation
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
