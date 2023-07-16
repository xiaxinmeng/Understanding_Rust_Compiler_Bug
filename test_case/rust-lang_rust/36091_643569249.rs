
warning: unused import: `$i`
 --> src/lib.rs:1:44
  |
1 | macro_rules! m { ($($i:ident)*) => { use $($i)::*; } }
  |                                            ^^
2 | 
3 | m!(std cmp);
  | ------------ in this macro invocation
  |
  = note: `#[warn(unused_imports)]` on by default
  = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

