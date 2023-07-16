plain
[00:02:45]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:02:45]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:02:45]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:02:47]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4560 | impl_from_itself!(u8, #[stable(since = "1.27.0")]);
[00:02:49]      | --------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4561 | impl_from_itself!(u16, #[stable(since = "1.27.0")]);
[00:02:49]      | ---------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4562 | impl_from_itself!(u32, #[stable(since = "1.27.0")]);
[00:02:49]      | ---------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4563 | impl_from_itself!(u64, #[stable(since = "1.27.0")]);
[00:02:49]      | ---------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4564 | impl_from_itself!(u128, #[stable(since = "1.27.0")]);
[00:02:49]      | ----------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4566 | impl_from_itself!(i8, #[stable(since = "1.27.0")]);
[00:02:49]      | --------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4567 | impl_from_itself!(i16, #[stable(since = "1.27.0")]);
[00:02:49]      | ---------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4568 | impl_from_itself!(i32, #[stable(since = "1.27.0")]);
[00:02:49]      | ---------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4569 | impl_from_itself!(i64, #[stable(since = "1.27.0")]);
[00:02:49]      | ---------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4570 | impl_from_itself!(i128, #[stable(since = "1.27.0")]);
[00:02:49]      | ----------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4572 | impl_from_itself!(f32, #[stable(since = "1.27.0")]);
[00:02:49]      | ---------------------------------------------------- in this macro invocation
[00:02:49] 
[00:02:49] error[E0546]: missing 'feature'
[00:02:49]     --> libcore/num/mod.rs:4550:9
[00:02:49]      |
[00:02:49] 4550 |         #[$attr]
[00:02:49] ...
[00:02:49] ...
[00:02:49] 4573 | impl_from_itself!(f64, #[stable(since = "1.27.0")]);
[00:02:49]      | ---------------------------------------------------- in this macro invocation
[00:02:51]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:02:51]    Compiling cmake v0.1.30
[00:02:51]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:02:51]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4560 |   impl_from_itself!(u8, #[stable(since = "1.27.0")]);
[00:02:55]      |   --------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4561 |   impl_from_itself!(u16, #[stable(since = "1.27.0")]);
[00:02:55]      |   ---------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4562 |   impl_from_itself!(u32, #[stable(since = "1.27.0")]);
[00:02:55]      |   ---------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4563 |   impl_from_itself!(u64, #[stable(since = "1.27.0")]);
[00:02:55]      |   ---------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4564 |   impl_from_itself!(u128, #[stable(since = "1.27.0")]);
[00:02:55]      |   ----------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4566 |   impl_from_itself!(i8, #[stable(since = "1.27.0")]);
[00:02:55]      |   --------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4567 |   impl_from_itself!(i16, #[stable(since = "1.27.0")]);
[00:02:55]      |   ---------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4568 |   impl_from_itself!(i32, #[stable(since = "1.27.0")]);
[00:02:55]      |   ---------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4569 |   impl_from_itself!(i64, #[stable(since = "1.27.0")]);
[00:02:55]      |   ---------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4570 |   impl_from_itself!(i128, #[stable(since = "1.27.0")]);
[00:02:55]      |   ----------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4572 |   impl_from_itself!(f32, #[stable(since = "1.27.0")]);
[00:02:55]      |   ---------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:55] 
[00:02:55] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:02:55]     --> libcore/num/mod.rs:4553:13
[00:02:55]      |
[00:02:55] 4553 | /             fn from(nb: $nb) -> $nb {
[00:02:55] 4554 | |                 nb
[00:02:55] 4555 | |             }
[00:02:55]      | |_____________^ cannot specialize default item `from`
[00:02:55] ...
[00:02:55] 4573 |   impl_from_itself!(f64, #[stable(since = "1.27.0")]);
[00:02:55]      |   ---------------------------------------------------- in this macro invocation
[00:02:55]     ::: libcore/convert.rs:402:1
[00:02:55]      |
[00:02:55]      |
[00:02:55] 402  | / impl<T> From<T> for T {
[00:02:55] 403  | |     fn from(t: T) -> T { t }
[00:02:55] 404  | | }
[00:02:55]      | |_- parent `impl` is here
[00:02:55]      |
[00:02:55]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:02:56] error: aborting due to 24 previous errors
[00:02:56] 
[00:02:56] Some errors occurred: E0520, E0546.
[00:02:56] For more information about an error, try `rustc --explain E0520`.
[00:02:56] For more information about an error, try `rustc --explain E0520`.
[00:02:56] error: Could not compile `core`.
[00:02:56] 
[00:02:56] Caused by:
[00:02:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1b8789e893adb899 -C extra-filename=-1b8789e893adb899 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:02:56] warning: build failed, waiting for other jobs to finish...
[00:03:09] error: build failed
[00:03:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:09] expected success, got: exit code: 101
[00:03:09] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:09] travis_fold:end:stage0-std

[00:03:09] travis_time:end:stage0-std:start=1524560775177751058,finish=1524560799651037700,duration=24473286642


[00:03:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:09] Build completed unsuccessfully in 0:00:25
[00:03:09] Makefile:79: recipe for target 'tidy' failed
[00:03:09] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00169a4c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
