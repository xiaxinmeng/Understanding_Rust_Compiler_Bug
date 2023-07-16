plain
[00:03:40] + python2.7 ../x.py dist --host aarch64-unknown-linux-gnu --target aarch64-unknown-linux-gnu
[00:03:41]     Finished dev [unoptimized] target(s) in 0.19s
[00:03:42] travis_fold:end:log-system-info
git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[00:03:42] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "target `x86_64-unknown-linux-gnu` is not configured as a host, only as a target"', src/libcore/result.rs:999:5
[00:03:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host aarch64-unknown-linux-gnu --target aarch64-unknown-linux-gnu
[00:03:42] Build completed unsuccessfully in 0:00:01
travis_time:end:0b14f16c:start=1561180095610756890,finish=1561180320740464693,duration=225129707803
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:0294ffd2:start=1561180321491628837,finish=1561180321497636306,duration=6007469
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00160e70
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0528b291
travis_time:start:0528b291
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0010f2c0
$ dmesg | grep -i kill
