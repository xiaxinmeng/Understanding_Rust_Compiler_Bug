plain
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:20] 
[01:08:20] running 231 tests
[01:09:37] ...F...............i.....................................................F..........................
[01:10:22] .........i...........................................................F.........................F....
[01:10:31] failures:
[01:10:31] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:489:22
[01:10:31] 
[01:10:31] ---- [rustdoc] rustdoc/assoc-consts.rs stdout ----
[01:10:31] ---- [rustdoc] rustdoc/assoc-consts.rs stdout ----
[01:10:31]  
[01:10:31] error: htmldocck failed!
[01:10:31] status: exit code: 1
[01:10:31] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/assoc-consts.rs"
[01:10:31] ------------------------------------------
[01:10:31] 
[01:10:31] ------------------------------------------
[01:10:31] stderr:
[01:10:31] stderr:
[01:10:31] ------------------------------------------
[01:10:31] 114: @has check failed
[01:10:31]  `XPATH PATTERN` did not match
[01:10:31]      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT0 in trait."
[01:10:31] 123: @has check failed
[01:10:31]  `XPATH PATTERN` did not match
[01:10:31]      // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT2 in trait."
[01:10:31] Encountered 2 errors
[01:10:31] 
[01:10:31] ------------------------------------------
[01:10:31] 
[01:10:31] 
[01:10:31] thread '[rustdoc] rustdoc/assoc-consts.rs' panicked at 'explicit panic', tools/co01:10:31] thread '[rustdoc] rustdoc/issue-12834.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3001:9
[01:10:31] ---- [rustdoc] rustdoc/manual_impl.rs stdout ----
[01:10:31]  
[01:10:31]  
[01:10:31] error: htmldocck failed!
[01:10:31] status: exit code: 1
[01:10:31] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/manual_impl.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/manual_impl.rs"
[01:10:31] ------------------------------------------
[01:10:31] 
[01:10:31] ------------------------------------------
[01:10:31] stderr:
[01:10:31] stderr:
[01:10:31] ------------------------------------------
[01:10:31] 37: @has check failed
[01:10:31]  `XPATH PATTERN` did not match
[01:10:31]  // @has - '//*[@class="docblock"]' 'Docs associated with the trait b_method definition.'
[01:10:31] 38: @has check failed
[01:10:31]  `XPATH PATTERN` did not match
[01:10:31]  // @has - '//*[@class="docblock"]' 'Docs associated with the trait b_method definition.'
[01:10:31] 39: @has check failed
[01:10:31]  `XPATH PATTERN` did not match
[01:10:31]  // @has - '//*[@class="docblock"]' 'Docs associated with the trait c_method definition.'
[01:10:31] 58: @has check failed
[01:10:31]  `XPATH PATTERN` did not match
[01:10:31]  // @has - '//*[@class="docblock"]' 'Docs associated with the trait b_method definition.'
[01:10:31] 78: @has check failed
[01:10:31]  `XPATH PATTERN` did not match
[01:10:31]  // @has - '//*[@class="docblock"]' 'Docs associated with the trait a_method definition.'
[01:10:31] Encountered 5 errors
[01:10:31] 
[01:10:31] ------------------------------------------
[01:10:31] 
[01:10:31] 
[01:10:31] thread '[rustdoc] rustdoc/manual_impl.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3001:9
[01:10:31] 
[01:10:31] ---- [rustdoc] rustdoc/search-index-summaries.rs stdout ----
[01:10:31]  
[01:10:31] error: htmldocck failed!
[01:10:31] status: exit code: 1
[01:10:31] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/search-index-summaries.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/search-index-summaries.rs"
[01:10:31] ------------------------------------------
[01:10:31] 
[01:10:31] ------------------------------------------
[01:10:31] stderr:
[01:10:31] stderr:
[01:10:31] ------------------------------------------
[01:10:31] 13: @has check failed
[01:10:31]  `PATTERN` did not match
[01:10:31]  // @has 'search-index.js' 'Foo short link.'
[01:10:31] Encountered 1 errors
[01:10:31] 
[01:10:31] ------------------------------------------
[01:10:31] 
---
[01:10:31] test result: FAILED. 225 passed; 4 failed; 2 ignored; 0 measured; 0 filtered out
[01:10:31] 
[01:10:31] 
[01:10:31] 
[01:10:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:31] 
[01:10:31] 
[01:10:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:31] Build completed unsuccessfully in 0:26:33
[01:10:31] Build completed unsuccessfully in 0:26:33
[01:10:31] make: *** [check] Error 1
[01:10:31] Makefile:58: recipe for target 'check' failed
travis_time:end:091031ca:start=15255363579409 find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:0fc71328:start=1525540591226231257,finish=1525540591232662503,duration=6431246
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09e61c96
