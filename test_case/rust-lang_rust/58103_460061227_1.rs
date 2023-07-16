\n\nSee [RFC 911] for more details on the design of `const fn`s.\n\n[RFC 911]: https://github.com/rust-lang/rfcs/blob/master/text/0911-const-fn.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-call.rs","byte_start":63,"byte_end":67,"line_start":6,"line_end":6,"column_start":17,"column_end":21,"is_primary":true,"text":[{"text":"    let _ = [0; f(2)];","highlight_start":17,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants\n  --> /checkout/src/test/ui/consts/const-call.rs:6:17\n   |\nLL |     let _ = [0; f(2)];\n   |                 ^^^^\n\n"}
[01:00:58] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-call.rs","byte_start":63,"byte_end":67,"line_start":6,"line_end":6,"column_start":17,"column_end":21,"is_primary":true,"text":[{"text":"    let _ = [0; f(2)];","highlight_start":17,"highlight_end":21}],"label":"calling non-const function `f`","suggested_replacement":null,"suggestion_applicability":null,"expansion":nulnote: #[deny(const_err)] on by default
[01:00:58] 7 error: aborting due to previous error
[01:00:58] 8 
[01:00:58] 
[01:00:58] - For more information about this error, try `rustc --explain E0080`.
[01:00:58] - For more information about this error, try `rustc --explain E0080`.
[01:00:58] 10 
[01:00:58] 
[01:00:58] 
[01:00:58] The actual stderr differed from the expected stderr.
[01:00:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3/const-eval-overflow-3.stderr
[01:00:58] To update references, rerun the tests and pass the `--bless` flag
[01:00:58] To only update this specific test, also pass `--test-args consts/const-eval/const-eval-overflow-3.rs`
[01:00:58] error: 1 errors occurred comparing output.
[01:00:58] status: exit code: 1
[01:00:58] status: exit code: 1
[01:00:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3/auxiliary" "-A" "unused"
[01:00:58] ------------------------------------------
[01:00:58] 
[01:00:58] ------------------------------------------
[01:00:58] stderr:
[01:00:58] stderr:
[01:00:58] ------------------------------------------
[01:00:58] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const-eval-overflow-3.rs","byte_start":545,"byte_end":558,"line_start":20,"line_end":20,"column_start":11,"column_end":24,"is_primary":false,"text":[{"text":"    = [0; (i8::MAX + 1) as usize];","highlight_start":11,"highlight_end":24}],"label":"attempt to add with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/const-eval-overflow-3.rs","byte_start":545,"byte_end":567,"line_start":20,"line_end":20,"column_start":11,"column_end":33,"is_primary":true,"text":[{"text":"    = [0; (i8::MAX + 1) as usize];","highlight_start":11,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-3.rs:20:11\n   |\nLL |     = [0; (i8::MAX + 1) as usize];\n   |           -------------^^^^^^^^^\n   |           |\n   |           attempt to add with overflow\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[01:00:58] 
[01:00:58] ------------------------------------------
[01:00:58] 
[01:00:58] thread '[ui] ui/consts/const-eval/const-eval-overflow-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:00:58] thread '[ui] ui/consts/const-eval/const-eval-overflow-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:00:58] 
[01:00:58] ---- [ui] ui/consts/const-eval/const-eval-overflow-4.rs stdout ----
[01:00:58] diff of stderr:
[01:00:58] 
[01:00:58] - error[E0080]: evaluation of constant value failed
[01:00:58] + error: any use of this value will cause an error
[01:00:58] 3    |
[01:00:58] 3    |
[01:00:58] 4 LL |     : [u32; (i8::MAX as i8 + 1i8) as usize]
[01:00:58] -    |             ^^^^^^^^^^^^^^^^^^^^^ attempt to add with overflow
[01:00:58] +    |             ---------------------^^^^^^^^^
[01:00:58] +    |             |
[01:00:58] +    |             attempt to add with overflow
---
[01:00:58] 10 
[01:00:58] 
[01:00:58] 
[01:00:58] The actual stderr differed from the expected stderr.
[01:00:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4/const-eval-overflow-4.stderr
[01:00:58] To update references, rerun the tests and pass the `--bless` flag
[01:00:58] To only update this specific test, also pass `--test-args consts/const-eval/const-eval-overflow-4.rs`
[01:00:58] error: 1 errors occurred comparing output.
[01:00:58] status: exit code: 1
[01:00:58] status: exit code: 1
[01:00:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4/auxiliary" "-A" "unused"
[01:00:58] ------------------------------------------
[01:00:58] 
[01:00:58] ------------------------------------------
[01:00:58] stderr:
[01:00:58] stderr:
[01:00:58] ------------------------------------------
[01:00:58] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const-eval-overflow-4.rs","byte_start":300,"byte_end":321,"line_start":13,"line_end":13,"column_start":13,"column_end":34,"is_primary":false,"text":[{"text":"    : [u32; (i8::MAX as i8 + 1i8) as usize]","highlight_start":13,"highlight_end":34}],"label":"attempt to add with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/const-eval-overflow-4.rs","byte_start":300,"byte_end":330,"line_start":13,"line_end":13,"column_start":13,"column_end":43,"is_primary":true,"text":[{"text":"    : [u32; (i8::MAX as i8 + 1i8) as usize]","highlight_start":13,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-4.rs:13:13\n   |\nLL |     : [u32; (i8::MAX as i8 + 1i8) as usize]\n   |             ---------------------^^^^^^^^^\n   |             |\n   |             attempt to add with overflow\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[01:00:58] 
[01:00:58] ------------------------------------------
[01:00:58] 
[01:00:58] thread '[ui] ui/consts/const-eval/const-eval-overflow-4.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:00:58] thread '[ui] ui/consts/const-eval/const-eval-overflow-4.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:00:58] 
[01:00:58] ---- [ui] ui/consts/const-eval/match-test-ptr-null.rs stdout ----
[01:00:58] diff of stderr:
[01:00:58] 
[01:00:58] 12 LL |             0 => 42, //~ ERROR constant contains unimplemented expression type
[01:00:58] 14 
[01:00:58] - error[E0080]: evaluation of constant value failed
[01:00:58] -   --> $DIR/match-test-ptr-null.rs:7:13
[01:00:58] -   --> $DIR/match-test-ptr-null.rs:7:13
[01:00:58] + error: any use of this value will cause an error
[01:00:58] 17    |
[01:00:58] 17    |
[01:00:58] - LL |             0 => 42, //~ ERROR constant contains unimplemented expression type
[01:00:58] -    |             ^ "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
[01:00:58] + LL |       let _: [u8; 0] = [4; {
[01:00:58] +    |  __________________________^
[01:00:58] + LL | |         match &1 as *const i32 as usize { //~ ERROR casting pointers to integers in constants
[01:00:58] + LL | |             0 => 42, //~ ERROR constant contains unimplemented expression type
[01:00:58] +    | |             - "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
[01:00:58] + LL | |             //~^ NOTE "pointer arithmetic or comparison" needs an rfc before being allowed
[01:00:58] + ...  |
[01:00:58] + LL | |         }
[01:00:58] + LL | |     }];
[01:00:58] +    |
[01:00:58] +    = note: #[deny(const_err)] on by default
[01:00:58] 20 
[01:00:58] 21 error: aborting due to 3 previous errors
---
[01:00:58] 25 
[01:00:58] 
[01:00:58] 
[01:00:58] The actual stderr differed from the expected stderr.
[01:00:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/match-test-ptr-null.stderr
[01:00:58] To update references, rerun the tests and pass the `--bless` flag
[01:00:58] To only update this specific test, also pass `--test-args consts/const-eval/match-test-ptr-null.rs`
[01:00:58] error: 1 errors occurred comparing output.
[01:00:58] status: exit code: 1
[01:00:58] status: exit code: 1
[01:00:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/c; // here is good\n    let x = FOO.func(); // or even here!\n}\n