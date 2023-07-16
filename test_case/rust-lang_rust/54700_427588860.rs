plain
[00:53:00] ................................................................................................i... 2200/4556
[00:53:04] .................................................................................................... 2300/4556
[00:53:08] .................................................................................................... 2400/4556
[00:53:12] .................................................................................................... 2500/4556
[00:53:16] ........iiiiiiiii................................................................................... 2600/4556
[00:53:22] .................................................................................................... 2800/4556
[00:53:26] .................................................................................................... 2900/4556
[00:53:28] ............................i....................................................................... 3000/4556
[00:53:31] ........................................................................................i.i..ii..... 3100/4556
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:15] 
[01:06:15] running 107 tests
[01:06:18] i..ii...iii....i...i.........i..iii...........i.....i....ii...i.i.ii..............i...ii..ii.i....ii 100/107
[01:06:19] test result: ok. 77 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:06:19] 
[01:06:19]  finished in 3.354
[01:06:19] travis_fold:end:test_codegen
---
[01:31:10] travis_fold:end:stage0-linkchecker

[01:31:10] travis_time:end:stage0-linkchecker:start=1538844452915796099,finish=1538844455193037581,duration=2277241482

[01:32:39] std/vec/struct.Vec.html:1255: broken link - std/vec/result/enum.Result.html
[01:32:39] std/vec/struct.Vec.html:1257: broken link - std/vec/result/enum.Result.html
[01:32:39] std/vec/struct.Vec.html:1278: broken link - std/vec/result/enum.Result.html
[01:32:39] std/vec/struct.Vec.html:1280: broken link - std/vec/result/enum.Result.html
[01:32:39] std/vec/struct.Vec.html:1303: broken link - std/vec/result/enum.Result.html
[01:32:39] std/vec/struct.Vec.html:1305: broken link - std/vec/result/enum.Result.html
[01:33:31] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:33:31] 
[01:33:31] 
[01:33:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:33:31] expected success, got: exit code: 101
[01:33:31] expected success, got: exit code: 101
[01:33:31] 
[01:33:31] 
[01:33:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:31] Build completed unsuccessfully in 0:45:13
[01:33:31] make: *** [check] Error 1
[01:33:31] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0778553f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
