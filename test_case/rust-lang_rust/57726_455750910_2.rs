\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/lint_for_crate.rs","byte_start":422,"byte_end":444,"line_start":19,"line_end":19,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"impl LintPass for Pass {","highlight_start":1,"highlight_end":23}],"label":"missing `name` in implementation","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`name` from trait: `fn(&Self) -> &'static str`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0046]: not all trait items implemented, missing: `name`\n  --> /checkout/src/test/ui-fulldeps/auxiliary/lint_for_crate.rs:19:1\n   |\nLL | impl LintPass for Pass {\n   | ^^^^^^^^^^^^^^^^^^^^^^ missing `name` in implementation\n   |\n   = note: `name` from trait: `fn(&Self) -> &'static str`\n\n"}
[01:14:00] {"message":"For more information about this error, try `rustc --explain E0046`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0046`.\n"}
[01:14:00] 
[01:14:00] ------------------------------------------
[01:14:00] 
[01:14:00] 
[01:14:00] thread '[ui] ui-fulldeps/issue-15778-fail.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:14:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:14:00] 
[01:14:00] ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
[01:14:00] 
[01:14:00] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs" failed to compile: 
[01:14:00] status: exit code: 1
[01:14:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
[01:14:00] ------------------------------------------
[01:14:00] 
[01:14:00] ------------------------------------------
[01:14:00] stderr:
[01:14:00] stderr:
[01:14:00] ------------------------------------------
[01:14:00] {"message":"unused import: `LateLintPassObject`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs","byte_start":271,"byte_end":289,"line_start":12,"line_end":12,"column_start":69,"column_end":87,"is_primary":true,"text":[{"text":"use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};","highlight_start":69,"highlight_end":87}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `LateLintPassObject`\n  --> /checkout/src/test/ui-fulldeps/auxiliary/lint_group_plugin_test.rs:12:69\n   |\nLL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};\n   |                                                                     ^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:14:00] {"message":"not all trait items implemented, missing: `name`","code":{"code":"E0046","explanation":"\nItems are missing in a trait implementation. Erroneous code example:\n\n