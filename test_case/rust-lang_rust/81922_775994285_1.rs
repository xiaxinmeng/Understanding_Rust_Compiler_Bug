
warning: unused variable: `a`
 --> src/main.rs:3:13
  |
3 |         let $i = 42;
  |             ^^ help: if this is intentional, prefix it with an underscore: `_a`
...
9 |     foo!(a);
  |     -------- in this macro invocation
  |
  = note: `#[warn(unused_variables)]` on by default
  = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 1 warning emitted
