\n\nAlternatively, if we don't control the struct's definition, or mutable shared\nownership is truly required, we can use `Rc` and `Rete","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `x`\n  --> /checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs:318:22\n   |\nLL |                 drop(x);\n   |                      - value moved here\nLL |                 drop(x); //[ast]~ ERROR use of moved value: `x`\n   |                      ^ value used here after move\n   |\n   = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait\n\n"}
[00:48:22] {"message":"aborting due to 32 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 32 previous errors\n\n"}
[00:48:22] {"message":"Some errors occurred: E0382, E0499, E0502, E0503.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0382, E0499, E0502, E0503.\n"}
[00:48:22] {"message":"For more information about an error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0382`.\n"}
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] thread '[ui] ui/borrowck/borrowck-describe-lvalue.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:22] 
[00:48:22] 
[00:48:22] ---- [ui] ui/borrowck/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.rs#mir stdout ----
[00:48:22] diff of stderr:
[00:48:22] 
[00:48:22] 8    |         ^^^^^^ assignment to borrowed `_a` occurs here
[00:48:22] 9 ...
[00:48:22] 10 LL |     drop(b);
[00:48:22] -    |          - borrow later used here
[00:48:22] +    |          - borrow used here, in later iteration of loop
[00:48:22] 13 error: aborting due to previous error
[00:48:22] 14 
[00:48:22] 
[00:48:22] 
[00:48:22] 
[00:48:22] The actual stderr differed from the expected stderr.
[00:48:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.mir/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.mir.stderr
[00:48:22] To update references, rerun the tests and pass the `--bless` flag
[00:48:22] To only update this specific test, also pass `--test-args borrowck/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.rs`
[00:48:22] 
[00:48:22] error in revision `mir`: 1 errors occurred comparing output.
[00:48:22] status: exit code: 1
[00:48:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.mir/auxiliary" "-A" "unused"
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] ------------------------------------------
[00:48:22] stderr:
[00:48:22] stderr:
[00:48:22] ------------------------------> /checkout/src/test/ui/borrowck/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.rs:19:9\n   |\nLL |     let b = &mut _a;\n   |             ------- borrow of `_a` occurs here\n...\nLL |         _a = 4; //[ast]~ ERROR cannot assign to `_a`\n   |         ^^^^^^ assignment to borrowed `_a` occurs here\n...\nLL |     drop(b);\n   |          - borrow used here, in later iteration of loop\n\n"}
[00:48:22] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:22] {"message":"For more information about this error, try `rustc --explain E0506`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0506`.\n"}
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] thread '[ui] ui/borrowck/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:22] 
[00:48:22] 
[00:48:22] ---- [ui] ui/borrowck/borrowck-match-already-borrowed.rs#mir stdout ----
[00:48:22] diff of stderr:
[00:48:22] 
[00:48:22] 8    |         ^^^^^^ use of borrowed `foo`
[00:48:22] 9 ...
[00:48:22] 10 LL |     drop(p);
[00:48:22] -    |          - borrow later used here
[00:48:22] +    |          - borrow used here, in later iteration of loop
[00:48:22] 12 
[00:48:22] 13 error[E0503]: cannot use `foo.0` because it was mutably borrowed
[00:48:22] 
[00:48:22] 
[00:48:22] The actual stderr differed from the expected stderr.
[00:48:22] The actual stderr differed from the expected stderr.
{\n    let mut value = 3;\n    // Create a mutable borrow of `value`. This borrow\n    // lives until the end of this function.\n    let _borrow = &mut value;\n    let _sum = value + 1; // error: cannot use `value` because\n                          //        it was mutably borrowed\n}\n