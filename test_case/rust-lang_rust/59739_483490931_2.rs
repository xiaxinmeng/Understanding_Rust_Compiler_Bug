\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr_feature_gate.rs","byte_start":147,"byte_end":171,"line_start":7,"line_end":7,"column_start":3,"column_end":27,"is_primary":true,"text":[{"text":"#[rustc_allow_const_fn_ptr]","highlight_start":3,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/29642","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add #![feature(rustc_attrs)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: unless otherwise specified, attributes with the prefix `rustc_` are reserved for internal compiler diagnostics\n  --> /checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr_feature_gate.rs:7:3\n   |\nLL | #[rustc_allow_const_fn_ptr]\n   |   ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/29642\n   = help: add #![feature(rustc_attrs)] to the crate attributes to enable\n\n"}
[01:07:03] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:07:03] 
[01:07:03] ------------------------------------------
[01:07:03] 
[01:07:03] 
[01:07:03] thread '[ui] ui/consts/min_const_fn/allow_const_fn_ptr_feature_gate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3469:9
[01:07:03] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:03] 
[01:07:03] ---- [ui] ui/feature-gates/feature-gate-async-await.rs stdout ----
[01:07:03] diff of stderr:
[01:07:03] 
[01:07:03] - error[E0658]: async fn is unstable (see issue #50547)
[01:07:03] + error[E0658]: async fn is unstable
[01:07:03] 3    |
[01:07:03] 3    |
[01:07:03] 4 LL | async fn foo() {}
[01:07:03] 7    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:07:03] 7    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:07:03] 8    = help: add #![feature(async_await)] to the crate attributes to enable
[01:07:03] 9 
[01:07:03] - error[E0658]: async blocks are unstable (see issue #50547)
[01:07:03] + error[E0658]: async blocks are unstable
[01:07:03] 12    |
[01:07:03] 12    |
[01:07:03] 13 LL |     let _ = async {};
[01:07:03] 16    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:07:03] 16    = note: for more information, see https://github.com/rust-lang/rust/issues/50547
[01:07:03] 17    = help: add #![feature(async_await)] to the crate attributes to enable
[01:07:03] 18 
[01:07:03] - error[E0658]: async closures are unstable (see issue #50547)
[01:07:03] + error[E0658]: async closures are unstable
[01:07:03] 21    |
[01:07:03] 21    |
[01:07:03] 22 LL |     let _ = async || {};
[01:07:03] 
[01:07:03] The actual stderr differed from the expected stderr.
[01:07:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/feature-gate-async-await.stderr
[01:07:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/feature-gate-async-await.stderr
[01:07:03] To update references, rerun the tests and pass the `--bless` flag
[01:07:03] To only update this specific test, also pass `--test-args feature-gates/feature-gate-async-await.rs`
[01:07:03] error: 1 errors occurred comparing output.
[01:07:03] status: exit code: 1
[01:07:03] status: exit code: 1
[01:07:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await/auxiliary" "-A" "unused"
[01:07:03] ------------------------------------------
[01:07:03] 
[01:07:03] ------------------------------------------
[01:07:03] stderr:
[01:07:03] stderr:
[01:07:03] ------------------------------------------
[01:07:03] {"message":"async fn is unstable","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n