\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/lint-group-plugin.rs","byte_start":565,"byte_end":587,"line_start":15,"line_end":15,"column_start":11,"column_end":33,"is_primary":true,"text":[{"text":"#![plugin(lint_group_plugin_test)]","highlight_start":11,"highlight_end":33}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `lint_group_plugin_test`\n  --> /checkout/src/test/ui-fulldeps/lint-group-plugin.rs:15:11\n   |\nLL | #![plugin(lint_group_plugin_test)]\n   |           ^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
[02:43:47] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[02:43:47] {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/lint-group-plugin.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
[02:43:47] diff of stderr:
[02:43:47] 
[02:43:47] - error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
[02:43:47] -   --> $DIR/lint-plugin-forbid-attrs.rs:20:9
[02:43:47] + error[E0463]: can't find crate for `lint_plugin_test`
[02:43:47] +   --> $DIR/lint-plugin-forbid-attrs.rs:15:11
[02:43:47] 3    |
[02:43:47] - LL | #![forbid(test_lint)]
[02:43:47] -    |           --------- `forbid` level set here
[02:43:47] - ...
[02:43:47] - LL | #[allow(test_lint)]
[02:43:47] -    |         ^^^^^^^^^ overruled by previous forbid
[02:43:47] + LL | #![plugin(lint_plugin_test)]
[02:43:47] 9 
[02:43:47] 9 
[02:43:47] - error: item is named 'lintme'
[02:43:47] -    |
[02:43:47] -    |
[02:43:47] - LL | fn lintme() { } //~ ERROR item is named 'lintme'
[02:43:47] -    |
[02:43:47] - note: lint level defined here
[02:43:47] -   --> $DIR/lint-plugin-forbid-attrs.rs:16:11
[02:43:47] -    |
[02:43:47] -    |
[02:43:47] - LL | #![forbid(test_lint)]
[02:43:47] + error: aborting due to previous error
[02:43:47] 21 
[02:43:47] - error: aborting due to 2 previous errors
[02:43:47] - 
[02:43:47] - 
[02:43:47] - For more information about this error, try `rustc --explain E0453`.
[02:43:47] + For more information about this error, try `rustc --explain E0463`.
[02:43:47] 25 
[02:43:47] 
[02:43:47] 
[02:43:47] The actual stderr differed from the expected stderr.
[02:43:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
[02:43:47] To update references, rerun the tests and pass the `--bless` flag
[02:43:47] To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`
[02:43:47] error: 1 errors occurred comparing output.
[02:43:47] status: exit code: 1
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lint-plugin-forbid-attrs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-A" "unused"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"can't find crate for `lint_plugin_test`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n