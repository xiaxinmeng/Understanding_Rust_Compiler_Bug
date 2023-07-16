plain
[00:03:55]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:04:01]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:04:01]    Compiling cmake v0.1.31
[00:04:01]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:04:05] error[E0053]: method `div` has an incompatible type for trait
[00:04:05]     |
[00:04:05]     |
[00:04:05] 619 |     fn div(self, rhs: Duration) -> Duration {
[00:04:05]     |                                    ^^^^^^^^ expected f64, found struct `time::Duration`
[00:04:05]    ::: libcore/ops/arith.rs:441:31
[00:04:05]     |
[00:04:05]     |
[00:04:05] 441 |     fn div(self, rhs: RHS) -> Self::Output;
[00:04:05]     |                               ------------ type in trait
[00:04:05]     |
[00:04:05]     = note: expected type `fn(time::Duration, time::Duration) -> f64`
[00:04:05]                found type `fn(time::Duration, time::Duration) -> time::Duration`
[00:04:05]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:04:06] error: aborting due to previous error
[00:04:06] 
[00:04:06] For more information about this error, try `rustc --explain E0053`.
