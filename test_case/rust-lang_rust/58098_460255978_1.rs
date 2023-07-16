\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs","byte_start":1325,"byte_end":1330,"line_start":46,"line_end":46,"column_start":38,"column_end":43,"is_primary":true,"text":[{"text":"            allow_internal_unstable: false,","highlight_start":38,"highlight_end":43}],"label":"expected struct `std::vec::Vec`, found bool","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `std::vec::Vec<syntax::ast::Symbol>`\n   found type `bool`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:46:38\n   |\nLL |             allow_internal_unstable: false,\n   |                                      ^^^^^ expected struct `std::vec::Vec`, found bool\n   |\n   = note: expected type `std::vec::Vec<syntax::ast::Symbol>`\n              found type `bool`\n\n"}
[01:12:55] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[01:12:55] 
[01:12:55] ------------------------------------------
[01:12:55] 
[01:12:55] 
[01:12:55] thread '[run-pass] run-pass-fulldeps/plugin-args-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:12:55] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:55] 
[01:12:55] ---- [run-pass] run-pass-fulldeps/plugin-args-2.rs stdout ----
[01:12:55] 
[01:12:55] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile: 
[01:12:55] status: exit code: 1
[01:12:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-2/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-2/auxiliary"
[01:12:55] ------------------------------------------
[01:12:55] 
[01:12:55] ------------------------------------------
[01:12:55] stderr:
[01:12:55] stderr:
[01:12:55] ------------------------------------------
[01:12:55] {"message":"unused import: `syntax::ptr::P`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs","byte_start":397,"byte_end":411,"line_start":17,"line_end":17,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use syntax::ptr::P;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `syntax::ptr::P`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:17:5\n   |\nLL | use syntax::ptr::P;\n   |     ^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:12:55] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n