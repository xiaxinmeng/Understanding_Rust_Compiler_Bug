
   Compiling playground v0.0.1 (/playground)
warning: value assigned to `first` is never read
  --> src/main.rs:8:17
   |
8  |                 first = false;
   |                 ^^^^^
...
17 |     delimit!();
   |     ----------- in this macro invocation
   |
   = note: `#[warn(unused_assignments)]` on by default
   = help: maybe it is overwritten before being read?
   = note: this warning originates in the macro `delimit` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 1 warning emitted
