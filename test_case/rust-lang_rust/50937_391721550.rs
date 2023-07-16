plain
[00:03:08]  Downloading phf_generator v0.7.21
[00:03:08]  Downloading matches v0.1.6
[00:03:08]  Downloading percent-encoding v1.0.1
[00:03:08]  Downloading idna v0.1.4
[00:03:08]  Downloading chalk-engine v0.6.0
[00:03:09]  Downloading compiletest_rs v0.3.9
[00:03:09]  Downloading regex v0.2.10
[00:03:09]  Downloading is-match v0.1.0
[00:03:09]  Downloading error-chain v0.11.0
---

[00:05:49] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:49] tidy error: /checkout/src/librustc_traits/chalk_context.rs: incorrect license
[00:05:49] tidy error: /checkout/src/librustc/ty/context.rs:1532: line longer than 100 chars
[00:05:49] tidy error: /checkout/src/librustc/ty/context.rs:1547: line longer than 100 chars
[00:05:51] Dependencies not on the whitelist:
[00:05:51] * chalk-engine 
[00:05:51] * chalk-macros 
[00:05:51] * rustc-hash 
[00:05:51] some tidy checks failed
[00:05:51] 
[00:05:51] 
[00:05:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:51] 
[00:05:51] 
[00:05:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:51] Build completed unsuccessfully in 0:02:28
[00:05:51] Build completed unsuccessfully in 0:02:28
[00:05:51] make: *** [tidy] Error 1
[00:05:51] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dfebf70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
