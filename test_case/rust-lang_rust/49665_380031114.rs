plain
[00:01:07] configure: rust.quiet-tests     := True
---
[00:46:20] ..............................................................................i.....................
[00:46:27] .....................i..............................................................................
---
[00:47:10] i.........................................................................i.........................
[00:47:17] ....................................................................................................
[00:47:24] ....................................................................................................
[00:47:32] ...................................................F................................................
---
[00:47:34] 1 warning: #[target_feature = ".."] is deprecated and will eventually be removed, use #[target_feature(enable = "..")] instead
[00:47:34] -   --> $DIR/target-feature-wrong.rs:19:1
[00:47:34] +   --> $DIR/target-feature-wrong.rs:21:1
[00:47:34] 3    |
[00:47:34] 4 LL | #[target_feature = "+sse2"]
[00:47:34] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:47:34]
[00:47:34] 6
[00:47:34] 7 error: the feature named `foo` is not valid for this target
[00:47:34] -   --> $DIR/target-feature-wrong.rs:21:18
[00:47:34] +   --> $DIR/target-feature-wrong.rs:23:18
[00:47:34] 9    |
[00:47:34] 10 LL | #[target_feature(enable = "foo")]
[00:47:34] 11    |                  ^^^^^^^^^^^^^^
[00:47:34]
[00:47:34] 12
[00:47:34] 13 error: #[target_feature(..)] only accepts sub-keys of `enable` currently
[00:47:34] -   --> $DIR/target-feature-wrong.rs:23:18
[00:47:34] +   --> $DIR/target-feature-wrong.rs:25:18
[00:47:34] 15    |
[00:47:34] 16 LL | #[target_feature(bar)]
[00:47:34] 17    |                  ^^^
[00:47:34]
[00:47:34] 18
[00:47:34] 19 error: #[target_feature(..)] only accepts sub-keys of `enable` currently
[00:47:34] -   --> $DIR/target-feature-wrong.rs:25:18
[00:47:34] +   --> $DIR/target-feature-wrong.rs:27:18
[00:47:34] 21    |
[00:47:34] 22 LL | #[target_feature(disable = "baz")]
[00:47:34] 23    |                  ^^^^^^^^^^^^^^^
[00:47:34]
[00:47:34] 24
[00:47:34] 25 error: #[target_feature(..)] can only be applied to `unsafe` function
[00:47:34] -   --> $DIR/target-feature-wrong.rs:29:1
[00:47:34] +   --> $DIR/target-feature-wrong.rs:31:1
[00:47:34] 27    |
[00:47:34] 28 LL | #[target_feature(enable = "sse2")]
---
[00:47:34] 34 LL | #[target_feature(enable = "sse2")]
[00:47:34] 35    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:47:34]
[00:47:34] 38    | -------------- not a function
[00:47:34] 39
[00:47:34] 40 error: cannot use #[inline(always)] with #[target_feature]
[00:47:34] -   --> $DIR/target-feature-wrong.rs:36:1
[00:47:34] +   --> $DIR/target-feature-wrong.rs:39:1
[00:47:34] 42    |
[00:47:34] 43 LL | #[inline(always)]
---
[00:47:34] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'target-feature-wrong.rs'
[00:47:34]
[00:47:34] error: 1 errors occurred comparing output.
[00:47:34] status: exit code: 101
[00:47:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature-wrong.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:47:34] {"message":"#[target_feature = \"..\"] is deprecated and will eventually be removed, use #[target_feature(enable = \"..\")] instead","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":614,"byte_end":641,"line_start":21,"line_end":21,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"#[target_feature = \"+sse2\"]","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: #[target_feature = \"..\"] is deprecated and will eventually be removed, use #[target_feature(enable = \"..\")] instead\n  --> /checkout/src/test/ui/target-feature-wrong.rs:21:1\n   |\nLL | #[target_feature = \"+sse2\"]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:47:34] {"message":"the feature named `foo` is not valid for this target","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":681,"byte_end":695,"line_start":23,"line_end":23,"column_start":18,"column_end":32,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"foo\")]","highlight_start":18,"highlight_end":32}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: the feature named `foo` is not valid for this target\n  --> /checkout/src/test/ui/target-feature-wrong.rs:23:18\n   |\nLL | #[target_feature(enable = \"foo\")]\n   |                  ^^^^^^^^^^^^^^\n\n"}
[00:47:34] {"message":"#[target_feature(..)] only accepts sub-keys of `enable` currently","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":753,"byte_end":756,"line_start":25,"line_end":25,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"#[target_feature(bar)]","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] only accepts sub-keys of `enable` currently\n  --> /checkout/src/test/ui/target-feature-wrong.rs:25:18\n   |\nLL | #[target_feature(bar)]\n   |                  ^^^\n\n"}
[00:47:34] {"message":"#[target_feature(..)] only accepts sub-keys of `enable` currently","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":810,"byte_end":825,"line_start":27,"line_end":27,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"#[target_feature(disable = \"baz\")]","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] only accepts sub-keys of `enable` currently\n  --> /checkout/src/test/ui/target-feature-wrong.rs:27:18\n   |\nLL | #[target_feature(disable = \"baz\")]\n   |                  ^^^^^^^^^^^^^^^\n\n"}
[00:47:34] {"message":"#[target_feature(..)] can only be applied to `unsafe` function","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":882,"byte_end":916,"line_start":31,"line_end":31,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"sse2\")]","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] can only be applied to `unsafe` function\n  --> /checkout/src/test/ui/target-feature-wrong.rs:31:1\n   |\nLL | #[target_feature(enable = \"sse2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:47:34] {"message":"attribute should be applied to a function","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1062,"byte_end":1076,"line_start":37,"line_end":37,"column_start":1,"column_end":15,"is_primary":false,"text":[{"text":"mod another {}","highlight_start":1,"highlight_end":15}],"label":"not a function","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":983,"byte_end":1017,"line_start":35,"line_end":35,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"sse2\")]","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: attribute should be applied to a function\n  --> /checkout/src/test/ui/target-feature-wrong.rs:35:1\n   |\nLL | #[target_feature(enable = \"sse2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nLL | //~^ ERROR: should be applied to a function\nLL | mod another {}\n   | -------------- not a function\n\n"}
[00:47:34] {"message":"cannot use #[inline(always)] with #[target_feature]","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":1078,"byte_end":1095,"line_start":39,"line_end":39,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"#[inline(always)]","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: cannot use #[inline(always)] with #[target_feature]\n  --> /checkout/src/test/ui/target-feature-wrong.rs:39:1\n   |\nLL | #[inline(always)]\n   | ^^^^^^^^^^^^^^^^^\n\n"}
[00:47:34] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
---
[00:47:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:34] expected success, got: exit code: 101
[00:47:34]
[00:47:34]
[00:47:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:34] Build completed unsuccessfully in 0:02:31
[00:47:34] make: *** [check] Error 1
[00:47:34] Makefile:58: recipe for target 'check' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0e674552:start=1523351694589122313,finish=1523351694595554908,duration=6432595
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:3a119266
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:3a119266:start=1523351694601538821,finish=1523351694608076895,duration=6538074
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:253edb28
$ dmesg | grep -i kill
[   10.770602] init: failsafe main process (1097) killed by TERM signal
