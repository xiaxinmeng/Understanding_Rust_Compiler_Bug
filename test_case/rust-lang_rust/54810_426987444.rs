plain
[00:52:40] ...............................................................................................i.... 2200/4550
[00:52:44] .................................................................................................... 2300/4550
[00:52:48] .................................................................................................... 2400/4550
[00:52:52] .................................................................................................... 2500/4550
[00:52:55] .......iiiiiiiii.................................................................................... 2600/4550
[00:53:01] .................................................................................................... 2800/4550
[00:53:05] .................................................................................................... 2900/4550
[00:53:08] ..........................i......................................................................... 3000/4550
[00:53:11] ......................................................................................i.i..ii....... 3100/4550
---
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:58] 
[00:53:58] running 2872 tests
[00:54:09] .................................................................................................... 100/2872
[00:54:20] .............................................................F...............i...................... 200/2872
[00:54:38] .................................................................................................... 400/2872
[00:54:46] .................................................................................................... 500/2872
[00:54:57] .................................................................................................... 600/2872
[00:55:11] .................................................................................................... 700/2872
[00:55:11] .................................................................................................... 700/2872
[00:55:22] .................................................................................................... 800/2872
[00:55:31] .................................................................................................... 900/2872
[00:55:44] .....................FFF............................................................................ 1000/2872
[00:56:05] .................................................................................................... 1200/2872
[00:56:14] .................................................................................................... 1300/2872
[00:56:26] ...........................................................................i........................ 1400/2872
[00:56:37] .................................................................................................... 1500/2872
[00:56:37] .................................................................................................... 1500/2872
[00:56:48] ............................................i..........F............................................ 1600/2872
[00:57:16] .................................................................................................... 1800/2872
[00:57:26] .....................................................................i.............................. 1900/2872
[00:57:38] .......................................i............................................................ 2000/2872
[00:58:00] .................................................................................................... 2100/2872
[00:58:00] .................................................................................................... 2100/2872
[00:58:07] .................................................................................................... 2200/2872
[00:58:24] .ii.....................................................................i....i...................... 2300/2872
[00:58:37] ...............i.................................................................................... 2400/2872
[00:58:51] .................................................................................................... 2500/2872
[00:59:17] ....................................................F............................................... 2600/2872
[00:59:35] .................................................................................................... 2800/2872
[00:59:46] ........................................................................
[00:59:46] failures:
[00:59:46] 
[00:59:46] 
[00:59:46] ---- [run-pass] run-pass/async-await.rs stdout ----
[00:59:46] normalized stderr:
[00:59:46] warning: struct is never constructed: `Foo`
[00:59:46]   --> $DIR/async-await.rs:122:1
[00:59:46]    |
[00:59:46] LL | struct Foo;
[00:59:46]    |
[00:59:46]    = note: #[warn(dead_code)] on by default
[00:59:46] 
[00:59:46] warning: method is never used: `async_method`
[00:59:46] warning: method is never used: `async_method`
[00:59:46]   --> $DIR/async-await.rs:129:5
[00:59:46]    |
[00:59:46] LL |     async fn async_method(x: u8) -> u8 {
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] The actual stderr differed from the expected stderr.
[00:59:46] Actual stderr saved to /  = note: #[warn(dead_code)] on by default
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] The actual stderr differed from the expected stderr.
[00:59:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/existential-minimal/existential-minimal.stderr
[00:59:46] To update references, rerun the tests and pass the `--bless` flag
[00:59:46] To only update this specific test, also pass `--test-args impl-trait/existential-minimal.rs`
[00:59:46] error: 1 errors occurred comparing output.
[00:59:46] status: exit code: 0
[00:59:46] status: exit code: 0
[00:59:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/impl-trait/existential-minimal.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/existential-minimal/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/existential-minimal/auxiliary"
[00:59:46] ------------------------------------------
[00:59:46] 
[00:59:46] ------------------------------------------
[00:59:46] stderr:
[00:59:46] stderr:
[00:59:46] ------------------------------------------
[00:59:46] {"message":"function is never used: `foo`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/existential-minimal.rs","byte_start":494,"byte_end":526,"line_start":15,"line_end":15,"c^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:46]    = note: #[warn(dead_code)] on by default
[00:59:46] 
[00:59:46] warning: struct is never constructed: `Bar`
[00:59:46]   --> $DIR/issue-49376.rs:20:1
[00:59:46]   --> $DIR/issue-49376.rs:20:1
[00:59:46]    |
[00:59:46] LL | struct Bar {}
[00:59:46] 
[00:59:46] warning: function is never used: `foo`
[00:59:46]   --> $DIR/issue-49376.rs:24:1
[00:59:46]    |
[00:59:46]    |
[00:59:46] LL | fn foo() -> impl Foo {
[00:59:46] 
[00:59:46] warning: function is never used: `test_impl_ops`
[00:59:46]   --> $DIR/issue-49376.rs:28:1
[00:59:46]    |
[00:59:46]    |
[00:59:46] LL | fn test_impl_ops() -> impl Add + Sub + Mul + Div { 1 }
[00:59:46] 
[00:59:46] warning: function is never used: `test_impl_assign_ops`
[00:59:46]   --> $DIR/issue-49376.rs:29:1
[00:59:46]    |
[00:59:46]    |
[00:59:46] LL | fn test_impl_assign_ops() -> impl AddAssign + SubAssign + MulAssign + DivAssign { 1 }
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] The actual stderr differed from the expected stderr.
[00:59:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/issue-49376/issue-49376.stderr
[00:59:46] To update references, rerun the tests and pass the `--bless` flag
[00:59:46] To only update this specific test, also pass `--test-args impl-trait/issue-49376.rs`
[00:59:46] error: 1 errors occurred comparing output.
[00:59:46] status: exit code: 0
[00:59:46] status: exit code: 0
[00:59:46] comma_code)] on by default\n\n"}
[00:59:46] {"message":"struct is never constructed: `Bar`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/issue-49376.rs","byte_start":639,"byte_end":649,"line_start":20,"line_end":20,"column_start":1,"column_end":11,"is_primary":true,"text":[{"text":"struct Bar {}","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: struct is never constructed: `Bar`\n  --> /checkout/src/test/run-pass/impl-trait/issue-49376.rs:20:1\n   |\nLL | struct Bar {}\n   | ^^^^^^^^^^\n\n"}
[00:59:46] {"message":"function is never used: `foo`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/issue-49376.rs","byte_start":697,"byte_end":717,"line_start":24,"line_end":24,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"fn foo() -> impl Foo {","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: function is never used: `foo`\n  --> /checkout/src/test/run-pass/impl-trait/issue-49376.rs:24:1\n   |\nLL | fn foo() -> impl Foo {\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:59:46] {"message":"function is never used: `test_impl_ops`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/issue-49376.rs","byte_start":734,"byte_end":782,"line_start":28,"line_end":28,"co:46] 
[00:59:46] ---- [run-pass] run-pass/issues/issue-49556.rs stdout ----
[00:59:46] warning: function is never used: `iter`
[00:59:46]   --> $DIR/issue-49556.rs:12:1
[00:59:46]    |
[00:59:46]    |
[00:59:46] LL | fn iter<'a>(data: &'a [usize]) -> impl Iterator<Item = usize> + 'a {
[00:59:46]    |
[00:59:46]    = note: #[warn(dead_code)] on by default
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] The actual stderr differed from the expected stderr.
[00:59:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-49556/issue-49556.stderr
[00:59:46] To update references, rerun the tests and pass the `--bless` flag
[00:59:46] To only update this specific test, also pass `--test-args issues/issue-49556.rs`
[00:59:46] error: 1 errors occurred comparing output.
[00:59:46] status: exit code: 0
[00:59:46] status: exit code: 0
[00:59:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-49556.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-49556/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-49556/auxiliary"
[00:59:46] ------------------------------------------
[00:59:46] 
[00:59:46] 
[00:59:46] ------------------------------------s":[{"file_name":"/checkout/src/test/run-pass/traits/conservative_impl_trait.rs","byte_start":490,"byte_end":537,"line_start":14,"line_end":14,"column_start":1,"column_end":48,"is_primary":true,"text":[{"text":"fn batches(n: &u32) -> impl Iterator<Item=&u32> {","highlight_start":1,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: function is never used: `batches`\n  --> /checkout/src/test/run-pass/traits/conservative_impl_trait.rs:14:1\n   |\nLL | fn batches(n: &u32) -> impl Iterator<Item=&u32> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[00:59:46] ------------------------------------------
[00:59:46] 
[00:59:46] thread '[run-pass] run-pass/traits/conservative_impl_trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:59:46] 
---
[00:59:46] test result: FAILED. 2856 passed; 6 failed; 10 ignored; 0 measured; 0 filtered out
[00:59:46] 
[00:59:46] 
[00:59:46] 
[00:59:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:46] 
[00:59:46] 
[00:59:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:46] Build completed unsuccessfully in 0:11:50
[00:59:46] Build completed unsuccessfully in 0:11:50
[00:59:46] Makefile:58: recipe for target 'check' failed
[00:59:46] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02fa2c9c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
