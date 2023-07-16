plain
[00:55:39]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:55:39] [RUSTC-TIMING] panic_unwind test:false 0.267
[00:55:39] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-cloudabi`
[00:55:39] 
[00:55:40] error[E0432]: unresolved import `sys::time::dur2intervals`
[00:55:40]   --> src/libstd/sys/cloudabi/condvar.rs:16:5
[00:55:40] 16 | use sys::time::dur2intervals;
[00:55:40] 16 | use sys::time::dur2intervals;
[00:55:40]    |     ^^^^^^^^^^^^^^^^^^^^^^^^ no `dur2intervals` in `sys::cloudabi::time`
[00:55:40] 
[00:55:40] error[E0432]: unresolved import `sys::time::dur2intervals`
[00:55:40]   --> src/libstd/sys/cloudabi/thread.rs:19:5
[00:55:40] 19 | use sys::time::dur2intervals;
[00:55:40] 19 | use sys::time::dur2intervals;
[00:55:40]    |     ^^^^^^^^^^^^^^^^^^^^^^^^ no `dur2intervals` in `sys::cloudabi::time`
[00:55:44] error: aborting due to 2 previous errors
[00:55:44] 
[00:55:44] For more information about this error, try `rustc --explain E0432`.
[00:55:44] [RUSTC-TIMING] std test:false 4.412
---
travis_time:end:1105928a:start=1544704463754479568,finish=1544704463764222095,duration=9742527
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2bfcad48
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cbc6355
travis_time:start:0cbc6355
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04edab62
$ dmesg | grep -i kill
