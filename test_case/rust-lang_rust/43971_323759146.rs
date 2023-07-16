
[00:46:56] testing https://github.com/BurntSushi/xsv
[00:46:56] Initialized empty Git repository in /checkout/obj/build/ct/xsv/.git/
[00:46:56] fatal: Could not parse object 'a9a7163f2a2953cea426fee1216bec914fe2f56a'.
[00:46:56] From https://github.com/BurntSushi/xsv
[00:46:56]  * branch            master     -> FETCH_HEAD
[00:46:56] fatal: Could not parse object 'a9a7163f2a2953cea426fee1216bec914fe2f56a'.
[00:46:57] From https://github.com/BurntSushi/xsv
[00:46:57]  * branch            master     -> FETCH_HEAD
[00:46:57] fatal: Could not parse object 'a9a7163f2a2953cea426fee1216bec914fe2f56a'.
[00:50:31] fatal: unable to access 'https://github.com/BurntSushi/xsv/': Failed to connect to github.com port 443: Connection timed out
[00:50:31] thread 'main' panicked at 'assertion failed: status.success()', /checkout/src/tools/cargotest/main.rs:106:12
[00:50:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:31] 
[00:50:31] 
[00:50:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[00:50:31] expected success, got: exit code: 101
[00:50:31] 
[00:50:31] 
[00:50:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/cargotest src/tools/cargo src/tools/rls src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty
[00:50:31] Build completed unsuccessfully in 0:03:39
[00:50:31] make: *** [check-aux] Error 1
[00:50:31] Makefile:56: recipe for target 'check-aux' failed
