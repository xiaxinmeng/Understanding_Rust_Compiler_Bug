\n\n* The `#[repr(C)]` attribute can only be placed on structs and enums.\n* The `#[repr(packed)]` and `#[repr(simd)]` attributes only work on structs.\n* The `#[repr(u8)]`, `#[repr(i16)]`, etc attributes only work on enums.\n\nThese attributes do not work on typedefs, since typedefs are just aliases.\n\nRepresentations like `#[repr(u8)]`, `#[repr(i64)]` are for selecting the\ndiscriminant size for enums with no data fields on any of the variants, e.g.\n`enum Color {Red, Blue, Green}`, effectively setting the size of the enum to\nthe size of the provided type. Such an enum can be cast to a value of the same\ntype as well. In short, `#[repr(u8)]` makes the enum behave like an integer\nwith a constrained set of allowed values.\n\nOnly field-less enums can be cast to numerical primitives, so this attribute\nwill not apply to structs.\n\n`#[repr(packed)]` reduces padding to make the struct size smaller. The\nrepresentation of enums isn't strictly defined in Rust, and this attribute\nwon't work on enums.\n\n`#[repr(simd)]` will give a struct consisting of a homogeneous series of machine\ntypes (i.e. `u8`, `i32`, etc) a representation that permits vectorization via\nSIMD. This doesn't make much sense for enums since they don't consist of a\nsingle list of data.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/attr-usage-repr.rs","byte_start":881,"byte_end":901,"line_start":32,"line_end":32,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"enum EAlign { A, B }","highlight_start":1,"highlight_end":21}],"label":"not a struct or union","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/attr-usage-repr.rs","byte_start":821,"byte_end":829,"line_start":31,"line_end":31,"column_start":8,"column_end":16,"is_primary":true,"text":[{"text":"#[repr(align(8))] //~ ERROR: attribute should be applied to struct","highlight_start":8,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applirs stdout ----
[00:42:54] 
[00:42:54] 1 error[E0565]: unsupported literal
[00:42:54] -   --> $DIR/E0565.rs:14:8
[00:42:54] +   --> $DIR/E0565.rs:12:8
[00:42:54] +   --> $DIR/E0565.rs:12:8
[00:42:54] 3    |
[00:42:54] 4 LL | #[repr("C")] //~ ERROR E0565
[00:42:54] 
[00:42:54] 
[00:42:54] The actual stderr differed from the expected stderr.
[00:42:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0565/E0565.stderr
[00:42:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0565/E0565.stderr
[00:42:54] To update references, rerun the tests and pass the `--bless` flag
[00:42:54] To only update this specific test, also pass `--test-args error-codes/E0565.rs`
[00:42:54] error: 1 errors occurred comparing output.
[00:42:54] status: exit code: 1
[00:42:54] status: exit code: 1
[00:42:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0565.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0565/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0565/auxiliary" "-A" "unused"
[00:42:54] ------------------------------------------
[00:42:54] 
[00:42:54] ------------------------------------------
[00:42:54] stderr:
[00:42:54] stderr:
[00:42:54] ------------------------------------------
[00:42:54] {"message":"unsupported literal","code":{"code":"E0565","explanation":"\nA literal was used in a built-in attribute that doesn't support f of stderr:
[00:42:54] 1 error[E0565]: unsupported literal
[00:42:54] -   --> $DIR/E0565-1.rs:14:14
[00:42:54] +   --> $DIR/E0565-1.rs:12:14
[00:42:54] 3    |
[00:42:54] 3    |
[00:42:54] 4 LL | #[deprecated("since")] //~ ERROR E0565
[00:42:54] 
[00:42:54] 
[00:42:54] The actual stderr differed from the expected stderr.
[00:42:54] The actual stderr differed from the expected stderr.
[00:42:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0565-1/E0565-1.stderr
[00:42:54] To update references, rerun the tests and pass the `--bless` flag
[00:42:54] To only update this specific test, also pass `--test-args error-codes/E0565-1.rs`
[00:42:54] error: 1 errors occurred comparing output.
[00:42:54] status: exit code: 1
[00:42:54] status: exit code: 1
[00:42:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0565-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0565-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0565-1/auxiliary" "-A" "unused"
[00:42:54] ------------------------------------------
[00:42:54] 
[00:42:54] ------------------------------------------
[00:42:54] stderr:
[00:42:54] stderr:
[00:42:54] ------------------------------------------
[00:42:54] {"message":"unsupported literal","code":{"code":"E0565","explanation":"\nA literal was used in a built-in attribute that doesn't ue-43925.rs stdout ----
[00:42:54] 
[00:42:54] 
[00:42:54] 1 error: invalid argument for `cfg(..)`
[00:42:54] +   --> $DIR/issue-43925.rs:11:24
[00:42:54] 3    |
[00:42:54] 3    |
[00:42:54] 4 LL | #[link(name="foo", cfg("rlib"))] //~ ERROR invalid argument for `cfg(..)`
[00:42:54] 
[00:42:54] 
[00:42:54] The actual stderr differed from the expected stderr.
[00:42:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-43925/issue-43925.stderr
[00:42:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-43925/issue-43925.stderr
[00:42:54] To update references, rerun the tests and pass the `--bless` flag
[00:42:54] To only update this specific test, also pass `--test-args issue-43925.rs`
[00:42:54] error: 1 errors occurred comparing output.
[00:42:54] status: exit code: 1
[00:42:54] status: exit code: 1
[00:42:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-43925.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-43925/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-43925/auxiliary" "-A" "unused"
[00:42:54] ------------------------------------------
[00:42:54] 
[00:42:54] ------------------------------------------
[00:42:54] stderr:
[00:42:54] stderr:
[00:42:54] ------------------------------------------
[00:42:54] {"message":"invalid argument for `cfg(..)`","code":null,"level":"error "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expected-comma-found-token/auxiliary" "-A" "unused"
[00:42:54] ------------------------------------------
[00:42:54] 
[00:42:54] ------------------------------------------
[00:42:54] stderr:
[00:42:54] stderr:
[00:42:54] ------------------------------------------
[00:42:54] {"message":"`main` function not found in crate `expected_comma_found_token`","code":{"code":"E0601","explanation":"\nNo `main` function was found in a binary crate. To fix this error, add a\n`main` function. For example:\n\n