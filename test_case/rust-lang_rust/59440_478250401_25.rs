\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-14772.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// compile-flags: --test","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)\n   |\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:14:36] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] 
---
[01:14:36] 
[01:14:36] 
[01:14:36] The actual stderr differed from the expected stderr.
[01:14:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134/issue-28134.stderr
[01:14:36] To update references, rerun the tests and pass the `--bless` flag
[01:14:36] To only update this specific test, also pass `--test-args issues/issue-28134.rs`
[01:14:36] error: 1 errors occurred comparing output.
[01:14:36] status: exit code: 1
[01:14:36] status: exit code: 1
[01:14:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134/auxiliary" "-A" "unused"
[01:14:36] ------------------------------------------
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] stderr:
[01:14:36] stderr:
[01:14:36] ------------------------------------------
[01:14:36] {"message":"only functions may be used as tests","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-28134.rs","byte_start":26,"byte_end":34,"line_start":3,"line_end":3,"column_start":1,"column_end":9,"is_primary":true,"text":[{"text":"#![test] //~ ERROR only functions may be used as tests","highlight_start":1,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: only functions may be used as tests\n  --> /checkout/src/test/ui/issues/issue-28134.rs:3:1\n   |\nLL | #![test] //~ ERROR only functions may be used as tests\n   | ^^^^^^^^\n\n"}
[01:14:36] {"message":"use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n