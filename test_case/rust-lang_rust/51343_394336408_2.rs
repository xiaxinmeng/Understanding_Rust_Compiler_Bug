\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/asm-out-assign-imm.rs","byte_start":847,"byte_end":885,"line_start":31,"line_end":31,"column_start":9,"column_end":47,"is_primary":true,"text":[{"text":"        asm!(\"mov $1, $0\" : \"=r\"(x) : \"r\"(5));","highlight_start":9,"highlight_end":47}],"label":"cannot assign twice to immutable variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/asm-out-assign-imm.rs","byte_start":807,"byte_end":812,"line_start":28,"line_end":28,"column_start":5,"column_end":10,"is_primary":false,"text":[{"text":"    x = 1;","highlight_start":5,"highlight_end":10}],"label":"first assignment to `x`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0384]: cannot assign twice to immutable variable `x`\n  --> /checkout/src/test/ui/asm-out-assign-imm.rs:31:9\n   |\nLL |     x = 1;\n   |     ----- first assignment to `x`\n...\nLL |         asm!(\"mov $1, $0\" : \"=r\"(x) : \"r\"(5));\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable\n\n"}
[00:50:43] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:43] {"message":"For more information about this error, try `rustc --explain E0384`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0384`.\n"}
[00:50:43] ------------------------------------------
[00:50:43] 
[00:50:43] thread '[ui] ui/asm-out-assign-imm.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:50:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:43] 
[00:50:43] ---- [ui] ui/target-feature-wrong.rs stdout ----
[00:50:43] diff of stderr:
[00:50:43] 
[00:50:43] 1 error: #[target_feature] attribute must be of the form #[target_feature(..)]
[00:50:43] +   --> $DIR/target-feature-wrong.rs:23:1
[00:50:43] 3    |
[00:50:43] 3    |
[00:50:43] 4 LL | #[target_feature = "+sse2"]
[00:50:43] 
[00:50:43] 6 
[00:50:43] 6 
[00:50:43] 7 error: the feature named `foo` is not valid for this target
[00:50:43] +   --> $DIR/target-feature-wrong.rs:25:18
[00:50:43] 9    |
[00:50:43] 9    |
[00:50:43] 10 LL | #[target_feature(enable = "foo")]
[00:50:43] 
[00:50:43] 12 
[00:50:43] 12 
[00:50:43] 13 error: #[target_feature(..)] only accepts sub-keys of `enable` currently
[00:50:43] +   --> $DIR/target-feature-wrong.rs:27:18
[00:50:43] 15    |
[00:50:43] 15    |
[00:50:43] 16 LL | #[target_feature(bar)]
[00:50:43] 
[00:50:43] 18 
[00:50:43] 18 
[00:50:43] 19 error: #[target_feature(..)] only accepts sub-keys of `enable` currently
[00:50:43] +   --> $DIR/target-feature-wrong.rs:29:18
[00:50:43] 21    |
[00:50:43] 21    |
[00:50:43] 22 LL | #[target_feature(disable = "baz")]
[00:50:43] 
[00:50:43] 24 
[00:50:43] 24 
[00:50:43] 25 error: #[target_feature(..)] can only be applied to `unsafe` function
[00:50:43] +   --> $DIR/target-feature-wrong.rs:33:1
[00:50:43] 27    |
[00:50:43] 27    |
[00:50:43] 28 LL | #[target_feature(enable = "sse2")]
[00:50:43] 
[00:50:43] 30 
[00:50:43] 31 error: attribute should be applied to a function
[00:50:43] -   --> $DIR/target-feature-wrong.rs:35:1
[00:50:43] -   --> $DIR/target-feature-wrong.rs:35:1
[00:50:43] +   --> $DIR/target-feature-wrong.rs:37:1
[00:50:43] 33    |
[00:50:43] 34 LL | #[target_feature(enable = "sse2")]
[00:50:43] 
[00:50:43] 
[00:50:43] 38    | -------------- not a function
[00:50:43] 39 
[00:50:43] 40 error: cannot use #[inline(always)] with #[target_feature]
[00:50:43] +   --> $DIR/target-feature-wrong.rs:41:1
[00:50:43] 42    |
[00:50:43] 42    |
[00:50:43] 43 LL | #[inline(always)]
[00:50:43] 
[00:50:43] 
[00:50:43] The actual stderr differed from the expected stderr.
[00:50:43] The actual stderr differed from the expected stderr.
[00:50:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong/target-feature-wrong.stderr
[00:50:43] To update references, rerun the tests and pass the `--bless` flag
[00:50:43] To only update this specific test, also pass `--test-args target-feature-wrong.rs`
[00:50:43] error: 1 errors occurred comparing output.
[00:50:43] status: exit code: 101
[00:50:43] status: exit code: 101
[00:50:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature-wrong.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong/auxiliary" "-A" "unused"
[00:50:43] ------------------------------------------
[00:50:43] 
[00:50:43] ------------------------------------------
[00:50:43] stderr:
[00:50:43] stderr:
[00:50:43] ------------------------------------------
[00:50:43] {"message":"#[target_feature] attribute must be of the form #[target_feature(..)]","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":648,"byte_end":675,"line_start":23,"line_end":23,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"#[target_feature = \"+sse2\"]","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature] attribute must be of the form #[target_feature(..)]\n  --> /checkout/src/test/ui/target-feature-wrong.rs:23:1\n   |\nLL | #[target_feature = \"+sse2\"]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:43] {"message":"the feature named `foo` is not valid for this target","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":725,"byte_end":739,"line_start":25,"line_end":25,"column_start":18,"column_end":32,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"foo\")]","highlight_start":18,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the feature named `foo` is not valid for this target\n  --> /checkout/src/test/ui/target-feature-wrong.rs:25:18\n   |\nLL | #[target_feature(enable = \"foo\")]\n   |                  ^^^^^^^^^^^^^^\n\n"}
[00:50:43] {"message":"#[target_feature(..)] only accepts sub-keys of `enable` currently","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":797,"byte_end":800,"line_start":27,"line_end":27,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"#[target_feature(bar)]","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] only accepts sub-keys of `enable` currently\n  --> /checkout/src/test/ui/target-feature-wrong.rs:27:18\n   |\nLL | #[target_feature(bar)]\n   |                  ^^^\n\n"}
[00:50:43] {"message":"#[target_feature(..)] only accepts sub-keys of `enable` currently","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":854,"byte_end":869,"line_start":29,"line_end":29,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"#[target_feature(disable = \"baz\")]","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] only accepts sub-keys of `enable` currently\n  --> /checkout/src/test/ui/target-feature-wrong.rs:29:18\n   |\nLL | #[target_feature(disable = \"baz\")]\n   |                  ^^^^^^^^^^^^^^^\n\n"}
[00:50:43] {"message":"#[target_feature(..)] can only be applied to `unsafe` function","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":926,"byte_end":960,"line_start":33,"line_end":33,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"sse2\")]","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] can only be applied to `unsafe` function\n  --> /checkout/src/test/ui/target-feature-wrong.rs:33:1\n   |\nLL | #[target_feature(enable = \"sse2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:43] {"message":"attribute should be applied to a function","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1106,"byte_end":1120,"line_start":39,"line_end":39,"column_start":1,"column_end":15,"is_primary":false,"text":[{"text":"mod another {}","highlight_start":1,"highlight_end":15}],"label":"not a function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1027,"byte_end":1061,"line_start":37,"line_end":37,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"sse2\")]","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute should be applied to a function\n  --> /checkout/src/test/ui/target-feature-wrong.rs:37:1\n   |\nLL | #[target_feature(enable = \"sse2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nLL | //~^ ERROR: should be applied to a function\nLL | mod another {}\n   | -------------- not a function\n\n"}
[00:50:43] {"message":"cannot use #[inline(always)] with #[target_feature]","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1122,"byte_end":1139,"line_start":41,"line_end":41,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"#[inline(always)]","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: cannot use #[inline(always)] with #[target_feature]\n  --> /checkout/src/test/ui/target-feature-wrong.rs:41:1\n   |\nLL | #[inline(always)]\n   | ^^^^^^^^^^^^^^^^^\n\n"}
[00:50:43] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[00:50:43] ------------------------------------------
[00:50:43] 
[00:50:43] thread '[ui] ui/target-feature-wrong.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:50:43] 
---
[00:50:43] 
[00:50:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:43] 
[00:50:43] 
[00:50:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:43] 
[00:50:43] 
[00:50:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:43] Build completed unsuccessfully in 0:02:53
[00:50:43] Build completed unsuccessfully in 0:02:53
[00:50:43] make: *** [check] Error 1
[00:50:43] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16a32268
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
