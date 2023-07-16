plain

[00:04:33] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:34] tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:107: line longer than 100 chars
[00:04:34] tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:110: line longer than 100 chars
[00:04:34] tidy error: /checkout/src/librustc_mir/build/matches/mod.rs:1057: line longer than 100 chars
[00:04:34] tidy error: /checkout/src/librustc_mir/build/matches/mod.rs:1059: line longer than 100 chars
[00:04:34] tidy error: /checkout/src/librustc_mir/build/matches/mod.rs:1084: line longer than 100 chars
[00:04:34] tidy error: /checkout/src/librustc_mir/build/matches/mod.rs:1087: line longer than 100 chars
[00:04:34] tidy error: /checkout/src/librustc_mir/build/block.rs:147: line longer than 100 chars
[00:04:35] some tidy checks failed
[00:04:35] 
[00:04:35] 
[00:04:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:35] 
[00:04:35] 
[00:04:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:35] Build completed unsuccessfully in 0:01:39
[00:04:35] Build completed unsuccessfully in 0:01:39
[00:04:35] make: *** [tidy] Error 1
[00:04:35] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d9bfb13
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
