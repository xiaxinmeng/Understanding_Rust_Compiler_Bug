\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/custom_test_frameworks/mismatch.rs","byte_start":170,"byte_end":187,"line_start":9,"line_end":9,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"fn wrong_kind(){}","highlight_start":1,"highlight_end":18}],"label":"the trait `example_runner::Testable` is not implemented for `test::TestDescAndFn`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required for the cast to the object type `dyn example_runner::Testable`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `test::TestDescAndFn: example_runner::Testable` is not satisfied\n  --> /checkout/src/test/ui/custom_test_frameworks/mismatch.rs:9:1\n   |\nLL | fn wrong_kind(){}\n   | ^^^^^^^^^^^^^^^^^ the trait `example_runner::Testable` is not implemented for `test::TestDescAndFn`\n   |\n   = note: required for the cast to the object type `dyn example_runner::Testable`\n\n"}
[01:14:03] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:14:03] {"message":"Some errors occurred: E0277, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0658.\n"}
[01:14:03] 
[01:14:03] ------------------------------------------
[01:14:03] 
[01:14:03] thread '[ui] ui/custom_test_frameworks/mismatch.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:14:03] thread '[ui] ui/custom_test_frameworks/mismatch.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:14:03] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:03] 
[01:14:03] ---- [ui] ui/inaccessible-test-modules.rs stdout ----
[01:14:03] diff of stderr:
[01:14:03] 
[01:14:03] 16    |     no `__test_reexports` in the root
[01:14:03] 17    |     help: a similar name exists in the module: `__test_reexports`
[01:14:03] - error: aborting due to 2 previous errors
[01:14:03] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:03] +   --> $DIR/inaccessible-test-modules.rs:9:1
[01:14:03] +    |
[01:14:03] +    |
[01:14:03] + LL | fn baz() {}
[01:14:03] +    |
[01:14:03] +    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:14:03] 20 
[01:14:03] - For more information about this error, try `rustc --explain E0432`.
---
[01:14:03] 22 
[01:14:03] 
[01:14:03] 
[01:14:03] The actual stderr differed from the expected stderr.
[01:14:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inaccessible-test-modules/inaccessible-test-modules.stderr
[01:14:03] To update references, rerun the tests and pass the `--bless` flag
[01:14:03] To only update this specific test, also pass `--test-args inaccessible-test-modules.rs`
[01:14:03] error: 1 errors occurred comparing output.
[01:14:03] status: exit code: 1
[01:14:03] status: exit code: 1
[01:14:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inaccessible-test-modules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inaccessible-test-modules/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inaccessible-test-modules/auxiliary" "-A" "unused"
[01:14:03] ------------------------------------------
[01:14:03] 
[01:14:03] ------------------------------------------
[01:14:03] stderr:
[01:14:03] stderr:
[01:14:03] ------------------------------------------
[01:14:03] {"message":"unresolved import `__test`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n