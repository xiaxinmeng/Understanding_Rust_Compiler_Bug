plain
[00:49:17] 
[00:49:17] running 4579 tests
[00:49:20] .................................................................................................... 100/4579
[00:49:23] .................................................................................................... 200/4579
[00:49:26] ........................................................................F........................... 300/4579
[00:49:29] ...............................................................F.........F.......................... 400/4579
[00:49:35] .......................i............................................................................ 600/4579
[00:49:40] .................................................................................................... 700/4579
[00:49:45] ...................................i...........i.................................................... 800/4579
[00:49:48] ......................................................iiiii......................................... 900/4579
---
[00:51:45] .................................................................................................... 4400/4579
[00:51:48] .................................................................................................... 4500/4579
the `Copy` trait
[00:51:50] 10 
[00:51:50] + error[E0718]: cannot assign to `src.0` when `src` is not initialized
[00:51:50] +   --> $DIR/borrowck-issue-48962.rs:32:5
[00:51:50] +    |
[00:51:50] + LL |     src.0 = 66; //~ ERROR use of moved value: `src` [E0382]
[00:51:50] + 
[00:51:50] + 
[00:51:50] 11 error[E0382]: use of moved value: `src`
[00:51:50] 13    |
[00:51:50] 
[00:51:50] 18    |
[00:51:50] 18    |
[00:51:50] 19    = note: move occurs because `src` has type `&mut (i32, i32)`, which does not implement the `Copy` trait
[00:51:50] - error: aborting due to 2 previous errors
[00:51:50] + error: aborting due to 4 previous errors
[00:51:50] 22 
[00:51:50] - For more information about this error, try `rustc --explain E0382`.
[00:51:50] - For more information about this error, try `rustc --explain E0382`.
[00:51:50] + Some errors occurred: E0382, E0718.
[00:51:50] + For more information about an error, try `rustc --explain E0382`.
[00:51:50] 24 
[00:51:50] 
[00:51:50] 
[00:51:50] The actual stderr differed from the expected stderr.
[00:51:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-48962/borrowck-issue-48962.stderr
[00:51:50] To update references, rerun the tests and pass the `--bless` flag
[00:51:50] To only update this specific test, also pass `--test-args borrowck/borrowck-issue-48962.rs`
[00:51:50] error: 1 errors occurred comparing output.
[00:51:50] status: exit code: 1
[00:51:50] status: exit code: 1
[00:51:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-issue-48962.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-48962/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-48962/auxiliary" "-A" "unused"
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] ------------------------------------------
[00:51:50] stderr:
[00:51:50] stderr:
[00:51:50] ------------------------------------------
[00:51:50] {"message":"cannot assign to `src.next` when `src` is not initialized","code":{"code":"E0718","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-issue-48962.rs","byte_start":670,"byte_end":678,"line_start":26,"line_end":26,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    src.next = None; //~ ERROR use of moved value: `src` [E0382]","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0718]: cannot assign to `src.next` when `src` is not initialized\n  --> /checkout/src/test/ui/borrowck/borrowck-issue-48962.rs:26:5\n   |\nLL |     src.next = None; //~ ERROR use of moved value: `src` [E0382]\n   |     ^^^^^^^^\n\n"}
[00:51:50] {"message":"use of moved value: `src`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n``ement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `src`\n  --> /checkout/src/test/ui/borrowck/borrowck-issue-48962.rs:32:5\n   |\nLL |     {src};\n   |      --- value moved here\nLL |     src.0 = 66; //~ ERROR use of moved value: `src` [E0382]\n   |     ^^^^^^^^^^ value used here after move\n   |\n   = note: move occurs because `src` has type `&mut (i32, i32)`, which does not implement the `Copy` trait\n\n"}
[00:51:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:51:50] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:51:50] {"message":"Some errors occurred: E0382, E0718.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0382, E0718.\n"}
[00:51:50] {"message":"For more information about an error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0382`.\n"}
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] thread '[ui] ui/borrowck/borrowck-issue-48962.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:51:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:50] 
[00:51:50] ---- [ui] ui/borrowck/borrowck-uninit-ref-chain.rs#mir stdout ----
[00:51:50] diff of stderr:
[00:51:50] 
[00:51:50] 16 LL |     let _y = &**x; //[ast]~ ERROR use of possibly uninitialized variable:use of possibly uninitialized variable: `**a.y` [E0381]
[00:51:50] 29    |              ^^^^^^ use of possibly uninitialized `**a.y`
[00:51:50] - error: aborting due to 5 previous errors
[00:51:50] + error: aborting due to 9 previous errors
[00:51:50] 32 
[00:51:50] - For more information about this error, try `rustc --explain E0381`.
[00:51:50] - For more information about this error, try `rustc --explain E0381`.
[00:51:50] + Some errors occurred: E0381, E0718.
[00:51:50] + For more information about an error, try `rustc --explain E0381`.
[00:51:50] 34 
[00:51:50] 
[00:51:50] 
[00:51:50] The actual stderr differed from the expected stderr.
[00:51:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-ref-chain.mir/borrowck-uninit-ref-chain.mir.stderr
[00:51:50] To update references, rerun the tests and pass the `--bless` flag
[00:51:50] To only update this specific test, also pass `--test-args borrowck/borrowck-uninit-ref-chain.rs`
[00:51:50] 
[00:51:50] error in revision `mir`: 1 errors occurred comparing output.
[00:51:50] status: exit code: 1
[00:51:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-uninit-ref-chain.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-ref-chain.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-ref-chain.mir/auxiliary" "-A" "unused"
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] ------------------------------------------
[00:51:50] stderr:
[00:51:50] stderr:
[00:51:50] ------------------------------------------
[00:51:50] {"message":"borrow of possibly uninitialized variable: `**x`","code":{"code":"E0381","explanation":"\nIt is not allowed to use or capture an uninitialized variable. For example:\n\n