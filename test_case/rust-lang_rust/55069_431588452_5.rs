\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/cast_static_lifetime.rs","byte_start":555,"byte_end":559,"line_start":16,"line_end":16,"column_start":19,"column_end":23,"is_primary":true,"text":[{"text":"    let y: &u32 = (&x) as &'static u32;","highlight_start":19,"highlight_end":23}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/cast_static_lifetime.rs","byte_start":577,"byte_end":578,"line_start":17,"line_end":17,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`x` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/cast_static_lifetime.rs","byte_start":555,"byte_end":575,"line_start":16,"line_end":16,"column_start":19,"column_end":39,"is_primary":false,"text":[{"text":"    let y: &u32 = (&x) as &'static u32;","highlight_start":19,"highlight_end":39}],"label":"type annotation requires that `x` is borrowed for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `x` does not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/cast_static_lifetime.rs:16:19\n   |\nLL |     let y: &u32 = (&x) as &'static u32;\n   |                   ^^^^----------------\n   |                   |\n   |                   borrowed value does not live long enough\n   |                   type annotation requires that `x` is borrowed for `'static`\nLL | }\n   | - `x` dropped here while still borrowed\n\n"}
[00:47:05] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:05] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:47:05] ------------------------------------------
[00:47:05] 
[00:47:05] thread '[ui] ui/nll/user-annotations/cast_static_lifetime.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:47:05] 
[00:47:05] 
[00:47:05] ---- [ui] ui/nll/user-annotations/normalization.rs stdout ----
[00:47:05] diff of stderr:
[00:47:05] 
[00:47:05] 2   --> $DIR/normalization.rs:12:31
[00:47:05] 3    |
[00:47:05] 4 LL |     let b: <() as Foo>::Out = &a; //~ ERROR
[00:47:05] -    |                               ^^ borrowed value does not live long enough
[00:47:05] +    |            ----------------   ^^ borrowed value does not live long enough
[00:47:05] +    |            |
[00:47:05] +    |            type annotation requires that `a` is borrowed for `'static`
[00:47:05] 6 LL | }
[00:47:05] 7    | - `a` dropped here while still borrowed
[00:47:05] -    |
[00:47:05] -    = note: borrowed value must be valid for the static lifetime...
[00:47:05] 11 error: aborting due to previous error
[00:47:05] 12 
[00:47:05] 
[00:47:05] 
[00:47:05] 
[00:47:05] The actual stderr differed from the expected stderr.
[00:47:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/normalization/normalization.stderr
[00:47:05] To update references, rerun the tests and pass the `--bless` flag
[00:47:05] To only update this specific test, also pass `--test-args nll/user-annotations/normalization.rs`
[00:47:05] error: 1 errors occurred comparing output.
[00:47:05] status: exit code: 1
[00:47:05] status: exit code: 1
[00:47:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/normalization.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/normalization/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/normalization/auxiliary" "-A" "unused"
[00:47:05] ------------------------------------------
[00:47:05] 
[00:47:05] ------------------------------------------
[00:47:05] stderr:
[00:47:05] stderr:
[00:47:05] ------------------------------------------
[00:47:05] {"message":"`a` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n