\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/union_promotion.rs","byte_start":579,"byte_end":681,"line_start":19,"line_end":21,"column_start":29,"column_end":6,"is_primary":true,"text":[{"text":"    let x: &'static bool = &unsafe { //~ borrowed value does not live long enough","highlight_start":29,"highlight_end":82},{"text":"        Foo { a: &1 }.b == Foo { a: &2 }.b","highlight_start":1,"highlight_end":43},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/union_promotion.rs","byte_start":683,"byte_end":684,"line_start":22,"line_end":22,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/const-eval/union_promotion.rs:19:29\n   |\nLL |       let x: &'static bool = &unsafe { //~ borrowed value does not live long enough\n   |  _____________________________^\nLL | |         Foo { a: &1 }.b == Foo { a: &2 }.b\nLL | |     };\n   | |_____^ creates a temporary which is freed while still in use\nLL |   }\n   |   - temporary value is freed at the end of this statement\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:59:39] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:59:39] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[00:59:39] ------------------------------------------
[00:59:39] 
[00:59:39] thread '[ui (nll)] ui/consts/const-eval/union_promotion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:59:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:39] 
[00:59:39] ---- [ui (nll)] ui/consts/const-int-conversion.rs stdout ----
[00:59:39] diff of stderr:
[00:59:39] 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 3    |
[00:59:39] 3    |
[00:59:39] 4 LL |     let x: &'static i32 = &(5_i32.reverse_bits());
[00:59:39] 9    |
[00:59:39] 9    |
[00:59:39] 10    = note: borrowed value must be valid for the static lifetime...
[00:59:39] 11 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 14    |
[00:59:39] 14    |
[00:59:39] 15 LL |     let y: &'static i32 = &(i32::from_be_bytes([0x12, 0x34, 0x56, 0x78]));
[00:59:39] 20    |
[00:59:39] 20    |
[00:59:39] 21    = note: borrowed value must be valid for the static lifetime...
[00:59:39] 22 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 25    |
[00:59:39] 25    |
[00:59:39] 26 LL |     let z: &'static i32 = &(i32::from_le_bytes([0x12, 0x34, 0x56, 0x78]));
[00:59:39] 31    |
[00:59:39] 31    |
[00:59:39] 32    = note: borrowed value must be valid for the static lifetime...
[00:59:39] 33 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 36    |
[00:59:39] 36    |
[00:59:39] 37 LL |     let a: &'static i32 = &(i32::from_be(i32::from_ne_bytes([0x80, 0, 0, 0])));
[00:59:39] 42    |
[00:59:39] 42    |
[00:59:39] 43    = note: borrowed value must be valid for the static lifetime...
[00:59:39] 44 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 47    |
[00:59:39] 47    |
[00:59:39] 48 LL |     let b: &'static [u8] = &(0x12_34_56_78_i32.to_be_bytes());
[00:59:39] 53    |
[00:59:39] 53    |
[00:59:39] 54    = note: borrowed value must be valid for the static lifetime...
[00:59:39] 55 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 58    |
[00:59:39] 58    |
[00:59:39] 59 LL |     let c: &'static [u8] = &(0x12_34_56_78_i32.to_le_bytes());
[00:59:39] 64    |
[00:59:39] 64    |
[00:59:39] 65    = note: borrowed value must be valid for the static lifetime...
[00:59:39] 66 
[00:59:39] - error[E0714]: temporary value dropped while borrowed
[00:59:39] + error[E0716]: temporary value dropped while borrowed
[00:59:39] 69    |
[00:59:39] 69    |
[00:59:39] 70 LL |     let d: &'static [u8] = &(i32::min_value().to_be().to_ne_bytes());
[00:59:39] 77 
[00:59:39] 78 error: aborting due to 7 previous errors
[00:59:39] 79 
[00:59:39] - For more information about this error, try `rustc --explain E0714`.
[00:59:39] - For more information about this error, try `rustc --explain E0714`.
[00:59:39] + For more information about this error, try `rustc --explain E0716`.
[00:59:39] 81 
[00:59:39] 
[00:59:39] 
[00:59:39] The actual stderr differed from the expected stderr.
[00:59:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion.nll/const-int-conversion.nll.stderr
[00:59:39] To update references, rerun the tests and pass the `--bless` flag
[00:59:39] To only update this specific test, also pass `--test-args consts/const-int-conversion.rs`
[00:59:39] error: 1 errors occurred comparing output.
[00:59:39] status: exit code: 1
[00:59:39] status: exit code: 1
[00:59:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-conversion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion.nll/auxiliary" "-A" "unused"
[00:59:39] ------------------------------------------
[00:59:39] 
[00:59:39] ------------------------------------------
[00:59:39] stderr:
[00:59:39] stderr:
[00:59:39] ------------------------------------------
[00:59:39] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n