\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/issue-7575.rs","byte_start":1967,"byte_end":1973,"line_start":82,"line_end":82,"column_start":7,"column_end":13,"is_primary":true,"text":[{"text":"    t.is_str()","highlight_start":7,"highlight_end":13}],"label":"this is an associated function, not a method","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"found the following associated functions; to be used as methods, functions must have a `self` parameter","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the candidate is defined in the trait `ManyImplTrait`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/span/issue-7575.rs","byte_start":1385,"byte_end":1404,"line_start":57,"line_end":57,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    fn is_str() -> bool {","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"to disambiguate the method call, write `ManyImplTrait::is_str(t)` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"items from traits can only be used if the trait is implemented and in scope","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"the following trait defines an item `is_str`, perhaps you need to implement it:\ncandidate #1: `ManyImplTrait`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `is_str` found for type `T` in the current scope\n  --> /checkout/src/test/ui/span/issue-7575.rs:82:7\n   |\nLL |     t.is_str()\n   |       ^^^^^^ this is an associated function, not a method\n   |\n   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter\nnote: the candidate is defined in the trait `ManyImplTrait`\n  --> /checkout/src/test/ui/span/issue-7575.rs:57:5\n   |\nLL |     fn is_str() -> bool {\n   |     ^^^^^^^^^^^^^^^^^^^\n   = help: to disambiguate the method call, write `ManyImplTrait::is_str(t)` instead\n   = help: items from traits can only be used if the trait is implemented and in scope\n   = note: the following trait defines an item `is_str`, perhaps you need to implement it:\n           candidate #1: `ManyImplTrait`\n\n"}
[00:51:58] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:51:58] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] thread '[ui] ui/span/issue-7575.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:51:58] 
[00:51:58] 
[00:51:58] ---- [ui] ui/suggestions/suggest-methods.rs stdout ----
[00:51:58] diff of stderr:
[00:51:58] 
[00:51:58] 7 LL |     f.bat(1.0); //~ ERROR no method named
[00:51:58] 9    |
[00:51:58] -    = help: did you mean `bar`?
[00:51:58] +    = help: did you mean `bag`?
[00:51:58] 11 
[00:51:58] 11 
[00:51:58] 12 error[E0599]: no method named `is_emtpy` found for type `std::string::String` in the current scope
[00:51:58] 13   --> $DIR/suggest-methods.rs:31:15
[00:51:58] 
[00:51:58] The actual stderr differed from the expected stderr.
[00:51:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-methods/suggest-methods.stderr
[00:51:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-methods/suggest-methods.stderr
[00:51:58] To update references, rerun the tests and pass the `--bless` flag
[00:51:58] To only update this specific test, also pass `--test-args suggestions/suggest-methods.rs`
[00:51:58] error: 1 errors occurred comparing output.
[00:51:58] status: exit code: 1
[00:51:58] status: exit code: 1
[00:51:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-methods.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-methods/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-methods/auxiliary" "-A" "unused"
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] ------------------------------------------
[00:51:58] stderr:
[00:51:58] stderr:
[00:51:58] ------------------------------------------
[00:51:58] {"message":"no method named `bat` found for type `Foo` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n