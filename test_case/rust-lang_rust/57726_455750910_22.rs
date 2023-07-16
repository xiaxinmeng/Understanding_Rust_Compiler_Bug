\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs","byte_start":498,"byte_end":520,"line_start":20,"line_end":20,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"impl LintPass for Pass {","highlight_start":1,"highlight_end":23}],"label":"missing `name` in implementation","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`name` from trait: `fn(&Self) -> &'static str`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0046]: not all trait items implemented, missing: `name`\n  --> /checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs:20:1\n   |\nLL | impl LintPass for Pass {\n   | ^^^^^^^^^^^^^^^^^^^^^^ missing `name` in implementation\n   |\n   = note: `name` from trait: `fn(&Self) -> &'static str`\n\n"}
[01:14:00] {"message":"For more information about this error, try `rustc --explain E0046`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0046`.\n"}
[01:14:00] 
[01:14:00] ------------------------------------------
[01:14:00] 
[01:14:00] 
[01:14:00] thread '[ui] ui-fulldeps/lint_tool_cmdline_allow.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:14:00] 
[01:14:00] ---- [ui] ui-fulldeps/lint_tool_test.rs stdout ----
[01:14:00] 
[01:14:00] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs" failed to compile: 
[01:14:00] status: exit code: 1
[01:14:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint_tool_test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_test/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint_tool_test/auxiliary"
[01:14:00] ------------------------------------------
[01:14:00] 
[01:14:00] ------------------------------------------
[01:14:00] stderr:
[01:14:00] stderr:
[01:14:00] ------------------------------------------
[01:14:00] {"message":"not all trait items implemented, missing: `name`","code":{"code":"E0046","explanation":"\nItems are missing in a trait implementation. Erroneous code example:\n\n