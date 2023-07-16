plain
travis_time:end:0e283fe8:start=1553039102026059358,finish=1553039197332546611,duration=95306487253
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:52] 
[01:16:52] running 120 tests
[01:17:17] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:17:21] .i......iii.i.....ii
[01:17:21] 
[01:17:21]  finished in 29.141
[01:17:21] travis_fold:end:test_debuginfo

---
[01:45:31] test builder::__test::test_with_no_doc_stage0 ... ok
[01:45:31] 
[01:45:31] failures:
[01:45:31] 
[01:45:31] ---- builder::__test::build_default stdout ----
[01:45:31] thread 'main' panicked at 'git could not determine LLVM submodule commit hash: fatal: Needed a single revision
[01:45:31] ', src/bootstrap/native.rs:87:13
[01:45:31] 
[01:45:31] ---- builder::__test::build_with_target_flag stdout ----
[01:45:31] ---- builder::__test::build_with_target_flag stdout ----
[01:45:31] thread 'main' panicked at 'git could not determine LLVM submodule commit hash: fatal: Needed a single revision
[01:45:31] ', src/bootstrap/native.rs:87:13
[01:45:31] ---- builder::__test::dist_baseline stdout ----
[01:45:31] ---- builder::__test::dist_baseline stdout ----
[01:45:31] thread 'main' panicked at 'git could not determine LLVM submodule commit hash: fatal: Needed a single revision
[01:45:31] ', src/bootstrap/native.rs:87:13
[01:45:31] ---- builder::__test::dist_with_hosts stdout ----
[01:45:31] ---- builder::__test::dist_with_hosts stdout ----
[01:45:31] thread 'main' panicked at 'git could not determine LLVM submodule commit hash: fatal: Needed a single revision
[01:45:31] ', src/bootstrap/native.rs:87:13
[01:45:31] ---- builder::__test::dist_with_same_targets_and_hosts stdout ----
[01:45:31] ---- builder::__test::dist_with_same_targets_and_hosts stdout ----
[01:45:31] thread 'main' panicked at 'git could not determine LLVM submodule commit hash: fatal: Needed a single revision
[01:45:31] ', src/bootstrap/native.rs:87:13
[01:45:31] ---- builder::__test::dist_with_target_flag stdout ----
[01:45:31] ---- builder::__test::dist_with_target_flag stdout ----
[01:45:31] thread 'main' panicked at 'git could not determine LLVM submodule commit hash: fatal: Needed a single revision
[01:45:31] ', src/bootstrap/native.rs:87:13
[01:45:31] ---- builder::__test::dist_with_targets stdout ----
[01:45:31] ---- builder::__test::dist_with_targets stdout ----
[01:45:31] thread 'main' panicked at 'git could not determine LLVM submodule commit hash: fatal: Needed a single revision
[01:45:31] ', src/bootstrap/native.rs:87:13
[01:45:31] ---- builder::__test::dist_with_targets_and_hosts stdout ----
[01:45:31] ---- builder::__test::dist_with_targets_and_hosts stdout ----
[01:45:31] thread 'main' panicked at 'git could not determine LLVM submodule commit hash: fatal: Needed a single revision
[01:45:31] ', src/bootstrap/native.rs:87:13
[01:45:31] ---- builder::__test::test_exclude stdout ----
[01:45:31] Skipping Set({"src/tools/tidy"}) because it is excluded
[01:45:31] Suite("src/test/ui") not skipped for "test::Ui" -- not in ["src/test/run-pass", "src/tools/tidy"]
[01:45:31] Suite("src/test/ui") not skipped for "test::Ui" -- not in ["src/test/run-pass", "src/tools/tidy"]
[01:45:31] thread 'main' panicked at 'git could not determine LLVM submodule commit hash: fatal: Needed a single revision
[01:45:31] ', src/bootstrap/native.rs:87:13
[01:45:31] 
[01:45:31] failures:
[01:45:31]     builder::__test::build_default
[01:45:31]     builder::__test::build_with_target_flag
---
[01:45:31] 
[01:45:31] 
[01:45:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:31] Build completed unsuccessfully in 0:40:16
[01:45:31] make: *** [check] Error 1
[01:45:31] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ec1f8b4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 20 01:32:18 UTC 2019
---
travis_time:end:189b2b4e:start=1553045539726213899,finish=1553045539732469580,duration=6255681
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03e7a346
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08966ff6
travis_time:start:08966ff6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01ab1139
$ dmesg | grep -i kill
