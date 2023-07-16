plain
Resolving deltas: 100% (611170/611170), completed with 4856 local objects.
---
[00:00:45] configure: rust.quiet-tests     := True
---
[00:43:14] ..........................................................................i.........................
[00:43:20] .................i..................................................................................
---
[00:43:57] .............................................................................................i......
[00:44:04] ....................................................................i...............................
[00:44:11] ....................................................................................................
[00:44:18] ....................................................................................................
[00:44:26] ...........................................F........................................................
---
[00:44:27] 40 error: cannot use #[inline(always)] with #[target_feature]
[00:44:27] -   --> $DIR/target-feature-wrong.rs:36:1
[00:44:27] +   --> $DIR/target-feature-wrong.rs:37:1
[00:44:27] 42    |
[00:44:27] 43 LL | #[inline(always)]
---
[00:44:27] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'target-feature-wrong.rs'
[00:44:27]
[00:44:27] error: 1 errors occurred comparing output.
[00:44:27] status: exit code: 101
[00:44:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature-wrong.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:44:27] {"message":"#[target_feature = \"..\"] is deprecated and will eventually be removed, use #[target_feature(enable = \"..\")] instead","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":580,"byte_end":607,"line_start":19,"line_end":19,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"#[target_feature = \"+sse2\"]","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: #[target_feature = \"..\"] is deprecated and will eventually be removed, use #[target_feature(enable = \"..\")] instead\n  --> /checkout/src/test/ui/target-feature-wrong.rs:19:1\n   |\nLL | #[target_feature = \"+sse2\"]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:44:27] {"message":"the feature named `foo` is not valid for this target","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":647,"byte_end":661,"line_start":21,"line_end":21,"column_start":18,"column_end":32,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"foo\")]","highlight_start":18,"highlight_end":32}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: the feature named `foo` is not valid for this target\n  --> /checkout/src/test/ui/target-feature-wrong.rs:21:18\n   |\nLL | #[target_feature(enable = \"foo\")]\n   |                  ^^^^^^^^^^^^^^\n\n"}
[00:44:27] {"message":"#[target_feature(..)] only accepts sub-keys of `enable` currently","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":719,"byte_end":722,"line_start":23,"line_end":23,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"#[target_feature(bar)]","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] only accepts sub-keys of `enable` currently\n  --> /checkout/src/test/ui/target-feature-wrong.rs:23:18\n   |\nLL | #[target_feature(bar)]\n   |                  ^^^\n\n"}
[00:44:27] {"message":"#[target_feature(..)] only accepts sub-keys of `enable` currently","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":776,"byte_end":791,"line_start":25,"line_end":25,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"#[target_feature(disable = \"baz\")]","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] only accepts sub-keys of `enable` currently\n  --> /checkout/src/test/ui/target-feature-wrong.rs:25:18\n   |\nLL | #[target_feature(disable = \"baz\")]\n   |                  ^^^^^^^^^^^^^^^\n\n"}
[00:44:27] {"message":"#[target_feature(..)] can only be applied to `unsafe` function","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":848,"byte_end":882,"line_start":29,"line_end":29,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"sse2\")]","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] can only be applied to `unsafe` function\n  --> /checkout/src/test/ui/target-feature-wrong.rs:29:1\n   |\nLL | #[target_feature(enable = \"sse2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:44:27] {"message":"attribute should be applied to a function","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1028,"byte_end":1042,"line_start":35,"line_end":35,"column_start":1,"column_end":15,"is_primary":false,"text":[{"text":"mod another {}","highlight_start":1,"highlight_end":15}],"label":"not a function","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":949,"byte_end":983,"line_start":33,"line_end":33,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"sse2\")]","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: attribute should be applied to a function\n  --> /checkout/src/test/ui/target-feature-wrong.rs:33:1\n   |\nLL | #[target_feature(enable = \"sse2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nLL | //~^ ERROR: should be applied to a function\nLL | mod another {}\n   | -------------- not a function\n\n"}
[00:44:27] {"message":"cannot use #[inline(always)] with #[target_feature]","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1044,"byte_end":1061,"line_start":37,"line_end":37,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"#[inline(always)]","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: cannot use #[inline(always)] with #[target_feature]\n  --> /checkout/src/test/ui/target-feature-wrong.rs:37:1\n   |\nLL | #[inline(always)]\n   | ^^^^^^^^^^^^^^^^^\n\n"}
[00:44:27] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
---
[00:44:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:27] expected success, got: exit code: 101
[00:44:27]
[00:44:27]
[00:44:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:27] Build completed unsuccessfully in 0:02:30
[00:44:27] make: *** [check] Error 1
[00:44:27] Makefile:58: recipe for target 'check' failed
---
$ dmesg | grep -i kill
[   10.866782] init: failsafe main process (1093) killed by TERM signal
