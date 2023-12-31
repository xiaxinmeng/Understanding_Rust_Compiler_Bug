\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":451,"byte_end":451,"line_start":16,"line_end":16,"column_start":52,"column_end":52,"is_primary":true,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":52,"highlight_end":52}],"label":"lifetime `'static` required","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":451,"byte_end":451,"line_start":16,"line_end":16,"column_start":52,"column_end":52,"is_primary":false,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":52,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `existential type`","def_site_span":{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":451,"byte_end":451,"line_start":16,"line_end":16,"column_start":52,"column_end":52,"is_primary":false,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":52,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"add explicit lifetime `'static` to type","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":438,"byte_end":441,"line_start":16,"line_end":16,"column_start":39,"column_end":42,"is_primary":true,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":39,"highlight_end":42}],"label":null,"suggested_replacement":"&'static u8","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in parameter type\n  --> /checkout/src/test/ui/async-fn-multiple-lifetimes.rs:16:52\n   |\nLL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}\n   |                                       ---          ^ lifetime `'static` required\n   |                                       |\n   |                                       help: add explicit lifetime `'static` to type: `&'static u8`\n\n"}
[01:17:04] {"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}
[01:17:04] {"message":"Some errors occurred: E0106, E0309, E0621, E0623, E0707, E0709.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0106, E0309, E0621, E0623, E0707, E0709.\n"}
[01:17:04] 
[01:17:04] ------------------------------------------
[01:17:04] 
[01:17:04] thread '[ui] ui/async-fn-multiple-lifetimes.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
---
[01:17:04] 
[01:17:04] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:17:04] 
[01:17:04] 
[01:17:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:04] 
[01:17:04] 
[01:17:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:04] Build completed unsuccessfully in 0:04:52
[01:17:04] Build completed unsuccessfully in 0:04:52
[01:17:04] Makefile:48: recipe for target 'check' failed
[01:17:04] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:248ec51b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 22 11:09:17 UTC 2019
---
travis_time:end:0c44601a:start=1553252959195513693,finish=1553252959201669181,duration=6155488
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00bdc27c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2469f016
travis_time:start:2469f016
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c1c75f2
$ dmesg | grep -i kill
