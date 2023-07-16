plain
[01:13:58]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:13:58] [RUSTC-TIMING] panic_unwind test:false 0.281
[01:13:58] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-redox`
[01:13:58] 
[01:14:02] error[E0599]: no method named `map` found for type `sys::redox::time::Timespec` in the current scope
[01:14:02]    --> libstd/sys/redox/time.rs:193:36
[01:14:02] 21  | struct Timespec {
[01:14:02] 21  | struct Timespec {
[01:14:02]     | --------------- method `map` not found for this
[01:14:02] ...
[01:14:02] 193 |         self.t.add_duration(other).map(|t| SystemTime { t })
[01:14:02]     |
[01:14:02]     = note: the method `map` exists but the following trait bounds were not satisfied:
[01:14:02]     = note: the method `map` exists but the following trait bounds were not satisfied:
[01:14:02]             `&mut sys::redox::time::Timespec : core::iter::Iterator`
[01:14:02]     = note: the following trait defines an item `map`, perhaps you need to implement it:
[01:14:02]             candidate #1: `core::iter::Iterator`
[01:14:02] 
[01:14:02] error: aborting due to previous error
---
[01:14:02] 
[01:14:02] To learn more, run the command again with --verbose.
[01:14:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-redox" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:14:02] expected success, got: exit code: 101
[01:14:02] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[01:14:02] travis_fold:end:stage2-std

[01:14:02] travis_time:end:stage2-std:start=1542343362191920766,finish=1542343413595164716,duration=51403243950

---
travis_time:end:02ad9620:start=1542343415600548897,finish=1542343415609620272,duration=9071375
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0af297e4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0189cbf4
travis_time:start:0189cbf4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0291edfb
$ dmesg | grep -i kill
