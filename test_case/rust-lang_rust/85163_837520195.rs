plain
Checking which error codes lack tests...
tidy: Skipping binary file check, read-only filesystem
* 625 error codes
* highest error code: E0783
tidy error: /checkout/compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:518: trailing whitespace
tidy error: /checkout/compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:521: trailing whitespace
Found 0 error codes with no tests
Done!
* 335 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:00:36
