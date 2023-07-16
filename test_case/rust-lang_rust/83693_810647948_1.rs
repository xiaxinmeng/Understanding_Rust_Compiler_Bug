
error[E0412]: cannot find type `x` in this scope
 --> src/lib.rs:3:6
  |
3 |     <x as Fn(&usize)>::call
  |      ^ not found in this scope

error[E0229]: associated type bindings are not allowed here
 --> src/lib.rs:3:11
  |
3 |     <x as Fn(&usize)>::call
  |           ^^^^^^^^^^ associated type not allowed here

thread 'rustc' panicked at 'assertion failed: !substs.has_escaping_bound_vars()', compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:1475:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
