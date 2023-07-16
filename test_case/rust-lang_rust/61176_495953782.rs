plain
[01:36:44] Verifying status of rustfmt...
[01:36:44] Verifying status of clippy-driver...
[01:36:44] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:36:44] 
[01:36:44] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:36:44] 
[01:36:44] If you do intend to update 'clippy-driver', please check the error messages above and
[01:36:44] commit another update.
[01:36:44] 
[01:36:44] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:36:44] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:36:44] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:20fc563c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May 25 22:41:11 UTC 2019
---
travis_time:end:01c017ea:start=1558824073262463895,finish=1558824073274550191,duration=12086296
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0136ee52
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05d7f880
travis_time:start:05d7f880
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:097df940
$ dmesg | grep -i kill
