\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires const-evaluating `Foo::bytes::{{constant}}`...","code":null,"level":"note","spans":[{"file_name":"libcore/mem.rs","byte_start":9563,"byte_end":9589,"line_start":290,"line_end":290,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires computing layout of `Foo`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when const-evaluating `Foo::bytes::{{constant}}`","code":null,"level":"note","spans":[{"file_name":"libcore/mem.rs","byte_start":9563,"byte_end":9589,"line_start":290,"line_end":290,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing layout of `Foo`\n   |\nnote: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...\nnote: ...which requires const-evaluating `Foo::bytes::{{constant}}`...\n   = note: ...which again requires computing layout of `Foo`, completing the cycle\nnote: cycle used when const-evaluating `Foo::bytes::{{constant}}`\n\n"}
[00:46:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:20] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] thread '[ui] ui/consts/const-size_of-cycle.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:20] 
[00:46:20] ---- [ui] ui/impl-trait/impl-generic-mismatch.rs stdout ----
[00:46:20] diff of stderr:
[00:46:20] 
[00:46:20] 29    |
[00:46:20] 30 LL |     fn hash(&self, hasher: &mut impl Hasher) {}
[00:46:20] 31    |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`
[00:46:20] -    | 
[00:46:20] -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
[00:46:20] -    |
[00:46:20] - LL |     fn hash<H: Hasher>(&self, state: &mut H);
[00:46:20] -    |             - declaration in trait here
[00:46:20] 38 error: aborting due to 3 previous errors
[00:46:20] 39 
[00:46:20] 
[00:46:20] 
[00:46:20] 
[00:46:20] The actual stderr differed from the expected stderr.
[00:46:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/impl-generic-mismatch.stderr
[00:46:20] To update references, rerun the tests and pass the `--bless` flag
[00:46:20] To only update this specific test, also pass `--test-args impl-trait/impl-generic-mismatch.rs`
[00:46:20] error: 1 errors occurred comparing output.
[00:46:20] status: exit code: 1
[00:46:20] status: exit code: 1
[00:46:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/auxiliary" "-A" "unused"
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] ------------------------------------------
[00:46:20] stderr:
[00:46:20] stderr:
[00:46:20] ------------------------------------------
[00:46:20] {"message":"method `foo` has incompatible signature for trait","code":{"code":"E0643","explanation":"\nThis error indicates that there is a mismatch between generic parameters and\nimpl Trait parameters in a trait declaration versus its impl.\n\n