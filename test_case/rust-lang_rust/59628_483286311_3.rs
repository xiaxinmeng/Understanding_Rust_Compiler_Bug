\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref.rs","byte_start":64,"byte_end":72,"line_start":4,"line_end":4,"column_start":27,"column_end":35,"is_primary":true,"text":[{"text":"    let _result = &Ok(42).as_deref();","highlight_start":27,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the method `as_deref` exists but the following trait bounds were not satisfied:\n`{integer} : std::ops::Deref`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"there is a method with a similar name","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref.rs","byte_start":64,"byte_end":72,"line_start":4,"line_end":4,"column_start":27,"column_end":35,"is_primary":true,"text":[{"text":"    let _result = &Ok(42).as_deref();","highlight_start":27,"highlight_end":35}],"label":null,"suggested_replacement":"as_ref","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `as_deref` found for type `std::result::Result<{integer}, _>` in the current scope\n  --> /checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref.rs:4:27\n   |\nLL |     let _result = &Ok(42).as_deref();\n   |                           ^^^^^^^^ help: there is a method with a similar name: `as_ref`\n   |\n   = note: the method `as_deref` exists but the following trait bounds were not satisfied:\n           `{integer} : std::ops::Deref`\n\n"}
[01:08:43] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] 
[01:08:43] thread '[ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3469:9
[01:08:43] 
[01:08:43] ---- [ui] ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut.rs stdout ----
[01:08:43] diff of stderr:
[01:08:43] 
[01:08:43] 2   --> $DIR/result-as_deref_mut.rs:4:31
[01:08:43] 3    |
[01:08:43] 4 LL |     let _result = &mut Ok(42).as_deref_mut();
[01:08:43] -    |                               ^^^^^^^^^^^^ help: did you mean: `as_deref_err`
[01:08:43] +    |                               ^^^^^^^^^^^^ help: there is a method with a similar name: `as_deref_err`
[01:08:43] 7    = note: the method `as_deref_mut` exists but the following trait bounds were not satisfied:
[01:08:43] 7    = note: the method `as_deref_mut` exists but the following trait bounds were not satisfied:
[01:08:43] 8            `{integer} : std::ops::DerefMut`
[01:08:43] 
[01:08:43] The actual stderr differed from the expected stderr.
[01:08:43] The actual stderr differed from the expected stderr.
[01:08:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut/result-as_deref_mut.stderr
[01:08:43] To update references, rerun the tests and pass the `--bless` flag
[01:08:43] To only update this specific test, also pass `--test-args issues/issue-50264-inner-deref-trait/result-as_deref_mut.rs`
[01:08:43] error: 1 errors occurred comparing output.
[01:08:43] status: exit code: 1
[01:08:43] status: exit code: 1
[01:08:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50264-inner-deref-trait/result-as_deref_mut/auxiliary" "-A" "unused"
[01:08:43] ------------------------------------------
[01:08:43] 
[01:08:43] ------------------------------------------
[01:08:43] stderr:
[01:08:43] stderr:
[01:08:43] ------------------------------------------
[01:08:43] {"message":"no method named `as_deref_mut` found for type `std::result::Result<{integer}, _>` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n