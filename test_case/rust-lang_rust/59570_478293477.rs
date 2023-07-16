plain
[01:09:25] 
[01:09:26] error: hidden lifetime parameters in types are deprecated
[01:09:26]    --> src/libstd/sys/wasi/fd.rs:284:24
[01:09:26]     |
[01:09:26] 284 |         ri_data: &mut [IoVecMut],
[01:09:26] 
[01:09:31] error: aborting due to previous error
[01:09:31] 
[01:09:31] [RUSTC-TIMING] std test:false 6.175
---
travis_time:end:14eb772e:start=1553983722671698088,finish=1553983722679876870,duration=8178782
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b6fb149
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:160b0226
travis_time:start:160b0226
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05dde3c0
$ dmesg | grep -i kill
