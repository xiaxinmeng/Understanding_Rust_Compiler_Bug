\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/test-on-macro.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// compile-pass","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)\n   |\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:14:03] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:14:03] 
[01:14:03] ------------------------------------------
[01:14:03] 
[01:14:03] 
[01:14:03] thread '[ui] ui/test-on-macro.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:14:03] 
[01:14:03] ---- [ui] ui/test-warns-dead-code.rs stdout ----
[01:14:03] diff of stderr:
[01:14:03] 
[01:14:03] - error: function is never used: `dead`
[01:14:03] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:03] 3    |
[01:14:03] 3    |
[01:14:03] - LL | fn dead() {}
[01:14:03] -    |
[01:14:03] - note: lint level defined here
[01:14:03] -   --> $DIR/test-warns-dead-code.rs:3:9
[01:14:03] -    |
[01:14:03] -    |
[01:14:03] - LL | #![deny(dead_code)]
[01:14:03] +    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:14:03] 12 
[01:14:03] 13 error: aborting due to previous error
[01:14:03] 14 
[01:14:03] 14 
[01:14:03] 
[01:14:03] + For more information about this error, try `rustc --explain E0658`.
[01:14:03] 15 
[01:14:03] 
[01:14:03] 
[01:14:03] The actual stderr differed from the expected stderr.
[01:14:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-warns-dead-code/test-warns-dead-code.stderr
[01:14:03] To update references, rerun the tests and pass the `--bless` flag
[01:14:03] To only update this specific test, also pass `--test-args test-warns-dead-code.rs`
[01:14:03] error: 1 errors occurred comparing output.
[01:14:03] status: exit code: 1
[01:14:03] status: exit code: 1
[01:14:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-warns-dead-code.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-warns-dead-code/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-warns-dead-code/auxiliary" "-A" "unused"
[01:14:03] ------------------------------------------
[01:14:03] 
[01:14:03] ------------------------------------------
[01:14:03] stderr:
[01:14:03] stderr:
[01:14:03] ------------------------------------------
[01:14:03] {"message":"use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n