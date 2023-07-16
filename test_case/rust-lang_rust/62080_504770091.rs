plain
[01:51:54] Verifying status of rustfmt...
[01:51:54] Verifying status of clippy-driver...
[01:51:54] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:51:54] 
[01:51:54] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:51:54] 
[01:51:54] If you do intend to update 'clippy-driver', please check the error messages above and
[01:51:54] commit another update.
[01:51:54] 
[01:51:54] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:51:54] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:51:54] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:107aed9d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jun 23 17:08:04 UTC 2019
---
travis_time:end:00cf1d13:start=1561309685543240894,finish=1561309685550531744,duration=7290850
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0395c6f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12ff8d60
travis_time:start:12ff8d60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:196d35fb
$ dmesg | grep -i kill
