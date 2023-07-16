plain
travis_time:end:0189e62d:start=1550414341396125647,finish=1550414415105717292,duration=73709591645
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:55]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:03:56] error[E0615]: attempted to take value of method `iter` on type `&[T]`
[00:03:56]     --> src/liballoc/collections/vec_deque.rs:2188:28
[00:03:56]      |
[00:03:56] 2188 |             let res = back.iter.try_fold(init, &mut f);
[00:03:56]      |                            ^^^^ help: use parentheses to call the method: `iter()`
[00:03:58] error: aborting due to previous error
[00:03:58] 
[00:03:58] For more information about this error, try `rustc --explain E0615`.
[00:03:58] error: Could not compile `alloc`.
[00:03:58] error: Could not compile `alloc`.
[00:03:58] 
[00:03:58] To learn more, run the command again with --verbose.
[00:03:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:58] expected success, got: exit code: 101
[00:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:58] Build completed unsuccessfully in 0:00:39
[00:03:58] make: *** [all] Error 1
[00:03:58] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:160f7b64
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 17 14:44:22 UTC 2019
---
travis_time:end:12f8b540:start=1550414663475853408,finish=1550414663481594249,duration=5740841
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03de22b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:044f51fe
travis_time:start:044f51fe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:026175f0
$ dmesg | grep -i kill
