plain
[01:17:07]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:17:07] [RUSTC-TIMING] panic_unwind test:false 0.280
[01:17:07] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-redox`
[01:17:07] 
[01:17:11] error[E0599]: no method named `map` found for type `sys::redox::time::Timespec` in the current scope
[01:17:11]    --> libstd/sys/redox/time.rs:193:36
[01:17:11] 21  | struct Timespec {
[01:17:11] 21  | struct Timespec {
[01:17:11]     | --------------- method `map` not found for this
[01:17:11] ...
[01:17:11] 193 |         self.t.add_duration(other).map(|t| SystemTime { t })
[01:17:11]     |
[01:17:11]     = note: the method `map` exists but the following trait bounds were not satisfied:
[01:17:11]     = note: the method `map` exists but the following trait bounds were not satisfied:
[01:17:11]             `&mut sys::redox::time::Timespec : core::iter::Iterator`
[01:17:11]     = note: the following trait defines an item `map`, perhaps you need to implement it:
[01:17:11]             candidate #1: `core::iter::Iterator`
[01:17:11] 
[01:17:11] error: aborting due to previous error
---
travis_time:end:04468ea8:start=1542377798971280896,finish=1542377798980624878,duration=9343982
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00febc62
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0147cdf0
travis_time:start:0147cdf0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1cf88c40
$ dmesg | grep -i kill
