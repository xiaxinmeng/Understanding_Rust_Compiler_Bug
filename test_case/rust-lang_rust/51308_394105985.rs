plain

[00:05:08] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:09] tidy error: /checkout/src/test/ui/const-eval/index_out_of_bounds.rs: missing trailing newline
[00:05:09] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:244: line longer than 100 chars
[00:05:09] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:249: line longer than 100 chars
[00:05:09] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:253: line longer than 100 chars
[00:05:09] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:274: line longer than 100 chars
[00:05:09] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:549: line longer than 100 chars
[00:05:10] some tidy checks failed
[00:05:10] 
[00:05:10] 
[00:05:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:10] 
[00:05:10] 
[00:05:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:10] Build completed unsuccessfully in 0:01:52
[00:05:10] Build completed unsuccessfully in 0:01:52
[00:05:10] Makefile:79: recipe for target 'tidy' failed
[00:05:10] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0113d94a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
