\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/borrowed-referent-issue-38899.rs","byte_start":729,"byte_end":744,"line_start":24,"line_end":24,"column_start":21,"column_end":36,"is_primary":true,"text":[{"text":"    let p: &'a u8 = &*block.current;","highlight_start":21,"highlight_end":36}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/borrowed-referent-issue-38899.rs","byte_start":666,"byte_end":676,"line_start":22,"line_end":22,"column_start":13,"column_end":23,"is_primary":false,"text":[{"text":"    let x = &mut block;","highlight_start":13,"highlight_end":23}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkoutorrowed-temporary-error.rs stdout ----
[00:43:19] 
[00:43:19] 
[00:43:19] 7 LL |     });
[00:43:19] 8    |       - temporary value only lives until here
[00:43:19] 9 LL |     println!("{:?}", x);
[00:43:19] +    |                      - borrow used here in later iteration of loop
[00:43:19] 11 
[00:43:19] 12 error: aborting due to previous error
[00:43:19] 13 
[00:43:19] 13 
[00:43:19] 
[00:43:19] 
[00:43:19] The actual stderr differed from the expected stderr.
[00:43:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-temporary-error/borrowed-temporary-error.stderr
[00:43:19] To update references, rerun the tests and pass the `--bless` flag
[00:43:19] To only update this specific test, also pass `--test-args nll/borrowed-temporary-error.rs`
[00:43:19] error: 1 errors occurred comparing output.
[00:43:19] status: exit code: 1
[00:43:19] status: exit code: 1
[00:43:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/borrowed-temporary-error.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-temporary-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-temporary-error/auxiliary" "-A" "unused"
[00:43:19] ------------------------------------------
[00:43:19] 
[00:43:19] 
[00:43:19] --------------------e_start":38,"line_end":38,"column_start":11,"column_end":12,"is_primary":false,"text":[{"text":"    deref(p);","highlight_start":11,"highlight_end":12}],"label":"borrow used here in later iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `y` does not live long enough\n  --> /checkout/src/test/ui/nll/capture-ref-in-struct.rs:31:16\n   |\nLL |             y: &y,\n   |                ^^ borrowed value does not live long enough\n...\nLL |     }\n   |     - `y` dropped here while still borrowed\nLL | \nLL |     deref(p);\n   |           - borrow used here in later iteration of loop\n\n"}
[00:43:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:19] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:43:19] ------------------------------------------
[00:43:19] 
[00:43:19] thread '[ui] ui/nll/capture-ref-in-struct.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:43:19] 
[00:43:19] 
[00:43:19] ---- [ui] ui/nll/closure-access-spans.rs stdout ----
[00:43:19] diff of stderr:
[00:43:19] 
[00:43:19] 8    |     |
[00:43:19] 9    |     immutable borrow occurs here
[00:43:19] 10 LL |     r.use_mut();
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:43:19] 12 
[00:43:19] 12 
[00:43:19] 13 error[E0499]: cannot borrow `x` as mutable more than once at a time
[00:43:19] 
[00:43:19] 20    |     |
[00:43:19] 21    |     second mutable borrow occurs here
[00:43:19] 21    |     second mutable borrow occurs here
[00:43:19] 22 LL |     r.use_mut();
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:43:19] 24 
[00:43:19] 24 
[00:43:19] 25 error[E0500]: closure requires unique access to `x` but it is already borrowed
[00:43:19] 
[00:43:19] 32    |     |
[00:43:19] 33    |     closure construction occurs here
[00:43:19] 33    |     closure construction occurs here
[00:43:19] 34 LL |     r.use_mut();
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:43:19] 36 
[00:43:19] 36 
[00:43:19] 37 error[E0503]: cannot use `x` because it was mutably borrowed
[00:43:19] 
[00:43:19] 
[00:43:19] 42 LL |     move || x; //~ ERROR
[00:43:19] 43    |             ^ use of borrowed `x`
[00:43:19] 44 LL |     r.use_ref();
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:43:19] 46 
[00:43:19] 47 error[E0505]: cannot move out of `x` because it is borrowed
[00:43:19] 48   --> $DIR/closure-access-spans.rs:41:5
[00:43:19] 48   --> $DIR/closure-access-spans.rs:41:5
[00:43:19] 
[00:43:19] 54    |     |
[00:43:19] 55    |     move out of `x` occurs here
[00:43:19] 56 LL |     r.use_ref();
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:43:19] 58 
[00:43:19] 58 
[00:43:19] 59 error[E0382]: borrow of moved value: `x`
[00:43:19] 
[00:43:19] 
[00:43:19] The actual stderr differed from the expected stderr.
[00:43:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-access-spans/closure-access-spans.stderr
[00:43:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-access-spans/closure-access-spans.stderr
[00:43:19] To update references, rerun the tests and pass the `--bless` flag
[00:43:19] To only update this specific test, also pass `--test-args nll/closure-access-spans.rs`
[00:43:19] error: 1 errors occurred comparing output.
[00:43:19] status: exit code: 1
[00:43:19] status: exit code: 1
[00:43:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-access-spans.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-access-spans/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-access-spans/auxiliary" "-A" "unused"
[00:43:19] ------------------------------------------
[00:43:19] 
[00:43:19] ------------------------------------------
[00:43:19] stderr:
[00:43:19] stderr:
[00:43:19] ------------------------------------------
[00:43:19] {"message":"cannot borrow `x` as immutable because it is also borrowed as mutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n