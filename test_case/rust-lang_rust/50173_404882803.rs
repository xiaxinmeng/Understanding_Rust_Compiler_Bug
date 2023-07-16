plain
[00:17:21]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:17:22]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:17:23]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:17:23]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:17:26] thread 'main' panicked at 'assertion failed: !value.has_escaping_regions()', librustc/ty/sty.rs:754:9
[00:17:26] 
[00:17:26] error: internal compiler error: unexpected panic
[00:17:26] 
[00:17:26] 
[00:17:26] note: the compiler unexpectedly panicked. this is a bug.
[00:17:26] 
[00:17:26] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:17:26] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:17:26] 
[00:17:26] 
[00:17:26] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:17:26] 
[00:17:26] note: some of the compiler flags provided by cargo are hidden
[00:17:26] error: Could not compile `core`.
[00:17:26] 
[00:17:26] Caused by:
[00:17:26]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1cbcabaa1ea822b5 -C extra-filename=-1cbcabaa1ea822b5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:17:26]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1cbcabaa1ea822b5 -C extra-filename=-1cbcabaa1ea822b5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:17:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:17:26] expected success, got: exit code: 101
[00:17:26] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:17:26] travis_fold:end:stage1-std

[00:17:26] travis_time:end:stage1-std:start=1531498967729240718,finish=1531498981526380563,duration=13797139845


[00:17:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:26] Build completed unsuccessfully in 0:13:59
[00:17:26] Makefile:28: recipe for target 'all' failed
[00:17:26] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0eddafa0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:09a2ba7b:start=1531498982105784632,finish=1531498982112169948,duration=6385316
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01c1ee4e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0172d4ac
$ dmesg | grep -i kill
