plain
travis_time:end:07765be8:start=1549883109322738929,finish=1549883205873991314,duration=96551252385
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:40]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:06:45] error: This node does not have a stability attribute
[00:06:45]     --> src/libstd/collections/hash/map.rs:2342:1
[00:06:45]      |
[00:06:45] 2342 | unsafe impl<'a, K: 'a + Sync, V: 'a + Sync> Sync for OccupiedEntry<'a, K, V> {}
[00:06:45] 
[00:06:45] error: This node does not have a stability attribute
[00:06:45]     --> src/libstd/collections/hash/map.rs:2343:1
[00:06:45]      |
[00:06:45]      |
[00:06:45] 2343 | unsafe impl<'a, K: 'a + Send, V: 'a + Send> Send for OccupiedEntry<'a, K, V> {}
[00:06:45] 
[00:06:45] error: This node does not have a stability attribute
[00:06:45]     --> src/libstd/collections/hash/map.rs:2366:1
[00:06:45]      |
[00:06:45]      |
[00:06:45] 2366 | unsafe impl<'a, K: 'a + Sync, V: 'a + Sync> Sync for VacantEntry<'a, K, V> {}
[00:06:45] 
[00:06:45] error: This node does not have a stability attribute
[00:06:45]     --> src/libstd/collections/hash/map.rs:2367:1
[00:06:45]      |
[00:06:45]      |
[00:06:45] 2367 | unsafe impl<'a, K: 'a + Send, V: 'a + Send> Send for VacantEntry<'a, K, V> {}
[00:06:45] 
[00:06:45] error: aborting due to 4 previous errors
[00:06:45] 
[00:06:45] error: Could not compile `std`.
[00:06:45] error: Could not compile `std`.
[00:06:45] 
[00:06:45] To learn more, run the command again with --verbose.
[00:06:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:06:45] expected success, got: exit code: 101
[00:06:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:45] Build completed unsuccessfully in 0:00:46
[00:06:45] Makefile:18: recipe for target 'all' failed
[00:06:45] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e86bba4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 11:13:40 UTC 2019
---
travis_time:end:0050c1fc:start=1549883620966935299,finish=1549883620972276615,duration=5341316
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1bdb3441
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0db17cd6
travis_time:start:0db17cd6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13edd330
$ dmesg | grep -i kill
