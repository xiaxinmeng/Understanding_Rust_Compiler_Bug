
[0Ktidy check
[00:04:05] tidy error: /checkout/src/test/run-make/git_clone_sha1.sh:9: line longer than 100 chars
[00:04:05] tidy error: /checkout/src/test/run-make/git_clone_sha1.sh: incorrect license
[00:04:06] some tidy checks failed
[00:04:06] 
[00:04:06] 
[00:04:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:06] expected success, got: exit code: 1
[00:04:06] 
[00:04:06] 
[00:04:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:06] Build completed unsuccessfully in 0:00:55
[00:04:06] make: *** [tidy] Error 1
[00:04:06] Makefile:79: recipe for target 'tidy' failed
