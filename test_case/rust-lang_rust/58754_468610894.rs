plain
[01:31:17]    Compiling rustfix v0.4.4
[01:31:17] [RUSTC-TIMING] rand_pcg test:false 0.405
[01:31:18] [RUSTC-TIMING] rand_chacha test:false 0.731
[01:31:21] [RUSTC-TIMING] rustfix test:false 4.833
[01:31:25] error[E0063]: missing field `exclude_should_panic` in initializer of `test::TestOpts`
[01:31:25]    |
[01:31:25] 98 |     test::TestOpts {
[01:31:25]    |     ^^^^^^^^^^^^^^ missing `exclude_should_panic`
[01:31:25] 
---
[01:53:28]    Compiling compiletest_rs v0.3.18
[01:53:28]    Compiling lazy_static v0.2.11
[01:53:28] [RUSTC-TIMING] lazy_static test:false 0.142
[01:53:28]    Compiling colored v1.6.0
[01:53:30] error[E0063]: missing field `exclude_should_panic` in initializer of `test::TestOpts`
[01:53:30]    |
[01:53:30] 98 |     test::TestOpts {
[01:53:30]    |     ^^^^^^^^^^^^^^ missing `exclude_should_panic`
[01:53:30] 
---
[01:53:31] Verifying status of rustfmt...
[01:53:31] Verifying status of clippy-driver...
[01:53:31] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:53:31] 
[01:53:31] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:53:31] 
[01:53:31] If you do intend to update 'clippy-driver', please check the error messages above and
[01:53:31] commit another update.
[01:53:31] 
[01:53:31] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:53:31] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:53:31] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:11ee4d3c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar  1 09:59:51 UTC 2019
---
travis_time:end:04695c08:start=1551434393294584240,finish=1551434393300364508,duration=5780268
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:17d08858
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:186706a7
travis_time:start:186706a7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:26632a10
$ dmesg | grep -i kill
