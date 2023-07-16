plain
[00:27:33] curl: (6) Could not resolve host: static.rust-lang.org
[00:27:33] 
[00:27:33] spurious failure, trying again
[00:31:27] curl: (6) Could not resolve host: static.rust-lang.org
[00:31:27] failed to run: curl -s --retry 3 -Sf -o /tmp/tmpyydmix.sha256 https://static.rust-lang.org/dist/2018-06-30/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:31:27] Build completed unsuccessfully in 0:19:26
[00:31:27] Makefile:58: recipe for target 'check' failed
[00:31:27] make: *** [check] Error 1
[00:31:27] 
[00:31:27] 
[00:31:27] command did not execute successfully: "make" "check"
[00:31:27] 
[00:31:27] 
[00:31:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:31:27] Build completed unsuccessfully in 0:28:47
---
travis_time:end:08bed90d:start=1531205497364462230,finish=1531205497372361178,duration=7898948
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04985211
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1578da7c
$ dmesg | grep -i kill
