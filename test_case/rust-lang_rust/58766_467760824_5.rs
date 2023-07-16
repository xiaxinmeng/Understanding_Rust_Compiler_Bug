\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-ptr-unique.rs","byte_start":167,"byte_end":184,"line_start":8,"line_end":8,"column_start":33,"column_end":50,"is_primary":true,"text":[{"text":"    let x: &'static *mut u32 = &(unique.as_ptr());","highlight_start":33,"highlight_end":50}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-ptr-unique.rs","byte_start":242,"byte_end":243,"line_start":10,"line_end":10,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-ptr-unique.rs","byte_start":146,"byte_end":163,"line_start":8,"line_end":8,"column_start":12,"column_end":29,"is_primary":false,"text":[{"text":"    let x: &'static *mut u32 = &(unique.as_ptr());","highlight_start":12,"highlight_end":29}],"label":"type annotation requires that borrow lasts for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/const-ptr-unique.rs:8:33\n   |\nLL |     let x: &'static *mut u32 = &(unique.as_ptr());\n   |            -----------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use\n   |            |\n   |            type annotation requires that borrow lasts for `'static`\nLL |     //~^ ERROR borrowed value does not live long enough\nLL | }\n   | - temporary value is freed at the end of this statement\n\n"}
[01:38:49] {"message":"For more information about this error, try `rustc --explain E0716`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0716`.\n"}
[01:38:49] 
[01:38:49] ------------------------------------------
[01:38:49] 
[01:38:49] 
[01:38:49] thread '[ui (nll)] ui/consts/const-ptr-unique.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:38:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:38:49] 
[01:38:49] ---- [ui (nll)] ui/consts/const-ptr-nonnull.rs stdout ----
[01:38:49] diff of stderr:
[01:38:49] 
[01:38:49] - error[E0597]: borrowed value does not live long enough
[01:38:49] + error[E0716]: temporary value dropped while borrowed
[01:38:49] 3    |
[01:38:49] 3    |
[01:38:49] 4 LL |     let x: &'static NonNull<u32> = &(NonNull::dangling());
[01:38:49] 
[01:38:49] -    |                                     ^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[01:38:49] +    |            ---------------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[01:38:49] +    |            type annotation requires that borrow lasts for `'static`
[01:38:49] 6 ...
[01:38:49] 7 LL | }
[01:38:49] 7 LL | }
[01:38:49] -    | - temporary value only lives until here
[01:38:49] -    |
[01:38:49] -    = note: borrowed value must be valid for the static lifetime...
[01:38:49] +    | - temporary value is freed at the end of this statement
[01:38:49] 11 
[01:38:49] - error[E0597]: borrowed value does not live long enough
[01:38:49] + error[E0716]: temporary value dropped while borrowed
[01:38:49] 14    |
[01:38:49] 14    |
[01:38:49] 15 LL |     let x: &'static NonNull<u32> = &(non_null.cast());
[01:38:49] 
[01:38:49] -    |                                     ^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[01:38:49] +    |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[01:38:49] +    |            type annotation requires that borrow lasts for `'static`
[01:38:49] +    |            type annotation requires that borrow lasts for `'static`
[01:38:49] 17 LL |     //~^ ERROR borrowed value does not live long enough
[01:38:49] 18 LL | }
[01:38:49] -    | - temporary value only lives until here
[01:38:49] -    |
[01:38:49] -    = note: borrowed value must be valid for the static lifetime...
[01:38:49] +    | - temporary value is freed at the end of this statement
[01:38:49] 23 error: aborting due to 2 previous errors
[01:38:49] 24 
[01:38:49] 
[01:38:49] - For more information about this error, try `rustc --explain E0597`.
[01:38:49] - For more information about this error, try `rustc --explain E0597`.
[01:38:49] + For more information about this error, try `rustc --explain E0716`.
[01:38:49] 26 
[01:38:49] 
[01:38:49] 
[01:38:49] The actual stderr differed from the expected stderr.
[01:38:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull.nll/const-ptr-nonnull.nll.stderr
[01:38:49] To update references, rerun the tests and pass the `--bless` flag
[01:38:49] To only update this specific test, also pass `--test-args consts/const-ptr-nonnull.rs`
[01:38:49] error: 1 errors occurred comparing output.
[01:38:49] status: exit code: 1
[01:38:49] status: exit code: 1
[01:38:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull.nll/auxiliary" "-A" "unused"
[01:38:49] ------------------------------------------
[01:38:49] 
[01:38:49] ------------------------------------------
[01:38:49] stderr:
[01:38:49] stderr:
[01:38:49] ------------------------------------------
[01:38:49] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n