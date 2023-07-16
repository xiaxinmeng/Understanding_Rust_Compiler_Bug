plain
travis_time:end:053ee1e0:start=1545120573675720183,finish=1545120574678870273,duration=1003150090
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:03:25]     Checking arena v0.0.0 (/checkout/src/libarena)
[00:03:25]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:03:26]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:03:39]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:03:43] error[E0425]: cannot find function `continue_unblocked` in module `rayon_core`
[00:03:43]    --> src/librustc/ty/query/job.rs:224:21
[00:03:43] 224 |         rayon_core::continue_unblocked();
[00:03:43]     |                     ^^^^^^^^^^^^^^^^^^ not found in `rayon_core`
[00:03:43] 
[00:04:02] error: aborting due to previous error
[00:04:02] error: aborting due to previous error
[00:04:02] 
[00:04:02] For more information about this error, try `rustc --explain E0425`.
[00:04:02] error: Could not compile `rustc`.
[00:04:02] 
[00:04:02] To learn more, run the command again with --verbose.
[00:04:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:04:02] Build completed unsuccessfully in 0:03:01
travis_time:end:2f018950:start=1545120583159960648,finish=1545120825707478576,duration=242547517928
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:1a302ae4:start=1545120826103270363,finish=1545120826107763497,duration=4493134
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0012bf28
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00b8ee70
travis_time:start:00b8ee70
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c674736
$ dmesg | grep -i kill
