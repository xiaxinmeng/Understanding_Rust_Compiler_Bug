plain
[01:33:55] Verifying status of rustfmt...
[01:33:55] Verifying status of clippy-driver...
[01:33:55] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:33:55] 
[01:33:55] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:33:55] 
[01:33:55] If you do intend to update 'clippy-driver', please check the error messages above and
[01:33:55] commit another update.
[01:33:55] 
[01:33:55] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:33:55] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:33:55] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:33acbe00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 28 09:55:53 UTC 2019
---
travis_time:end:00548a12:start=1559037354580858304,finish=1559037354586786720,duration=5928416
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d0231dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06a25364
travis_time:start:06a25364
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:260a7b4a
$ dmesg | grep -i kill
