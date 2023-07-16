plain
[00:46:43] 
[00:46:43] 
[00:46:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:46:43] Build completed unsuccessfully in 0:39:55
[00:46:43] make: *** [all] Error 1
[00:46:43] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:030be704
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
