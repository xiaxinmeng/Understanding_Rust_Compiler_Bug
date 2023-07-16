\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-12997-2.rs","byte_start":104,"byte_end":124,"line_start":6,"line_end":6,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"fn bar(x: isize) { }","highlight_start":1,"highlight_end":21}],"label":"expected isize, found mutable reference","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `isize`\n   found type `&mut test::Bencher`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/issues/issue-12997-2.rs:6:1\n   |\nLL | fn bar(x: isize) { }\n   | ^^^^^^^^^^^^^^^^^^^^ expected isize, found mutable reference\n   |\n   = note: expected type `isize`\n              found type `&mut test::Bencher`\n\n"}
[01:14:36] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:14:36] {"message":"Some errors occurred: E0308, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0308, E0658.\n"}
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] 
[01:14:36] thread '[ui] ui/issues/issue-12997-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
---
[01:14:36] 
[01:14:36] 
[01:14:36] The actual stderr differed from the expected stderr.
[01:14:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772/issue-14772.stderr
[01:14:36] To update references, rerun the tests and pass the `--bless` flag
[01:14:36] To only update this specific test, also pass `--test-args issues/issue-14772.rs`
[01:14:36] error: 1 errors occurred comparing output.
[01:14:36] status: exit code: 1
[01:14:36] status: exit code: 1
[01:14:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-14772.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772/auxiliary" "-A" "unused"
[01:14:36] ------------------------------------------
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] stderr:
[01:14:36] stderr:
[01:14:36] ------------------------------------------
[01:14:36] {"message":"only functions may be used as tests","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-14772.rs","byte_start":34,"byte_end":44,"line_start":4,"line_end":4,"column_start":1,"column_end":11,"is_primary":true,"text":[{"text":"mod foo {} //~ ERROR only functions may be used as tests","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: only functions may be used as tests\n  --> /checkout/src/test/ui/issues/issue-14772.rs:4:1\n   |\nLL | mod foo {} //~ ERROR only functions may be used as tests\n   | ^^^^^^^^^^\n\n"}
[01:14:36] {"message":"use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n