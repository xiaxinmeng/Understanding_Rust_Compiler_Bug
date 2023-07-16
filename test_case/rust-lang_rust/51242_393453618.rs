plain

[00:04:43] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:44] tidy error: /checkout/src/test/ui/borrowck/mut-borrow-of-mut-ref.rs: incorrect license
[00:04:45] some tidy checks failed
[00:04:45] 
[00:04:45] 
[00:04:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:45] 
[00:04:45] 
[00:04:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:45] Build completed unsuccessfully in 0:01:47
[00:04:45] Build completed unsuccessfully in 0:01:47
[00:04:45] make: *** [tidy] Error 1
[00:04:45] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12a8f498
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
