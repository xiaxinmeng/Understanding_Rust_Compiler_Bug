plain
[01:38:49] 
[01:38:49] ---- [ui (nll)] ui/consts/const-ptr-unique.rs stdout ----
[01:38:49] diff of stderr:
[01:38:49] 
[01:38:49] - error[E0597]: borrowed value does not live long enough
[01:38:49] + error[E0716]: temporary value dropped while borrowed
[01:38:49] 2   --> $DIR/const-ptr-unique.rs:8:33
[01:38:49] 3    |
[01:38:49] 4 LL |     let x: &'static *mut u32 = &(unique.as_ptr());
[01:38:49] 
[01:38:49] -    |                                 ^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[01:38:49] +    |            -----------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[01:38:49] +    |            type annotation requires that borrow lasts for `'static`
[01:38:49] +    |            type annotation requires that borrow lasts for `'static`
[01:38:49] 6 LL |     //~^ ERROR borrowed value does not live long enough
[01:38:49] 7 LL | }
[01:38:49] -    | - temporary value only lives until here
[01:38:49] -    |
[01:38:49] -    = note: borrowed value must be valid for the static lifetime...
[01:38:49] +    | - temporary value is freed at the end of this statement
[01:38:49] 12 error: aborting due to previous error
[01:38:49] 13 
[01:38:49] 
[01:38:49] - For more information about this error, try `rustc --explain E0597`.
[01:38:49] - For more information about this error, try `rustc --explain E0597`.
[01:38:49] + For more information about this error, try `rustc --explain E0716`.
[01:38:49] 15 
[01:38:49] 
[01:38:49] 
[01:38:49] The actual stderr differed from the expected stderr.
[01:38:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique.nll/const-ptr-unique.nll.stderr
[01:38:49] To update references, rerun the tests and pass the `--bless` flag
[01:38:49] To only update this specific test, also pass `--test-args consts/const-ptr-unique.rs`
[01:38:49] error: 1 errors occurred comparing output.
[01:38:49] status: exit code: 1
[01:38:49] status: exit code: 1
[01:38:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-unique.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique.nll/auxiliary" "-A" "unused"
[01:38:49] ------------------------------------------
[01:38:49] 
[01:38:49] ------------------------------------------
[01:38:49] stderr:
[01:38:49] stderr:
[01:38:49] ------------------------------------------
[01:38:49] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n