plain
[01:23:17] travis_fold:end:stage0-linkchecker

[01:23:17] travis_time:end:stage0-linkchecker:start=1525377345213092017,finish=1525377347912520037,duration=2699428020

[01:23:31] reference/items/traits.html:212: broken link fragment `#methods` pointing to `reference/items/functions.html`
[01:23:32] reference/print.html:2110: broken link fragment `#methods` pointing to `reference/items/functions.html`
[01:23:32] reference/print.html:3190: broken link - reference/enumerations.html
[01:23:32] reference/attributes.html:524: broken link - reference/enumerations.html
[01:23:33] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:23:33] 
[01:23:33] 
[01:23:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:23:33] expected success, got: exit code: 101
[01:23:33] expected success, got: exit code: 101
[01:23:33] 
[01:23:33] 
[01:23:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:33] Build completed unsuccessfully in 0:41:57
[01:23:33] Makefile:58: recipe for target 'check' failed
[01:23:33] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08f75d00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
