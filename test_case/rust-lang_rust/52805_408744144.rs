plain
[01:00:56] test [run-pass] run-pass/trivial-message.rs ... ok
[01:00:56] test [run-pass] run-pass/try-operator-hygiene.rs ... ok
[01:00:57] test [run-pass] run-pass/try-macro.rs ... ok

Broadcast message from root@travis-job-feee2b13-5b2a-4a1d-ad05-039941501fc5
 (unknown) at 4:33 ...
The system is going down for power off NOW!
[01:00:57] 
[01:00:57] Session terminated, terminating shell... ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:2abf60c1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
