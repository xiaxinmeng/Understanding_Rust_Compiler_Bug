\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":530,"byte_end":556,"line_start":14,"line_end":14,"column_start":17,"column_end":43,"is_primary":true,"text":[{"text":"    bytes: [u8; std::mem::size_of::<Foo>()]","highlight_start":17,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires const-evaluating `Foo::bytes::{{constant}}`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/libcore/mem.rs","byte_start":9563,"byte_end":9589,"line_start":290,"line_end":290,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    intrinsics::size_of::<T>()","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires computing layout of `Foo`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...","code":null,"leveed when const-evaluating + checking `Foo::bytes::{{constant}}`\n  --> /checkout/src/test/ui/consts/const-size_of-cycle.rs:14:17\n   |\nLL |     bytes: [u8; std::mem::size_of::<Foo>()]\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: ...which requires const-evaluating `Foo::bytes::{{constant}}`...\n  --> /checkout/src/libcore/mem.rs:290:5\n   |\nLL |     intrinsics::size_of::<T>()\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\nnote: ...which requires computing layout of `Foo`...\nnote: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...\nnote: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}`...\n  --> /checkout/src/test/ui/consts/const-size_of-cycle.rs:14:17\n   |\nLL |     bytes: [u8; std::mem::size_of::<Foo>()]\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}`, completing the cycle\nnote: cycle used when processing `Foo`\n  --> /checkout/src/test/ui/consts/const-size_of-cycle.rs:13:1\n   |\nLL | struct Foo {\n   | ^^^^^^^^^^\n\n"}
[00:46:00] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:00] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:46:00] ------------------------------------------
[00:46:00] 
[00:46:00] thread '[ui] ui/consts/const-size_of-cycle.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:00] 
[00:46:00] 
[00:46:00] ---- [ui] ui/issues/issue-44415.rs stdout ----
[00:46:00] diff of stderr:
[00:46:00] 
[00:46:00] 1 error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}`
[00:46:00] +   --> $DIR/issue-44415.rs:16:17
[00:46:00] 3    |
[00:46:00] 3    |
[00:46:00] 4 LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
[00:46:00] 
[00:46:00] 6    |
[00:46:00] 6    |
[00:46:00] 7 note: ...which requires const-evaluating `Foo::bytes::{{constant}}`...
[00:46:00] +   --> $DIR/issue-44415.rs:16:26
[00:46:00] 9    |
[00:46:00] 9    |
[00:46:00] 10 LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
[00:46:00] 
[00:46:00] 
[00:46:00] 12 note: ...which requires computing layout of `Foo`...
[00:46:00] 13 note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:46:00] 14 note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}`...
[00:46:00] +   --> $DIR/issue-44415.rs:16:17
[00:46:00] 16    |
[00:46:00] 16    |
[00:46:00] 17 LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
[00:46:00] 
[00:46:00] 
[00:46:00] 19    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}`, completing the cycle
[00:46:00] 20 note: cycle used when processing `Foo`
[00:46:00] +   --> $DIR/issue-44415.rs:15:1
[00:46:00] 22    |
[00:46:00] 22    |
[00:46:00] 23 LL | struct Foo {
[00:46:00] 
[00:46:00] 
[00:46:00] The actual stderr differed from the expected stderr.
[00:46:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/issue-44415.stderr
[00:46:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/issue-44415.stderr
[00:46:00] To update references, rerun the tests and pass the `--bless` flag
[00:46:00] To only update this specific test, also pass `--test-args issues/issue-44415.rs`
[00:46:00] error: 1 errors occurred comparing output.
[00:46:00] status: exit code: 1
[00:46:00] status: exit code: 1
[00:46:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44415.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/auxiliary" "-A" "unused"
[00:46:00] ------------------------------------------
[00:46:00] 
[00:46:00] ------------------------------------------
[00:46:00] stderr:
[00:46:00] stderr:
[00:46:00] ------------------------------------------
[00:46:00] {"message":"cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be construc{"message":"...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-44415.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2018 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires const-evaluating + checking `Foo::bytes::{{constant}}`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-44415.rs","byte_start":548,"byte_end":554,"line_start":16,"line_end":16,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires const-evaluating + checking `Foo::bytes::{{constant}}`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing `Foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-44415.rs","byte_start":519,"byte_end":529,"line_start":15,"line_end":15,"column_start":1,"column_end":11,"is_primary":true,"text":[{"text":"struct Foo {","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}`\n  --> /checkout/src/test/ui/issues/issue-44415.rs:16:17\n   |\nLL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],\n   |                 ^^^^^^\n   |\nnote: ...which requires const-evaluating `Foo::bytes::{{constant}}`...\n  --> /checkout/src/test/ui/issues/issue-44415.rs:16:26\n   |\nLL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nnote: ...which requires computing layout of `Foo`...\nnote: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...\nnote: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}`...\n  --> /checkout/src/test/ui/issues/issue-44415.rs:16:17\n   |\nLL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],\n   |                 ^^^^^^\n   = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}`, completing the cycle\nnote: cycle used when processing `Foo`\n  --> /checkout/src/test/ui/issues/issue-44415.rs:15:1\n   |\nLL | struct Foo {\n   | ^^^^^^^^^^\n\n"}
[00:46:00] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:00] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:46:00] ------------------------------------------
[00:46:00] 
[00:46:00] thread '[ui] ui/issues/issue-44415.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:00] 
[00:46:00] 
[00:46:00] ---- [ui] ui/issues/issue-52023-array-size-pointer-cast.rs stdout ----
[00:46:00] diff of stderr:
[00:46:00] 
[00:46:00] 6    |
[00:46:00] 7    = help: add #![feature(const_raw_ptr_to_usize_cast)] to the crate attributes to enable
[00:46:00] 8 
[00:46:00] - error: aborting due to previous error
[00:46:00] + error[E0080]: it is undefined behavior to use this value
[00:46:00] +   --> $DIR/issue-52023-array-size-pointer-cast.rs:12:17
[00:46:00] +    |
[00:46:00] + LL |     let _ = [0; (&0 as *const i32) as usize]; //~ ERROR casting pointers to integers in constants
[00:46:00] +    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected the type usize
[00:46:00] +    |
[00:46:00] +    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:46:00] - For more information about this error, try `rustc --explain E0658`.
[00:46:00] + error: aborting due to 2 previous errors
[00:46:00] + 
[00:46:00] + Some errors occurred: E0080, E0658.
[00:46:00] + Some errors occurred: E0080, E0658.
[00:46:00] + For more information about an error, try `rustc --explain E0080`.
[00:46:00] 12 
[00:46:00] 
[00:46:00] 
[00:46:00] The actual stderr differed from the expected stderr.
[00:d. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n