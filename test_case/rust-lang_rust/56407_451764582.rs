plain
[00:25:55] [RUSTC-TIMING] panic_unwind test:false 0.302
[00:26:02] error: missing documentation for macro
[00:26:02]  --> src/libstd/../stdsimd/stdsimd/arch/detect/arch/aarch64.rs:6:1
[00:26:02]   |
[00:26:02] 6 | macro_rules! is_aarch64_feature_detected {
[00:26:02]   |
[00:26:02] note: lint level defined here
[00:26:02]  --> src/libstd/lib.rs:214:9
[00:26:02]   |
[00:26:02]   |
[00:26:02] 21| #![deny(missing_docs)]
[00:26:02] 
[00:26:04] error: aborting due to previous error
[00:26:04] 
[00:26:04] [RUSTC-TIMING] std test:false 8.659
---
travis_time:end:11c2303d:start=1546800471345656423,finish=1546800471359243396,duration=13586973
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01bde644
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0311ac00
travis_time:start:0311ac00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:005e3bde
$ dmesg | grep -i kill
