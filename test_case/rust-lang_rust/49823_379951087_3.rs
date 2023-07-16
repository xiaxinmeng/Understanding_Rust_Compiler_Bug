\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/compile-fail-fulldeps/proc-macro/auxiliary/attributes-included.rs","byte_start":3662,"byte_end":3668,"line_start":125,"line_end":125,"column_start":46,"column_end":52,"is_primary":true,"text":[{"text":"        TokenTree::Term(tt) => assert_eq!(tt.as_str(), \"foo\"),","highlight_start":46,"highlight_end":52}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0599]: no method named `as_str` found for type `&proc_macro::Term` in the current scope\n  --> /checkout/src/test/compile-fail-fulldeps/proc-macro/auxiliary/attributes-included.rs:125:46\n   |\nLL |         TokenTree::Term(tt) => assert_eq!(tt.as_str(), \"foo\"),\n   |                                              ^^^^^^\n\n"}
[01:04:38] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:04:38] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
---
[01:04:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:38] expected success, got: exit code: 101
[01:04:38]
[01:04:38]
[01:04:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:38] Build completed unsuccessfully in 0:23:01
[01:04:38] make: *** [check] Error 1
[01:04:38] Makefile:58: recipe for target 'check' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:37640cec:start=1523326250043798910,finish=1523326250052703169,duration=8904259
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:10ce25fe
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:10ce25fe:start=1523326250060450167,finish=1523326250068704804,duration=8254637
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05cb7900
$ dmesg | grep -i kill
[   11.149527] init: failsafe main process (1093) killed by TERM signal
