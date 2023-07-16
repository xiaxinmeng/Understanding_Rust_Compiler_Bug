plain
[01:01:31]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:01:32] [RUSTC-TIMING] panic_unwind test:false 0.290
[01:01:32] warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`
[01:01:32] 
[01:01:35] error[E0624]: method `into_inner` is private
[01:01:35]   --> src/libstd/sys/sgx/ext/arch.rs:44:25
[01:01:35]    |
[01:01:35] 44 |             0 => Ok(out.into_inner()),
[01:01:35] 
[01:01:35] error[E0624]: method `into_inner` is private
[01:01:35] error[E0624]: method `into_inner` is private
[01:01:35]   --> src/libstd/sys/sgx/ext/arch.rs:72:16
[01:01:35] 72 |         report.into_inner()
[01:01:35]    |                ^^^^^^^^^^
[01:01:35] 
[01:01:37] error: aborting due to 2 previous errors
---
travis_time:end:0ddf7486:start=1549821839519904601,finish=1549821839535653988,duration=15749387
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1aaf1660
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:33a39318
travis_time:start:33a39318
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0722a705
$ dmesg | grep -i kill
