plain
[01:34:01] 
[01:34:01] ---- [ui (nll)] ui/consts/const-ptr-nonnull.rs stdout ----
[01:34:01] diff of stderr:
[01:34:01] 
[01:34:01] - error[E0597]: borrowed value does not live long enough
[01:34:01] + error[E0716]: temporary value dropped while borrowed
[01:34:01] 3    |
[01:34:01] 3    |
[01:34:01] 4 LL |     let x: &'static NonNull<u32> = &(NonNull::dangling());
[01:34:01] 
[01:34:01] -    |                                     ^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[01:34:01] +    |            ---------------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[01:34:01] +    |            type annotation requires that borrow lasts for `'static`
[01:34:01] 6 ...
[01:34:01] 7 LL | }
[01:34:01] 7 LL | }
[01:34:01] -    | - temporary value only lives until here
[01:34:01] -    |
[01:34:01] -    = note: borrowed value must be valid for the static lifetime...
[01:34:01] +    | - temporary value is freed at the end of this statement
[01:34:01] 11 
[01:34:01] - error[E0597]: borrowed value does not live long enough
[01:34:01] + error[E0716]: temporary value dropped while borrowed
[01:34:01] 14    |
[01:34:01] 14    |
[01:34:01] 15 LL |     let x: &'static NonNull<u32> = &(non_null.cast());
[01:34:01] 
[01:34:01] -    |                                     ^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[01:34:01] +    |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[01:34:01] +    |            type annotation requires that borrow lasts for `'static`
[01:34:01] +    |            type annotation requires that borrow lasts for `'static`
[01:34:01] 17 LL |     //~^ ERROR borrowed value does not live long enough
[01:34:01] 18 LL | }
[01:34:01] -    | - temporary value only lives until here
[01:34:01] -    |
[01:34:01] -    = note: borrowed value must be valid for the static lifetime...
[01:34:01] +    | - temporary value is freed at the end of this statement
[01:34:01] 23 error: aborting due to 2 previous errors
[01:34:01] 24 
[01:34:01] 
[01:34:01] - For more information about this error, try `rustc --explain E0597`.
[01:34:01] - For more information about this error, try `rustc --explain E0597`.
[01:34:01] + For more information about this error, try `rustc --explain E0716`.
[01:34:01] 26 
[01:34:01] 
[01:34:01] 
[01:34:01] The actual stderr differed from the expected stderr.
[01:34:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull.nll/const-ptr-nonnull.nll.stderr
[01:34:01] To update references, rerun the tests and pass the `--bless` flag
[01:34:01] To only update this specific test, also pass `--test-args consts/const-ptr-nonnull.rs`
[01:34:01] error: 1 errors occurred comparing output.
[01:34:01] status: exit code: 1
[01:34:01] status: exit code: 1
[01:34:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull.nll/auxiliary" "-A" "unused"
[01:34:01] ------------------------------------------
[01:34:01] 
[01:34:01] ------------------------------------------
[01:34:01] stderr:
[01:34:01] stderr:
[01:34:01] ------------------------------------------
[01:34:01] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n