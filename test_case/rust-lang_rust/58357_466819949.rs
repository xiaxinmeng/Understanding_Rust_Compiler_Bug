plain
[01:04:38] [RUSTC-TIMING] panic_unwind test:false 0.305
[01:04:38] warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`
[01:04:38] 
[01:04:41] error[E0308]: mismatched types
[01:04:41]    --> src/libstd/sys/sgx/net.rs:111:19
[01:04:41] 111 |         self.read(buf)
[01:04:41]     |                   ^^^ types differ in mutability
[01:04:41]     |
[01:04:41]     = note: expected type `&mut [u8]`
[01:04:41]     = note: expected type `&mut [u8]`
[01:04:41]                found type `&io::IoVecMut<'_>`
[01:04:41] error[E0308]: mismatched types
[01:04:41] error[E0308]: mismatched types
[01:04:41]    --> src/libstd/sys/sgx/net.rs:123:19
[01:04:41] 123 |         self.read(buf)
[01:04:41]     |                   ^^^ types differ in mutability
[01:04:41]     |
[01:04:41]     = note: expected type `&mut [u8]`
[01:04:41]     = note: expected type `&mut [u8]`
[01:04:41]                found type `&io::IoVec<'_>`
[01:04:44] error: aborting due to 2 previous errors
[01:04:44] 
[01:04:44] For more information about this error, try `rustc --explain E0308`.
[01:04:44] [RUSTC-TIMING] std test:false 5.752
---
travis_time:end:027323f4:start=1551044304323950905,finish=1551044304362673927,duration=38723022
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e9acc63
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10ebc355
travis_time:start:10ebc355
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0182a9cc
$ dmesg | grep -i kill
