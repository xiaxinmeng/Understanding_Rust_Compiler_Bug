\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gates/feature-gate-async-await.rs","byte_start":149,"byte_end":160,"line_start":7,"line_end":7,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    let _ = async || {}; //~ ERROR async closures are unstable","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/50547","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add #![feature(async_await)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: async closures are unstable\n  --> /checkout/src/test/ui/feature-gates/feature-gate-async-await.rs:7:13\n   |\nLL |     let _ = async || {}; //~ ERROR async closures are unstable\n   |             ^^^^^^^^^^^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/50547\n   = help: add #![feature(async_await)] to the crate attributes to enable\n\n"}
[01:07:03] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:07:03] 
[01:07:03] ------------------------------------------
[01:07:03] 
[01:07:03] 
[01:07:03] thread '[ui] ui/feature-gates/feature-gate-async-await.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3469:9
[01:07:03] 
[01:07:03] ---- [ui] ui/feature-gates/feature-gate-async-await-2015-edition.rs stdout ----
[01:07:03] diff of stderr:
[01:07:03] 
[01:07:03] 16 LL |     let _ = async || { true };
[01:07:03] 18 
[01:07:03] 18 
[01:07:03] - error[E0658]: async fn is unstable (see issue #50547)
[01:07:03] + error[E0658]: async fn is unstable
[01:07:03] 21    |
[01:07:03] 21    |
[01:07:03] 22 LL | async fn foo() {}
[01:07:03] 
[01:07:03] The actual stderr differed from the expected stderr.
[01:07:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await-2015-edition/feature-gate-async-await-2015-edition.stderr
[01:07:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await-2015-edition/feature-gate-async-await-2015-edition.stderr
[01:07:03] To update references, rerun the tests and pass the `--bless` flag
[01:07:03] To only update this specific test, also pass `--test-args feature-gates/feature-gate-async-await-2015-edition.rs`
[01:07:03] error: 1 errors occurred comparing output.
[01:07:03] status: exit code: 1
[01:07:03] status: exit code: 1
[01:07:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-async-await-2015-edition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await-2015-edition/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-async-await-2015-edition/auxiliary" "-A" "unused"
[01:07:03] ------------------------------------------
[01:07:03] 
[01:07:03] ------------------------------------------
[01:07:03] stderr:
[01:07:03] stderr:
[01:07:03] ------------------------------------------
[01:07:03] {"message":"`async fn` is not permitted in the 2015 edition","code":{"code":"E0670","explanation":"\nRust 2015 does not permit the use of `async fn`.\n\nExample of erroneous code:\n\n