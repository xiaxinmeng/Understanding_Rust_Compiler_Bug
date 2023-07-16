\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/cyclic-trait-hierarchy.rs","byte_start":167,"byte_end":169,"line_start":11,"line_end":11,"column_start":15,"column_end":17,"is_primary":true,"text":[{"text":"pub trait T2: T1 { }","highlight_start":15,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires computing the supertraits of `T1`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/incremental/cyclic-trait-hierarchy.rs","byte_start":98,"byte_end":100,"line_start":6,"line_end":6,"column_start":15,"column_end":17,"is_primary":true,"text":[{"text":"pub trait T1: T2 { }","highlight_start":15,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires computing the supertraits of `T2`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when running analysis passes on this crate","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/incremental/cyclic-trait-hierarchy.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Adapated from rust-lang/rust#58813","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the supertraits of `T2`\n  --> /checkout/src/test/incremental/cyclic-trait-hierarchy.rs:11:15\n   |\nLL | pub trait T2: T1 { }\n   |               ^^\n   |\nnote: ...which requires computing the supertraits of `T1`...\n  --> /checkout/src/test/incremental/cyclic-trait-hierarchy.rs:6:15\n   |\nLL | pub trait T1: T2 { }\n   |               ^^\n   = note: ...which again requires computing the supertraits of `T2`, completing the cycle\n   = note: cycle used when running analysis passes on this crate\n\n"}
[01:19:50] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:19:50] 
[01:19:50] ------------------------------------------
[01:19:50] 
[01:19:50] 
[01:19:50] thread '[incremental] incremental/cyclic-trait-hierarchy.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:50] 
[01:19:50] 
[01:19:50] failures:
[01:19:50] failures:
[01:19:50]     [incremental] incremental/cyclic-trait-hierarchy.rs
[01:19:50] test result: FAILED. 98 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:19:50] 
[01:19:50] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:19:50] 
[01:19:50] 
[01:19:50] 
[01:19:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:50] 
[01:19:50] 
[01:19:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:50] Build completed unsuccessfully in 0:11:49
[01:19:50] Build completed unsuccessfully in 0:11:49
[01:19:50] make: *** [check] Error 1
[01:19:50] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03bf16b2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar  4 14:17:17 UTC 2019
