plain
[01:47:21] 
[01:47:21] testing https://github.com/BurntSushi/xsv
[01:47:21] Initialized empty Git repository in /checkout/obj/build/ct/xsv/.git/
[01:47:21] fatal: Could not parse object '66956b6bfd62d6ac767a6b6499c982eae20a2c9f'.
[01:47:41] fatal: unable to access 'https://github.com/BurntSushi/xsv/': Could not resolve host: github.com
[01:47:41] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:47:41] 
[01:47:41] 
[01:47:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:47:41] expected success, got: exit code: 101
[01:47:41] expected success, got: exit code: 101
[01:47:41] 
[01:47:41] 
[01:47:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:47:41] Build completed unsuccessfully in 0:44:32
[01:47:41] Makefile:60: recipe for target 'check-aux' failed
[01:47:41] make: *** [check-aux] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10fdc69c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
