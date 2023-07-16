plain
[01:20:58]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:21:24] [RUSTC-TIMING] core test:false 31.161
[01:21:24]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[01:21:24] [RUSTC-TIMING] rustc_std_workspace_core test:false 0.055
[01:21:24] error: could not find native static library `c`, perhaps an -L flag is missing?
[01:21:24] error: aborting due to previous error
[01:21:24] 
[01:21:24] [RUSTC-TIMING] libc test:false 0.174
[01:21:24] error: Could not compile `libc`.
---
travis_time:end:151207e6:start=1556679843417166760,finish=1556679843423988956,duration=6822196
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b884722
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18e83961
travis_time:start:18e83961
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:043cdcb0
$ dmesg | grep -i kill
