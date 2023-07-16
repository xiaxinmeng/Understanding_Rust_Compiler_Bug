\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/auxiliary/cond_plugin.rs","byte_start":1486,"byte_end":1516,"line_start":43,"line_end":43,"column_start":13,"column_end":43,"is_primary":true,"text":[{"text":"            quote!(if $test { $rhs } else)","highlight_start":13,"highlight_end":43}],"label":"could not find `Group` in `{{root}}`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/auxiliary/cond_plugin.rs","byte_start":1486,"byte_end":1516,"line_start":43,"line_end":43,"column_start":13,"column_end":43,"is_primary":false,"text":[{"text":"            quote!(if $test { $rhs } else)","highlight_start":13,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":n--------------
[00:56:32] thread '[run-pass] run-pass/macro-quote-cond.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:56:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:32] 
[00:56:32] ---- [run-pass] run-pass/macro-quote-test.rs stdout ----
[00:56:32] ---- [run-pass] run-pass/macro-quote-test.rs stdout ----
[00:56:32] 
[00:56:32] error: auxiliary build of "/checkout/src/test/run-pass/auxiliary/hello_macro.rs" failed to compile: 
[00:56:32] status: exit code: 1
[00:56:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/auxiliary/hello_macro.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-quote-test/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-quote-test/auxiliary"
[00:56:32] ------------------------------------------
[00:56:32] 
[00:56:32] ------------------------------------------
[00:56:32] stderr:
[00:56:32] stderr:
[00:56:32] ------------------------------------------
[00:56:32] {"message":"failed to resolve: could not find `TokenStream` in `{{root}}`","code":{"code":"E0433","explanation":"\nAn undeclared type or module was used.\n\nErroneous code example:\n\n