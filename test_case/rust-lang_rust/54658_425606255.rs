plain
[00:47:50] ....................................................................................................
[00:47:53] ....................................................................................................
[00:47:56] ....................................................................................................
[00:47:59] ...................................i................................................................
[00:48:04] .............................F......................................................................
[00:48:10] .................................................................................................i..
[00:48:13] ....................................................................................................
[00:48:16] ....................................................................................................
[00:48:21] ...................................................i................................................
---
[00:48:22] 
[00:48:22] - warning: unused extern crate
[00:48:22] -   --> $DIR/remove-extern-crate.rs:19:1
[00:48:22] -    |
[00:48:22] - LL | extern crate core;
[00:48:22] -    |
[00:48:22] - note: lint level defined here
[00:48:22] -   --> $DIR/remove-extern-crate.rs:17:9
[00:48:22] -    |
[00:48:22] -    |
[00:48:22] - LL | #![warn(rust_2018_idioms)]
[00:48:22] -    |         ^^^^^^^^^^^^^^^^
[00:48:22] -    = note: #[warn(unused_extern_crates)] implied by #[warn(rust_2018_idioms)]
[00:48:22] - 
[00:48:22] - warning: `extern crate` is not idiomatic in the new edition
[00:48:22] -    |
[00:48:22] -    |
[00:48:22] - LL | extern crate core as another_name;
[00:48:22] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
[00:48:22] - 
[00:48:22] - warning: `extern crate` is not idiomatic in the new edition
[00:48:22] -    |
[00:48:22] -    |
[00:48:22] - LL |     extern crate core;
[00:48:22] -    |     ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
[00:48:22] - 
[00:48:22] 
[00:48:22] 
[00:48:22] 
[00:48:22] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/remove-extern-crate.stderr`: No such file or directory (os error 2)
[00:48:22] tdb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:22] 
[00:48:22] 
[00:48:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:22] Build completed unsuccessfully in 0:03:12
[00:48:22] Build completed unsuccessfully in 0:03:12
[00:48:22] Makefile:58: recipe for target 'check' failed
[00:48:22] make: *** [check] Error 1
ckout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14a4a7ef
travis_time:start:14a4a7ef
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:015cb8ad
$ dmesg | grep -i kill
