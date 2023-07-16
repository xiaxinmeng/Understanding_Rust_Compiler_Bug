plain
[01:11:50] Verifying status of rustfmt...
[01:11:50] Verifying status of clippy-driver...
[01:11:50] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:11:50] 
[01:11:50] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:11:50] 
[01:11:50] If you do intend to update 'clippy-driver', please check the error messages above and
[01:11:50] commit another update.
[01:11:50] 
[01:11:50] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:11:50] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:11:50] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:02765622
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:299fe835:start=1535037367143821111,finish=1535037367151604476,duration=7783365
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:168643bc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b556cff
travis_time:start:0b556cff
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1bc992cd
$ dmesg | grep -i kill
