\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-34229.rs","byte_start":85,"byte_end":95,"line_start":2,"line_end":2,"column_start":46,"column_end":56,"is_primary":true,"text":[{"text":"#[derive(PartialEq, PartialOrd)] struct Nope(Comparable);","highlight_start":46,"highlight_end":56}],"label":"no implementation for `Comparable < Comparable` and `Comparable > Comparable`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/issues/issue-34229.rs","byte_start":60,"byte_end":70,"line_start":2,"line_end":2,"column_start":21,"column_end":31,"is_primary":false,"text":[{"text":"#[derive(PartialEq, PartialOrd)] struct Nope(Comparable);","highlight_start":21,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialOrd)]","def_site_span":null}}],"children":[{"message":"the trait `std::cmp::PartialOrd` is not implemented for `Comparable`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"required by `std::cmp::PartialOrd::partial_cmp`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: can't compare `Comparable` with `Comparable`\n  --> /checkout/src/test/ui/issues/issue-34229.rs:2:46\n   |\nLL | #[derive(PartialEq, PartialOrd)] struct Nope(Comparable);\n   |                                              ^^^^^^^^^^ no implementation for `Comparable < Comparable` and `Comparable > Comparable`\n   |\n   = help: the trait `std::cmp::PartialOrd` is not implemented for `Comparable`\n   = note: required by `std::cmp::PartialOrd::partial_cmp`\n\n"}
[00:46:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:20] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] thread '[ui] ui/issues/issue-34229.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:46:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:20] 
[00:46:20] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:46:20] 
[00:46:20] 
[00:46:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:20] 
[00:46:20] 
[00:46:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:20] Build completed unsuccessfully in 0:03:22
[00:46:20] Build completed unsuccessfully in 0:03:22
[00:46:20] make: *** [check] Error 1
[00:46:20] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17b48cfe
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0fa77722:start=1539641198423935658,finish=1539641198428606784,duration=4671126
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:000a06aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ca414e0
travis_time:start:0ca414e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e78e9f5
$ dmesg | grep -i kill
