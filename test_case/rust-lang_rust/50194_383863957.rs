plain
[00:02:54]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:02:57]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:02:57]    Compiling cmake v0.1.30
[00:02:57]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4560 |   impl_from_itself!(u8, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ---------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4561 |   impl_from_itself!(u16, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ----------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4562 |   impl_from_itself!(u32, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ----------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4563 |   impl_from_itself!(u64, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ----------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4564 |   impl_from_itself!(u128, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ------------------------------------------------------------------------------ in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4566 |   impl_from_itself!(i8, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ---------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4567 |   impl_from_itself!(i16, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ----------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4568 |   impl_from_itself!(i32, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ----------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4569 |   impl_from_itself!(i64, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ----------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4570 |   impl_from_itself!(i128, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ------------------------------------------------------------------------------ in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4572 |   impl_from_itself!(f32, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ----------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] 
[00:03:01] error[E0520]: `from` specializes an item from a parent `impl`, but that item is not marked `default`
[00:03:01]     --> libcore/num/mod.rs:4553:13
[00:03:01]      |
[00:03:01] 4553 | /             fn from(nb: $nb) -> $nb {
[00:03:01] 4554 | |                 nb
[00:03:01] 4555 | |             }
[00:03:01]      | |_____________^ cannot specialize default item `from`
[00:03:01] ...
[00:03:01] 4573 |   impl_from_itself!(f64, #[stable(feature = "from_itself", since = "1.27.0")]);
[00:03:01]      |   ----------------------------------------------------------------------------- in this macro invocation
[00:03:01]     ::: libcore/convert.rs:402:1
[00:03:01]      |
[00:03:01]      |
[00:03:01] 402  | / impl<T> From<T> for T {
[00:03:01] 403  | |     fn from(t: T) -> T { t }
[00:03:01] 404  | | }
[00:03:01]      | |_- parent `impl` is here
[00:03:01]      |
[00:03:01]      = note: to specialize, `from` in the parent `impl` must be marked `default`
[00:03:01] error: aborting due to 12 previous errors
[00:03:01] 
[00:03:01] For more information about this error, try `rustc --explain E0520`.
[00:03:01] error: Could not compile `core`.
[00:03:01] error: Could not compile `core`.
[00:03:01] 
[00:03:01] Caused by:
[00:03:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1b8789e893adb899 -C extra-filename=-1b8789e893adb899 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:01] warning: build failed, waiting for other jobs to finish...
[00:03:02] error: build failed
[00:03:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:02] expected success, got: exit code: 101
[00:03:02] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:02] travis_fold:end:stage0-std

[00:03:02] travis_time:end:stage0-std:start=1524561598328528597,finish=1524561610242990133,duration=11914461536


[00:03:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:02] Build completed unsuccessfully in 0:00:13
[00:03:02] Makefile:79: recipe for target 'tidy' failed
[00:03:02] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08cd2e52
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
