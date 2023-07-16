plain
[01:23:24] travis_time:end:stage0-rustdoc-themes:start=1530135345900665780,finish=1530135347050704467,duration=1150038687

[01:23:24] rustdoc: [theme-checker] Starting tests!
[01:23:24]  - Checking "/checkout/src/librustdoc/html/static/themes/dark.css"... FAILED
[01:23:24]   Missing ".stab.non-exhaustive" rule
[01:23:24] 
[01:23:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustdoc-themes" "/checkout/obj/build/bootstrap/debug/rustdoc" "/checkout/src/librustdoc/html/static/themes"
[01:23:24] expected success, got: exit code: 1
[01:23:24] 
[01:23:24] 
[01:23:24] 
[01:23:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:24] Build completed unsuccessfully in 0:42:42
[01:23:24] make: *** [check] Error 1
[01:23:24] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:123a773c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
