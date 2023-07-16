\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs","byte_start":4406,"byte_end":4411,"line_start":115,"line_end":115,"column_start":22,"column_end":27,"is_primary":true,"text":[{"text":"                word.ident.segments.last().unwrap().ident","highlight_start":22,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0609]: no field `ident` on type `&syntax::ast::MetaItem`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:115:22\n   |\nLL |                 word.ident.segments.last().unwrap().ident\n   |                      ^^^^^\n\n"}
[01:00:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:10] {"message":"For more information about this error, try `rustc --explain E0609`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0609`.\n"}
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] thread '[run-pass] run-pass-fulldeps/macro-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:00:10] 
[01:00:10] 
[01:00:10] ---- [run-pass] run-pass-fulldeps/plugin-lib-ok-in-plugin.rs stdout ----
[01:00:10] 
[01:00:10] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile: 
[01:00:10] status: exit code: 1
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-lib-ok-in-plugin/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-g0:10] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile: 
[01:00:10] status: exit code: 1
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-plus-extern-crate/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-plus-extern-crate/auxiliary"
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] ------------------------------------------
[01:00:10] stderr:
[01:00:10] stderr:
[01:00:10] ------------------------------------------
[01:00:10] {"message":"unused macro definition","code":{"code":"unused_macros","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs","byte_start":1010,"byte_end":1053,"line_start":33,"line_end":33,"column_start":1,"column_end":44,"is_primary":true,"text":[{"text":"macro_rules! unexported_macro { () => (3) }","highlight_start":1,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_macros)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused macro definition\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:33:1\n   |\nLL | macro_rules! unexported_macro { () => (3) }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_macros)] on by default\n\n"}
[01:00:10] {"message":"no field `ident` on type `&syntax::ast::MetaItem`","code":{"code":"E0609","explanation":"\nAttempted to access a non-existent field in a struct.\n\nErroneous code example:\n\n