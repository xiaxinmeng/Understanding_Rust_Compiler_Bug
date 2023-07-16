plain
travis_time:end:0bb57674:start=1549883007691695116,finish=1549883080850997965,duration=73159302849
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:23]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:05:23]    Compiling rustc-demangle v0.1.10
[00:05:23]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:05:28]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:30] error[E0599]: no method named `raw_capacity` found for type `&collections::hash::map::HashMap<K, V, S>` in the current scope
[00:05:30]     |
[00:05:30]     |
[00:05:30] 754 |         self.resize_policy.capacity(self.raw_capacity())
[00:05:30]     |
[00:05:30]     |
[00:05:30]     = note: the method `raw_capacity` exists but the following trait bounds were not satisfied:
[00:05:30]             `K : core::cmp::Eq`
[00:05:30]             `K : core::hash::Hash`
[00:05:30]             `S : core::hash::BuildHasher`
[00:05:30]     = help: did you mean `capacity`?
[00:05:30] error[E0308]: mismatched types
[00:05:30]     --> src/libstd/collections/hash/set.rs:1106:26
[00:05:30]      |
[00:05:30]      |
[00:05:30] 1106 |         IntoIter { iter: self.map.into_iter() }
[00:05:30]      |                          ^^^^^^^^^^^^^^^^^^^^ expected struct `collections::hash::map::IntoIter`, found struct `collections::hash::map::Iter`
[00:05:30]      |
[00:05:30]      = note: expected type `collections::hash::map::IntoIter<T, ()>`
[00:05:30]                 found type `collections::hash::map::Iter<'_, T, ()>`
[00:05:32] error: aborting due to 2 previous errors
[00:05:32] 
[00:05:32] Some errors occurred: E0308, E0599.
[00:05:32] For more information about an error, try `rustc --explain E0308`.
[00:05:32] For more information about an error, try `rustc --explain E0308`.
[00:05:32] error: Could not compile `std`.
[00:05:32] 
[00:05:32] To learn more, run the command again with --verbose.
[00:05:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:32] expected success, got: exit code: 101
[00:05:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:32] Build completed unsuccessfully in 0:00:45
[00:05:32] Makefile:18: recipe for target 'all' failed
[00:05:32] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ea9f618
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 11:10:22 UTC 2019
---
travis_time:end:0bc97597:start=1549883423057842165,finish=1549883423062193761,duration=4351596
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:119f24c4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01b9d974
travis_time:start:01b9d974
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0383054d
$ dmesg | grep -i kill
