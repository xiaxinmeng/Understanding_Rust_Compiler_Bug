plain
[00:43:34] travis_fold:start:stage1-test
travis_time:start:stage1-test
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:34]     Finished release [optimized] target(s) in 0.26s
[00:43:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
[00:43:34] expected success, got: signal: 9
[00:43:34] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:43:34] travis_fold:end:stage1-test

[00:43:34] travis_time:end:stage1-test:start=1530711748658475458,finish=1530711749257516580,duration=599041122


[00:43:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:34] Build completed unsuccessfully in 0:00:13
[00:43:34] make: *** [check] Error 1
[00:43:34] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05d0c637
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
