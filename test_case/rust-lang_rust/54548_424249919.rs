plain
tidy check
[00:04:50] * 559 error codes
[00:04:50] * highest error code: E0715
[00:04:50] * 232 features
[00:04:51] crate `rustc-ap-syntax` is duplicated in `Cargo.lock`
[00:04:51]   * rustc-ap-syntax 253.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:04:51]   * rustc-ap-syntax 237.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:04:51] some tidy checks failed
[00:04:51] 
[00:04:51] 
[00:04:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:51] 
[00:04:51] 
[00:04:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:51] Build completed unsuccessfully in 0:00:55
[00:04:51] Build completed unsuccessfully in 0:00:55
[00:04:51] Makefile:79: recipe for target 'tidy' failed
[00:04:51] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08a77445
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:049d7d66:start=1537858073990873645,finish=1537858073995879050,duration=5005405
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02693dcf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:141124c8
travis_time:start:141124c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d49628e
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   10.102613] init: failsafe main process (1095) killed by TERM signal
[   41.528830] init: plymouth-upstart-bridge main process (509) killed by TERM signal
travis_fold:end:after_failure.6

Done. Your build exited with 1.
