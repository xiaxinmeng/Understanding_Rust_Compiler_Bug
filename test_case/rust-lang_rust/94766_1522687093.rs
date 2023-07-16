
error: unexpected token: `1`
 --> src/main.rs:3:14
  |
3 |         $val.$i
  |              ^^
...
9 |     let d = idx!(v, 1);
  |             ---------- in this macro invocation
  |
  = note: this error originates in the macro `idx` (in Nightly builds, run with -Z macro-backtrace for more info)

error: macro expansion ignores token `1` and any following
 --> src/main.rs:3:14
  |
3 |         $val.$i
  |              ^^
...
9 |     let d = idx!(v, 1);
  |             ---------- caused by the macro expansion here
  |
  = note: the usage of `idx!` is likely invalid in expression context
