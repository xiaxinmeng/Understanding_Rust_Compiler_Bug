plain
[01:20:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:20:24] 
[01:20:24] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:20:24] 
[01:20:24] note: compiler flags: -Z always-encode-mir -Z mir-emit-retag -Z mir-opt-level=0 -C opt-level=2 -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[01:20:24] note: some of the compiler flags provided by cargo are hidden
[01:20:24] 
[01:20:24] [RUSTC-TIMING] clippy_lints test:false 8.659
[01:20:24] error: Could not compile `clippy_lints`.
---
[01:24:59] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:24:59] 
[01:24:59] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:24:59] 
[01:24:59] note: compiler flags: -Z always-encode-mir -Z mir-emit-retag -Z mir-opt-level=0 -C opt-level=2 -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[01:24:59] note: some of the compiler flags provided by cargo are hidden
[01:24:59] 
[01:24:59] [RUSTC-TIMING] clippy_lints test:false 12.181
[01:24:59] error: Could not compile `clippy_lints`.
---
[01:32:13] Verifying status of rustfmt...
[01:32:13] Verifying status of clippy-driver...
[01:32:13] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:32:13] 
[01:32:13] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:32:13] 
[01:32:13] If you do intend to update 'clippy-driver', please check the error messages above and
[01:32:13] commit another update.
[01:32:13] 
[01:32:13] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:32:13] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:32:13] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:0589d082
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 20 01:32:00 UTC 2019
---
travis_time:end:312634f1:start=1547947921223721965,finish=1547947921230439909,duration=6717944
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06ee4ac0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10b269d0
travis_time:start:10b269d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:070af196
$ dmesg | grep -i kill
