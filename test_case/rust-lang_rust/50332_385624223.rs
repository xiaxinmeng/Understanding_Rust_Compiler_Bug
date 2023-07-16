plain

[00:04:22] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:22] tidy error: /checkout/src/librustc/ty/context.rs:2286: trailing whitespace
[00:04:22] tidy error: /checkout/src/librustc/ty/context.rs:2320: trailing whitespace
[00:04:22] tidy error: /checkout/src/librustc/ty/flags.rs:264: trailing whitespace
[00:04:23] some tidy checks failed
[00:04:23] 
[00:04:23] 
[00:04:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:23] 
[00:04:23] 
[00:04:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:23] Build completed unsuccessfully in 0:01:45
[00:04:23] Build completed unsuccessfully in 0:01:45
[00:04:23] make: *** [tidy] Error 1
[00:04:23] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3bbd10d8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
