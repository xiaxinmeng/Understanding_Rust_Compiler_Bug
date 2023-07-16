\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-for-loop-head-linkage.rs","byte_start":668,"byte_end":674,"line_start":18,"line_end":18,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        vector[1] = 5;   //~ ERROR cannot borrow","highlight_start":9,"highlight_end":15}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-for-loop-head-linkage.rs","byte_start":550,"byte_end":557,"line_start":15,"line_end":15,"column_start":15,"column_end":22,"is_primary":false,"text":[{"text":"    for &x in &vector {","highlight_start":15,"highlight_end":22}],"label":"immutable borrow occurs here","suggested_replacement":null,"sugge19] ---- [ui (nll)] ui/borrowck/borrowck-mut-borrow-linear-errors.rs#ast stdout ----
[00:56:19] 
[00:56:19] 
[00:56:19] 4 LL |             1 => { addr.push(&mut x); } //[ast]~ ERROR [E0499]
[00:56:19] 5    |                    ----      ^^^^^^ second mutable borrow occurs here
[00:56:19] 6    |                    |
[00:56:19] -    |                    first borrow used here, in later iteration of loop
[00:56:19] 8 ...
[00:56:19] 8 ...
[00:56:19] 9 LL |             _ => { addr.push(&mut x); } //[ast]~ ERROR [E0499]
[00:56:19] 10    |                              ------ first mutable borrow occurs here
[00:56:19] 13   --> $DIR/borrowck-mut-borrow-linear-errors.rs:25:30
[00:56:19] 14    |
[00:56:19] 14    |
[00:56:19] 15 LL |             1 => { addr.push(&mut x); } //[ast]~ ERROR [E0499]
[00:56:19] -    |                    ---- first borrow used here, in later iteration of loop
[00:56:19] +    |                    ---- first borrow later used here
[00:56:19] 17 LL |             //[mir]~^ ERROR [E0499]
[00:56:19] 18 LL |             2 => { addr.push(&mut x); } //[ast]~ ERROR [E0499]
[00:56:19] 19    |                              ^^^^^^ second mutable borrow occurs here
[00:56:19] 
[00:56:19] The actual stderr differed from the expected stderr.
[00:56:19] The actual stderr differed from the expected stderr.
[00:56:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mut-borrow-linear-errors.ast.nll/borrowck-mut-borrow-linear-errors.ast.nll.stderr
[00:56:19] To update references, rerun the tests and pass the `--bless` flag
[00:56:19] To only update this specific test, also pass `--test-args borrowck/borrowck-mut-borrow-linear-errors.rs`
[00:56:19] 
[00:56:19] error in revision `ast`: 1 errors occurred comparing output.
[00:56:19] status: exit code: 1
[00:56:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-mut-borrow-linear-errors.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mut-borrow-linear-errors.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mut-borrow-linear-errors.ast.nll/auxiliary" "-A" "unused"
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] ------------------------------------------
[00:56:19] stderr:
[00:56:19] stderr:
[00:56:19] ------------------------------------------
[00:56:19] {"message":"cannot borrow `x` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n