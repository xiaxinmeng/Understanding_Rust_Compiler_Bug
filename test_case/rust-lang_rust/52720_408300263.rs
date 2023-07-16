plain
[00:14:38] * wrapping_iter_arith              lib      stable       1.14.0  
[00:14:38] * wrapping_neg                     lib      stable       1.10.0  
[00:14:38] * wrapping_next_power_of_two       lib      unstable     None    
[00:14:38] * wrapping_ref                     lib      stable       1.14.0  
[00:14:39] invalid license Unlicense OR MIT in /checkout/obj/build/tmp/distcheck/src/vendor/termcolor/Cargo.toml
[00:14:39] some tidy checks failed
[00:14:39] 
[00:14:39] 
[00:14:39] command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/tmp/distcheck/src" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
[00:14:39] 
[00:14:39] 
[00:14:39] failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test
[00:14:39] Build completed unsuccessfully in 0:01:56
[00:14:39] Build completed unsuccessfully in 0:01:56
[00:14:39] make: *** [check] Error 1
[00:14:39] Makefile:58: recipe for target 'check' failed
[00:14:39] 
[00:14:39] 
[00:14:39] command did not execute successfully: "make" "check"
[00:14:39] 
[00:14:39] 
[00:14:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:14:39] Build completed unsuccessfully in 0:11:58
---
travis_time:end:118b35b5:start=1532661794807952092,finish=1532661794816837939,duration=8885847
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b32562e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d01f062
travis_time:start:0d01f062
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a73cd4c
$ dmesg | grep -i kill
