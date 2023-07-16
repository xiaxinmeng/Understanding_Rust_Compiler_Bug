plain

[00:04:38] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:39] tidy error: /checkout/src/libstd/sys/unix/process/process_fuchsia.rs:85: trailing whitespace
[00:04:39] tidy error: /checkout/src/libstd/sys/unix/process/process_fuchsia.rs:95: line longer than 100 chars
[00:04:39] tidy error: /checkout/src/libstd/sys/unix/process/process_fuchsia.rs:96: line longer than 100 chars
[00:04:40] some tidy checks failed
[00:04:40] 
[00:04:40] 
[00:04:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:40] 
[00:04:40] 
[00:04:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:40] Build completed unsuccessfully in 0:01:37
[00:04:40] Build completed unsuccessfully in 0:01:37
[00:04:40] Makefile:79: recipe for target 'tidy' failed
[00:04:40] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:019ac250
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
