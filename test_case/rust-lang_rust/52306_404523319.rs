plain
[00:19:54]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:19:55]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:19:56]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:19:56]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:19:58] thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:345:21
[00:19:58] 
[00:19:58] error: internal compiler error: unexpected panic
[00:19:58] 
[00:19:58] 
[00:19:58] note: the compiler unexpectedly panicked. this is a bug.
[00:19:58] 
[00:19:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:19:58] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:19:58] 
[00:19:58] 
[00:19:58] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:19:58] 
[00:19:58] note: some of the compiler flags provided by cargo are hidden
[00:19:58] error: Could not compile `core`.
[00:19:58] 
[00:19:58] Caused by:
[00:19:58]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1cbcabaa1ea822b5 -C extra-filename=-1cbcabaa1ea822b5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:19:58]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1cbcabaa1ea822b5 -C extra-filename=-1cbcabaa1ea822b5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:19:58] warning: build failed, waiting for other jobs to finish...
[00:19:59] error: build failed
[00:19:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:19:59] expected success, got: exit code: 101
[00:19:59] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:19:59] travis_fold:end:stage1-std

[00:19:59] travis_time:end:stage1-std:start=1531404199840154760,finish=1531404215303774964,duration=15463620204


[00:19:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:59] Build completed unsuccessfully in 0:16:13
[00:19:59] Makefile:28: recipe for target 'all' failed
[00:19:59] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f5190b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:24441ef8:start=1531404215840513787,finish=1531404215846402189,duration=5888402
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0178f6d4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04022fb8
$ dmesg | grep -i kill
