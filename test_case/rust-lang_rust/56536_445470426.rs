plain
[01:18:58]    Compiling git2-curl v0.8.2
[01:18:59] [RUSTC-TIMING] ignore test:false 27.947
[01:19:00] [RUSTC-TIMING] git2_curl test:false 2.074
[01:19:00]    Compiling cargo v0.33.0 (/checkout/src/tools/cargo)
[01:19:04] warning: use of deprecated item 'failure::core::str::<impl str>::trim_right': superseded by `trim_end`
[01:19:04]    --> src/tools/cargo/src/cargo/core/compiler/custom_build.rs:469:45
[01:19:04]     |
[01:19:04] 469 |                 (Some(a), Some(b)) => (a, b.trim_right()),
[01:19:04]     |
[01:19:04]     = note: #[warn(deprecated)] on by default
[01:19:04] 
[01:22:04] [RUSTC-TIMING] cargo test:false 183.582
---
[01:22:31] [RUSTC-TIMING] num_traits test:false 3.145
[01:22:31]    Compiling proptest v0.8.7
[01:22:53] [RUSTC-TIMING] proptest test:false 22.167
[01:22:53]    Compiling cargo v0.33.0 (/checkout/src/tools/cargo)
[01:22:57] error: use of deprecated item 'failure::core::str::<impl str>::trim_right': superseded by `trim_end`
[01:22:57]    --> src/tools/cargo/src/cargo/core/compiler/custom_build.rs:469:45
[01:22:57]     |
[01:22:57] 469 |                 (Some(a), Some(b)) => (a, b.trim_right()),
[01:22:57]     |
[01:22:57] note: lint level defined here
[01:22:57]    --> src/tools/cargo/src/cargo/lib.rs:1:24
[01:22:57]     |
---
[01:24:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static"
[01:24:16] expected success, got: exit code: 101
[01:24:16] 
[01:24:16] 
[01:24:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:24:16] Build completed unsuccessfully in 0:26:59
[01:24:16] make: *** [check-aux] Error 1
[01:24:16] Makefile:60: recipe for target 'check-aux' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00f47191
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  8 16:15:25 UTC 2018
---
travis_time:end:01cc1e45:start=1544285726571070908,finish=1544285726580276770,duration=9205862
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02bc3ae2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2091b7a8
travis_time:start:2091b7a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a7ff02e
$ dmesg | grep -i kill
