plain
[01:20:07] travis_fold:end:stage0-linkchecker

[01:20:07] travis_time:end:stage0-linkchecker:start=1529051071470430727,finish=1529051074430069706,duration=2959638979

[01:20:22] alloc/index.html:25: broken link - alloc/arc/index.html
[01:20:23] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:20:23] 
[01:20:23] 
[01:20:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:20:23] expected success, got: exit code: 101
[01:20:23] expected success, got: exit code: 101
[01:20:23] 
[01:20:23] 
[01:20:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:23] Build completed unsuccessfully in 0:39:22
[01:20:23] Makefile:58: recipe for target 'check' failed
[01:20:23] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0204aa58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
