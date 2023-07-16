\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/test-inner-fn.rs","byte_start":287,"byte_end":298,"line_start":14,"line_end":14,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        fn bar() {}","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)\n  --> /checkout/src/test/ui/lint/test-inner-fn.rs:14:9\n   |\nLL |         fn bar() {}\n   |         ^^^^^^^^^^^\n   |\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:14:36] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] 
---
[01:14:36] 10 
[01:14:36] 
[01:14:36] 
[01:14:36] The actual stderr differed from the expected stderr.
[01:14:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/ambiguous-builtin-attrs-test.stderr
[01:14:36] To update references, rerun the tests and pass the `--bless` flag
[01:14:36] To only update this specific test, also pass `--test-args proc-macro/ambiguous-builtin-attrs-test.rs`
[01:14:36] error: 1 errors occurred comparing output.
[01:14:36] status: exit code: 1
[01:14:36] status: exit code: 1
[01:14:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/auxiliary" "-A" "unused"
[01:14:36] ------------------------------------------
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] stderr:
[01:14:36] stderr:
[01:14:36] ------------------------------------------
[01:14:36] {"message":"cannot find value `NonExistent` in this scope","code":{"code":"E0425","explanation":"\nAn unresolved name was used.\n\nErroneous code examples:\n\n