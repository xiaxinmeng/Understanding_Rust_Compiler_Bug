
[00:04:43] travis_fold:end:stage0-tidy
[0K
[00:04:43] travis_time:end:stage0-tidy:start=1508177191622429747,finish=1508177201331920288,duration=9709490541
[0K
[00:04:43] tidy error: /checkout/src/bootstrap/tool.rs:119: line longer than 100 chars
[00:04:44] some tidy checks failed
[00:04:44] 
[00:04:44] 
[00:04:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "--no-vendor" "--quiet"
[00:04:44] expected success, got: exit code: 1
[00:04:44] 
[00:04:44] 
[00:04:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:44] Build completed unsuccessfully in 0:01:13
[00:04:44] Makefile:77: recipe for target 'tidy' failed
[00:04:44] make: *** [tidy] Error 1
