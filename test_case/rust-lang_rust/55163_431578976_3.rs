\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs","byte_start":548,"byte_end":564,"line_start":15,"line_end":15,"column_start":11,"column_end":27,"is_primary":true,"text":[{"text":"#![plugin(lint_plugin_test)]","highlight_start":11,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `lint_plugin_test`\n  --> /checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs:15:11\n   |\nLL | #![plugin(lint_plugin_test)]\n   |           ^^^^^^^^^^^^^^^^ can't find crate\n\n"}
[02:43:47] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[02:43:47] {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/lint-plugin-forbid-attrs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
[02:43:47] 
[02:43:47] error: test compilation failed although it shouldn't!
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-load.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lint-plugin-cmdline-load/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-Z" "extra-plugins=lint_plugin_test" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-A" "unused"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"can't find crate for `lint_plugin_test`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n