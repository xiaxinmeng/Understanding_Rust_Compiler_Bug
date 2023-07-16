plain
[00:00:56]                                  Dload  Upload   Total   Spent    Left  Speed
[00:00:57] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100 1057k  100 1057k    0     0  1154k      0 --:--:-- --:--:-- --:--:-- 1154k
[00:00:57] usr/bin/dig: error while loading shared libraries: libcrypto.so.1.0.0: cannot open shared object file: No such file or directory

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 127.
travis_time:start:13435dd3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02119458:start=1533879501416111753,finish=1533879501432310998,duration=16199245
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04b801bc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a8b0024
travis_time:start:0a8b0024
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:269817ec
$ dmesg | grep -i kill
