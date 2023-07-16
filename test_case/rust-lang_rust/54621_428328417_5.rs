\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\3] 17    |              ^^^^ use of possibly uninitialized `**x`
[00:48:33] 18 
[00:48:33] + error[E0718]: cannot assign to `a.x` when `a` is not initialized
[00:48:33] +   --> $DIR/borrowck-uninit-ref-chain.rs:34:5
[00:48:33] +    |
[00:48:33] + LL |     a.x = 0;
[00:48:33] + 
[00:48:33] + 
[00:48:33] + error[E0718]: cannot assign to `a.x` when `a` is not initialized
[00:48:33] +   --> $DIR/borrowck-uninit-ref-chain.rs:39:5
[00:48:33] +    |
[00:48:33] + LL |     a.x = &&0;
[00:48:33] + 
[00:48:33] + 
[00:48:33] + error[E0718]: cannot assign to `a.x` when `a` is not initialized
[00:48:33] +   --> $DIR/borrowck-uninit-ref-chain.rs:45:5
[00:48:33] +    |
[00:48:33] + LL |     a.x = 0;
[00:48:33] + 
[00:48:33] + 
[00:48:33] 19 error[E0381]: borrow of possibly uninitialized variable: `a.y`
[00:48:33] 21    |
[00:48:33] 
[00:48:33] 
[00:48:33] 22 LL |     let _b = &a.y; //[ast]~ ERROR use of possibly uninitialized variable: `a.y` [E0381]
[00:48:33] 23    |              ^^^^ use of possibly uninitialized `a.y`
[00:48:33] 24 
[00:48:33] + error[E0718]: cannot assign to `a.x` when `a` is not initialized
[00:48:33] +   --> $DIR/borrowck-uninit-ref-chain.rs:50:5
[00:48:33] +    |
[00:48:33] + LL |     a.x = &&0;
[00:48:33] + 
[00:48:33] + 
[00:48:33] 25 error[E0381]: borrow of possibly uninitialized variable: `**a.y`
[00:48:33] 27    |
[00:48:33] 
[00:48:33] 
[00:48:33] 28 LL |     let _b = &**a.y; //[ast]~ ERROR use of possibly uninitialized variable: `**a.y` [E0381]
[00:48:33] 29    |              ^^^^^^ use of possibly uninitialized `**a.y`
[00:48:33] - error: aborting due to 5 previous errors
[00:48:33] + error: aborting due to 9 previous errors
[00:48:33] 32 
[00:48:33] - For more information about this error, try `rustc --explain E0381`.
[00:48:33] - For more information about this error, try `rustc --explain E0381`.
[00:48:33] + Some errors occurred: E0381, E0718.
[00:48:33] + For more information about an error, try `rustc --explain E0381`.
[00:48:33] 34 
[00:48:33] 
[00:48:33] 
[00:48:33] The actual stderr differed from the expected stderr.
[00:48:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-ref-chain.mir/borrowck-uninit-ref-chain.mir.stderr
[00:48:33] To update references, rerun the tests and pass the `--bless` flag
[00:48:33] To only update this specific test, also pass `--test-args borrowck/borrowck-uninit-ref-chain.rs`
[00:48:33] 
[00:48:33] error in revision `mir`: 1 errors occurred comparing output.
[00:48:33] status: exit code: 1
[00:48:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-uninit-ref-chain.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-ref-chain.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-ref-chain.mir/auxiliary" "-A" "u; // ok!\n}\n