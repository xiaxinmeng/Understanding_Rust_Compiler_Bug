plain

[00:05:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:12] tidy error: /checkout/src/libsyntax/parse/parser.rs:2420: line longer than 100 chars
[00:05:13] some tidy checks failed
[00:05:13] 
[00:05:13] 
[00:05:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:13] 
[00:05:13] 
[00:05:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:13] Build completed unsuccessfully in 0:02:00
[00:05:13] Build completed unsuccessfully in 0:02:00
[00:05:13] make: *** [tidy] Error 1
[00:05:13] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00830bc8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
