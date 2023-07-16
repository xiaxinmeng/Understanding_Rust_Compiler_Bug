compile_fail,E0502\nfn bar(x: &mut i32) {}\nfn foo(a: &mut i32) {\n    let ref y = a; // a is borrowed as                   -- borrow used here, in later iteration of loop
[00:51:04] +    |                     -- borrow later used here
[00:51:04] 6 LL |         p = &x;
[00:51:04] 7    |             ^^ borrowed value does not live long enough
[00:51:04] 8 LL |     }
[00:51:04] 
[00:51:04] The actual stderr differed from the expected stderr.
[00:51:04] The actual stderr differed from the expected stderr.
[00:51:04] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-variable.nll/regions-escape-loop-via-variable.nll.stderr
[00:51:04] To update references, rerun the tests and pass the `--bless` flag
[00:51:04] To only update this specific test, also pass `--test-args span/regions-escape-loop-via-variable.rs`
[00:51:04] error: 1 errors occurred comparing output.
[00:51:04] status: exit code: 1
[00:51:04] status: exit code: 1
[00:51:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/regions-escape-loop-via-variable.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-variable.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-variable.nll/auxiliary" "-A" "unused"
[00:51:04] ------------------------------------------
[00:51:04] 
[00:51:04] ------------------------------------------
[00:51:04] stderr:
[00:51:04] stderr:
[00:51:04] -------------------/src/test/ui/span/regions-escape-loop-via-variable.rs","byte_start":706,"byte_end":708,"line_start":20,"line_end":20,"column_start":21,"column_end":23,"is_primary":false,"text":[{"text":"        let x = 1 + *p;","highlight_start":21,"highlight_end":23}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `x` does not live long enough\n  --> /checkout/src/test/ui/span/regions-escape-loop-via-variable.rs:21:13\n   |\nLL |         let x = 1 + *p;\n   |                     -- borrow later used here\nLL |         p = &x;\n   |             ^^ borrowed value does not live long enough\nLL |     }\n   |     - `x` dropped here while still borrowed\n\n"}
[00:51:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:04] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:51:04] ------------------------------------------
[00:51:04] 
[00:51:04] thread '[ui (nll)] ui/span/regions-escape-loop-via-variable.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:51:04] 
[00:51:04] 
[00:51:04] ---- [ui (nll)] ui/span/regions-escape-loop-via-vec.rs stdout ----
[00:51:04] diff of stderr:
[00:51:04] 
[00:51:04] 7    |           ^ use of borrowed `x`
[00:51:04] 8 LL |         let mut z = x; //~ ERROR cannot use `x` because it was mutably borrowed
^^^^^^ use of borrowed `x`
[00:51:04] 
[00:51:04] The actual stderr differed from the expected stderr.
[00:51:04] The actual stderr differed from the expected stderr.
[00:51:04] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-vec.nll/regions-escape-loop-via-vec.nll.stderr
[00:51:04] To update references, rerun the tests and pass the `--bless` flag
[00:51:04] To only update this specific test, also pass `--test-args span/regions-escape-loop-via-vec.rs`
[00:51:04] error: 1 errors occurred comparing output.
[00:51:04] status: exit code: 1
[00:51:04] status: exit code: 1
[00:51:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/regions-escape-loop-via-vec.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-vec.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-escape-loop-via-vec.nll/auxiliary" "-A" "unused"
[00:51:04] ------------------------------------------
[00:51:04] 
[00:51:04] ------------------------------------------
[00:51:04] stderr:
[00:51:04] stderr:
[00:51:04] ------------------------------------------
[00:51:04] {"message":"cannot use `x` because it was mutably borrowed","code":{"code":"E0503","explanation":"\nA value was used after it was mutably borrowed.\n\nExample of erroneous code:\n\n