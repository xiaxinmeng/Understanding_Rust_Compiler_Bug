plain

[00:04:35] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:36] tidy error: /checkout/src/test/ui/raw_string_hash_count.rs:12: line longer than 100 chars
[00:04:37] 
[00:04:37] 
[00:04:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:37] 
[00:04:37] 
[00:04:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:37] Build completed unsuccessfully in 0:01:51
[00:04:37] Build completed unsuccessfully in 0:01:51
[00:04:37] some tidy checks failed
[00:04:37] Makefile:79: recipe for target 'tidy' failed
[00:04:37] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0831ec76
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
