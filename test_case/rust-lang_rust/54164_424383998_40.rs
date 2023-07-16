\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-int-conversion.rs","byte_start":1230,"byte_end":1270,"line_start":26,"line_end":26,"column_start":29,"column_end":69,"is_primary":true,"text":[{"text":"    let d: &'static [u8] = &(i32::min_value().to_be().to_ne_bytes());","highlight_start":29,"highlight_end":69}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-int-conversion.rs","byte_start":1317,"byte_end":1318,"line_start":28,"line_end":28,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/const-int-conversion.rs:26:29\n   |\nLL |     let d: &'static [u8] = &(i32::min_value().to_be().to_ne_bytes());\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use\nLL |         //~^ ERROR does not live long enough\nLL | }\n   | - temporary value is freed at the end of this statement\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:59:39] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[00:59:39] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[00:59:39] ------------------------------------------
[00:59:39] 
[00:59:39] thread '[ui (nll)] ui/consts/const-int-conversion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:59:39] 
[00:59:39] 
[00:59:39] ---- [ui (nll)] ui/consts/const-int-overflowing.rs stdout ----
[00:59:39] diff of stderr:
[00:59:39] 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 3    |
[00:59:39] 3    |
[00:59:39] 4 LL |     let x: &'static (i32, bool) = &(5_i32.overflowing_add(3)); //~ ERROR does not live long enough
[00:59:39] 9    |
[00:59:39] 9    |
[00:59:39] 10    = note: borrowed value must be valid for the static lifetime...
[00:59:39] 11 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 14    |
[00:59:39] 14    |
[00:59:39] 15 LL |     let y: &'static (i32, bool) = &(5_i32.overflowing_sub(3)); //~ ERROR does not live long enough
[00:59:39] 20    |
[00:59:39] 20    |
[00:59:39] 21    = note: borrowed value must be valid for the static lifetime...
[00:59:39] 22 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 25    |
[00:59:39] 25    |
[00:59:39] 26 LL |     let z: &'static (i32, bool) = &(5_i32.overflowing_mul(3)); //~ ERROR does not live long enough
[00:59:39] 32 
[00:59:39] 33 error: aborting due to 3 previous errors
[00:59:39] 34 
[00:59:39] - For more information about this error, try `rustc --explain E0714`.
[00:59:39] - For more information about this error, try `rustc --explain E0714`.
[00:59:39] + For more information about this error, try `rustc --explain E0716`.
[00:59:39] 36 
[00:59:39] 
[00:59:39] 
[00:59:39] The actual stderr differed from the expected stderr.
[00:59:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing.nll/const-int-overflowing.nll.stderr
[00:59:39] To update references, rerun the tests and pass the `--bless` flag
[00:59:39] To only update this specific test, also pass `--test-args consts/const-int-overflowing.rs`
[00:59:39] error: 1 errors occurred comparing output.
[00:59:39] status: exit code: 1
[00:59:39] status: exit code: 1
[00:59:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-overflowing.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing.nll/auxiliary" "-A" "unused"
[00:59:39] ------------------------------------------
[00:59:39] 
[00:59:39] ------------------------------------------
[00:59:39] stderr:
[00:59:39] stderr:
[00:59:39] ------------------------------------------
[00:59:39] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n