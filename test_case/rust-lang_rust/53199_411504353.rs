plain
[01:01:18]    Compiling clippy v0.0.212 (file:///checkout/src/tools/clippy)
[01:01:19] error: environment variable `PROFILE` not defined
[01:01:19]   --> tools/clippy/tests/compile-test.rs:16:42
[01:01:19]    |
[01:01:19] 16 |         PathBuf::from(concat!("target/", env!("PROFILE"), "/clippy-driver"))
[01:01:19] 
[01:01:19] error: environment variable `PROFILE` not defined
[01:01:19]   --> tools/clippy/tests/compile-test.rs:24:34
[01:01:19]    |
[01:01:19]    |
[01:01:19] 24 |         Path::new("target").join(env!("PROFILE"))
[01:01:19] 
[01:01:19] 
[01:01:19] error: environment variable `OUT_DIR` not defined
[01:01:19]   --> tools/clippy/tests/compile-test.rs:54:38
[01:01:19]    |
[01:01:19] 54 |         let mut path = PathBuf::from(env!("OUT_DIR"));
[01:01:19] 
[01:01:19] error: aborting due to 3 previous errors
[01:01:19] 
[01:01:19] error: Could not compile `clippy`.
---
[01:28:55] Verifying status of rustfmt...
[01:28:55] Verifying status of clippy-driver...
[01:28:55] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:28:55] 
[01:28:55] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:28:55] 
[01:28:55] If you do intend to update 'clippy-driver', please check the error messages above and
[01:28:55] commit another update.
[01:28:55] 
[01:28:55] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:28:55] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:28:55] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:003665b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:37a0b30c:start=1533752590308115785,finish=1533752590315522341,duration=7406556
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0021b1f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19f1f76c
travis_time:start:19f1f76c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1cbdb400
$ dmesg | grep -i kill
