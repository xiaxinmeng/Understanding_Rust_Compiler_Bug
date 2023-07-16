plain
[00:21:24] travis_time:end:stage1-std:start=1540363879286596275,finish=1540363923975660345,duration=44689064070

[00:21:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:24] Build completed unsuccessfully in 0:17:29
[00:21:24] Makefile:28: recipe for target 'all' failed
[00:21:24] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18f35d00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
