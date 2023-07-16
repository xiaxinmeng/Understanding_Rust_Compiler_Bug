plain
[01:31:57] travis_fold:end:stage0-linkchecker

[01:31:57] travis_time:end:stage0-linkchecker:start=1529234518845616896,finish=1529234521979587632,duration=3133970736

[01:32:09] core/ptr/index.html:14: broken link - core/ptr/fn.copy.html
[01:32:15] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:32:15] 
[01:32:15] 
[01:32:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:32:15] expected success, got: exit code: 101
[01:32:15] expected success, got: exit code: 101
[01:32:15] 
[01:32:15] 
[01:32:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:15] Build completed unsuccessfully in 0:45:04
[01:32:15] Makefile:58: recipe for target 'check' failed
[01:32:15] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:265bbdde
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
