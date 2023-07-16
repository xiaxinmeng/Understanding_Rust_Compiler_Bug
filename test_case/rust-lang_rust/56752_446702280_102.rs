\n\n### The trait cannot use `Self` as a type parameter in the supertrait listis(), tts.to_vec());","highlight_start":1,"highlight_end":80},{"text":"    let expr = parser.parse_expr().unwrap();","highlight_start":1,"highlight_end":45},{"text":"    MacEager::expr(quote_expr!(&mut *cx, $expr))","highlight_start":1,"highlight_end":49},{"text":"}","highlight_start":1,"highlight_end":2}],"label":"the trait `syntax::ext::base::MacResult` cannot be made into an object","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait cannot require that `Self : Sized`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0038]: the trait `syntax::ext::base::MacResult` cannot be made into an object\n  --> /checkout/src/test/ui-fulldeps/auxiliary/macro_crate_test.rs:57:1\n   |\nLL | / fn expand_identity(cx: &mut ExtCtxt, _span: Span, tts: &[TokenTree])\nLL | |                    -> Box<MacResult+'static> {\nLL | |     // Parse an expression and emit it unchanged.\nLL | |     let mut parser = parse::new_parser_from_tts(cx.parse_sess(), tts.to_vec());\nLL | |     let expr = parser.parse_expr().unwrap();\nLL | |     MacEager::expr(quote_expr!(&mut *cx, $expr))\nLL | | }\n   | |_^ the trait `syntax::ext::base::MacResult` cannot be made into an object\n   |\n   = note: the trait cannot require that `Self : Sized`\n\n"}
[00:57:27] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:57:27] {"message":"For more information about this error, try `rustc --explain E0038`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0038`.\n"}
[00:57:27] ------------------------------------------
[00:57:27] 
[00:57:27] thread '[ui] ui-fulldeps/plugin-plus-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:57:27] 
---
[00:57:27] test result: FAILED. 19 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out
[00:57:27] 
[00:57:27] 
[00:57:27] 
[00:57:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:27] 
[00:57:27] 
[00:57:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:27] Build completed unsuccessfully in 0:11:20
[00:57:27] Build completed unsuccessfully in 0:11:20
[00:57:27] Makefile:58: recipe for target 'check' failed
[00:57:27] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:163ad282
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 12 18:57:15 UTC 2018
