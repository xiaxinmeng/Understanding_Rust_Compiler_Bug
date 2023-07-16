\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-for-loop-head-linkage.rs","byte_start":201,"byte_end":207,"line_start":8,"line_end":8,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        vector[1] = 5;   //~ ERROR cannot borrow","highlight_start":9,"highlight_end":15}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-for-loop-head-linkage.rs","byte_start":83,"byte_end":90,"line_start":5,"line_end":5,"column_start":15,"column_end":22,"is_primary":false,"text":[{"text":"    for &x in &vector {","highlight_start":15,"highlight_end":22}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-for-loop-head-linkage.rs","byte_start":83,"byte_end":90,"line_start":5,"line_end":5,"column_start":15,"column_end":22,"is_primary":false,"text":[{"text":"    for &x in &vector {","highlight_start":15,"highlight_end":22}],"label":"immutable borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `vector` as mutable because it is also borrowed as immutable\n  --> /checkout/src/test/ui/borrowck/borrowck-for-loop-head-linkage.rs:8:9\n   |\nLL |     for &x in &vector {\n   |               -------\n   |               |\n   |               immutable borrow occurs here\n   |               immutable borrow later used here\n...\nLL |         vector[1] = 5;   //~ ERROR cannot borrow\n   |         ^^^^^^ mutable borrow occurs here\n\n"}
[01:30:33] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] thread '[ui (nll)] ui/borrowck/borrowck-for-loop-head-linkage.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:30:33] thread '[ui (nll)] ui/borrowck/borrowck-for-loop-head-linkage.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:30:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:30:33] 
[01:30:33] ---- [ui (nll)] ui/borrowck/borrowck-mut-borrow-linear-errors.rs#ast stdout ----
[01:30:33] diff of stderr:
[01:30:33] 
[01:30:33] 4 LL |             1 => { addr.push(&mut x); } //[ast]~ ERROR [E0499]
[01:30:33] 5    |                    ----      ^^^^^^ second mutable borrow occurs here
[01:30:33] 6    |                    |
[01:30:33] -    |                    first borrow used here, in later iteration of loop
[01:30:33] 8 ...
[01:30:33] 8 ...
[01:30:33] 9 LL |             _ => { addr.push(&mut x); } //[ast]~ ERROR [E0499]
[01:30:33] 10    |                              ------ first mutable borrow occurs here
[01:30:33] 13   --> $DIR/borrowck-mut-borrow-linear-errors.rs:15:30
[01:30:33] 14    |
[01:30:33] 14    |
[01:30:33] 15 LL |             1 => { addr.push(&mut x); } //[ast]~ ERROR [E0499]
[01:30:33] -    |                    ---- first borrow used here, in later iteration of loop
[01:30:33] +    |                    ---- first borrow later used here
[01:30:33] 17 LL |             //[mir]~^ ERROR [E0499]
[01:30:33] 18 LL |             2 => { addr.push(&mut x); } //[ast]~ ERROR [E0499]
[01:30:33] 19    |                              ^^^^^^ second mutable borrow occurs here
[01:30:33] 
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mut-borrow-linear-errors.ast.nll/borrowck-mut-borrow-linear-errors.ast.nll.stderr
[01:30:33] To update references, rerun the tests and pass the `--bless` flag
[01:30:33] To only update this specific test, also pass `--test-args borrowck/borrowck-mut-borrow-linear-errors.rs`
[01:30:33] 
[01:30:33] error in revision `ast`: 1 errors occurred comparing output.
[01:30:33] status: exit code: 1
[01:30:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-mut-borrow-linear-errors.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mut-borrow-linear-errors.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mut-borrow-linear-errors.ast.nll/auxiliary" "-A" "unused"
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] stderr:
[01:30:33] stderr:
[01:30:33] ------------------------------------------
[01:30:33] {"message":"cannot borrow `x` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n