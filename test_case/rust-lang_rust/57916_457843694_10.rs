\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn_unsafe.rs","byte_start":2216,"byte_end":2231,"line_start":63,"line_end":63,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Foo { x: () }.y","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(const_fn_union)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: unions in const fn are unstable (see issue #51909)\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_unsafe.rs:63:5\n   |\nLL |     Foo { x: () }.y\n   |     ^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(const_fn_union)] to the crate attributes to enable\n\n"}
[01:00:53] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[01:00:53] {"message":"Some errors occurred: E0133, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0133, E0658.\n"}
[01:00:53] 
[01:00:53] ------------------------------------------
[01:00:53] 
[01:00:53] thread '[ui] ui/consts/min_const_fn/min_const_fn_unsafe.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3287:9
---
[01:00:53] 16 
[01:00:53] - error[E0208]: []
[01:00:53] -   --> $DIR/feature-gate-rustc-attrs-1.rs:8:1
[01:00:53] -    |
[01:00:53] - LL | fn main() {} //~ ERROR []
[01:00:53] + error: aborting due to 2 previous errors
[01:00:53] 22 
[01:00:53] - error: aborting due to 3 previous errors
[01:00:53] - 
[01:00:53] - 
[01:00:53] - Some errors occurred: E0208, E0658.
[01:00:53] - For more information about an error, try `rustc --explain E0208`.
[01:00:53] + For more information about this error, try `rustc --explain E0658`.
[01:00:53] 27 
[01:00:53] 
[01:00:53] 
[01:00:53] The actual stderr differed from the expected stderr.
[01:00:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustc-attrs-1/feature-gate-rustc-attrs-1.stderr
[01:00:53] To update references, rerun the tests and pass the `--bless` flag
[01:00:53] To only update this specific test, also pass `--test-args feature-gates/feature-gate-rustc-attrs-1.rs`
[01:00:53] error: 1 errors occurred comparing output.
[01:00:53] status: exit code: 1
[01:00:53] status: exit code: 1
[01:00:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-rustc-attrs-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustc-attrs-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustc-attrs-1/auxiliary" "-A" "unused"
[01:00:53] ------------------------------------------
[01:00:53] 
[01:00:53] ------------------------------------------
[01:00:53] stderr:
[01:00:53] stderr:
[01:00:53] ------------------------------------------
[01:00:53] {"message":"the `#[rustc_variance]` attribute is just used for rustc unit tests and will never be stable (see issue #29642)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n