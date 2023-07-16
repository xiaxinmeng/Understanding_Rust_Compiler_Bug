plain
travis_time:end:14460264:start=1546997940802994049,finish=1546997942213812449,duration=1410818400
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:47]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:51]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:09]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:17] error[E0107]: wrong number of type arguments: expected 2, found 1
[00:07:17]   --> src/librustc/ty/query/job.rs:88:10
[00:07:17]    |
[00:07:17] 88 |     ) -> TryGetJob<'a, 'tcx, D> {
[00:07:17] 
[00:07:18] error: aborting due to previous error
[00:07:18] 
[00:07:18] For more information about this error, try `rustc --explain E0107`.
[00:07:18] For more information about this error, try `rustc --explain E0107`.
[00:07:18] error: Could not compile `rustc`.
[00:07:18] warning: build failed, waiting for other jobs to finish...
[00:07:35] error: build failed
[00:07:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:35] expected success, got: exit code: 101
[00:07:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:35] Build completed unsuccessfully in 0:03:53
[00:07:35] Makefile:18: recipe for target 'all' failed
[00:07:35] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:064fa528
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan  9 01:47:09 UTC 2019
---
travis_time:end:0080b464:start=1546998430010384289,finish=1546998430014825046,duration=4440757
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:036cfe5b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3ba99a3b
travis_time:start:3ba99a3b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0386c8af
$ dmesg | grep -i kill
