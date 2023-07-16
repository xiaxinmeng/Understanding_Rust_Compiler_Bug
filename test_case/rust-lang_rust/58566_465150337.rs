plain
[01:30:22] 
[01:30:22] ---- [ui (nll)] ui/borrowck/borrowck-uninit-field-access.rs#ast stdout ----
[01:30:22] diff of stderr:
[01:30:22] 
[01:30:22] 20 LL |     let _moved = (line2.origin, line2.middle);
[01:30:22] 21    |                                 ------------ value moved here
[01:30:22] 22 LL |     line2.consume(); //[ast]~ ERROR use of partially moved value: `line2` [E0382]
[01:30:22] -    |     ^^^^^ value used here after move
[01:30:22] +    |     ^^^^^ value used here after partial move
[01:30:22] 24    |
[01:30:22] 25    = note: move occurs because `line2.middle` has type `Point`, which does not implement the `Copy` trait
[01:30:22] 
[01:30:22] 
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] The actual stderr differed from the expected stderr.
[01:30:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-field-access.ast.nll/borrowck-uninit-field-access.ast.nll.stderr
[01:30:22] To update references, rerun the tests and pass the `--bless` flag
[01:30:22] To only update this specific test, also pass `--test-args borrowck/borrowck-uninit-field-access.rs`
[01:30:22] 
[01:30:22] error in revision `ast`: 1 errors occurred comparing output.
[01:30:22] status: exit code: 1
[01:30:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-uninit-field-access.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-field-access.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-field-access.ast.nll/auxiliary" "-A" "unused"
[01:30:22] ------------------------------------------
[01:30:22] 
[01:30:22] ------------------------------------------
[01:30:22] stderr:
[01:30:22] stderr:
[01:30:22] ------------------------------------------
[01:30:22] {"message":"use of possibly uninitialized variable: `a`","code":{"code":"E0381","explanation":"\nIt is not allowed to use or capture an uninitialized variable. For example:\n\n