plain
[00:15:11] [RUSTC-TIMING] syntax_pos test:false 3.304
[00:15:11]    Compiling rustc_errors v0.0.0 (file:///checkout/obj/build/tmp/distcheck/src/librustc_errors)
[00:15:18] [RUSTC-TIMING] rustc_errors test:false 6.472

Broadcast message from root@travis-job-94fb62c3-c331-4289-8878-1d3e91ba872e
 (unknown) at 4:15 ...
The system is going down for power off NOW!
[00:15:22] 
[00:15:22] Session terminated, terminating shell... ...terminated.
[00:15:22] Makefile:58: recipe for target 'check' failed
[00:15:22] make: *** [check] Terminated

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:347b8ed9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
