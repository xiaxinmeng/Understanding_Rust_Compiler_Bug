plain
[00:03:14]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:30]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:03:30]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:03:30]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:31] error[E0658]: use of unstable library feature 'split_ascii_whitespace' (see issue #48656)
[00:03:31]    |
[00:03:31]    |
[00:03:31] 83 | pub use core::str::SplitAsciiWhitespace;
[00:03:31]    |
[00:03:31]    |
[00:03:31]    = help: add #![feature(split_ascii_whitespace)] to the crate attributes to enable
[00:03:31] 
[00:03:31] error[E0658]: use of unstable library feature 'split_ascii_whitespace' (see issue #48656)
[00:03:31]     |
[00:03:31]     |
[00:03:31] 168 |     str_core_methods!();
[00:03:31]     |
[00:03:31]     |
[00:03:31]     = help: add #![feature(split_ascii_whitespace)] to the crate attributes to enable
[00:03:31] 
[00:03:32]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:32]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:34] error: aborting due to 2 previous errors
[00:03:34] error: aborting due to 2 previous errors
[00:03:34] 
[00:03:34] For more information about this error, try `rustc --explain E0658`.
[00:03:34] error: Could not compile `alloc`.
[00:03:34] 
[00:03:34] Caused by:
[00:03:34]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=5c6ca57f52fc716b -C extra-filename=-5c6ca57f52fc716b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-fb1e36473ec4786e.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-bad063b3019d016c.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-af41331a61619951/out` (exit code: 101)
[00:03:37] error: build failed
[00:03:37] error: build failed
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:37] expected success, got: exit code: 101
[00:03:37] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:37] travis_fold:end:stage0-std

[00:03:37] travis_time:end:stage0-std:start=1525582438216033631,finish=1525582475852451658,duration=37636418027


[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:39
[00:03:37] Makefile:79: recipe for target 'tidy' failed
[00:03:37] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0111ae34
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
