\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/inaccessible-test-modules.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// compile-flags:--test","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)\n   |\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:14:03] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:14:03] {"message":"Some errors occurred: E0432, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0432, E0658.\n"}
[01:14:03] 
[01:14:03] ------------------------------------------
[01:14:03] 
[01:14:03] thread '[ui] ui/inaccessible-test-modules.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:14:03] thread '[ui] ui/inaccessible-test-modules.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:14:03] 
[01:14:03] ---- [ui] ui/issues/issue-12997-1.rs stdout ----
[01:14:03] diff of stderr:
[01:14:03] 
[01:14:03] 10 LL | fn bar(x: isize, y: isize) { }
[01:14:03] 12 
[01:14:03] - error: aborting due to 2 previous errors
[01:14:03] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:03] +    |
---
[01:14:03] 15 
[01:14:03] 
[01:14:03] 
[01:14:03] The actual stderr differed from the expected stderr.
[01:14:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1/issue-12997-1.stderr
[01:14:03] To update references, rerun the tests and pass the `--bless` flag
[01:14:03] To only update this specific test, also pass `--test-args issues/issue-12997-1.rs`
[01:14:03] error: 1 errors occurred comparing output.
[01:14:03] status: exit code: 1
[01:14:03] status: exit code: 1
[01:14:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12997-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1/auxiliary" "-A" "unused"
[01:14:03] ------------------------------------------
[01:14:03] 
[01:14:03] ------------------------------------------
[01:14:03] stderr:
[01:14:03] stderr:
[01:14:03] ------------------------------------------
[01:14:03] {"message":"functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-12997-1.rs","byte_start":106,"byte_end":118,"line_start":6,"line_end":6,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"fn foo() { } //~ ERROR functions used as benches","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`\n  --> /checkout/src/test/ui/issues/issue-12997-1.rs:6:1\n   |\nLL | fn foo() { } //~ ERROR functions used as benches\n   | ^^^^^^^^^^^^\n\n"}
[01:14:03] {"message":"functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-12997-1.rs","byte_start":165,"byte_end":195,"line_start":9,"line_end":9,"column_start":1,"column_end":31,"is_primary":true,"text":[{"text":"fn bar(x: isize, y: isize) { } //~ ERROR functions used as benches","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`\n  --> /checkout/src/test/ui/issues/issue-12997-1.rs:9:1\n   |\nLL | fn bar(x: isize, y: isize) { } //~ ERROR functions used as benches\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:14:03] {"message":"use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n