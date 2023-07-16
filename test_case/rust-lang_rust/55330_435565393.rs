plain
[00:05:36]     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:05:40] error[E0599]: no method named `has_escaping_regions` found for type `&rustc::ty::TyS<'_>` in the current scope
[00:05:40]    --> librustc_typeck/coherence/builtin.rs:179:25
[00:05:40]     |
[00:05:40] 179 |         assert!(!source.has_escaping_regions());
[00:05:40] 
[00:05:41] error: aborting due to previous error
[00:05:41] 
[00:05:41] For more information about this error, try `rustc --explain E0599`.
[00:05:41] For more information about this error, try `rustc --explain E0599`.
[00:05:41] error: Could not compile `rustc_typeck`.
[00:05:41] warning: build failed, waiting for other jobs to finish...
[00:06:14] error: build failed
[00:06:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:14] expected success, got: exit code: 101
[00:06:14] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:06:14] travis_fold:end:stage0-rustc

[00:06:14] travis_time:end:stage0-rustc:start=1541227565963350433,finish=1541227737210481472,duration=171247131039

---
travis_time:end:1d115b8c:start=1541227737913947912,finish=1541227737922304716,duration=8356804
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:25f07528
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2e77bdf0
travis_time:start:2e77bdf0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07f571a0
$ dmesg | grep -i kill
