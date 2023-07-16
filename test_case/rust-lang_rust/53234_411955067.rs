plain
[00:00:58]                                  Dload  Upload   Total   Spent    Left  Speed
[00:00:58] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100 70080  100 70080    0     0   145k      0 --:--:-- --:--:-- --:--:--  145k
[00:00:58] usr/bin/dig: error while loading shared libraries: libcrypto.so.1.0.0: cannot open shared object file: No such file or directory

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 127.
travis_time:start:13136fd4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03f1fe4a:start=1533867402463335460,finish=1533867402469373485,duration=6038025
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05afd38c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e35e874
travis_time:start:0e35e874
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:063d18f0
$ dmesg | grep -i kill
