plain

[00:04:25] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:25] tidy error: /checkout/src/libsyntax/parse/parser.rs:2321: line longer than 100 chars
[00:04:25] tidy error: /checkout/src/librustc/hir/lowering.rs:3595: line longer than 100 chars
[00:04:27] some tidy checks failed
[00:04:27] 
[00:04:27] 
[00:04:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:27] 
[00:04:27] 
[00:04:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:27] Build completed unsuccessfully in 0:01:45
[00:04:27] Build completed unsuccessfully in 0:01:45
[00:04:27] Makefile:79: recipe for target 'tidy' failed
[00:04:27] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:169bb700
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
