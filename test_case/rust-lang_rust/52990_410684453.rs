plain
[01:22:26] test string.rs - string::String (line 87) ... ok
[01:22:26] test string.rs - string::String (line 237) ... ok
[01:22:26] test string.rs - string::String (line 263) ... ok

Broadcast message from root@travis-job-1dc12a19-54b5-4b69-ac12-eb94a4c63300
 (unknown) at 11:58 ...
The system is going down for power off NOW!
[01:22:26] 
[01:22:26] Session terminated, terminating shell...make: *** wait: No child processes.  Stop.
[01:22:26] make: *** Waiting for unfinished jobs....
[01:22:26] make: *** wait: No child processes.  Stop.
[01:22:26]  ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:09a360a6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
