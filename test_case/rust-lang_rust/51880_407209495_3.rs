\n\nThe number of supplied parameters must exactly match the numberrap/debug/bootstrap test
[00:53:10] Build completed unsuccessfully in 0:09:04
[00:53:10] Makefile:58: recipe for target 'check' failed
[00:53:10] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15e68262
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1dd7aef5:start=1532381869257576198,finish=1532381869317116840,duration=59540642
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02bda55e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01137f3c
$ dmesg | grep -i kill
