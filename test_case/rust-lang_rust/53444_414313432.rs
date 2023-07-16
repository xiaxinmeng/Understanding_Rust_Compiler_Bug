plain
[00:20:30]     |            ^^^^^^^^^^^^^
[00:20:30] 
/build/bootstrap/debug/bootstrap build
[00:20:31] Build completed unsuccessfully in 0:15:54
[00:20:31] Makefile:28: recipe for target 'all' failed
[00:20:31] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c22ad92
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
