plain
[00:14:10] [RUSTC-TIMING] rustc_target test:false 23.454
[00:14:11] [RUSTC-TIMING] rustc_errors test:false 17.635
[00:15:20] [RUSTC-TIMING] syntax test:false 68.801
[00:15:20]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:15:44] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105011 ms
[00:15:58] [RUSTC-TIMING] syntax_ext test:false 38.317
[00:20:09] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:21:54] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 139.
travis_time:start:01498b19
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr  6 13:13:15 UTC 2019
---
travis_time:end:01979940:start=1554556396680703543,finish=1554556396690862177,duration=10158634
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04fda73d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00e57656
travis_time:start:00e57656
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:012cf744
$ dmesg | grep -i kill
