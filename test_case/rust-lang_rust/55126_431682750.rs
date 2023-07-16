plain
[00:53:56] .................................................................................................... 2200/4659
[00:54:00] .....................................i.............................................................. 2300/4659
[00:54:04] .................................................................................................... 2400/4659
[00:54:08] .................................................................................................... 2500/4659
[00:54:11] ....................................................iiiiiiiii....................................... 2600/4659
[00:54:18] .................................................................................................... 2800/4659
[00:54:22] .................................................................................................... 2900/4659
[00:54:25] ..................................................................................i................. 3000/4659
[00:54:27] .................................................................................................... 3100/4659
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:59] 
[01:07:59] running 111 tests
[01:08:02] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:08:02] ..iiii.....
[01:08:02] 
[01:08:02]  finished in 3.619
[01:08:02] travis_fold:end:test_codegen

---
[01:42:30] travis_time:end:stage0-rustdoc-themes:start=1540139169611373121,finish=1540139170641376269,duration=1030003148

[01:42:30] rustdoc: [theme-checker] Starting tests!
[01:42:30]  - Checking "/checkout/src/librustdoc/html/static/themes/dark.css"... FAILED
[01:42:30]   Missing "#titles > div:not(:last-child)" rule
[01:42:30] 
[01:42:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustdoc-themes" "/checkout/obj/build/bootstrap/debug/rustdoc" "/checkout/src/librustdoc/html/static/themes"
[01:42:30] expected success, got: exit code: 1
[01:42:30] 
[01:42:30] 
[01:42:30] 
[01:42:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:30] Build completed unsuccessfully in 0:53:22
[01:42:30] Makefile:58: recipe for target 'check' failed
[01:42:30] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:017bab05
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
