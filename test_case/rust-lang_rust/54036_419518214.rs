plain
[00:03:50] DirectMap2M:     3082240 kB
[00:03:50] DirectMap1G:    14680064 kB
[00:03:50] + python2.7 ../x.py dist --host aarch64-unknown-linux-gnu --target aarch64-unknown-linux-gnu
[00:03:50]     Finished dev [unoptimized] target(s) in 0.25s
[00:03:51] thread 'main' panicked at 'failed to find sha', libcore/option.rs:989:5
[00:03:51] travis_fold:end:log-system-info
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host aarch64-unknown-linux-gnu --target aarch64-unknown-linux-gnu
[00:03:51] Build completed unsuccessfully in 0:00:01
travis_time:end:1684db80:start=1536342708314694063,finish=1536342940317103245,duration=232002409182
---
travis_time:end:11648ebb:start=1536342940768440001,finish=1536342940778331463,duration=9891462
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fdb523b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14cb95b2
travis_time:start:14cb95b2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04efc280
$ dmesg | grep -i kill
