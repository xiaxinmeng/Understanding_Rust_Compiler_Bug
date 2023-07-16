plain
[00:26:23] 
[00:26:23] To learn more, run the command again with --verbose.
[00:26:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:26:23] expected success, got: exit code: 101
[00:26:23] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1135:9
[00:26:23] travis_fold:end:stage1-rustc

[00:26:23] travis_time:end:stage1-rustc:start=1537472019540332412,finish=1537472266405550200,duration=246865217788


[00:26:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:23] Build completed unsuccessfully in 0:21:13
[00:26:23] make: *** [all] Error 1
[00:26:23] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:093ed5fa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
