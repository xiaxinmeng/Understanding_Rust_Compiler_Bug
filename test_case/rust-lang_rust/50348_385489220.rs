plain
travis_time:start:tidy
tidy check
[00:04:47] * 539 error codes
[00:04:47] * highest error code: E0911
[00:04:48] tidy error: /checkout/src/libstd/sys/unix/ext/net.rs:1499: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:04:48] some tidy checks failed
[00:04:48] 
[00:04:48] 
[00:04:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:48] 
[00:04:48] 
[00:04:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:48] Build completed unsuccessfully in 0:01:52
[00:04:48] Build completed unsuccessfully in 0:01:52
[00:04:48] make: *** [tidy] Error 1
[00:04:48] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04e3e82a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
