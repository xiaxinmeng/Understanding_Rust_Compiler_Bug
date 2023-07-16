plain
[01:12:17] Verifying status of rust-by-example...
[01:12:17] Verifying status of rls...
[01:12:17] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:12:17] 
[01:12:17] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:12:17] 
[01:12:17] If you do intend to update 'rls', please check the error messages above and
[01:12:17] commit another update.
[01:12:17] 
[01:12:17] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:12:17] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:12:17] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:112e1596
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02d0c378:start=1535061914806517539,finish=1535061914814243149,duration=7725610
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12b77004
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26bf9115
travis_time:start:26bf9115
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:066c47f0
$ dmesg | grep -i kill
