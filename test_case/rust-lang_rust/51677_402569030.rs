plain
[00:17:38] * wrapping_iter_arith              lib      stable       1.14.0  
[00:17:38] * wrapping_neg                     lib      stable       1.10.0  
[00:17:38] * wrapping_next_power_of_two       lib      unstable     None    
[00:17:38] * wrapping_ref                     lib      stable       1.14.0  
[00:17:39] invalid license Apache-2.0 in /checkout/obj/build/tmp/distcheck/src/vendor/ordslice/Cargo.toml
[00:17:39] some tidy checks failed
[00:17:39] 
[00:17:39] 
[00:17:39] command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/tmp/distcheck/src" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
[00:17:39] 
[00:17:39] 
[00:17:39] failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test
[00:17:39] Build completed unsuccessfully in 0:03:12
[00:17:39] Build completed unsuccessfully in 0:03:12
[00:17:39] make: *** [check] Error 1
[00:17:39] Makefile:58: recipe for target 'check' failed
[00:17:39] 
[00:17:39] 
[00:17:39] command did not execute successfully: "make" "check"
[00:17:39] 
[00:17:39] 
[00:17:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:17:39] Build completed unsuccessfully in 0:14:11
---
travis_time:end:12aded4c:start=1530744607504242697,finish=1530744607510667113,duration=6424416
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c9765c0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04601da2
$ dmesg | grep -i kill
