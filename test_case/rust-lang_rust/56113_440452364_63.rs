compile_fail,E0503\nfn main()rimary":false,"text":[{"text":"    let r = &mut x;","highlight_start":13,"highlight_end":19}],"label":"borrow of `x` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-match-already-borrowed.rs","byte_start":995,"byte_end":996,"line_start":38,"line_end":38,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        y => y + 2, //[ast]~ ERROR [E0503]","highlight_start":9,"highlight_end":10}],"label":"use of borrowed `x`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-match-already-borrowed.rs","byte_start":1090,"byte_end":1091,"line_start":41,"line_end":41,"column_start":10,"column_end":11,"is_primary":false,"text":[{"text":"    drop(r);","highlight_start":10,"highlight_end":11}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0503]: cannot use `x` because it was mutably borrowed\n  --> /checkout/src/test/ui/borrowck/borrowck-match-already-borrowed.rs:38:9\n   |\nLL |     let r = &mut x;\n   |             ------ borrow of `x` occurs here\n...\nLL |         y => y + 2, //[ast]~ ERROR [E0503]\n   |         ^ use of borrowed `x`\n...\nLL |     drop(r);\n   |          - borrow later used here\n\n"}
[00:48:22] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:48:22] {"message":"For more information about this error, try `rustc --explain E0503`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0503`.\n"}
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] thread '[ui] ui/borrowck/borrowck-match-already-borrowed.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:22] 
[00:48:22] 
[00:48:22] ---- [ui] ui/borrowck/borrowck-overloaded-index-ref-index.rs#mir stdout ----
[00:48:22] diff of stderr:
[00:48:22] 
[00:48:22] 7    |                      ^^ immutable borrow occurs here
[00:48:22] 8 ...
[00:48:22] 9 LL |     drop(rs);
[00:48:22] -    |          -- mutable borrow later used here
[00:48:22] +    |          -- mutable borrow used here, in later iteration of loop
[00:48:22] 12 error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
[00:48:22] 13   --> $DIR/borrowck-overloaded-index-ref-index.rs:65:7
[00:48:22] 
[00:48:22] 19    |       ^^ immutable borrow occurs here
[00:48:22] 19    |       ^^ immutable borrow occurs here
[00:48:22] 20 ...
[00:48:22] 21 LL |     drop(rs);
[00:48:22] -    |          -- mutable borrow later used here
[00:48:22] +    |          -- mutable borrow used here, in later iteration of loop
[00:48:22] 23 
[00:48:22] 24 error[E0594]: cannot assign to data in a `&` reference
[00:48:22] 
[00:48:22] 
[00:48:22] The actual stderr differed from the expected stderr.
[00:48:22] The actual stderr differed from the expected stderr.
[00:48:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-ref-index.mir/borrowck-overloaded-index-ref-index.mir.stderr
[00:48:22] To update references, rerun the tests and pass the `--bless` flag
[00:48:22] To only update this specific test, also pass `--test-args borrowck/borrowck-overloaded-index-ref-index.rs`
[00:48:22] 
[00:48:22] error in revision `mir`: 1 errors occurred comparing output.
[00:48:22] status: exit code: 1
[00:48:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-overloaded-index-ref-index.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-ref-index.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-ref-index.mir/auxiliary" "-A" "unused"
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] ------------------------------------------
[00:48:22] stderr:
[00:48:22] stderr:
[00:48:22] ------------------------------------------
[00:48:22] {"message":"cannot borrow `s` as immutable because it is also borrowed as mutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n