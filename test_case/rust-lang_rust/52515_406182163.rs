plain
[01:06:37] [TIMING] Analysis { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 2.534
[01:06:37] Dist src
[01:06:48] [TIMING] Src -- 11.134
[01:06:48] Create plain source tarball
[01:08:59] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:10:51] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:12:43] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:12:43] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:589:17
[01:12:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:12:43] Build completed unsuccessfully in 1:07:30
travis_time:end:060100c0:start=1531980959479563728,finish=1531985323281084576,duration=4363801520848

---
travis_time:end:0fd56ba0:start=1531985325561377875,finish=1531985325567960592,duration=6582717
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01d74d68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05bc4b9a
travis_time:start:05bc4b9a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00656100
$ dmesg | grep -i kill
