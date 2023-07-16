plain
[00:47:30] ....................................................................................................
[00:47:35] ....................................................................................................
[00:47:41] ...............................................................................................i....
[00:47:46] ..........................................................................i.........................
[00:47:51] .........................................................................................F.F........
[00:47:56] ....................................................................................................
[00:48:02] ..............................................................................F.....................
[00:48:07] .......i.................iiiiiiiii...................................................
[00:48:07] 
[00:48:07] ---- [ui] ui/rfc-2005-default-binding-mode/enum.rs stdout ----
[00:48:07] diff of stderr:
[00:48:07] 
[00:48:07] 
[00:48:07] 2   --> $DIR/enum.rs:19:5
[00:48:07] 3    |
[00:48:07] 4 LL |     let Wrap(x) = &Wrap(3);
[00:48:07] -    |              - consider changing this to `x`
[00:48:07] +    |              - consider changing this to `ref mut`
[00:48:07] 6 LL |     *x += 1; //~ ERROR cannot assign to immutable
[00:48:07] 7    |     ^^^^^^^ cannot borrow as mutable
[00:48:07] 
[00:48:07] 10   --> $DIR/enum.rs:23:9
[00:48:07] 11    |
[00:48:07] 11    |
[00:48:07] 12 LL |     if let Some(x) = &Some(3) {
[00:48:07] -    |                 - consider changing this to `x`
[00:48:07] +    |                 - consider changing this to `ref mut`
[00:48:07] 14 LL |         *x += 1; //~ ERROR cannot assign to immutable
[00:48:07] 15    |         ^^^^^^^ cannot borrow as mutable
[00:48:07] 
[00:48:07] 18   --> $DIR/enum.rs:29:9
[00:48:07] 19    |
[00:48:07] 19    |
[00:48:07] 20 LL |     while let Some(x) = &Some(3) {
[00:48:07] -    |                    - consider changing this to `x`
[00:48:07] +    |                    - consider changing this to `ref mut`
[00:48:07] 22 LL |         *x += 1; //~ ERROR cannot assign to immutable
[00:48:07] 23    |         ^^^^^^^ cannot borrow as mutable
[00:48:07] 
[00:48:07] 
[00:48:07] The actual stderr differed from the expected stderr.
[00:48:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/enum.stderr
[00:48:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/enum.stderr
[00:48:07] To update references, rerun the tests and pass the `--bless` flag
[00:48:07] To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/enum.rs`
[00:48:07] error: 1 errors occurred comparing output.
[00:48:07] status: exit code: 101
[00:48:07] status: exit code: 101
[00:48:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/auxiliary" "-A" "unused"
[00:48:07] ------------------------------------------
[00:48:07] 
[00:48:07] ------------------------------------------
[00:48:07] stderr:
[00:48:07] stderr:
[00:48:07] ------------------------------------------
[00:48:07] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":568,"byte_end":575,"line_start":19,"line_end":19,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    *x += 1; //~ ERROR cannot assign to immutable","highlight_start":5,"highlight_end":12}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":549,"byte_end":550,"line_start":18,"line_end":18,"column_start":14,"column_end":15,"is_primary":false,"text":[{"text":"    let Wrap(x) = &Wrap(3);","highlight_start":14,"highlight_end":15}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:19:5\n   |\nLL |     let Wrap(x) = &Wrap(3);\n   |              - consider changing this to `ref mut`\nLL |     *x += 1; //~ ERROR cannot assign to immutable\n   |     ^^^^^^^ cannot borrow as mutable\n\n"}
[00:48:07] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":656,"byte_end":663,"line_start":23,"line_end":23,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":632,"byte_end":633,"line_start":22,"line_end":22,"column_start":17,"column_end":18,"is_primary":false,"text":[{"text":"    if let Some(x) = &Some(3) {","highlight_start":17,"highlight_end":18}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:23:9\n   |\nLL |     if let Some(x) = &Some(3) {\n   |                 - consider changing this to `ref mut`\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^ cannot borrow as mutable\n\n"}
[00:48:07] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":783,"byte_end":790,"line_start":29,"line_end":29,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":759,"byte_end":760,"line_start":28,"line_end":28,"column_start":20,"column_end":21,"is_primary":false,"text":[{"text":"    while let Some(x) = &Some(3) {","highlight_start":20,"highlight_end":21}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:29:9\n   |\nLL |     while let Some(x) = &Some(3) {\n   |                    - consider changing this to `ref mut`\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^ cannot borrow as mutable\n\n"}
[00:48:07] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:48:07] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:48:07] ------------------------------------------
[00:48:07] 
[00:48:07] thread '[ui] ui/rfc-2005-default-binding-mode/enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:48:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:07] 
[00:48:07] ---- [ui] ui/rfc-2005-default-binding-mode/explicit-mut.rs stdout ----
[00:48:07] diff of stderr:
[00:48:07] 
[00:48:07] 2   --> $DIR/explicit-mut.rs:17:13
[00:48:07] 3    |
[00:48:07] 4 LL |         Some(n) => {
[00:48:07] -    |              - consider changing this to `n`
[00:48:07] +    |              - consider changing this to `ref mut`
[00:48:07] 6 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:48:07] 7    |             ^^^^^^^ cannot borrow as mutable
[00:48:07] 
[00:48:07] 10   --> $DIR/explicit-mut.rs:25:13
[00:48:07] 11    |
[00:48:07] 11    |
[00:48:07] 12 LL |         Some(n) => {
[00:48:07] -    |              - consider changing this to `n`
[00:48:07] +    |              - consider changing this to `ref mut`
[00:48:07] 14 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:48:07] 15    |             ^^^^^^^ cannot borrow as mutable
[00:48:07] 
[00:48:07] 18   --> $DIR/explicit-mut.rs:33:13
[00:48:07] 19    |
[00:48:07] 19    |
[00:48:07] 20 LL |         Some(n) => {
[00:48:07] -    |              - consider changing this to `n`
[00:48:07] +    |              - consider changing this to `ref mut`
[00:48:07] 22 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:48:07] 23    |             ^^^^^^^ cannot borrow as mutable
[00:48:07] 
[00:48:07] 
[00:48:07] The actual stderr differed from the expected stderr.
[00:48:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/explicit-mut.stderr
[00:48:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/explicit-mut.stderr
[00:48:07] To update references, rerun the tests and pass the `--bless` flag
[00:48:07] To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/explicit-mut.rs`
[00:48:07] error: 1 errors occurred comparing output.
[00:48:07] status: exit code: 101
[00:48:07] status: exit code: 101
[00:48:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/auxiliary" "-A" "unused"
[00:48:07] ------------------------------------------
[00:48:07] 
[00:48:07] ------------------------------------------
[00:48:07] stderr:
[00:48:07] stderr:
[00:48:07] ------------------------------------------
[00:48:07] {"message":"cannot assign to immutable borrowed content `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":659,"byte_end":666,"line_start":17,"line_end":17,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":639,"byte_end":640,"line_start":16,"line_end":16,"column_start":14,"column_end":15,"is_primary":false,"text":[{"text":"        Some(n) => {","highlight_start":14,"highlight_end":15}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:17:13\n   |\nLL |         Some(n) => {\n   |              - consider changing this to `ref mut`\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot borrow as mutable\n\n"}
[00:48:07] {"message":"cannot assign to immutable borrowed content `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":828,"byte_end":835,"line_start":25,"line_end":25,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":808,"byte_end":809,"line_start":24,"line_end":24,"column_start":14,"column_end":15,"is_primary":false,"text":[{"text":"        Some(n) => {","highlight_start":14,"highlight_end":15}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:25:13\n   |\nLL |         Some(n) => {\n   |              - consider changing this to `ref mut`\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot borrow as mutable\n\n"}
[00:48:07] {"message":"cannot assign to immutable borrowed content `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":997,"byte_end":1004,"line_start":33,"line_end":33,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":977,"byte_end":978,"line_start":32,"line_end":32,"column_start":14,"column_end":15,"is_primary":false,"text":[{"text":"        Some(n) => {","highlight_start":14,"highlight_end":15}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:33:13\n   |\nLL |         Some(n) => {\n   |              - consider changing this to `ref mut`\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot borrow as mutable\n\n"}
[00:48:07] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:48:07] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:48:07] ------------------------------------------
[00:48:07] 
[00:48:07] thread '[ui] ui/rfc-2005-default-binding-mode/explicit-mut.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:48:07] 
[00:48:07] 
[00:48:07] ---- [ui] ui/suggestions/issue-51244.rs stdout ----
[00:48:07] 
[00:48:07] error: /checkout/src/test/ui/suggestions/issue-51244.rs:13: unexpected error: '13:5: 13:16: cannot assign to immutable borrowed content `*my_ref` [E0594]'
[00:48:07] 
[00:48:07] error: 1 unexpected errors found, 0 expected errors not found
[00:48:07] status: exit code: 101
[00:48:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-51244.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/auxiliary" "-A" "unused"
[00:48:07] unexpected errors (from JSON output): [
[00:48:07]     Error {
[00:48:07]         line_num: 13,
[00:48:07]         kind: Some(
[00:48:07]         ),
[00:48:07]         ),
[00:48:07]         msg: "13:5: 13:16: cannot assign to immutable borrowed content `*my_ref` [E0594]"
[00:48:07] ]
[00:48:07] 
[00:48:07] thread '[ui] ui/suggestions/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:48:07] 
---
[00:48:07] 
[00:48:07] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:48:07] 
[00:48:07] 
[00:48:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:07] 
[00:48:07] 
[00:48:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:07] Build completed unsuccessfully in 0:02:44
[00:48:07] Build completed unsuccessfully in 0:02:44
[00:48:07] make: *** [check] Error 1
[00:48:07] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c7393df
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
