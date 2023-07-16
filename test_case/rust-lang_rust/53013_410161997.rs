plain
[00:50:05] ....................................................................................................
[00:50:09] ....................................................................................................
[00:50:13] ....i...............................................................................................
[00:50:16] ........i...........................................................................................
[00:50:20] .............................................................................F......................
[00:50:28] failures:
[00:50:28] 
[00:50:28] ---- [ui] ui/rust-2018/edition-lint-infer-outlives.rs stdout ----
[00:50:28] diff of stderr:
[00:50:28] diff of stderr:
[00:50:28] 
[00:50:28] 1 error: outlives requirements can be inferred
[00:50:28] -   --> $DIR/edition-lint-infer-outlives.rs:20:17
[00:50:28] +   --> $DIR/edition-lint-infer-outlives.rs:21:17
[00:50:28] 3    |
[00:50:28] 4 LL | struct Ref<'a, T: 'a> {
[00:50:28] 
[00:50:28] 11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:50:28] 12 
[00:50:28] 13 error: outlives requirements can be inferred
[00:50:28] 13 error: outlives requirements can be inferred
[00:50:28] -   --> $DIR/edition-lint-infer-outlives.rs:31:23
[00:50:28] +   --> $DIR/edition-lint-infer-outlives.rs:32:23
[00:50:28] 15    |
[00:50:28] 16 LL | struct WhereRef<'a, T> where T: 'a {
[00:50:28] 
[00:50:28] 18 
[00:50:28] 19 error: outlives requirements can be inferred
[00:50:28] -   --> $DIR/edition-lint-infer-outlives.rs:36:21
[00:50:28] -   --> $DIR/edition-lint-infer-outlives.rs:36:21
[00:50:28] +   --> $DIR/edition-lint-infer-outlives.rs:37:21
[00:50:28] 21    |
[00:50:28] 22 LL | struct RefRef<'a, 'b: 'a, T: 'b> {
[00:50:28] 
[00:50:28] 24 
[00:50:28] 24 
[00:50:28] 25 error: outlivlity":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs","byte_start":550,"byte_end":580,"line_start":15,"line_end":15,"column_start":9,"column_end":39,"is_primary":true,"text":[{"text":"#![deny(explicit_outlives_requirements)]","highlight_start":9,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this bound","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs","byte_start":760,"byte_end":764,"line_start":21,"line_end":21,"column_start":17,"column_end":21,"is_primary":true,"text":[{"text":"struct Ref<'a, T: 'a> {","highlight_start":17,"highlight_end":21}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: outlives requirements can be inferred\n  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:21:17\n   |\nLL | struct Ref<'a, T: 'a> {\n   |                 ^^^^ help: remove this bound\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:15:9\n   |\nLL | #![deny(explicit_outlives_requirements)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:28] {"message":"outlives requirements can be inferred","code":{"code":"explicit_outlives_requirements","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/editem: 'a","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: outlives requirements can be inferred\n  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:43:32\n   |\nLL |   struct ItemRef<'a, T: Iterator>\n   |  ________________________________^\nLL | | //~^ ERROR outlives requirements can be inferred\nLL | | where\nLL | |     T::Item: 'a\n   | |_______________^ help: remove this bound\n\n"}
[00:50:28] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:50:28] ------------------------------------------
[00:50:28] 
[00:50:28] thread '[ui] ui/rust-2018/edition-lint-infer-outlives.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:50:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:28] 
[00:50:28] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:28] 
[00:50:28] 
[00:50:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:28] 
[00:50:28] 
[00:50:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:28] Build completed unsuccessfully in 0:02:17
[00:50:28] Build completed unsuccessfully in 0:02:17
[00:50:28] Makefile:58: recipe for target 'check' failed
[00:50:28] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2241c252
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
