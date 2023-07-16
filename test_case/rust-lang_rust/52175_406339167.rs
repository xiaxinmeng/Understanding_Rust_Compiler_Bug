plain
[00:47:11] ............................................................................................i.......
[00:47:15] ..................................................................................i.................
[00:47:18] ....................................................................................................
[00:47:21] ....................................................................................................
[00:47:24] .......................................................F............................................
[00:47:29] .............i....
[00:47:29] failures:
[00:47:29] 
[00:47:29] ---- [ui (nll)] ui/lifetimes/borrowck-let-suggestion.rs stdout ----
[00:47:29] ---- [ui (nll)] ui/lifetimes/borrowck-let-suggestion.rs stdout ----
[00:47:29] diff of stderr:
[00:47:29] 
[00:47:29] 5    |                 ^^^^^^^       - temporary value only lives until here
[00:47:29] 6    |                 |
[00:47:29] 7    |                 temporary value does not live long enough
[00:47:29] + LL |     //~^ ERROR borrowed value does not live long enough
[00:47:29] 8 LL |     x.use_mut();
[00:47:29] 9    |     - borrow later used here
[00:47:29] 
[00:47:29] 
[00:47:29] The actual stderr differed from the expected stderr.
[00:47:29] The actual stderr differed from the expected stderr.
[00:47:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/borrowck-let-suggestion.nll/borrowck-let-suggestion.nll.stderr
[00:47:29] To update references, rerun the tests and pass the `--bless` flag
[00:47:29] To only update this specific test, also pass `--test-args lifetimes/borrowck-let-suggestion.rs`
[00:47:29] error: 1 errors occurred comparing output.
[00:47:29] status: exit code: 101
[00:47:29] status: exit code: 101
[00:47:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/borrowck-let-suggestion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/borrowck-let-suggestion.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/borrowck-let-suggestion.nll/auxiliary" "-A" "unused"
[00:47:29] ------------------------------------------
[00:47:29] 
[00:47:29] ------------------------------------------
[00:47:29] stderr:
[00:47:29] stderr:
[00:47:29] ------------------------------------------
[00:47:29] {"message":"borrowed value does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n