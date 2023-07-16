plain
[00:48:30] -- Looking for sys/types.h
[00:48:40] -- Looking for sys/types.h - found
[00:48:40] -- Looking for termios.h

Broadcast message from root@travis-job-efe1035a-c29b-45db-83c6-c9983df10d07
 (unknown) at 7:05 ...
The system is going down for power off NOW!
[00:48:47] 
[00:48:47] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:1b77da62
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
