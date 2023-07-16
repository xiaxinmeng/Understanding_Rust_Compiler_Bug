plain

[00:04:56] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:57] tidy error: /checkout/src/libsyntax/ext/tt/macro_parser.rs:237: tab character
[00:04:57] tidy error: /checkout/src/libsyntax/ext/tt/macro_parser.rs:246: tab character
[00:04:58] some tidy checks failed
[00:04:58] 
[00:04:58] 
[00:04:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:58] 
[00:04:58] 
[00:04:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:58] Build completed unsuccessfully in 0:02:02
[00:04:58] Build completed unsuccessfully in 0:02:02
[00:04:58] Makefile:79: recipe for target 'tidy' failed
[00:04:58] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e2f5cfb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
