plain
[00:30:01] curl: (6) Could not resolve host: static.rust-lang.org
[00:30:01] 
[00:30:01] spurious failure, trying again
[00:33:54] curl: (6) Could not resolve host: static.rust-lang.org
[00:33:54] failed to run: curl -s --retry 3 -Sf -o /tmp/tmplRirpo.sha256 https://static.rust-lang.org/dist/2018-05-10/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:33:54] Build completed unsuccessfully in 0:19:26
[00:33:54] Makefile:58: recipe for target 'check' failed
[00:33:54] make: *** [check] Error 1
[00:33:54] 
[00:33:54] 
[00:33:54] command did not execute successfully: "make" "check"
[00:33:54] 
[00:33:54] 
[00:33:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:33:54] Build completed unsuccessfully in 0:29:55
---
travis_time:end:0c281b50:start=1529736941454631538,finish=1529736941461548975,duration=6917437
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0351594a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ef137f5
$ dmesg | grep -i kill
