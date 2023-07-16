\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-load.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2014 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `lint_plugin_test`\n\n"}
[02:43:47] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[02:43:47] {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/lint-plugin-cmdline-load.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs","byte_start":565,"byte_end":588,"line_start":18,"line_end":18,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs:18:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs","byte_start":670,"byte_end":693,"line_start":23,"line_end":23,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs:23:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs","byte_start":772,"byte_end":795,"line_start":28,"line_end":28,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs:28:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs","byte_start":900,"byte_end":923,"line_start":33,"line_end":33,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs:33:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs.rs stdout ----
[02:43:47] 
[02:43:47] error: auxiliary build of "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs" failed to compile: 
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs/auxiliary"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`","code":null,"level":"warning","spans":[],"children":[],"rendered":"warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs","byte_start":565,"byte_end":588,"line_start":18,"line_end":18,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs:18:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs","byte_start":670,"byte_end":693,"line_start":23,"line_end":23,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs:23:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs","byte_start":772,"byte_end":795,"line_start":28,"line_end":28,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs:28:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs","byte_start":900,"byte_end":923,"line_start":33,"line_end":33,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[proc_macro_attribute]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/builtin-attrs.rs:33:1\n   |\nLL | #[proc_macro_attribute]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[02:43:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] thread '[ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[02:43:47] 
[02:43:47] 
[02:43:47] ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
[02:43:47] 
[02:43:47] error: test compilation failed although it shouldn't!
[02:43:47] status: exit code: 1
[02:43:47] command: "/checkout/obj/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lint-plugin/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-musl/test/ui-fulldeps/lint-plugin/auxiliary" "-A" "unused"
[02:43:47] ------------------------------------------
[02:43:47] 
[02:43:47] ------------------------------------------
[02:43:47] stderr:
[02:43:47] stderr:
[02:43:47] ------------------------------------------
[02:43:47] {"message":"can't find crate for `lint_plugin_test`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n