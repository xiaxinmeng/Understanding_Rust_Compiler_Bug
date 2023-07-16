\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/stable/book/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/regions-escape-loop-via-vec.rs","byte_start":125,"byte_end":131,"line_start":4,"line_end":4,"column_start":23,"column_end":29,"is_primary":false,"text":[{"text":"    let mut _y = vec![&mut x];","highlight_start":23,"highlight_end":29}],"label":"borrow of `x` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/regions-escape-loop-via-vec.rs","byte_start":372,"byte_end":378,"line_start":9,"line_end":9,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        x += 1; //~ ERROR cannot assign","highlight_start":9,"highlight_end":15}],"label":"use of borrowed `x`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/regions-escape-loop-via-vec.rs","byte_start":298,"byte_end":300,"line_start":7,"line_end":7,"column_start":9,"column_end":11,"is_primary":false,"text":[{"text":"        _y.push(&mut z);","highlight_start":9,"highlight_end":11}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0503]: cannot use `x` because it was mutably borrowed\n  --> /checkout/src/test/ui/span/regions-escape-loop-via-vec.rs:9:9\n   |\nLL |     let mut _y = vec![&mut x];\n   |                       ------ borrow of `x` occurs here\n...\nLL |         _y.push(&mut z);\n   |         -- borrow later used here\nLL |         //~^ ERROR `z` does not live long enough\nLL |         x += 1; //~ ERROR cannot assign\n   |         ^^^^^^ use of borrowed `x`\n\n"}
[01:30:33] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:30:33] {"message":"Some errors occurred: E0503, E0597.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0503, E0597.\n"}
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] thread '[ui (nll)] ui/span/regions-escape-loop-via-vec.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:30:33] thread '[ui (nll)] ui/span/regions-escape-loop-via-vec.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:30:33] 
[01:30:33] ---- [ui (nll)] ui/vec/vec-mut-iter-borrow.rs stdout ----
[01:30:33] diff of stderr:
[01:30:33] 
[01:30:33] 5    |              -------
[01:30:33] 6    |              |
[01:30:33] 7    |              first mutable borrow occurs here
[01:30:33] -    |              first borrow used here, in later iteration of loop
[01:30:33] +    |              first borrow later used here
[01:30:33] 9 LL |         xs.push(1) //~ ERROR cannot borrow `xs`
[01:30:33] 10    |         ^^ second mutable borrow occurs here
[01:30:33] 
[01:30:33] 
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-mut-iter-borrow.nll/vec-mut-iter-borrow.nll.stderr
[01:30:33] To update references, rerun the tests and pass the `--bless` flag
[01:30:33] To only update this specific test, also pass `--test-args vec/vec-mut-iter-borrow.rs`
[01:30:33] error: 1 errors occurred comparing output.
[01:30:33] status: exit code: 1
[01:30:33] status: exit code: 1
[01:30:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/vec/vec-mut-iter-borrow.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-mut-iter-borrow.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-mut-iter-borrow.nll/auxiliary" "-A" "unused"
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] stderr:
[01:30:33] stderr:
[01:30:33] ------------------------------------------
[01:30:33] {"message":"cannot borrow `xs` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n