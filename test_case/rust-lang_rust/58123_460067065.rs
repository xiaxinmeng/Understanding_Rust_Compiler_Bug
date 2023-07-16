plain
travis_time:end:015f1616:start=1549211516335855140,finish=1549211517361644809,duration=1025789669
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:43]    Compiling rustc-demangle v0.1.10
[00:04:44] error[E0308]: mismatched types
[00:04:44]    --> src/liballoc/collections/binary_heap.rs:876:36
[00:04:44]     |
[00:04:44] 876 |             elt: ManuallyDrop::new(elt),
[00:04:44]     |                                    ^^^ expected type parameter, found &T
[00:04:44]     = note: expected type `T`
[00:04:44]                found type `&T`
[00:04:44] 
[00:04:46] error: aborting due to previous error
---
[00:04:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:46] expected success, got: exit code: 101
[00:04:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:46] Build completed unsuccessfully in 0:00:37
[00:04:46] make: *** [all] Error 1
[00:04:46] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:099616bc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 16:36:54 UTC 2019
---
travis_time:end:101f963a:start=1549211815336947275,finish=1549211815343399426,duration=6452151
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2cdaf37c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0da015a6
travis_time:start:0da015a6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:106eb4a1
$ dmesg | grep -i kill
