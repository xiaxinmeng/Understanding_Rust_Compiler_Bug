\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-ptr-nonnull.rs","byte_start":263,"byte_end":280,"line_start":9,"line_end":9,"column_start":37,"column_end":54,"is_primary":true,"text":[{"text":"    let x: &'static NonNull<u32> = &(non_null.cast());","highlight_start":37,"highlight_end":54}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-ptr-nonnull.rs","byte_start":338,"byte_end":339,"line_start":11,"line_end":11,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-ptr-nonnull.rs","byte_start":238,"byte_end":259,"line_start":9,"line_end":9,"column_start":12,"column_end":33,"is_primary":false,"text":[{"text":"    let x: &'static NonNull<u32> = &(non_null.cast());","highlight_start":12,"highlight_end":33}],"label":"type annotation requires that borrow lasts for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/const-ptr-nonnull.rs:9:37\n   |\nLL |     let x: &'static NonNull<u32> = &(non_null.cast());\n   |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use\n   |            |\n   |            type annotation requires that borrow lasts for `'static`\nLL |     //~^ ERROR borrowed value does not live long enough\nLL | }\n   | - temporary value is freed at the end of this statement\n\n"}
[01:34:01] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[01:34:01] 
[01:34:01] ------------------------------------------
[01:34:01] 
[01:34:01] 
[01:34:01] thread '[ui (nll)] ui/consts/const-ptr-nonnull.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:34:01] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:34:01] 
[01:34:01] ---- [ui (nll)] ui/consts/const-ptr-unique.rs stdout ----
[01:34:01] diff of stderr:
[01:34:01] 
[01:34:01] - error[E0597]: borrowed value does not live long enough
[01:34:01] + error[E0716]: temporary value dropped while borrowed
[01:34:01] 2   --> $DIR/const-ptr-unique.rs:8:33
[01:34:01] 3    |
[01:34:01] 4 LL |     let x: &'static *mut u32 = &(unique.as_ptr());
[01:34:01] 
[01:34:01] -    |                                 ^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[01:34:01] +    |            -----------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[01:34:01] +    |            type annotation requires that borrow lasts for `'static`
[01:34:01] +    |            type annotation requires that borrow lasts for `'static`
[01:34:01] 6 LL |     //~^ ERROR borrowed value does not live long enough
[01:34:01] 7 LL | }
[01:34:01] -    | - temporary value only lives until here
[01:34:01] -    |
[01:34:01] -    = note: borrowed value must be valid for the static lifetime...
[01:34:01] +    | - temporary value is freed at the end of this statement
[01:34:01] 12 error: aborting due to previous error
[01:34:01] 13 
[01:34:01] 
[01:34:01] - For more information about this error, try `rustc --explain E0597`.
[01:34:01] - For more information about this error, try `rustc --explain E0597`.
[01:34:01] + For more information about this error, try `rustc --explain E0716`.
[01:34:01] 15 
[01:34:01] 
[01:34:01] 
[01:34:01] The actual stderr differed from the expected stderr.
[01:34:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique.nll/const-ptr-unique.nll.stderr
[01:34:01] To update references, rerun the tests and pass the `--bless` flag
[01:34:01] To only update this specific test, also pass `--test-args consts/const-ptr-unique.rs`
[01:34:01] error: 1 errors occurred comparing output.
[01:34:01] status: exit code: 1
[01:34:01] status: exit code: 1
[01:34:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-unique.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique.nll/auxiliary" "-A" "unused"
[01:34:01] ------------------------------------------
[01:34:01] 
[01:34:01] ------------------------------------------
[01:34:01] stderr:
[01:34:01] stderr:
[01:34:01] ------------------------------------------
[01:34:01] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n