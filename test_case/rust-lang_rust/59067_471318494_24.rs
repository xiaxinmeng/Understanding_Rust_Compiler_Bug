\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-move-in-irrefut-pat.rs","byte_start":393,"byte_end":410,"line_start":17,"line_end":17,"column_start":15,"column_end":32,"is_primary":true,"text":[{"text":"    let &_x = &\"hi\".to_string();","highlight_start":15,"highlight_end":32}],"label":"cannot move out of borrowed content","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-move-in-irrefut-pat.rs","byte_start":388,"byte_end":390,"line_start":17,"line_end":17,"column_start":10,"column_end":12,"is_primary":false,"text":[{"text":"    let &_x = &\"hi\".to_string();","highlight_start":10,"highlight_end":12}],"label":"data moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `_x` has type `std::string::String`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-move-in-irrefut-pat.rs","byte_start":388,"byte_end":390,"line_start":17,"line_end":17,"column_start":10,"column_end":12,"is_primary":true,"text":[{"text":"    let &_x = &\"hi\".to_string();","highlight_start":10,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider removing the `&`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-move-in-irrefut-pat.rs","byte_start":387,"byte_end":390,"line_start":17,"line_end":17,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"    let &_x = &\"hi\".to_string();","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":"_x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/borrowck/borrowck-move-in-irrefut-pat.rs:17:15\n   |\nLL |     let &_x = &\"hi\".to_string();\n   |         ---   ^^^^^^^^^^^^^^^^^ cannot move out of borrowed content\n   |         ||\n   |         |data moved here\n   |         help: consider removing the `&`: `_x`\n   |\nnote: move occurs because `_x` has type `std::string::String`, which does not implement the `Copy` trait\n  --> /checkout/src/test/ui/borrowck/borrowck-move-in-irrefut-pat.rs:17:10\n   |\nLL |     let &_x = &\"hi\".to_string();\n   |          ^^\n\n"}
[01:38:31] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[01:38:31] 
[01:38:31] ------------------------------------------
[01:38:31] 
[01:38:31] 
[01:38:31] thread '[ui (nll)] ui/borrowck/borrowck-move-in-irrefut-pat.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3319:9
[01:38:31] 
[01:38:31] ---- [ui (nll)] ui/borrowck/borrowck-report-with-custom-diagnostic.rs stdout ----
[01:38:31] diff of stderr:
[01:38:31] 
[01:38:31] 22 LL |             y.use_ref();
[01:38:31] 23    |             - immutable borrow later used here
[01:38:31] 24 
[01:38:31] - error[E0499]: cannot borrow `x` as mutable more than once at a time
[01:38:31] + warning[E0499]: cannot borrow `x` as mutable more than once at a time
[01:38:31] 27    |
[01:38:31] 28 LL |         let y = &mut x;
[01:38:31] 
[01:38:31] 33 ...
[01:38:31] 33 ...
[01:38:31] 34 LL |         y.use_mut();
[01:38:31] +    |
[01:38:31] +    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:38:31] +    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:38:31] 36 
---
[01:38:31] 40 For more information about an error, try `rustc --explain E0499`.
[01:38:31] 
[01:38:31] 
[01:38:31] The actual stderr differed from the expected stderr.
[01:38:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-report-with-custom-diagnostic.nll/borrowck-report-with-custom-diagnostic.nll.stderr
[01:38:31] To update references, rerun the tests and pass the `--bless` flag
[01:38:31] To only update this specific test, also pass `--test-args borrowck/borrowck-report-with-custom-diagnostic.rs`
[01:38:31] error: 1 errors occurred comparing output.
[01:38:31] status: exit code: 1
[01:38:31] status: exit code: 1
[01:38:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-report-with-custom-diagnostic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-report-with-custom-diagnostic.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-report-with-custom-diagnostic.nll/auxiliary" "-A" "unused"
[01:38:31] ------------------------------------------
[01:38:31] 
[01:38:31] ------------------------------------------
[01:38:31] stderr:
[01:38:31] stderr:
[01:38:31] ------------------------------------------
[01:38:31] {"message":"cannot borrow `x` as immutable because it is also borrowed as mutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n