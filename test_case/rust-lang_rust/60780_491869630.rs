plain
[01:35:15] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:35:15] 
[01:35:15] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:35:15] 
[01:35:15] If you do intend to update 'miri', please check the error messages above and
[01:35:15] commit another update.
[01:35:15] 
[01:35:15] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:35:15] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:35:15] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:25733b0e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 13 15:24:44 UTC 2019
---
travis_time:end:002b63ee:start=1557761085616966747,finish=1557761085630524957,duration=13558210
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:364e4766
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:025075bf
travis_time:start:025075bf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:004725de
$ dmesg | grep -i kill
