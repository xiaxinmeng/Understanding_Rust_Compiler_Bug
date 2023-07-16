\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs","byte_start":2783,"byte_end":2790,"line_start":85,"line_end":85,"column_start":74,"column_end":81,"is_primary":true,"text":[{"text":"    std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method3();","highlight_start":74,"highlight_end":81}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0599]: no method named `method3` found for type `std::rc::Rc<&mut std::boxed::Box<&no_method_suggested_traits::Bar>>` in the current scope\n  --> /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:85:74\n   |\nLL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method3();\n   |                                                                          ^^^^^^^\n\n"}
[00:51:58] {"message":"aborting due to 24 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 24 previous errors\n\n"}
[00:51:58] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] thread '[ui] ui/impl-trait/no-method-suggested-traits.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:51:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:58] 
[00:51:58] ---- [ui] ui/span/issue-7575.rs stdout ----
[00:51:58] diff of stderr:
[00:51:58] 
[00:51:58] 28            candidate #1: `CtxtFn`
[00:51:58] 29            candidate #2: `OtherTrait`
[00:51:58] 30            candidate #3: `UnusedTrait`
[00:51:58] +    = help: did you mean `f8`?
[00:51:58] 31 
[00:51:58] 32 error[E0599]: no method named `fff` found for type `Myisize` in the current scope
[00:51:58] 
[00:51:58] 
[00:51:58] The actual stderr differed from the expected stderr.
[00:51:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575/issue-7575.stderr
[00:51:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575/issue-7575.stderr
[00:51:58] To update references, rerun the tests and pass the `--bless` flag
[00:51:58] To only update this specific test, also pass `--test-args span/issue-7575.rs`
[00:51:58] error: 1 errors occurred comparing output.
[00:51:58] status: exit code: 1
[00:51:58] status: exit code: 1
[00:51:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-7575.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575/auxiliary" "-A" "unused"
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] ------------------------------------------
[00:51:58] stderr:
[00:51:58] stderr:
[00:51:58] ------------------------------------------
[00:51:58] {"message":"no method named `f9` found for type `usize` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n