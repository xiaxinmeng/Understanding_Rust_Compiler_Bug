plain
travis_time:start:tidy
tidy check
[00:04:57] * 547 error codes
[00:04:57] * highest error code: E0911
[00:04:58] tidy error: /checkout/src/libstd/ffi/os_str.rs:420: mismatches to previous in: ["since"]
[00:04:58] tidy error: /checkout/src/libstd/ffi/os_str.rs:434: mismatches to previous in: ["since"]
[00:04:58] some tidy checks failed
[00:04:58] 
[00:04:58] 
[00:04:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:58] 
[00:04:58] 
[00:04:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:58] Build completed unsuccessfully in 0:01:52
[00:04:58] Build completed unsuccessfully in 0:01:52
[00:04:58] Makefile:79: recipe for target 'tidy' failed
[00:04:58] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03bf8b10
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
