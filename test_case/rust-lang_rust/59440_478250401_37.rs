\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-53675-a-test-called-panic.rs","byte_start":567,"byte_end":612,"line_start":33,"line_end":35,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn panic() {","highlight_start":5,"highlight_end":17},{"text":"        panic!(\"in stmt\");","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)\n  --> /checkout/src/test/ui/issues/issue-53675-a-test-called-panic.rs:33:5\n   |\nLL | /     fn panic() {\nLL | |         panic!(\"in stmt\");\nLL | |     }\n   | |_____^\n   |\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:14:36] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] 
---
[01:14:36] -   --> $DIR/test-inner-fn.rs:5:5
[01:14:36] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:36] +   --> $DIR/test-inner-fn.rs:4:1
[01:14:36] 3    |
[01:14:36] - LL |     #[test]
[01:14:36] + LL | / fn foo() {
[01:14:36] + LL | / fn foo() {
[01:14:36] + LL | |     #[test]
[01:14:36] + LL | |     fn bar() {}
[01:14:36] + LL | |     bar();
[01:14:36] + LL | | }
[01:14:36] 6    |
[01:14:36] 6    |
[01:14:36] -    = note: requested on the command line with `-D unnameable-test-items`
[01:14:36] 8 
[01:14:36] - error: cannot test inner items
[01:14:36] -   --> $DIR/test-inner-fn.rs:13:9
[01:14:36] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:36] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:36] +   --> $DIR/test-inner-fn.rs:6:5
[01:14:36] 11    |
[01:14:36] - LL |         #[test]
[01:14:36] + LL |     fn bar() {}
[01:14:36] +    |     ^^^^^^^^^^^
[01:14:36] +    |
[01:14:36] +    = help: add #![feature(rustc_private)] to the crate attributes to enable
---
[01:14:36] 16 
[01:14:36] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:36] +   --> $DIR/test-inner-fn.rs:12:5
[01:14:36] +    |
[01:14:36] + LL | /     fn foo() {
[01:14:36] + LL | |         #[test]
[01:14:36] + LL | |         fn bar() {}
[01:14:36] + LL | |         bar();
[01:14:36] + LL | |     }
[01:14:36] +    |
[01:14:36] +    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:14:36] + 
[01:14:36] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
---
[01:14:36] 17 
[01:14:36] 
[01:14:36] 
[01:14:36] The actual stderr differed from the expected stderr.
[01:14:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/test-inner-fn.stderr
[01:14:36] To update references, rerun the tests and pass the `--bless` flag
[01:14:36] To only update this specific test, also pass `--test-args lint/test-inner-fn.rs`
[01:14:36] error: 1 errors occurred comparing output.
[01:14:36] status: exit code: 1
[01:14:36] status: exit code: 1
[01:14:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/test-inner-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-D" "unnameable_test_items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/auxiliary" "-A" "unused"
[01:14:36] ------------------------------------------
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] stderr:
[01:14:36] stderr:
[01:14:36] ------------------------------------------
[01:14:36] {"message":"use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n