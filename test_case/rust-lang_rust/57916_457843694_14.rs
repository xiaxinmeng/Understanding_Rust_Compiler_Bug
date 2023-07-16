\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gates/feature-gate-rustc-attrs-1.rs","byte_start":227,"byte_end":241,"line_start":6,"line_end":6,"column_start":1,"column_end":15,"is_primary":true,"text":[{"text":"#[rustc_error] //~ ERROR the `#[rustc_error]` attribute is just used for rustc unit tests and will never be stable","highlight_start":1,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(rustc_attrs)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: the `#[rustc_error]` attribute is just used for rustc unit tests and will never be stable (see issue #29642)\n  --> /checkout/src/test/ui/feature-gates/feature-gate-rustc-attrs-1.rs:6:1\n   |\nLL | #[rustc_error] //~ ERROR the `#[rustc_error]` attribute is just used for rustc unit tests and will never be stable\n   | ^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(rustc_attrs)] to the crate attributes to enable\n\n"}
[01:00:53] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:00:53] 
[01:00:53] ------------------------------------------
[01:00:53] 
[01:00:53] thread '[ui] ui/feature-gates/feature-gate-rustc-attrs-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3287:9
[01:00:53] thread '[ui] ui/feature-gates/feature-gate-rustc-attrs-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3287:9
[01:00:53] 
[01:00:53] ---- [ui] ui/unsafe/ranged_ints2_const.rs stdout ----
[01:00:53] diff of stderr:
[01:00:53] 
[01:00:53] + error[E0133]: mutation of layout constrained field is unsafe and requires unsafe function or block
[01:00:53] +    |
[01:00:53] +    |
[01:00:53] + LL |     let y = &mut x.0; //~ ERROR references in const fn are unstable
[01:00:53] +    |             ^^^^^^^^ mutation of layout constrained field
[01:00:53] +    |
[01:00:53] +    = note: mutating layout constrained fields cannot statically be checked for valid values
[01:00:53] 1 error: mutable references in const fn are unstable
[01:00:53] 2   --> $DIR/ranged_ints2_const.rs:11:9
[01:00:53] 3    |
[01:00:53] 
[01:00:53] 
[01:00:53] 9    |
[01:00:53] 10 LL |     let y = unsafe { &mut x.0 }; //~ ERROR mutable references in const fn are unstable
[01:00:53] - 
[01:00:53] - 
[01:00:53] - error[E0133]: mutation of layout constrained field is unsafe and requires unsafe function or block
[01:00:53] -    |
[01:00:53] -    |
[01:00:53] - LL |     let y = &mut x.0; //~ ERROR references in const fn are unstable
[01:00:53] -    |             ^^^^^^^^ mutation of layout constrained field
[01:00:53] -    |
[01:00:53] -    = note: mutating layout constrained fields cannot statically be checked for valid values
[01:00:53] 21 error: aborting due to 3 previous errors
[01:00:53] 22 
[01:00:53] 
[01:00:53] 
[01:00:53] 
[01:00:53] The actual stderr differed from the expected stderr.
[01:00:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/ranged_ints2_const.stderr
[01:00:53] To update references, rerun the tests and pass the `--bless` flag
[01:00:53] To only update this specific test, also pass `--test-args unsafe/ranged_ints2_const.rs`
[01:00:53] error: 1 errors occurred comparing output.
[01:00:53] status: exit code: 1
[01:00:53] status: exit code: 1
[01:00:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints2_const.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/auxiliary" "-A" "unused"
[01:00:53] ------------------------------------------
[01:00:53] 
[01:00:53] ------------------------------------------
[01:00:53] stderr:
[01:00:53] stderr:
[01:00:53] ------------------------------------------
[01:00:53] {"message":"mutation of layout constrained field is unsafe and requires unsafe function or block","code":{"code":"E0133","explanation":"\nUnsafe code was used outside of an unsafe function or block.\n\nErroneous code example:\n\n