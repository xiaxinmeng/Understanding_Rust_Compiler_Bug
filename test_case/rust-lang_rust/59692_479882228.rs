plain
[01:31:36] [RUSTC-TIMING] miri test:false 86.660
[01:31:37] error[E0599]: no method named `deref` found for type `std::option::Option<std::string::String>` in the current scope
[01:31:37]    --> src/tools/miri/src/bin/cargo-miri.rs:296:60
[01:31:37]     |
[01:31:37] 296 |     let (subcommand, skip) = match std::env::args().nth(2).deref() {
[01:31:37] 
[01:31:37] error: aborting due to previous error
[01:31:37] 
[01:31:37] For more information about this error, try `rustc --explain E0599`.
---
[01:31:51] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:31:51] 
[01:31:51] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:31:51] 
[01:31:51] If you do intend to update 'miri', please check the error messages above and
[01:31:51] commit another update.
[01:31:51] 
[01:31:51] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:31:51] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:31:51] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:143c473c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr  4 12:45:54 UTC 2019
---
travis_time:end:2a853a1c:start=1554381955758529168,finish=1554381955770707674,duration=12178506
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0130c5f9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:146fcf07
travis_time:start:146fcf07
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09fd8528
$ dmesg | grep -i kill
