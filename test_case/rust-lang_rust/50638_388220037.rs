plain
[00:03:42]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:47] error[E0308]: mismatched types
[00:03:47]    --> libstd/sys/unix/fs.rs:473:28
[00:03:47]     |
[00:03:47] 473 |               if need_to_set {
[00:03:47]     |  ____________________________^
[00:03:47] 474 | |                 fd.set_cloexec()?;
[00:03:47] 475 | |             }
[00:03:47]     | |_____________^ expected enum `core::result::Result`, found ()
[00:03:47]     |
[00:03:47]     = note: expected type `core::result::Result<(), io::error::Error>`
[00:03:47]                found type `()`
[00:03:48] error: aborting due to previous error
[00:03:48] 
[00:03:48] For more information about this error, try `rustc --explain E0308`.
[00:03:48] error: Could not compile `std`.
[00:03:48] error: Could not compile `std`.
[00:03:48] 
[00:03:48] Caused by:
[00:03:48]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=761fbae3650d6e55 -C extra-filename=-761fbae3650d6e55 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-1d7f9c9946b2c2bc.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-48544fd5cf3dcf07.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-aa82747b5f7140a1.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-5657f35033ebd4c8.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-fb1e36473ec4786e.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-3742d05cb3a356c6.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-aa228a67f9f2430d.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-f9142bcb925af734.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-e6ced10bca67cf14.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-1ba2d219736013ac.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-96668d527ecf7799.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-600098a993bc3037.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-8c40d3ae621dd1f7.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-1e788572dd7deb7a.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/.libs -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-54b62960ded3ce36/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:03:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:48] expected success, got: exit code: 101
[00:03:48] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:48] travis_fold:end:stage0-std

[00:03:48] travis_time:end:stage0-std:start=1525996514396008291,finish=1525996558888250756,duration=44492242465


[00:03:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:48] Build completed unsuccessfully in 0:00:46
[00:03:48] make: *** [tidy] Error 1
[00:03:48] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2df32971
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
