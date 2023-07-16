plain
tidy check
[00:05:07] * 547 error codes
[00:05:07] * highest error code: E0911
[00:05:07] * 204 features
[00:05:07] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error("EOF while parsing a value", line: 1, column: 0)', libcore/result.rs:945:5
[00:05:07] 
[00:05:07] 
[00:05:07] 
[00:05:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:07] 
[00:05:07] 
[00:05:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:07] Build completed unsuccessfully in 0:01:55
[00:05:07] Build completed unsuccessfully in 0:01:55
[00:05:07] make: *** [tidy] Error 1
[00:05:07] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f629d78
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
