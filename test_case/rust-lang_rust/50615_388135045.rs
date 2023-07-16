plain

[00:05:58] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:58] tidy error: /checkout/src/librustc_codegen_llvm/back/symbol_export.rs:388: line longer than 100 chars
[00:05:58] tidy error: /checkout/src/librustc_codegen_llvm/back/link.rs:485: line longer than 100 chars
[00:05:58] tidy error: /checkout/src/librustc_codegen_llvm/lib.rs:236: line longer than 100 chars
[00:05:58] tidy error: /checkout/src/librustc_metadata/encoder.rs:1228: line longer than 100 chars
[00:05:58] tidy error: /checkout/src/librustc_driver/lib.rs:128: line longer than 100 chars
[00:05:59] some tidy checks failed
[00:05:59] 
[00:05:59] 
[00:05:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:59] 
[00:05:59] 
[00:05:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:59] Build completed unsuccessfully in 0:02:34
[00:05:59] Build completed unsuccessfully in 0:02:34
[00:05:59] make: *** [tidy] Error 1
[00:05:59] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00db7e3e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
