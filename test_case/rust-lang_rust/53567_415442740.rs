plain
[00:02:07]  Downloading cc v1.0.20
[00:02:08]  Downloading petgraph v0.4.12
[00:02:08]  Downloading serde_derive v1.0.72
[00:02:08]  Downloading getopts v0.2.17
[00:02:08]  Downloading ryu v0.2.4
[00:02:08]  Downloading cfg-if v0.1.5
[00:02:08]  Downloading fixedbitset v0.1.9
[00:02:08]  Downloading ordermap v0.3.5
[00:02:08]  Downloading proc-macro2 v0.4.13
[00:02:08]  Downloading proc-macro2 v0.4.13
[00:02:08]  Downloading quote v0.6.8
[00:02:08]  Downloading syn v0.14.9
[00:02:08]  Downloading unicode-xid v0.1.0
[00:02:08]    Compiling proc-macro2 v0.4.13
[00:02:08]    Compiling unicode-xid v0.1.0
[00:02:08]    Compiling serde v1.0.72
[00:02:08]    Compiling ryu v0.2.4
[00:02:09]    Compiling cfg-if v0.1.5
[00:02:09]    Compiling cc v1.0.20
[00:02:09]    Compiling itoa v0.4.2
[00:02:09]    Compiling libc v0.2.43
---
######################################################################## 100.0%
[00:16:00] extracting /checkout/obj/build/tmp/distcheck/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:16:01]    Compiling proc-macro2 v0.4.13
[00:16:01]    Compiling unicode-xid v0.1.0
[00:16:01]    Compiling ryu v0.2.4
[00:16:01]    Compiling cc v1.0.20
[00:16:01]    Compiling libc v0.2.43
[00:16:01]    Compiling cfg-if v0.1.5
[00:16:01]    Compiling itoa v0.4.2
---
travis_time:start:stage0-tidy
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
[00:17:02]    Compiling proc-macro2 v0.4.13
[00:17:02]    Compiling unicode-xid v0.1.0
[00:17:02]    Compiling ryu v0.2.4
[00:17:02]    Compiling itoa v0.4.2
[00:17:03] [RUSTC-TIMING] itoa test:false 0.339
[00:17:03] [RUSTC-TIMING] itoa test:false 0.339
[00:17:04] [RUSTC-TIMING] ryu test:false 0.807
[00:17:09]    Compiling syn v0.14.9
[00:17:16] [RUSTC-TIMING] serde test:false 12.460
[00:17:16]    Compiling serde_json v1.0.26
[00:17:24]    Compiling serde_derive v1.0.72
---
[00:17:53] * wrapping_iter_arith              lib      stable       1.14.0  
[00:17:53] * wrapping_neg                     lib      stable       1.10.0  
[00:17:53] * wrapping_next_power_of_two       lib      unstable     None    
[00:17:53] * wrapping_ref                     lib      stable       1.14.0  
[00:17:54] invalid license Apache-2.0 in /checkout/obj/build/tmp/distcheck/src/vendor/ryu/Cargo.toml
[00:17:54] some tidy checks failed
[00:17:54] 
[00:17:54] 
[00:17:54] command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/tmp/distcheck/src" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
[00:17:54] 
[00:17:54] 
[00:17:54] failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test
[00:17:54] Build completed unsuccessfully in 0:02:06
[00:17:54] Build completed unsuccessfully in 0:02:06
[00:17:54] Makefile:58: recipe for target 'check' failed
[00:17:54] make: *** [check] Error 1
[00:17:54] 
[00:17:54] 
[00:17:54] command did not execute successfully: "make" "check"
[00:17:54] 
[00:17:54] 
[00:17:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:17:54] Build completed unsuccessfully in 0:14:33
---
travis_time:end:12f18460:start=1535035124237549779,finish=1535035124244337096,duration=6787317
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:29eef088
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:009f2892
travis_time:start:009f2892
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04854b1e
$ dmesg | grep -i kill
