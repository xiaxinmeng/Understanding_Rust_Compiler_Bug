plain
[01:00:21]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:00:21] [RUSTC-TIMING] panic_unwind test:false 0.280
[01:00:21] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-cloudabi`
[01:00:21] 
[01:00:24] error[E0599]: no function or associated item named `actually_monotonic` found for type `sys::cloudabi::time::Instant` in the current scope
[01:00:24]    --> src/libstd/time.rs:182:27
[01:00:24]     |
[01:00:24] 182 |         if time::Instant::actually_monotonic() {
[01:00:24]     |            |
[01:00:24]     |            |
[01:00:24]     |            function or associated item not found in `sys::cloudabi::time::Instant`
[01:00:24]     | 
[01:00:24]    ::: src/libstd/sys/cloudabi/time.rs:8:1
[01:00:24] 8   | pub struct Instant {
[01:00:24] 8   | pub struct Instant {
[01:00:24]     | ------------------ function or associated item `actually_monotonic` not found for this
[01:00:26] error: aborting due to previous error
[01:00:26] 
[01:00:26] For more information about this error, try `rustc --explain E0599`.
[01:00:26] [RUSTC-TIMING] std test:false 5.086
---
travis_time:end:0119c2a7:start=1546672709559221610,finish=1546672709566034009,duration=6812399
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:001f5858
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01862410
travis_time:start:01862410
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:082b124f
$ dmesg | grep -i kill
