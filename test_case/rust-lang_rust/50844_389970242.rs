plain

[00:04:44] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:44] tidy error: /checkout/src/librustc_driver/lib.rs:703: line longer than 100 chars
[00:04:44] tidy error: /checkout/src/librustc_driver/lib.rs:715: line longer than 100 chars
[00:04:46] some tidy checks failed
[00:04:46] 
[00:04:46] 
[00:04:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:46] 
[00:04:46] 
[00:04:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:46] Build completed unsuccessfully in 0:01:50
[00:04:46] Build completed unsuccessfully in 0:01:50
[00:04:46] Makefile:79: recipe for target 'tidy' failed
[00:04:46] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0090c429
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.4
travis_time:start:1629c168
$ dmesg | grep -i kill
[   10.936563] init: failsafe main process (1092) killed by TERM signal
[   42.441870] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.4

Done. Your build exited with 1.
