\n\nMaybe you just misspelled the lint name or the lint doesn't exist anymore.\nEither way, try to update/remove it in order to fix the error.\n"},"level":"error","spans":[],"children":[{"message":"requested on the command line with `-A test_lint`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0602]: unknown lint: `test_lint`\n   |\n   = note: requested on the command line with `-A test_lint`\n\n"}
[02:43:47] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[02:43:47] {"message":"Some errors occurred: E0457, E0602.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0457, E0602.\n"}
[02:43:47] {"message":"For more information about an error, try `rustc --explain E0457`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0457`.\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/lint_tool_cmdline_allow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/proc-macro/attribute-spans-preserved.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/attribute-spans-preserved.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/attribute-spans-preserved.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/proc-macro/attribute-spans-preserved/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/proc-macro/attribute-spans-preserved/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/attribute-spans-preserved.rs","byte_start":566,"byte_end":589,"line_start":19,"line_end":19,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/attribute-spans-preserved.rs:19:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/proc-macro/attribute-spans-preserved.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/lint_tool_test.rs stdout ----
[02:43:47] diff of stderr:
[02:43:47] 
[02:43:47] - warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
[02:43:47] + error[E0457]: plugin `lint_tool_test` only found in rlib format, but must be available in dylib format
[02:43:47] +   --> $DIR/lint_tool_test.rs:16:11
[02:43:47] +    |
[02:43:47] + LL | #![plugin(lint_tool_test)]
[02:43:47] + 
[02:43:47] + 
[02:43:47] + warning: unknown lint: `test_lint`
[02:43:47] 3    |
[02:43:47] 3    |
[02:43:47] 4 LL | #![cfg_attr(foo, warn(test_lint))]
[02:43:47] 
[02:43:47] -    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
[02:43:47] 6    |
[02:43:47] -    = note: #[warn(renamed_and_removed_lints)] on by default
[02:43:47] +    = note: #[warn(unknown_lints)] on by default
[02:43:47] 8 
[02:43:47] 8 
[02:43:47] - warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
[02:43:47] + warning: unknown lint: `clippy_group`
[02:43:47] 11    |
[02:43:47] 11    |
[02:43:47] 12 LL | #![deny(clippy_group)]
[02:43:47] 
[02:43:47] -    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
[02:43:47] 14 
[02:43:47] 14 
[02:43:47] - warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
[02:43:47] + warning: unknown lint: `test_group`
[02:43:47] 17    |
[02:43:47] 17    |
[02:43:47] 18 LL | #[allow(test_group)]
[02:43:47] 
[02:43:47] -    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
[02:43:47] 20 
[02:43:47] 21 warning: unknown lint: `this_lint_does_not_exist`
[02:43:47] 22   --> $DIR/lint_tool_test.rs:37:8
[02:43:47] 
[02:43:47] 
[02:43:47] 23    |
[02:43:47] 24 LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
[02:43:47] -    |
[02:43:47] -    = note: #[warn(unknown_lints)] on by default
[02:43:47] 28 
[02:43:47] 28 
[02:43:47] - warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
[02:43:47] -    |
[02:43:47] -    |
[02:43:47] - LL | #![cfg_attr(foo, warn(test_lint))]
[02:43:47] -    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
[02:43:47] + error: aborting due to previous error
[02:43:47] 34 
[02:43:47] - error: item is named 'lintme'
[02:43:47] -    |
[02:43:47] -    |
[02:43:47] - LL | fn lintme() { } //~ ERROR item is named 'lintme'
[02:43:47] -    |
[02:43:47] - note: lint level defined here
[02:43:47] -   --> $DIR/lint_tool_test.rs:21:9
[02:43:47] -    |
[02:43:47] -    |
[02:43:47] - LL | #![deny(clippy_group)]
[02:43:47] -    |         ^^^^^^^^^^^^
[02:43:47] -    = note: #[deny(clippy::test_lint)] implied by #[deny(clippy::group)]
[02:43:47] - 
[02:43:47] - error: item is named 'lintmetoo'
[02:43:47] -    |
[02:43:47] -    |
[02:43:47] - LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
[02:43:47] -    |
[02:43:47] - note: lint level defined here
[02:43:47] -   --> $DIR/lint_tool_test.rs:21:9
[02:43:47] -    |
[02:43:47] -    |
[02:43:47] - LL | #![deny(clippy_group)]
[02:43:47] -    |         ^^^^^^^^^^^^
[02:43:47] -    = note: #[deny(clippy::test_group)] implied by #[deny(clippy::group)]
[02:43:47] - error: aborting due to 2 previous errors
---
[02:43:47] test result: FAILED. 2 passed; 40 failed; 0 ignored; 0 measured; 0 filtered out
[02:43:47] 
[02:43:47] 
[02:43:47] 
[02:43:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-musl" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "x86_64-linux-musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:43:47] 
[02:43:47] 
[02:43:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --host x86_64-unknown-linux-musl --target x86_64-unknown-linux-musl
[02:43:47] Build completed unsuccessfully in 2:24:33
