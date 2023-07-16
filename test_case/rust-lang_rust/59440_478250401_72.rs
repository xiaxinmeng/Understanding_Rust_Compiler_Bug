\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// compile-flags: --test","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(rustc_private)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)\n   |\n   = help: add #![feature(rustc_private)] to the crate attributes to enable\n\n"}
[01:14:36] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] 
---
[01:14:36] 
[01:14:36] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:36] +   --> $DIR/termination-trait-test-wrong-type.rs:6:1
[01:14:36] +    |
[01:14:36] + LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseIntError> {
[01:14:36] + LL | |     "0".parse()
[01:14:36] + LL | | }
[01:14:36] +    |
[01:14:36] +    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:14:36] + 
[01:14:36] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:36] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:14:36] +    |
[01:14:36] +    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:14:36] + 
[01:14:36] 1 error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseIntError>`
[01:14:36] 3    |
[01:14:36] 
[01:14:36] 
[01:14:36] 9    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseIntError>`
[01:14:36] 10    = note: required by `test::assert_test_result`
[01:14:36] - error: aborting due to previous error
[01:14:36] + error: aborting due to 3 previous errors
[01:14:36] 13 
[01:14:36] - For more information about this error, try `rustc --explain E0277`.
[01:14:36] - For more information about this error, try `rustc --explain E0277`.
[01:14:36] + Some errors occurred: E0277, E0658.
[01:14:36] + For more information about an error, try `rustc --explain E0277`.
[01:14:36] 15 
[01:14:36] 
[01:14:36] 
[01:14:36] The actual stderr differed from the expected stderr.
[01:14:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
[01:14:36] To update references, rerun the tests and pass the `--bless` flag
[01:14:36] To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`
[01:14:36] error: 1 errors occurred comparing output.
[01:14:36] status: exit code: 1
[01:14:36] status: exit code: 1
[01:14:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary" "-A" "unused"
[01:14:36] ------------------------------------------
[01:14:36] 
[01:14:36] ------------------------------------------
[01:14:36] stderr:
[01:14:36] stderr:
[01:14:36] ------------------------------------------
[01:14:36] {"message":"use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n