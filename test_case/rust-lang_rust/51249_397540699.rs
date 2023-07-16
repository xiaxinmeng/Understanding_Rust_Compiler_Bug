plain
[00:52:06] diff of stderr:
[00:52:06] 
[00:52:06] 2   --> $DIR/enum.rs:19:5
[00:52:06] 3    |
[00:52:06] 4 LL |     let Wrap(x) = &Wrap(3);
[00:52:06] -    |              - consider changing this to `x`
[00:52:06] +    |              - help: use a mutable reference instead: `x`
[00:52:06] 6 LL |     *x += 1; //~ ERROR cannot assign to immutable
[00:52:06] 7    |     ^^^^^^^ cannot borrow as mutable
[00:52:06] 
[00:52:06] 10   --> $DIR/enum.rs:23:9
[00:52:06] 11    |
[00:52:06] 11    |
[00:52:06] 12 LL |     if let Some(x) = &Some(3) {
[00:52:06] -    |                 - consider changing this to `x`
[00:52:06] +    |                 - help: use a mutable reference instead: `x`
[00:52:06] 14 LL |         *x += 1; //~ ERROR cannot assign to immutable
[00:52:06] 15    |         ^^^^^^^ cannot borrow as mutable
[00:52:06] 
[00:52:06] 18   --> $DIR/enum.rs:29:9
[00:52:06] 19    |
[00:52:06] 19    |
[00:52:06] 20 LL |     while let Some(x) = &Some(3) {
[00:52:06] -    |                    - consider changing this to `x`
[00:52:06] +    |                    - help: use a mutable reference instead: `x`
[00:52:06] 22 LL |         *x += 1; //~ ERROR cannot assign to immutable
[00:52:06] 23    |         ^^^^^^^ cannot borrow as mutable
[00:52:06] 
[00:52:06] 
[00:52:06] The actual stderr differed from the expected stderr.
[00:52:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/enum.stderr
[00:52:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/enum.stderr
[00:52:06] To update references, rerun the tests and pass the `--bless` flag
[00:52:06] To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/enum.rs`
[00:52:06] error: 1 errors occurred comparing output.
[00:52:06] status: exit code: 101
[00:52:06] status: exit code: 101
[00:52:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/encified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:19:5\n   |\nLL |     let Wrap(x) = &Wrap(3);\n   |              - help: use a mutable reference instead: `x`\nLL |     *x += 1; //~ ERROR cannot assign to immutable\n   |     ^^^^^^^ cannot borrow as mutable\n\n"}
[00:52:06] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":656,"byte_end":663,"line_start":23,"line_end":23,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a mutable reference instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":632,"byte_end":633,"line_start":22,"line_end":22,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"    if let Some(x) = &Some(3) {","highlight_start":17,"highlight_end":18}],"label":null,"suggested_replacement":"x","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:23:9\n   |\nLL |     if let Some(x) = &Some(3) {\n   |                 - help: use a mutable reference instead: `x`\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^ cannot borrow as mutable\n\n"}
[00:52:06] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":783,"byte_end":790,"line_start":29,"line_end":29,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a mutable reference instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":759,"byte_end":760,"line_start":28,"line_end":28,"column_start":20,"column_end":21,"is_primary":true,"text":[{"text":"    while let Some(x) = &Some(3) {","highlight_start":20,"highlight_end":21}],"label":null,"suggested_replacement":"x","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:29:9\n   |\nLL |     while let Some(x) = &Some(3) {\n   |                    - help: use a mutable reference instead: `x`\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^ cannot borrow as mutable\n\n"}
[00:52:06] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:52:06] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:52:06] ------------------------------------------
[00:52:06] 
[00:52:06] thread '[ui] ui/rfc-2005-default-binding-mode/enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:52:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:52:06] 
[00:52:06] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:52:06] 
[00:52:06] 
[00:52:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:06] 
[00:52:06] 
[00:52:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:06] Build completed unsuccessfully in 0:02:55
[00:52:06] Build completed unsuccessfully in 0:02:55
[00:52:06] Makefile:58: recipe for target 'check' failed
[00:52:06] make: *** [check] Error 1
travis_fold:start:after_failure.4
travis_time:start:053bded0
travis_time:start:053bded0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f52f171
$ dmesg | grep -i kill
