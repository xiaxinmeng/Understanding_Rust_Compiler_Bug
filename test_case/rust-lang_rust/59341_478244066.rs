plain

[00:57:51] travis_time:end:stage2-rustdoc:start=1553950914712906417,finish=1553951046477065235,duration=131764158818

[00:57:51] [TIMING] Rustdoc { compiler: Compiler { stage: 2, host: "i686-unknown-freebsd" } } -- 131.765
[00:57:51] thread 'main' panicked at 'Error: File "/checkout/obj/build/i686-unknown-freebsd/stage2/lib/librustc_target-0e054c52ca24953d.so" not found!', src/bootstrap/lib.rs:1278:17
[00:57:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host i686-unknown-freebsd --target i686-unknown-freebsd
[00:57:51] Build completed unsuccessfully in 0:54:40
travis_time:end:0b4c366b:start=1553947574738304446,finish=1553951047255660875,duration=3472517356429
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:21421974:start=1553951048914358174,finish=1553951048925974554,duration=11616380
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05b5bb74
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16568bce
travis_time:start:16568bce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15712a9c
$ dmesg | grep -i kill
