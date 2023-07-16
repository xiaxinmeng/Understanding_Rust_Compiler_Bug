\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err.rs","byte_start":65,"byte_end":77,"line_start":4,"line_end":4,"column_start":28,"column_end":40,"is_primary":true,"text":[{"text":"    let _result = &Err(41).as_deref_err();","highlight_start":28,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the method `as_deref_err` exists but the following trait bounds were not satisfied:\n`{integer} : std::ops::Deref`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"there is a method with a similar name","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err.rs","byte_start":65,"byte_end":77,"line_start":4,"line_end":4,"column_start":28,"column_end":40,"is_primary":true,"text":[{"text":"    let _result = &Err(41).as_deref_err();","highlight_start":28,"highlight_end":40}],"label":null,"suggested_replacement":"as_deref_ok","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `as_deref_err` found for type `std::result::Result<_, {integer}>` in the current scope\n  --> /checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_err.rs:4:28\n   |\nLL |     let _result = &Err(41).as_deref_err();\n   |                            ^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_ok`\n   |\n   = note: the method `as_deref_err` exists but the following trait bounds were not satisfied:\n           `{integer} : std::ops::Deref`\n\n"}
[01:08:43] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] 
[01:08:43] thread '[ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_err.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3469:9
[01:08:43] 
[01:08:43] ---- [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref.rs stdout ----
[01:08:43] diff of stderr:
[01:08:43] 
[01:08:43] 2   --> $DIR/result-as_deref.rs:4:27
[01:08:43] 3    |
[01:08:43] 4 LL |     let _result = &Ok(42).as_deref();
[01:08:43] -    |                           ^^^^^^^^ help: did you mean: `as_ref`
[01:08:43] 6    |
[01:08:43] 7    = note: the method `as_deref` exists but the following trait bounds were not satisfied:
[01:08:43] 7    = note: the method `as_deref` exists but the following trait bounds were not satisfied:
[01:08:43] 8            `{integer} : std::ops::Deref`
[01:08:43] 
[01:08:43] The actual stderr differed from the expected stderr.
[01:08:43] The actual stderr differed from the expected stderr.
[01:08:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref/result-as_deref.stderr
[01:08:43] To update references, rerun the tests and pass the `--bless` flag
[01:08:43] To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref.rs`
[01:08:43] error: 1 errors occurred comparing output.
[01:08:43] status: exit code: 1
[01:08:43] status: exit code: 1
[01:08:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref/auxiliary" "-A" "unused"
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] stderr:
[01:08:43] stderr:
[01:08:43] ------------------------------------------
[01:08:43] {"message":"no method named `as_deref` found for type `std::result::Result<{integer}, _>` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n