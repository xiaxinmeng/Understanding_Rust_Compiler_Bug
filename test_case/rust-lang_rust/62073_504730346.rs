plain
travis_time:end:0181d124:start=1561276171329198725,finish=1561276172175601976,duration=846403251
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:33] Assembling stage1 compiler (x86_64-unknown-linux-gnu)
[00:27:33] travis_fold:start:stage1-std
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:27:33] error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (exit code: 127)
[00:27:33] --- stderr
[00:27:33] /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc: error while loading shared libraries: libmimalloc.so: cannot open shared object file: No such file or directory
[00:27:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:27:33] expected success, got: exit code: 101
[00:27:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:27:33] Build completed unsuccessfully in 0:22:41
---
travis_time:end:2bf25b86:start=1561277837620972566,finish=1561277837626352804,duration=5380238
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06ee5be8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:074f5de0
travis_time:start:074f5de0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1798af00
$ dmesg | grep -i kill
