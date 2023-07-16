plain
[00:17:45] * wrapping_iter_arith              lib      stable       1.14.0  
[00:17:45] * wrapping_neg                     lib      stable       1.10.0  
[00:17:45] * wrapping_next_power_of_two       lib      unstable     None    
[00:17:45] * wrapping_ref                     lib      stable       1.14.0  
[00:17:46] invalid license Apache-2.0 in /checkout/obj/build/tmp/distcheck/src/vendor/bytesize/Cargo.toml
[00:17:46] some tidy checks failed
[00:17:46] 
[00:17:46] 
[00:17:46] command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/tmp/distcheck/src" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
[00:17:46] 
[00:17:46] 
[00:17:46] failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test
[00:17:46] Build completed unsuccessfully in 0:02:12
[00:17:46] Build completed unsuccessfully in 0:02:12
[00:17:46] Makefile:58: recipe for target 'check' failed
[00:17:46] make: *** [check] Error 1
[00:17:46] 
[00:17:46] 
[00:17:46] command did not execute successfully: "make" "check"
[00:17:46] 
[00:17:46] 
[00:17:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:17:46] Build completed unsuccessfully in 0:13:34
---
travis_time:end:11346a3f:start=1537525723946127287,finish=1537525723953254211,duration=7126924
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03080488
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0675ccd3
travis_time:start:0675ccd3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03ab2d84
$ dmesg | grep -i kill
