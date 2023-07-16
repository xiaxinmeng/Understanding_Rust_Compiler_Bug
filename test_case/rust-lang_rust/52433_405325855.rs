plain
[01:24:04]    Compiling miri v0.1.0 (file:///checkout/src/tools/miri)
[01:24:04]    Compiling byteorder v1.2.3
[01:24:04] [RUSTC-TIMING] byteorder test:false 0.814
[01:24:36] [RUSTC-TIMING] miri test:false 31.239
[01:24:37] error[E0433]: failed to resolve. Could not find `Item_` in `hir`
[01:24:37]     |
[01:24:37]     |
[01:24:37] 115 |                 if let hir::Item_::ItemFn(.., body_id) = i.node {
[01:24:37]     |                             ^^^^^ Could not find `Item_` in `hir`
[01:24:37] error: aborting due to previous error
[01:24:37] 
[01:24:37] For more information about this error, try `rustc --explain E0433`.
[01:24:37] [RUSTC-TIMING] miri test:false 1.229
---
[01:25:46] Verifying status of clippy-driver...
[01:25:46] Verifying status of miri...
[01:25:46] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:25:46] 
[01:25:46] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:25:46] 
[01:25:46] If you do intend to update 'miri', please check the error messages above and
[01:25:46] commit another update.
[01:25:46] 
[01:25:46] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:25:46] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:25:46] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:03749170
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2e377ead:start=1531763222439993729,finish=1531763222446999132,duration=7005403
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:201fd320
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:020833b0
travis_time:start:020833b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05a62bb2
$ dmesg | grep -i kill
