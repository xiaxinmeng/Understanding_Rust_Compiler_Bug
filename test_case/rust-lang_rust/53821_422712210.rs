plain
[00:50:46] ....................................................................................................
[00:50:49] ....................................................................................................
[00:50:52] ....................................................................................................
[00:50:55] .................i..................................................................................
[00:51:00] ...................................................................F................................
[00:51:05] ........F..................i...........i............................................................
[00:51:08] ..............................................iiiii.................................................
[00:51:13] ....................................................................................................
[00:51:15] ....................................................................................................
[00:51:17] ....................................................................................................
[00:51:19] ....................................................................................................
---
[00:56:52] ....................................................................................................
[00:56:55] ....................................................................................................
[00:56:59] ......................................................................................i.............
[00:57:02] ....................................................................................................
column_start":9,"column_end":31,"is_primary":false,"text":[{"text":"        Bar { a: &42 }.b as u8","highlight_start":9,"highlight_end":31}],"label":"a raw memory access tried to access part of a pointer value as raw bytes","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs","byte_start":943,"byte_end":948,"line_start":31,"line_end":31,"column_start":28,"column_end":33,"is_primary":false,"text":[{"text":"    let x: &'static u8 = &(bar() + 1);","highlight_start":28,"highlight_end":33}],"label":"inside call to `bar`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs","byte_start":941,"byte_end":953,"line_start":31,"line_end":31,"column_start":26,"column_end":38,"is_primary":true,"text":[{"text":"    let x: &'static u8 = &(bar() + 1);","highlight_start":26,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs","byte_start":514,"byte_end":523,"line_start":13,"line_end":13,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![deny(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: reaching threquires computing layout of `Foo`...
[00:57:10] 13 note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:57:10] 14 note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}`...
[00:57:10] +   --> $DIR/const-size_of-cycle.rs:16:17
[00:57:10] 16    |
[00:57:10] 16    |
[00:57:10] 17 LL |     bytes: [u8; std::mem::size_of::<Foo>()]
[00:57:10] 
[00:57:10] 
[00:57:10] 19    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}`, completing the cycle
[00:57:10] 20 note: cycle used when processing `Foo`
[00:57:10] +   --> $DIR/const-size_of-cycle.rs:15:1
[00:57:10] 22    |
[00:57:10] 22    |
[00:57:10] 23 LL | struct Foo {
[00:57:10] 
[00:57:10] 
[00:57:10] The actual stderr differed from the expected stderr.
[00:57:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:57:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:57:10] To update references, rerun the tests and pass the `--bless` flag
[00:57:10] To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
[00:57:10] error: 1 errors occurred comparing output.
[00:57:10] status: exit code: 1
[00:57:10] status: exit code: 1
[00:57:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "pre":[{"text":"    bytes: [u8; std::mem::size_of::<Foo>()]","highlight_start":17,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires const-evaluating + checking `Foo::bytes::{{constant}}`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing `Foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/consts/const-size_of-cycle.rs","byte_start":530,"byte_end":540,"line_start":15,"line_end":15,"column_start":1,"column_end":11,"is_primary":true,"text":[{"text":"struct Foo {","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}`\n  --> /checkout/src/test/ui/consts/const-size_of-cycle.rs:16:17\n   |\nLL |     bytes: [u8; std::mem::size_of::<Foo>()]\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: ...which requires const-evaluating `Foo::bytes::{{constant}}`...\n  --> /checkout/src/libcore/mem.rs:290:5\n   |\nLL |     intrinsics::size_of::<T>()\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\nnote: ...which requires computing layout of `Foo`...\nnote: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...\nnote: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}`...\n  --> /checkout/src/test/ui/consts/const-size_of-cycle.rs:16:17\n   |\nLL |     bytes: [u8; std::mem::size_of::<Foo>()]\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}`, completing the cycle\nnote: cycle used when processing `Foo`\n  --> /checkout/src/test/ui/consts/const-size_of-cycle.rs:15:1\n   |\nLL | struct Foo {\n   | ^^^^^^^^^^\n\n"}
[00:57:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:57:10] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:57:10] ------------------------------------------
[00:57:10] 
[00:57:10] thread '[ui] ui/consts/const-size_of-cycle.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:57:10] 
[00:57:10] 
[00:57:10] 
[00:57:10] failures:
[00:57:10]     [ui] ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs
[00:57:10] 
[00:57:10] test result: FAILED. 6771 passed; 2 failed; 28 ignored; 0 measured; 0 filtered out
[00:57:10] 
[00:57:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148532 ./obj/build/bootstrap/debug/incremental
134100 ./obj/build/bootstrap/debug/incremental/bootstrap-14ucxxomeo8gg
134096 ./obj/build/bootstrap/debug/incremental/bootstrap-14ucxxomeo8gg/s-f4xxvpm639-49hmdu-1c0ivx6vup97q
131068 ./obj6_64-unknown-linux-gnu/release/deps
35532 ./.git/modules/src/libcompiler_builtins/modules
35108 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
35036 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
