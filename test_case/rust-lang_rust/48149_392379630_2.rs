\n\nThe number of supplied parameters must exactly match the number of defined type\nparameters.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0087.rs","byte_start":607,"byte_end":622,"line_start":17,"line_end":17,"column_start":5,"column_end":20,"is_primary":false,"text":[{"text":"    bar::<f64, u64>(); //~ ERROR expected at most 1 type parameter, found 2 type parameters [E0087]","highlight_start":5,"highlight_end":20}],"label":"expected 1 type parameter","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/error-codes/E0087.rs","byte_start":618,"byte_end":621,"line_start":17,"line_end":17,"column_start":16,"column_end":19,"is_primary":true,"text":[{"text":"    bar::<f64, u64>(); //~ ERROR expected at most 1 type parameter, found 2 type parameters [E0087]","highlight_start":16,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0087]: too many type parameters provided: expected at most 1 type parameter, found 2 type parameters\n  --> /checkout/src/test/ui/error-codes/E0087.rs:17:16\n   |\nLL |     bar::<f64, u64>(); //~ ERROR expected at most 1 type parameter, found 2 type parameters [E0087]\n   |     -----------^^^- expected 1 type parameter\n\n"}
[00:47:21] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:21] {"message":"For more information about this error, try `rustc --explain E0087`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0087`.\n"}
[00:47:21] ------------------------------------------
[00:47:21] 
[00:47:21] thread '[ui] ui/error-codes/E0087.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:47:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:21] 
[00:47:21] ---- [ui] ui/error-codes/E0088.rs stdout ----
[00:47:21] diff of stderr:
[00:47:21] 
[00:47:21] 2   --> $DIR/E0088.rs:15:9
[00:47:21] 3    |
[00:47:21] 4 LL |     f::<'static>(); //~ ERROR E0088
[00:47:21] -    |         ^^^^^^^ expected 0 lifetime parameters
[00:47:21] +    |     ----^^^^^^^- expected 0 lifetime parameters
[00:47:21] 6 
[00:47:21] 7 error[E0088]: too many lifetime parameters provided: expected at most 1 lifetime parameter, found 2 lifetime parameters
[00:47:21] 8   --> $DIR/E0088.rs:16:18
[00:47:21] 9    |
[00:47:21] 9    |
[00:47:21] 10 LL |     g::<'static, 'static>(); //~ ERROR E0088
[00:47:21] -    |                  ^^^^^^^ expected 1 lifetime parameter
[00:47:21] +    |     -------------^^^^^^^- expected 1 lifetime parameter
[00:47:21] 13 error: aborting due to 2 previous errors
[00:47:21] 14 
[00:47:21] 
[00:47:21] 
[00:47:21] 
[00:47:21] The actual stderr differed from the expected stderr.
[00:47:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0088/E0088.stderr
[00:47:21] To update references, rerun the tests and pass the `--bless` flag
[00:47:21] To only update this specific test, also pass `--test-args error-codes/E0088.rs`
[00:47:21] error: 1 errors occurred comparing output.
[00:47:21] status: exit code: 101
[00:47:21] status: exit code: 101
[00:47:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0088.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0088/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0088/auxiliary" "-A" "unused"
[00:47:21] ------------------------------------------
[00:47:21] 
[00:47:21] ------------------------------------------
[00:47:21] stderr:
[00:47:21] stderr:
[00:47:21] ------------------------------------------
[00:47:21] {"message":"too many lifetime parameters provided: expected at most 0 lifetime parameters, found 1 lifetime parameter","code":{"code":"E0088","explanation":"\nYou gave too many lifetime parameters. Erroneous code example:\n\n