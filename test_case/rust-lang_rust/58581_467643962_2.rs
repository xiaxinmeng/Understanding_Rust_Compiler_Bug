\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs","byte_start":250,"byte_end":251,"line_start":16,"line_end":16,"column_start":12,"column_end":13,"is_primary":false,"text":[{"text":"    fn bar<U: Debug>(&self, _: &U);","highlight_start":12,"highlight_end":13}],"label":"declaration in trait here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs","byte_start":318,"byte_end":328,"line_start":20,"line_end":20,"column_start":23,"column_end":33,"is_primary":true,"text":[{"text":"    fn bar(&self, _: &impl Debug) { }","highlight_start":23,"highlight_end":33}],"label":"expected generic parameter, found `impl Trait`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try changing the `impl Trait` argument to a generic parameter","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs","byte_start":318,"byte_end":328,"line_start":20,"line_end":20,"column_start":23,"column_end":33,"is_primary":true,"text":[{"text":"    fn bar(&self, _: &impl Debug) { }","highlight_start":23,"highlight_end":33}],"label":null,"suggested_replacement":"U","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs","byte_start":306,"byte_end":306,"line_start":20,"line_end":20,"column_start":11,"column_end":11,"is_primary":true,"text":[{"text":"    fn bar(&self, _: &impl Debug) { }","highlight_start":11,"highlight_end":11}],"label":null,"suggested_replacement":"<U: Debug>","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0643]: method `bar` has incompatible signature for trait\n  --> /checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs:20:23\n   |\nLL |     fn bar<U: Debug>(&self, _: &U);\n   |            - declaration in trait here\n...\nLL |     fn bar(&self, _: &impl Debug) { }\n   |                       ^^^^^^^^^^ expected generic parameter, found `impl Trait`\nhelp: try changing the `impl Trait` argument to a generic parameter\n   |\nLL |     fn bar<U: Debug>(&self, _: &U) { }\n   |           ^^^^^^^^^^            ^\n\n"}
[01:13:49] {"message":"src/librustc_metadata/decoder.rs:484: entry: id not found: DefIndex(1:2333) in crate core with number 2","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_metadata/decoder.rs:484: entry: id not found: DefIndex(1:2333) in crate core with number 2\n\n"}
[01:13:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:49] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:13:49] {"message":"For more information about this error, try `rustc --explain E0643`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0643`.\n"}
[01:13:49] 
[01:13:49] 
[01:13:49] note: the compiler unexpectedly panicked. this is a bug.
[01:13:49] 
[01:13:49] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:13:49] 
[01:13:49] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:13:49] 
[01:13:49] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:13:49] 
[01:13:49] ------------------------------------------
[01:13:49] 
[01:13:49] thread '[ui] ui/impl-trait/impl-generic-mismatch.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:13:49] 
[01:13:49] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:13:49] 
[01:13:49] 
[01:13:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:49] 
[01:13:49] 
[01:13:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:49] Build completed unsuccessfully in 0:04:27
[01:13:49] Build completed unsuccessfully in 0:04:27
[01:13:49] make: *** [check] Error 1
[01:13:49] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:29886230
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 26 22:38:13 UTC 2019
---
travis_time:end:126c3699:start=1551220694317150556,finish=1551220694322276164,duration=5125608
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04659794
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05744b5b
travis_time:start:05744b5b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05568048
$ dmesg | grep -i kill
