
error: macro-expanded `extern crate` items cannot shadow names passed with `--extern`
  --> src/main.rs:3:9
   |
3  |         extern crate std as core;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
...
11 | define_other_core!();
   | --------------------- in this macro invocation
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at 'src/librustc_resolve/macros.rs:891:21: inconsistent resolution for a macro', /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/src/librustc_middle/util/bug.rs:34:26
