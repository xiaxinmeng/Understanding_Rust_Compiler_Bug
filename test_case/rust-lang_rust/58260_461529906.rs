plain
travis_time:end:018c9fd2:start=1549559987460830844,finish=1549560088498424737,duration=101037593893
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:28:51]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:28:52] error[E0432]: unresolved import `rustc_errors`
[00:28:52]   --> src/librustc_borrowck/lib.rs:18:5
[00:28:52]    |
[00:28:52] 18 | use rustc_errors as errors;
[00:28:52]    |     ^^^^^^^^^^^^^^^^^^^^^^ no `rustc_errors` external crate
[00:28:52] error: unused import: `rustc_errors as errors`
[00:28:52]   --> src/librustc_borrowck/lib.rs:18:5
[00:28:52]    |
[00:28:52] 18 | use rustc_errors as errors;
---
[00:29:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:29:59] expected success, got: exit code: 101
[00:29:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:29:59] Build completed unsuccessfully in 0:17:28
[00:29:59] make: *** [all] Error 1
[00:29:59] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12cd884c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 17:51:38 UTC 2019
---
travis_time:end:00cac845:start=1549561899707143911,finish=1549561899712315882,duration=5171971
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:17b6f3d7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1188e2b1
travis_time:start:1188e2b1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00de2ed4
$ dmesg | grep -i kill
