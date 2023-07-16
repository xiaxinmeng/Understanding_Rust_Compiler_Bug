\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs","byte_start":1790,"byte_end":1801,"line_start":46,"line_end":46,"column_start":37,"column_end":48,"is_primary":true,"text":[{"text":"                                    NodeId::new(0));","highlight_start":37,"highlight_end":48}],"label":"function or associated item not found in `syntax::ast::NodeId`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"items from traits can only be used if the trait is in scope","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"the following trait is implemented but not in scope, perhaps add a `use` for it:","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs","byte_start":652,"byte_end":652,"line_start":21,"line_end":21,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"use syntax::feature_gate::Features;","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use rustc_data_structures::indexed_vec::Idx;\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0599]: no function or associated item named `new` found for type `syntax::ast::NodeId` in the current scope\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/procedural_mbe_matching.rs:46:37\n   |\nLL |                                     NodeId::new(0));\n   |                                     ^^^^^^^^^^^ function or associated item not found in `syntax::ast::NodeId`\n   |\n   = help: items from traits can only be used if the trait is in scope\nhelp: the following trait is implemented but not in scope, perhaps add a `use` for it:\n   |\nLL | use rustc_data_structures::indexed_vec::Idx;\n   |\n\n"}
[01:15:26] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:15:26] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[01:15:26] ------------------------------------------
[01:15:26] 
[01:15:26] thread '[run-pass] run-pass-fulldeps/mbe_matching_test_macro.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:15:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:15:26] test result: FAILED. 96 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:15:26] 
[01:15:26] 
[01:15:26] 
[01:15:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:26] 
[01:15:26] 
[01:15:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:26] Build completed unsuccessfully in 0:26:18
[01:15:26] Build completed unsuccessfully in 0:26:18
[01:15:26] Makefile:58: recipe for target 'check' failed
[01:15:26] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0068d5b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
