plain
[01:20:51] 
[01:20:51] ---- [ui (nll)] ui/consts/min_const_fn/min_const_fn_dyn.rs stdout ----
[01:20:51] diff of stderr:
[01:20:51] 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 3    |
[01:20:51] 3    |
[01:20:51] 4 LL |     x.0.field;
[01:20:51] 5    |     ^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 6 
[01:20:51] 6 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 9    |
[01:20:51] 9    |
[01:20:51] 10 LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
[01:20:51] 11    |                                                                  ^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 12 
[01:20:51] 12 
[01:20:51] 13 warning[E0716]: temporary value dropped while borrowed
[01:20:51] 
[01:20:51] 24 
[01:20:51] 25 error: aborting due to 2 previous errors
[01:20:51] 26 
[01:20:51] 26 
[01:20:51] - For more information about this error, try `rustc --explain E0716`.
[01:20:51] + Some errors occurred: E0716, E0723.
[01:20:51] + For more information about an error, try `rustc --explain E0716`.
[01:20:51] 28 
[01:20:51] 
[01:20:51] 
[01:20:51] The actual stderr differed from the expected stderr.
[01:20:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn.nll/min_const_fn_dyn.nll.stderr
[01:20:51] To update references, rerun the tests and pass the `--bless` flag
[01:20:51] To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_dyn.rs`
[01:20:51] error: 1 errors occurred comparing output.
[01:20:51] status: exit code: 1
[01:20:51] status: exit code: 1
[01:20:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn.nll/auxiliary" "-A" "unused"
[01:20:51] ------------------------------------------
[01:20:51] 
[01:20:51] ------------------------------------------
[01:20:51] stderr:
[01:20:51] stderr:
[01:20:51] ------------------------------------------
[01:20:51] {"message":"trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)","code":{"code":"E0723","explanation":"\nAn feature unstable in `const` contexts was used.\n\nErroneous code example:\n\n