plain
travis_time:end:2f4870f8:start=1547259088866311308,finish=1547259091098189787,duration=2231878479
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:44] 
[00:03:44] error: aborting due to previous error
[00:03:44] 
[00:03:44] For more information about this error, try `rustc --explain E0463`.
[00:03:45] error: Could not compile `tidy`.
[00:03:45] To learn more, run the command again with --verbose.
[00:03:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
[00:03:45] expected success, got: exit code: 101
[00:03:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:45] Build completed unsuccessfully in 0:00:39
[00:03:45] make: *** [tidy] Error 1
[00:03:45] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18936552
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 12 02:15:26 UTC 2019
---
travis_time:end:011e61d0:start=1547259327492358199,finish=1547259327497102939,duration=4744740
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:080a56b4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:181cb7bc
travis_time:start:181cb7bc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11ba6860
$ dmesg | grep -i kill
