plain

[00:05:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:11] tidy error: /checkout/src/test/ui/existential_type.rs: missing trailing newline
[00:05:12] tidy error: /checkout/src/test/ui/feature-gate-existential-type.rs:13: gate-test test found referencing a nonexistent feature 'existential-type'
[00:05:12] tidy error: /checkout/src/test/ui/feature-gate-existential-type.rs:13: gate-test test found referencing a nonexistent feature 'existential-type'
[00:05:12] some tidy checks failed
[00:05:12] 
[00:05:12] 
[00:05:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:12] 
[00:05:12] 
[00:05:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:12] Build completed unsuccessfully in 0:02:04
[00:05:12] Build completed unsuccessfully in 0:02:04
[00:05:12] Makefile:79: recipe for target 'tidy' failed
[00:05:12] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c4a4a1c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
