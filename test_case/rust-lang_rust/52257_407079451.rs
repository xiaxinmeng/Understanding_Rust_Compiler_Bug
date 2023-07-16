plain
[01:12:41] travis_fold:end:stage0-rustdoc-themes

[01:12:41] travis_time:end:stage0-rustdoc-themes:start=1532356443941579459,finish=1532356445052331239,duration=1110751780

[01:12:41] thread 'main' panicked at 'read_dir failed: Os { code: 2, kind: NotFound, message: "No such file or directory" }', libcore/result.rs:945:5
[01:12:41] 
[01:12:41] 
[01:12:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustdoc-themes" "/checkout/obj/build/bootstrap/debug/rustdoc" "/checkout/src/librustdoc/html/static/themes"
[01:12:41] expected success, got: exit code: 101
[01:12:41] expected success, got: exit code: 101
[01:12:41] 
[01:12:41] 
[01:12:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:41] Build completed unsuccessfully in 0:31:52
[01:12:42] make: *** [check] Error 1
[01:12:42] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0012b193
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
