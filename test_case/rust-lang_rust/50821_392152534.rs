plain
[00:04:04]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:04:06]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:04:06]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:12]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:15] error[E0277]: `K` doesn't implement `core::fmt::Debug`
[00:04:15]      |
[00:04:15]      |
[00:04:15] 2278 |          .field("key", self.key())
[00:04:15]      |                        ^^^^^^^^^^ `K` cannot be formatted using `{:?}` because it doesn't implement `core::fmt::Debug`
[00:04:15]      |
[00:04:15]      = help: the trait `core::fmt::Debug` is not implemented for `K`
[00:04:15]      = help: consider adding a `where K: core::fmt::Debug` bound
[00:04:15]      = note: required for the cast to the object type `core::fmt::Debug`
[00:04:15] 
[00:04:15] error[E0277]: `V` doesn't implement `core::fmt::Debug`
[00:04:15]      |
[00:04:15]      |
[00:04:15] 2279 |          .field("value", self.get())
[00:04:15]      |                          ^^^^^^^^^^ `V` cannot be formatted using `{:?}` because it doesn't implement `core::fmt::Debug`
[00:04:15]      |
[00:04:15]      = help: the trait `core::fmt::Debug` is not implemented for `V`
[00:04:15]      = help: consider adding a `where V: core::fmt::Debug` bound
[00:04:15]      = note: required for the cast to the object type `core::fmt::Debug`
[00:04:17] error: aborting due to 2 previous errors
[00:04:17] 
[00:04:17] For more information about this error, try `rustc --explain E0277`.
[00:04:17] error: Could not compile `std`.
[00:04:17] error: Could not compile `std`.
[00:04:17] 
[00:04:17] Caused by:
[00:04:17]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=1eeccaae4c437516 -C extra-filename=-1eeccaae4c437516 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-472f8a98a21071c0.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-933c25e53a64afa3.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-de93819b3358210b.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-d6c788271c3d4cb5.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-7033dc785bf77e76.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-904ce106c1301515.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-48ff328ae5e9f1b4.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-71a0b5c1c68b7c91.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-d5e800cf2beb81d3.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-ecf5e4d4cc2515ca.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-469443ee7ab8941b.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-0e25c4bddec1c94e.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-0d1ebef792b1d9ca.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-7be48b1ecbc2ee28.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/.libs -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-7a2aed77e6522e53/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:04:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:17] expected success, got: exit code: 101
[00:04:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:04:17] travis_fold:end:stage0-std

[00:04:17] travis_time:end:stage0-std:start=1527274980881865755,finish=1527275036595429583,duration=55713563828


[00:04:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:17] Build completed unsuccessfully in 0:00:57
[00:04:17] Makefile:79: recipe for target 'tidy' failed
[00:04:17] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ab7dda2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
