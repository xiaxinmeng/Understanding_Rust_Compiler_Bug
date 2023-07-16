plain
[00:03:20]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:21]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:21]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:25]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:26] error: unexpected token `||` after pattern
[00:03:26]    --> libstd/sys/unix/fs.rs:894:40
[00:03:26]     |
[00:03:26] 894 |                     Some(libc::ENOSYS) || Some(libc::EPERM) => {
[00:03:26]     |                                        ^^ help: use a single `|` to specify multiple patterns: `|`
[00:03:26] error: aborting due to previous error
[00:03:26] 
[00:03:26] error: Could not compile `std`.
[00:03:26] 
[00:03:26] 
[00:03:26] Caused by:
[00:03:26]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=fc6b9a3d7065b2e2 -C extra-filename=-fc6b9a3d7065b2e2 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-e9cdce497aae9e81.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-90a13cda2e54742f.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-18039a1361938ca9.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-bd02867d7573c11e.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-a72c0fa0b2d5877e.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-223f966213a60acb.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-d6cd5f8b78fddf12.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-386e278237d725f5.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-5e6c2ac297f2c2e3.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-a2f6e9f263c82267.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-5f4d07ea9b3edda4.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-2bd12fd5ac9768f9.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-ccd411645051f1ef.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-ce1ae9dbeb02af61.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace -l static=backtrace -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-ffd422941bf53e42/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:03:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:26] expected success, got: exit code: 101
[00:03:26] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:26] travis_fold:end:stage0-std

[00:03:26] travis_time:end:stage0-std:start=1527838318998576622,finish=1527838351287992777,duration=32289416155


[00:03:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:26] Build completed unsuccessfully in 0:00:33
[00:03:26] Makefile:79: recipe for target 'tidy' failed
[00:03:26] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3261c588
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
