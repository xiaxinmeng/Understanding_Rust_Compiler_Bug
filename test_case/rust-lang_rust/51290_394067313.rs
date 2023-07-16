plain

[00:04:53] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:54] tidy error: /checkout/src/libstd/thread/mod.rs:800: tab character
[00:04:54] tidy error: /checkout/src/libstd/thread/mod.rs:801: tab character
[00:04:54] tidy error: /checkout/src/libstd/thread/mod.rs:802: tab character
[00:04:54] tidy error: /checkout/src/libstd/thread/mod.rs:890: tab character
[00:04:54] tidy error: /checkout/src/libstd/thread/mod.rs:891: tab character
[00:04:54] tidy error: /checkout/src/libstd/thread/mod.rs:892: tab character
[00:04:55] some tidy checks failed
[00:04:55] 
[00:04:55] 
[00:04:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:55] 
[00:04:55] 
[00:04:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:55] Build completed unsuccessfully in 0:01:55
[00:04:55] Build completed unsuccessfully in 0:01:55
[00:04:55] make: *** [tidy] Error 1
[00:04:55] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14db658c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
