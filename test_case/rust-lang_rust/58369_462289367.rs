plain
travis_time:end:00faed26:start=1549882393560818984,finish=1549882467197551580,duration=73636732596
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:47]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:03:47]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:03:47]    Compiling rustc-demangle v0.1.10
[00:03:53]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:03:54] error[E0119]: conflicting implementations of trait `core::marker::Send` for type `collections::hash::map::OccupiedEntry<'_, _, _>`:
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2343 | unsafe impl<'a, K: Send, V: Send> Send for OccupiedEntry<'a, K, V> {}
[00:03:54]      | ------------------------------------------------------------------ first implementation here
[00:03:54] ...
[00:03:54] 2367 | unsafe impl<'a, K: Send, V: Send> Send for OccupiedEntry<'a, K, V> {}
[00:03:54]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `collections::hash::map::OccupiedEntry<'_, _, _>`
[00:03:54] 
[00:03:54] error[E0119]: conflicting implementations of trait `core::marker::Sync` for type `collections::hash::map::OccupiedEntry<'_, _, _>`:
[00:03:54]      |
[00:03:54]      |
[00:03:54] 2342 | unsafe impl<'a, K: Sync, V: Sync> Sync for OccupiedEntry<'a, K, V> {}
[00:03:54]      | ------------------------------------------------------------------ first implementation here
[00:03:54] ...
[00:03:54] 2366 | unsafe impl<'a, K: Sync, V: Sync> Sync for OccupiedEntry<'a, K, V> {}
[00:03:54]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `collections::hash::map::OccupiedEntry<'_, _, _>`
[00:03:55] error: aborting due to 2 previous errors
[00:03:55] 
[00:03:55] For more information about this error, try `rustc --explain E0119`.
[00:03:55] error: Could not compile `std`.
[00:03:55] error: Could not compile `std`.
[00:03:55] 
[00:03:55] To learn more, run the command again with --verbose.
[00:03:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:55] expected success, got: exit code: 101
[00:03:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:55] Build completed unsuccessfully in 0:00:43
[00:03:55] make: *** [all] Error 1
[00:03:55] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2b12766f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 10:58:31 UTC 2019
---
travis_time:end:06e70fd8:start=1549882712448112337,finish=1549882712454029310,duration=5916973
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01237e53
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03649e7c
travis_time:start:03649e7c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e11e039
$ dmesg | grep -i kill
