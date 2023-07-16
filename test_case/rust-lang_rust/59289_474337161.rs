plain
[01:51:55] [RUSTC-TIMING] crossbeam_channel test:false 2.108
[01:51:55]    Compiling ignore v0.4.6
[01:52:08] [RUSTC-TIMING] ignore test:false 12.702
[01:52:08]    Compiling cargo v0.36.0 (/checkout/src/tools/cargo)
[01:52:09] warning: the item `libc` is imported redundantly
[01:52:09]     |
[01:52:09] 310 |         use libc;
[01:52:09]     |             ^^^^
[01:52:09]     |
[01:52:09]     |
[01:52:09]     = note: #[warn(unused_imports)] on by default
[01:52:09] 
[01:52:09] warning: the item `OsStr` is imported redundantly
[01:52:09]    --> src/tools/cargo/src/cargo/util/paths.rs:221:9
[01:52:09]     |
[01:52:09] 2   | use std::ffi::{OsStr, OsString};
[01:52:09]     |                ----- the item `OsStr` is already imported here
[01:52:09] 221 |     use std::ffi::OsStr;
[01:52:09]     |         ^^^^^^^^^^^^^^^
[01:52:09] 
[01:55:24] [RUSTC-TIMING] cargo test:false 195.917
---
[01:55:51] [RUSTC-TIMING] rusty_fork test:false 2.983
[01:55:51]    Compiling proptest v0.8.7
[01:56:14] [RUSTC-TIMING] proptest test:false 22.388
[01:56:14]    Compiling cargo v0.36.0 (/checkout/src/tools/cargo)
[01:56:15] warning: the item `libc` is imported redundantly
[01:56:15]    --> src/tools/cargo/tests/testsuite/death.rs:134:9
[01:56:15] 134 |     use libc;
[01:56:15]     |         ^^^^
[01:56:15]     |
[01:56:15]     = note: #[warn(unused_imports)] on by default
[01:56:15]     = note: #[warn(unused_imports)] on by default
[01:56:15] 
[01:56:16] error: the item `libc` is imported redundantly
[01:56:16]     |
[01:56:16] 310 |         use libc;
[01:56:16]     |             ^^^^
[01:56:16]     |
[01:56:16]     |
[01:56:16] note: lint level defined here
[01:56:16]    --> src/tools/cargo/src/cargo/lib.rs:1:24
[01:56:16]     |
[01:56:16] 1   | #![cfg_attr(test, deny(warnings))]
[01:56:16]     |                        ^^^^^^^^
[01:56:16]     = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:56:16] 
[01:56:16] error: the item `OsStr` is imported redundantly
[01:56:16]    --> src/tools/cargo/src/cargo/util/paths.rs:221:9
[01:56:16]     |
[01:56:16] 2   | use std::ffi::{OsStr, OsString};
[01:56:16]     |                ----- the item `OsStr` is already imported here
[01:56:16] 221 |     use std::ffi::OsStr;
[01:56:16]     |         ^^^^^^^^^^^^^^^
[01:56:16] 
[01:56:23] error: aborting due to 2 previous errors
---
[01:57:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static"
[01:57:40] expected success, got: exit code: 101
[01:57:40] 
[01:57:40] 
[01:57:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:57:40] Build completed unsuccessfully in 0:26:26
[01:57:40] make: *** [check-aux] Error 1
[01:57:40] Makefile:50: recipe for target 'check-aux' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0109f080
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 19 12:12:09 UTC 2019
---
travis_time:end:03cdde48:start=1552997531338367328,finish=1552997531345228959,duration=6861631
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10e3c88b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b80c3ef
travis_time:start:1b80c3ef
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01218110
$ dmesg | grep -i kill
