plain
[01:12:16] Verifying status of rustfmt...
[01:12:16] Verifying status of clippy-driver...
[01:12:16] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:12:16] 
[01:12:16] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:12:16] 
[01:12:16] If you do intend to update 'clippy-driver', please check the error messages above and
[01:12:16] commit another update.
[01:12:16] 
[01:12:16] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:12:16] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:12:16] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:1cfd49af
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 12 05:29:13 UTC 2018
---
travis_time:end:24a2d6ba:start=1544592553944423103,finish=1544592553952782775,duration=8359672
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ecac01b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:24a91520
travis_time:start:24a91520
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:016eef76
$ dmesg | grep -i kill
