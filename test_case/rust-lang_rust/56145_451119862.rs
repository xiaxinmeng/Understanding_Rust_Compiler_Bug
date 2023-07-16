plain
[00:03:44] * rc_raw                           lib      stable       1.17.0  
[00:03:44] * rc_unique                        lib      stable       1.4.0   
[00:03:44] * rc_weak                          lib      stable       1.4.0   
[00:03:44] * rchunks                          lib      stable       1.31.0  
[00:03:44] * re_rebalance_coherence           lang     unstable     1.32.0  
[00:03:44] * read_initializer                 lib      unstable     None    
[00:03:44] * receiver_into_iter               lib      stable       1.1.0   
[00:03:44] * receiver_trait                   lib      unstable     None    
[00:03:44] * receiver_try_iter                lib      stable       1.15.0  
---
[00:54:54] * rc_raw                           lib      stable       1.17.0  
[00:54:54] * rc_unique                        lib      stable       1.4.0   
[00:54:54] * rc_weak                          lib      stable       1.4.0   
[00:54:54] * rchunks                          lib      stable       1.31.0  
[00:54:54] * re_rebalance_coherence           lang     unstable     1.32.0  
[00:54:54] * read_initializer                 lib      unstable     None    
[00:54:54] * receiver_into_iter               lib      stable       1.1.0   
[00:54:54] * receiver_trait                   lib      unstable     None    
[00:54:54] * receiver_try_iter                lib      stable       1.15.0  
---
[01:02:49] test [run-pass] run-pass/ranges-precedence.rs ... ok
[01:02:49] test [run-pass] run-pass/raw-str.rs ... ok
[01:02:50] test [run-pass] run-pass/rcvr-borrowed-to-region.rs ... ok
[01:02:50] test [run-pass] run-pass/range_inclusive.rs ... ok
[01:02:50] test [run-pass] run-pass/re_rebalance_coherence/coherence-bigint-int.rs ... ok
[01:02:50] test [run-pass] run-pass/re_rebalance_coherence/coherence-bigint-vecint.rs ... ok
[01:02:50] test [run-pass] run-pass/re_rebalance_coherence/coherence-blanket.rs ... ok
[01:02:50] test [run-pass] run-pass/re_rebalance_coherence/coherence-impl-in-fn.rs ... ok
[01:02:50] test [run-pass] run-pass/re_rebalance_coherence/coherence-covered-type-parameter.rs ... ok
[01:02:51] test [run-pass] run-pass/raw-fat-ptr.rs ... ok
[01:02:51] test [run-pass] run-pass/re_rebalance_coherence/coherence-multidispatch-tuple.rs ... ok
[01:02:51] test [run-pass] run-pass/re_rebalance_coherence/coherence-negative-impls-safe.rs ... ok
[01:02:51] test [run-pass] run-pass/re_rebalance_coherence/coherence-iterator-vec-any-elem.rs ... ok
[01:02:51] test [run-pass] run-pass/re_rebalance_coherence/coherence-iterator-vec.rs ... ok
[01:02:51] test [run-pass] run-pass/re_rebalance_coherence/coherence-subtyping.rs ... ok
[01:02:51] test [run-pass] run-pass/re_rebalance_coherence/coherence-rfc447-constrained.rs ... ok
[01:02:51] test [run-pass] run-pass/re_rebalance_coherence/coherence-where-clause.rs ... ok
[01:02:51] test [run-pass] run-pass/re_rebalance_coherence/coherence_copy_like.rs ... ok
[01:02:51] test [run-pass] run-pass/reachable-unnameable-type-alias.rs ... ok
[01:02:51] test [run-pass] run-pass/re_rebalance_coherence/re-rebalance-coherence.rs ... FAILED
[01:02:52] test [run-pass] run-pass/reexport-star.rs ... ok
[01:02:52] test [run-pass] run-pass/reexport-should-still-link.rs ... ok
[01:02:52] test [run-pass] run-pass/reachable-unnameable-items.rs ... ok
[01:02:52] test [run-pass] run-pass/reexport-test-harness-main.rs ... ok
---
[01:04:37] test [run-pass] run-pass/zero-sized/zero-sized-vec-deque-push.rs ... ok
[01:04:37] 
[01:04:37] failures:
[01:04:37] 
[01:04:37] ---- [run-pass] run-pass/re_rebalance_coherence/re-rebalance-coherence.rs stdout ----
[01:04:37] normalized stderr:
[01:04:37] warning: struct is never constructed: `Oracle`
[01:04:37]   --> $DIR/re-rebalance-coherence.rs:9:1
[01:04:37] LL | struct Oracle;
[01:04:37]    | ^^^^^^^^^^^^^^
[01:04:37]    |
[01:04:37]    = note: #[warn(dead_code)] on by default
[01:04:37]    = note: #[warn(dead_code)] on by default
[01:04:37] 
[01:04:37] 
[01:04:37] 
[01:04:37] 
[01:04:37] The actual stderr differed from the expected stderr.
[01:04:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/re_rebalance_coherence/re-rebalance-coherence/re-rebalance-coherence.stderr
[01:04:37] To update references, rerun the tests and pass the `--bless` flag
[01:04:37] To only update this specific test, also pass `--test-args re_rebalance_coherence/re-rebalance-coherence.rs`
[01:04:37] error: 1 errors occurred comparing output.
[01:04:37] status: exit code: 0
[01:04:37] status: exit code: 0
[01:04:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/re_rebalance_coherence/re-rebalance-coherence.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/re_rebalance_coherence/re-rebalance-coherence/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/re_rebalance_coherence/re-rebalance-coherence/auxiliary"
[01:04:37] ------------------------------------------
[01:04:37] 
[01:04:37] ------------------------------------------
[01:04:37] stderr:
[01:04:37] stderr:
[01:04:37] ------------------------------------------
[01:04:37] {"message":"struct is never constructed: `Oracle`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/re_rebalance_coherence/re-rebalance-coherence.rs","byte_start":154,"byte_end":168,"line_start":9,"line_end":9,"column_start":1,"column_end":15,"is_primary":true,"text":[{"text":"struct Oracle;","highlight_start":1,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `Oracle`\n  --> /checkout/src/test/run-pass/re_rebalance_coherence/re-rebalance-coherence.rs:9:1\n   |\nLL | struct Oracle;\n   | ^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:04:37] ------------------------------------------
[01:04:37] 
[01:04:37] 
[01:04:37] thread '[run-pass] run-pass/re_rebalance_coherence/re-rebalance-coherence.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:04:37] 
[01:04:37] 
[01:04:37] failures:
[01:04:37] failures:
[01:04:37]     [run-pass] run-pass/re_rebalance_coherence/re-rebalance-coherence.rs
[01:04:37] test result: FAILED. 2942 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[01:04:37] 
[01:04:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:04:37] 
[01:04:37] 
[01:04:37] 
[01:04:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:37] 
[01:04:37] 
[01:04:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:37] Build completed unsuccessfully in 0:09:45
[01:04:37] Build completed unsuccessfully in 0:09:45
[01:04:37] make: *** [check] Error 1
[01:04:37] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00d40fc1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan  3 11:36:45 UTC 2019
---
travis_time:end:02df156c:start=1546515406765334116,finish=1546515406770947698,duration=5613582
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:024efc80
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bd0d3f5
travis_time:start:0bd0d3f5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f171c53
$ dmesg | grep -i kill
