plain

[00:04:59] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:59] tidy error: /checkout/src/liballoc/raw_vec.rs:413: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/liballoc/raw_vec.rs:491: line longer than 100 chars
[00:05:00] some tidy checks failed
[00:05:00] 
[00:05:00] 
[00:05:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:00] 
[00:05:00] 
[00:05:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:00] Build completed unsuccessfully in 0:02:00
[00:05:00] Build completed unsuccessfully in 0:02:00
[00:05:00] make: *** [tidy] Error 1
[00:05:00] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:043a73b4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
