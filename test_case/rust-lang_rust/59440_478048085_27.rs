\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-28134.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// compile-flags: --test","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)\n   |\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:14:03] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:14:03] 
[01:14:03] ------------------------------------------
[01:14:03] 
[01:14:03] 
[01:14:03] thread '[ui] ui/issues/issue-28134.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:14:03] 
[01:14:03] ---- [ui] ui/issues/issue-53675-a-test-called-panic.rs stdout ----
[01:14:03] 
[01:14:03] error: test compilation failed although it shouldn't!
[01:14:03] status: exit code: 1
[01:14:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53675-a-test-called-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53675-a-test-called-panic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53675-a-test-called-panic/auxiliary" "-A" "unused"
[01:14:03] ------------------------------------------
[01:14:03] 
[01:14:03] ------------------------------------------
[01:14:03] stderr:
[01:14:03] stderr:
[01:14:03] ------------------------------------------
[01:14:03] {"message":"use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n