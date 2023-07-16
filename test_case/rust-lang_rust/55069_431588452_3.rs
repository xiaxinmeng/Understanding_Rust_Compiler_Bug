\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/generator-region-requirements.rs","byte_start":364,"byte_end":365,"line_start":15,"line_end":15,"column_start":51,"column_end":52,"is_primary":true,"text":[{"text":"            GeneratorState::Complete(c) => return c,","highlight_start":51,"highlight_end":52}],"label":"lifetime `'static` required","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add explicit lifetime `'static` to the type of `x`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/generator/generator-region-requirements.rs","byte_start":180,"byte_end":188,"line_start":8,"line_end":8,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"fn dangle(x: &mut i32) -> &'static mut i32 {","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":"&'static mut i32","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in the type of `x`\n  --> /checkout/src/test/ui/generator/generator-region-requirements.rs:15:51\n   |\nLL | fn dangle(x: &mut i32) -> &'static mut i32 {\n   |              -------- help: add explicit lifetime `'static` to the type of `x`: `&'static mut i32`\n...\nLL |             GeneratorState::Complete(c) => return c,\n   |                                                   ^ lifetime `'static` required\n\n"}
[00:47:05] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:05] {"message":"For more information about this error, try `rustc --explain E0621`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0621`.\n"}
[00:47:05] ------------------------------------------
[00:47:05] 
[00:47:05] thread '[ui] ui/generator/generator-region-requirements.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:47:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:05] 
[00:47:05] ---- [ui] ui/nll/user-annotations/cast_static_lifetime.rs stdout ----
[00:47:05] diff of stderr:
[00:47:05] 
[00:47:05] 2   --> $DIR/cast_static_lifetime.rs:16:19
[00:47:05] 3    |
[00:47:05] 4 LL |     let y: &u32 = (&x) as &'static u32;
[00:47:05] -    |                   ^^^^ borrowed value does not live long enough
[00:47:05] +    |                   |
[00:47:05] +    |                   |
[00:47:05] +    |                   borrowed value does not live long enough
[00:47:05] +    |                   type annotation requires that `x` is borrowed for `'static`
[00:47:05] 6 LL | }
[00:47:05] 7    | - `x` dropped here while still borrowed
[00:47:05] -    |
[00:47:05] -    = note: borrowed value must be valid for the static lifetime...
[00:47:05] 11 error: aborting due to previous error
[00:47:05] 12 
[00:47:05] 
[00:47:05] 
[00:47:05] 
[00:47:05] The actual stderr differed from the expected stderr.
[00:47:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/cast_static_lifetime.stderr
[00:47:05] To update references, rerun the tests and pass the `--bless` flag
[00:47:05] To only update this specific test, also pass `--test-args nll/user-annotations/cast_static_lifetime.rs`
[00:47:05] error: 1 errors occurred comparing output.
[00:47:05] status: exit code: 1
[00:47:05] status: exit code: 1
[00:47:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/cast_static_lifetime.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/auxiliary" "-A" "unused"
[00:47:05] ------------------------------------------
[00:47:05] 
[00:47:05] ------------------------------------------
[00:47:05] stderr:
[00:47:05] stderr:
[00:47:05] ------------------------------------------
[00:47:05] {"message":"`x` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n