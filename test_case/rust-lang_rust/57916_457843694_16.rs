\n\nSee also https://doc.rust-lang.org/book/first-edition/unsafe.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/unsafe/ranged_ints2_const.rs","byte_start":234,"byte_end":242,"line_start":11,"line_end":11,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"    let y = &mut x.0; //~ ERROR references in const fn are unstable","highlight_start":13,"highlight_end":21}],"label":"mutation of layout constrained field","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"mutating layout constrained fields cannot statically be checked for valid values","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0133]: mutation of layout constrained field is unsafe and requires unsafe function or block\n  --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:11:13\n   |\nLL |     let y = &mut x.0; //~ ERROR references in const fn are unstable\n   |             ^^^^^^^^ mutation of layout constrained field\n   |\n   = note: mutating layout constrained fields cannot statically be checked for valid values\n\n"}
[01:00:53] {"message":"mutable references in const fn are unstable","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/unsafe/ranged_ints2_const.rs","byte_start":230,"byte_end":231,"line_start":11,"line_end":11,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let y = &mut x.0; //~ ERROR references in const fn are unstable","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: mutable references in const fn are unstable\n  --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:11:9\n   |\nLL |     let y = &mut x.0; //~ ERROR references in const fn are unstable\n   |         ^\n\n"}
[01:00:53] {"message":"mutable references in const fn are unstable","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/unsafe/ranged_ints2_const.rs","byte_start":461,"byte_end":462,"line_start":18,"line_end":18,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let y = unsafe { &mut x.0 }; //~ ERROR mutable references in const fn are unstable","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: mutable references in const fn are unstable\n  --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:18:9\n   |\nLL |     let y = unsafe { &mut x.0 }; //~ ERROR mutable references in const fn are unstable\n   |         ^\n\n"}
[01:00:53] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:00:53] 
[01:00:53] ------------------------------------------
[01:00:53] 
[01:00:53] thread '[ui] ui/unsafe/ranged_ints2_const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3287:9
[01:00:53] thread '[ui] ui/unsafe/ranged_ints2_const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3287:9
[01:00:53] 
[01:00:53] ---- [ui] ui/unsafe/ranged_ints3_const.rs stdout ----
[01:00:53] diff of stderr:
[01:00:53] 
[01:00:53] + error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe function or block
[01:00:53] +    |
[01:00:53] +    |
[01:00:53] + LL |     let y = &x.0; //~ ERROR cannot borrow a constant which may contain interior mutability
[01:00:53] +    |             ^^^^ borrow of layout constrained field with interior mutability
[01:00:53] +    |
[01:00:53] +    = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values
[01:00:53] + 
[01:00:53] 1 error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
[01:00:53] 3    |
[01:00:53] 
[01:00:53] 9    |
[01:00:53] 9    |
[01:00:53] 10 LL |     let y = unsafe { &x.0 }; //~ ERROR cannot borrow a constant which may contain interior mut
[01:00:53] - 
[01:00:53] - 
[01:00:53] - error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe function or block
[01:00:53] -    |
[01:00:53] -    |
[01:00:53] - LL |     let y = &x.0; //~ ERROR cannot borrow a constant which may contain interior mutability
[01:00:53] -    |             ^^^^ borrow of layout constrained field with interior mutability
[01:00:53] -    |
[01:00:53] -    = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values
[01:00:53] 21 error: aborting due to 3 previous errors
[01:00:53] 22 
[01:00:53] 
[01:00:53] 
[01:00:53] 
[01:00:53] The actual stderr differed from the expected stderr.
[01:00:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const/ranged_ints3_const.stderr
[01:00:53] To update references, rerun the tests and pass the `--bless` flag
[01:00:53] To only update this specific test, also pass `--test-args unsafe/ranged_ints3_const.rs`
[01:00:53] error: 1 errors occurred comparing output.
[01:00:53] status: exit code: 1
[01:00:53] status: exit code: 1
[01:00:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints3_const.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const/auxiliary" "-A" "unused"
[01:00:53] ------------------------------------------
[01:00:53] 
[01:00:53] ------------------------------------------
[01:00:53] stderr:
[01:00:53] stderr:
[01:00:53] ------------------------------------------
[01:00:53] {"message":"borrow of layout constrained field with interior mutability is unsafe and requires unsafe function or block","code":{"code":"E0133","explanation":"\nUnsafe code was used outside of an unsafe function or block.\n\nErroneous code example:\n\n