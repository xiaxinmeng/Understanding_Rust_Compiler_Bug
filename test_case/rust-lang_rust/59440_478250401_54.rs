\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs-test.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// aux-build:builtin-attrs.rs","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)\n   |\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:14:36] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:14:36] {"message":"Some errors occurred: E0425, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0425, E0658.\n"}
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] 
[01:14:36] thread '[ui] ui/proc-macro/ambiguous-builtin-attrs-test.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
---
[01:14:36] 13 
[01:14:36] 
[01:14:36] 
[01:14:36] The actual stderr differed from the expected stderr.
[01:14:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic/termination-trait-in-test-should-panic.stderr
[01:14:36] To update references, rerun the tests and pass the `--bless` flag
[01:14:36] To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs`
[01:14:36] error: 1 errors occurred comparing output.
[01:14:36] status: exit code: 1
[01:14:36] status: exit code: 1
[01:14:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic/auxiliary" "-A" "unused"
[01:14:36] ------------------------------------------
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] stderr:
[01:14:36] stderr:
[01:14:36] ------------------------------------------
[01:14:36] {"message":"functions using `#[should_panic]` must return `()`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs","byte_start":137,"byte_end":294,"line_start":11,"line_end":15,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn not_a_num() -> Result<(), ParseIntError> {","highlight_start":1,"highlight_end":46},{"text":"    //~^ ERROR functions using `#[should_panic]` must return `()`","highlight_start":1,"highlight_end":66},{"text":"    let _: u32 = \"abc\".parse()?;","highlight_start":1,"highlight_end":33},{"text":"    Ok(())","highlight_start":1,"highlight_end":11},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: functions using `#[should_panic]` must return `()`\n  --> /checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs:11:1\n   |\nLL | / fn not_a_num() -> Result<(), ParseIntError> {\nLL | |     //~^ ERROR functions using `#[should_panic]` must return `()`\nLL | |     let _: u32 = \"abc\".parse()?;\nLL | |     Ok(())\nLL | | }\n   | |_^\n\n"}
[01:14:36] {"message":"use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n