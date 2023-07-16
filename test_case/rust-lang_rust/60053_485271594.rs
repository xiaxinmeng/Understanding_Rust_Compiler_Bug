plain
[01:30:37] Verifying status of edition-guide...
[01:30:37] Verifying status of rls...
[01:30:37] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:30:37] 
[01:30:37] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:30:37] 
[01:30:37] If you do intend to update 'rls', please check the error messages above and
[01:30:37] commit another update.
[01:30:37] 
[01:30:37] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:30:37] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:30:37] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:1ddd8af1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 21 18:13:28 UTC 2019
---
travis_time:end:009ee14c:start=1555870410253555794,finish=1555870410269812185,duration=16256391
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:21cb2f48
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f98c480
travis_time:start:1f98c480
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11cd13cc
$ dmesg | grep -i kill
