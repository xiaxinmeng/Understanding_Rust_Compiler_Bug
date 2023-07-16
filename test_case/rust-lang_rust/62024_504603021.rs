plain
[01:47:48] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:47:48] 
[01:47:48] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:47:48] 
[01:47:48] If you do intend to update 'miri', please check the error messages above and
[01:47:48] commit another update.
[01:47:48] 
[01:47:48] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:47:48] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:47:48] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:2040fa4a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jun 21 23:26:46 UTC 2019
---
travis_time:end:346a021f:start=1561159607896014680,finish=1561159607908742974,duration=12728294
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0813904a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:035caadc
travis_time:start:035caadc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:002162a0
$ dmesg | grep -i kill
