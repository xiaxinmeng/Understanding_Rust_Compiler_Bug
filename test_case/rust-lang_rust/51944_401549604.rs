plain
[00:03:23]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:36]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:03:36]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:03:36]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:37] warning: conflicting implementations of trait `core::convert::Into<core::task::FutureObj<_>>` for type `boxed::PinBox<_>`: (E0119)
[00:03:37]    --> liballoc/boxed.rs:953:1
[00:03:37]     |
[00:03:37] 953 | impl<T, F: Future<Output = T> + Send + 'static> Into<FutureObj<T>> for PinBox<F> {
[00:03:37]     |
[00:03:37]     = note: #[warn(incoherent_fundamental_impls)] on by default
[00:03:37]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:03:37]     = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:03:37]     = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:03:37]     = note: conflicting implementation in crate `core`:
[00:03:37]             - impl<T, U> core::convert::Into<U> for T
[00:03:37]               where U: core::convert::From<T>;
[00:03:37]     = note: downstream crates may implement trait `core::convert::From<boxed::PinBox<_>>` for type `core::task::FutureObj<_>`
[00:03:37] 
[00:03:37] warning: conflicting implementations of trait `core::convert::Into<core::task::FutureObj<_>>` for type `boxed::Box<_>`: (E0119)
[00:03:37]    --> liballoc/boxed.rs:960:1
[00:03:37]     |
[00:03:37] 960 | impl<T, F: Future<Output = T> + Send + 'static> Into<FutureObj<T>> for Box<F> {
[00:03:37]     |
[00:03:37]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:03:37]     = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:03:37]     = note: conflicting implementation in crate `core`:
[00:03:37]     = note: conflicting implementation in crate `core`:
[00:03:37]             - impl<T, U> core::convert::Into<U> for T
[00:03:37]               where U: core::convert::From<T>;
[00:03:37]     = note: downstream crates may implement trait `core::convert::From<boxed::Box<_>>` for type `core::task::FutureObj<_>`
[00:03:37] 
[00:03:37] warning: conflicting implementations of trait `core::convert::Into<core::task::LocalFutureObj<_>>` for type `boxed::PinBox<_>`: (E0119)
[00:03:37]    --> liballoc/boxed.rs:967:1
[00:03:37]     |
[00:03:37] 967 | impl<T, F: Future<Output = T> + 'static> Into<LocalFutureObj<T>> for PinBox<F> {
[00:03:37]     |
[00:03:37]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:03:37]     = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:03:37]     = note: conflicting implementation in crate `core`:
[00:03:37]     = note: conflicting implementation in crate `core`:
[00:03:37]             - impl<T, U> core::convert::Into<U> for T
[00:03:37]               where U: core::convert::From<T>;
[00:03:37]     = note: downstream crates may implement trait `core::convert::From<boxed::PinBox<_>>` for type `core::task::LocalFutureObj<_>`
[00:03:37] 
[00:03:37] warning: conflicting implementations of trait `core::convert::Into<core::task::LocalFutureObj<_>>` for type `boxed::Box<_>`: (E0119)
[00:03:37]    --> liballoc/boxed.rs:974:1
[00:03:37]     |
[00:03:37] 974 | impl<T, F: Future<Output = T> + 'static> Into<LocalFutureObj<T>> for Box<F> {
[00:03:37]     |
[00:03:37]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:03:37]     = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:03:37]     = note: conflicting implementation in crate `core`:
[00:03:37]     = note: conflicting implementation in crate `core`:
[00:03:37]             - impl<T, U> core::convert::Into<U> for T
[00:03:37]               where U: core::convert::From<T>;
[00:03:37]     = note: downstream crates may implement trait `core::convert::From<boxed::Box<_>>` for type `core::task::LocalFutureObj<_>`
[00:03:37]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:37]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:41]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:55]     Finished release [optimized] target(s) in 45.87s
---
[00:21:45]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:22:32]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:22:32]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:22:32]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:22:33] error: conflicting implementations of trait `core::convert::Into<core::task::FutureObj<_>>` for type `boxed::PinBox<_>`: (E0119)
[00:22:33]    --> liballoc/boxed.rs:953:1
[00:22:33]     |
[00:22:33] 953 | impl<T, F: Future<Output = T> + Send + 'static> Into<FutureObj<T>> for PinBox<F> {
[00:22:33]     |
[00:22:33]     |
[00:22:33]     = note: #[deny(incoherent_fundamental_impls)] on by default
[00:22:33]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:22:33]     = note: conflicting implementation in crate `core`:
[00:22:33]     = note: conflicting implementation in crate `core`:
[00:22:33]             - impl<T, U> core::convert::Into<U> for T
[00:22:33]               where U: core::convert::From<T>;
[00:22:33]     = note: downstream crates may implement trait `core::convert::From<boxed::PinBox<_>>` for type `core::task::FutureObj<_>`
[00:22:33] 
[00:22:33] error: conflicting implementations of trait `core::convert::Into<core::task::FutureObj<_>>` for type `boxed::Box<_>`: (E0119)
[00:22:33]    --> liballoc/boxed.rs:960:1
[00:22:33]     |
[00:22:33] 960 | impl<T, F: Future<Output = T> + Send + 'static> Into<FutureObj<T>> for Box<F> {
[00:22:33]     |
[00:22:33]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:22:33]     = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:22:33]     = note: conflicting implementation in crate `core`:
[00:22:33]     = note: conflicting implementation in crate `core`:
[00:22:33]             - impl<T, U> core::convert::Into<U> for T
[00:22:33]               where U: core::convert::From<T>;
[00:22:33]     = note: downstream crates may implement trait `core::convert::From<boxed::Box<_>>` for type `core::task::FutureObj<_>`
[00:22:33] 
[00:22:33] error: conflicting implementations of trait `core::convert::Into<core::task::LocalFutureObj<_>>` for type `boxed::PinBox<_>`: (E0119)
[00:22:33]    --> liballoc/boxed.rs:967:1
[00:22:33]     |
[00:22:33] 967 | impl<T, F: Future<Output = T> + 'static> Into<LocalFutureObj<T>> for PinBox<F> {
[00:22:33]     |
[00:22:33]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:22:33]     = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:22:33]     = note: conflicting implementation in crate `core`:
[00:22:33]     = note: conflicting implementation in crate `core`:
[00:22:33]             - impl<T, U> core::convert::Into<U> for T
[00:22:33]               where U: core::convert::From<T>;
[00:22:33]     = note: downstream crates may implement trait `core::convert::From<boxed::PinBox<_>>` for type `core::task::LocalFutureObj<_>`
[00:22:33] 
[00:22:33] error: conflicting implementations of trait `core::convert::Into<core::task::LocalFutureObj<_>>` for type `boxed::Box<_>`: (E0119)
[00:22:33]    --> liballoc/boxed.rs:974:1
[00:22:33]     |
[00:22:33] 974 | impl<T, F: Future<Output = T> + 'static> Into<LocalFutureObj<T>> for Box<F> {
[00:22:33]     |
[00:22:33]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:22:33]     = note: for more information, see issue #46205 <https://github.com/rust-lang/rust/issues/46205>
[00:22:33]     = note: conflicting implementation in crate `core`:
[00:22:33]     = note: conflicting implementation in crate `core`:
[00:22:33]             - impl<T, U> core::convert::Into<U> for T
[00:22:33]               where U: core::convert::From<T>;
[00:22:33]     = note: downstream crates may implement trait `core::convert::From<boxed::Box<_>>` for type `core::task::LocalFutureObj<_>`
[00:22:33] error: aborting due to 4 previous errors
[00:22:33] 
[00:22:33] error: Could not compile `alloc`.
[00:22:33] 
[00:22:33] 
[00:22:33] Caused by:
[00:22:33]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=629be8a8a92e38ff -C extra-filename=-629be8a8a92e38ff --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-e3f2b558276b578b.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d5d59b383a13b958.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-57f7f72e03e89590/out` (exit code: 101)
[00:22:33] error: build failed
[00:22:33] error: build failed
[00:22:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:22:33] travis_fold:end:stage1-std


[00:22:33] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9

[00:22:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:22:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:33] Build completed unsuccessfully in 0:17:41
[00:22:33] Build completed unsuccessfully in 0:17:41
[00:22:33] make: *** [all] Error 1
[00:22:33] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27be0d9e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:24110906:start=1530374440454319768,finish=1530374440461076786,duration=6757018
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b1cc28e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0afa7f60
$ dmesg | grep -i kill
