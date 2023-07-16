plain
[01:46:27]     Updating git repository `https://github.com/servo/gaol`
[01:46:27]     Updating git repository `https://github.com/servo/devices`
[01:46:28]     Updating git repository `https://github.com/servo/rust-azure`
[01:46:29]     Updating git repository `https://github.com/servo/fontsan`
[01:54:30] warning: spurious network error (2 tries remaining): curl error: Failed to connect to github.com port 443: Connection timed out
[01:54:30] ; class=Net (12)
[01:58:17] warning: spurious network error (2 tries remaining): curl error: Failed to connect to github.com port 443: Connection timed out
[01:58:17] warning: spurious network error (2 tries remaining): curl error: Failed to connect to github.com port 443: Connection timed out
[01:58:17] ; class=Net (12)
[02:04:45]     Updating git repository `https://github.com/energymon/energymon-sys.git`
[02:04:46]     Updating git repository `https://github.com/djg/cubeb-pulse-rs`
[02:04:47]  Downloading libc v0.2.33
[02:05:02]  Downloading atomic_refcell v0.1.0
---
[02:14:41] fatal: Could not parse object '5e11c4852fe4aa086b0e4fe5885822fbe57ba928'.
[02:14:42] From https://github.com/Aaronepower/tokei
[02:14:42]  * branch            master     -> FETCH_HEAD
[02:14:42] fatal: Could not parse object '5e11c4852fe4aa086b0e4fe5885822fbe57ba928'.
[02:18:16] fatal: unable to access 'https://github.com/Aaronepower/tokei/': Failed to connect to github.com port 443: Connection timed out
[02:18:16] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[02:18:16] 
[02:18:16] 
[02:18:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[02:18:16] expected success, got: exit code: 101
[02:18:16] expected success, got: exit code: 101
[02:18:16] 
[02:18:16] 
[02:18:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[02:18:16] Build completed unsuccessfully in 1:12:46
[02:18:16] make: *** [check-aux] Error 1
[02:18:16] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1dd4b6ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
