plain
[00:03:51]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:03:55] warning: unnecessary parentheses around assigned value
[00:03:55]    --> libcore/time.rs:621:22
[00:03:55]     |
[00:03:55] 621 |         let nanos1 = (NPS * (self.secs as f64) + (self.nanos as f64));
[00:03:55]     |
[00:03:55]     = note: #[warn(unused_parens)] on by default
[00:03:55] 
[00:03:55] warning: unnecessary parentheses around assigned value
[00:03:55] warning: unnecessary parentheses around assigned value
[00:03:55]    --> libcore/time.rs:622:22
[00:03:55]     |
[00:03:55] 622 |         let nanos2 = (NPS * (rhs.secs as f64) + (rhs.nanos as f64));
[00:03:55] 
[00:03:57]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:57]    Compiling cmake v0.1.31
[00:03:57]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
---
[00:18:35]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:18:36] error: unnecessary parentheses around assigned value
[00:18:36]    --> libcore/time.rs:621:22
[00:18:36]     |
[00:18:36] 621 |         let nanos1 = (NPS * (self.secs as f64) + (self.nanos as f64));
[00:18:36]     |
[00:18:36]     = note: `-D unused-parens` implied by `-D warnings`
[00:18:36] 
[00:18:36] error: unnecessary parentheses around assigned value
[00:18:36] error: unnecessary parentheses around assigned value
[00:18:36]    --> libcore/time.rs:622:22
[00:18:36]     |
[00:18:36] 622 |         let nanos2 = (NPS * (rhs.secs as f64) + (rhs.nanos as f64));
[00:18:36] 
[00:18:38]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:18:38]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:18:38]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
---
[00:18:52] Caused by:
[00:18:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1cbcabaa1ea822b5 -C extra-filename=-1cbcabaa1ea822b5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:18:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:18:52] expected success, got: exit code: 101
[00:18:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:18:52] travis_fold:end:stage1-std

[00:18:52] travis_time:end:stage1-std:start=1532826884464708842,finish=1532826907795491003,duration=23330782161


[00:18:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:52] Build completed unsuccessfully in 0:15:02
[00:18:52] Makefile:28: recipe for target 'all' failed
[00:18:52] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1246c863
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:09fea60c:start=1532826908394679009,finish=1532826908402812828,duration=8133819
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:012d5ebc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:025e0a61
travis_time:start:025e0a61
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0718980c
$ dmesg | grep -i kill
