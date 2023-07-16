\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs","byte_start":6101,"byte_end":6103,"line_start":144,"line_end":144,"column_start":64,"column_end":66,"is_primary":true,"text":[{"text":"const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }","highlight_start":64,"highlight_end":66}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs","byte_start":6104,"byte_end":6105,"line_start":144,"line_end":144,"column_start":67,"column_end":68,"is_primary":false,"text":[{"text":"const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }","highlight_start":67,"highlight_end":68}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:144:64\n   |\nLL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }\n   |                                                                ^^ - temporary value is freed at the end of this statement\n   |                                                                |\n   |                                                                creates a temporary which is freed while still in use\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
---
[00:59:39] test result: FAILED. 6706 passed; 34 failed; 88 ignored; 0 measured; 0 filtered out
[00:59:39] 
[00:59:39] 
[00:59:39] 
[00:59:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:59:39] 
[00:59:39] 
[00:59:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:39] Build completed unsuccessfully in 0:14:06
[00:59:39] Build completed unsuccessfully in 0:14:06
[00:59:39] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04c1d47e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:09542f06:start=1537888319737880385,finish=1537888319742377923,duration=4497538
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11b4d87e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
