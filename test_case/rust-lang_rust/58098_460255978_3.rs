\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs","byte_start":1325,"byte_end":1330,"line_start":46,"line_end":46,"column_start":38,"column_end":43,"is_primary":true,"text":[{"text":"            allow_internal_unstable: false,","highlight_start":38,"highlight_end":43}],"label":"expected struct `std::vec::Vec`, found bool","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `std::vec::Vec<syntax::ast::Symbol>`\n   found type `bool`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:46:38\n   |\nLL |             allow_internal_unstable: false,\n   |                                      ^^^^^ expected struct `std::vec::Vec`, found bool\n   |\n   = note: expected type `std::vec::Vec<syntax::ast::Symbol>`\n              found type `bool`\n\n"}
[01:12:55] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[01:12:55] 
[01:12:55] ------------------------------------------
[01:12:55] 
---
[01:12:55] test result: FAILED. 44 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:12:55] 
[01:12:55] 
[01:12:55] 
[01:12:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:55] 
[01:12:55] 
[01:12:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:55] Build completed unsuccessfully in 0:15:35
[01:12:55] Build completed unsuccessfully in 0:15:35
[01:12:55] make: *** [check] Error 1
[01:12:55] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bc63a6c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb  4 13:49:53 UTC 2019
