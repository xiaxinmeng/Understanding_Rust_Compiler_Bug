\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/dropck_tarena_unsound_drop.rs","byte_start":1291,"byte_end":1297,"line_start":41,"line_end":41,"column_start":7,"column_end":13,"is_primary":true,"text":[{"text":"    f(&arena);","highlight_start":7,"highlight_end":13}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui-fulldeps/dropck_tarena_unsound_drop.rs","byte_start":1300,"byte_end":1301,"line_start":42,"line_end":42,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"} //~^ ERROR `arena` does not live long enough","highlight_start":1,"highlight_end":2}],"label":"`arena` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui-fulldeps/dropck_tarena_unsound_drop.rs","byte_start":1300,"byte_end":1301,"line_start":42,"line_end":42,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"} //~^ ERROR `arena` does not live long enough","highlight_start":1,"highlight_end":2}],"label":"borrow might be used here, when `arena` is dropped and runs the `Drop` code for type `arena::TypedArena`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `arena` does not live long enough\n  --> /checkout/src/test/ui-fulldeps/dropck_tarena_unsound_drop.rs:41:7\n   |\nLL |     f(&arena);\n   |       ^^^^^^ borrowed value does not live long enough\nLL | } //~^ ERROR `arena` does not live long enough\n   | -\n   | |\n   | `arena` dropped here while still borrowed\n   | borrow might be used here, when `arena` is dropped and runs the `Drop` code for type `arena::TypedArena`\n\n"}
[01:21:43] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[01:21:43] 
[01:21:43] ------------------------------------------
[01:21:43] 
[01:21:43] 
[01:21:43] thread '[ui] ui-fulldeps/dropck_tarena_unsound_drop.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3326:9
[01:21:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:21:43] 
[01:21:43] ---- [ui] ui-fulldeps/dropck_tarena_cycle_checked.rs stdout ----
[01:21:43] diff of stderr:
[01:21:43] 
[01:21:43] 1 error[E0597]: `arena` does not live long enough
[01:21:43] +   --> $DIR/dropck_tarena_cycle_checked.rs:116:7
[01:21:43] 3    |
[01:21:43] 3    |
[01:21:43] 4 LL |     f(&arena);
[01:21:43] -    |        ^^^^^ borrowed value does not live long enough
[01:21:43] +    |       ^^^^^^ borrowed value does not live long enough
[01:21:43] 6 LL | }
[01:21:43] -    | - `arena` dropped here while still borrowed
[01:21:43] -    |
[01:21:43] -    = note: values in a scope are dropped in the opposite order they are created
[01:21:43] +    | |
[01:21:43] +    | |
[01:21:43] +    | `arena` dropped here while still borrowed
[01:21:43] +    | borrow might be used here, when `arena` is dropped and runs the `Drop` code for type `arena::TypedArena`
[01:21:43] 11 error: aborting due to previous error
[01:21:43] 12 
[01:21:43] 
[01:21:43] 
[01:21:43] 
[01:21:43] The actual stderr differed from the expected stderr.
[01:21:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck_tarena_cycle_checked/dropck_tarena_cycle_checked.stderr
[01:21:43] To update references, rerun the tests and pass the `--bless` flag
[01:21:43] To only update this specific test, also pass `--test-args dropck_tarena_cycle_checked.rs`
[01:21:43] error: 1 errors occurred comparing output.
[01:21:43] status: exit code: 1
[01:21:43] status: exit code: 1
[01:21:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/dropck_tarena_cycle_checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck_tarena_cycle_checked/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck_tarena_cycle_checked/auxiliary" "-A" "unused"
[01:21:43] ------------------------------------------
[01:21:43] 
[01:21:43] ------------------------------------------
[01:21:43] stderr:
[01:21:43] stderr:
[01:21:43] ------------------------------------------
[01:21:43] {"message":"`arena` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a value was dropped while it was still borrowed\n\nExample of erroneous code:\n\n