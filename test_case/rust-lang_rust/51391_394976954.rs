plain

[00:05:07] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:08] tidy error: /checkout/src/librustdoc/clean/mod.rs:1271: trailing whitespace
[00:05:08] tidy error: /checkout/src/librustdoc/clean/mod.rs:1274: trailing whitespace
[00:05:08] tidy error: /checkout/src/librustdoc/clean/mod.rs:1276: trailing whitespace
[00:05:08] tidy error: /checkout/src/librustdoc/clean/mod.rs:1278: trailing whitespace
[00:05:09] some tidy checks failed
[00:05:09] 
[00:05:09] 
[00:05:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:09] 
[00:05:09] 
[00:05:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:09] Build completed unsuccessfully in 0:01:57
[00:05:09] Build completed unsuccessfully in 0:01:57
[00:05:09] make: *** [tidy] Error 1
[00:05:09] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e6bd72e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
