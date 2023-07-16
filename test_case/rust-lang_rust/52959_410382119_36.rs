\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/stable/book/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/borrowed-match-issue-45045.rs","byte_start":598,"byte_end":604,"line_start":22,"line_end":22,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let f = &mut e;","highlight_start":13,"highlight_end":19}],"label":"borrow of `e` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/borrowed-match-issue-45045.rs","byte_start":702,"byte_end":708,"line_start":25,"line_end":25,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        Xyz::A => println!(\"a\"),","highlight_start":9,"highlight_end":15}],"label":"use of borrowed `e`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/borrowed-match-issue-45045.rs","byte_start":839,"byte_end":850,"line_start":29,"line_end":29,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    *g = Xyz::B;","highlight_start":5,"highlight_end":16}],"label":"borrow used here in later iteration of loop","suggested_replacement":null,"suggeactual stderr differed from the expected stderr.
[00:43:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-local-error/borrowed-local-error.stderr
[00:43:19] To update references, rerun the tests and pass the `--bless` flag
[00:43:19] To only update this specific test, also pass `--test-args nll/borrowed-local-error.rs`
[00:43:19] error: 1 errors occurred comparing output.
[00:43:19] status: exit code: 1
[00:43:19] status: exit code: 1
[00:43:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/borrowed-local-error.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-local-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-local-error/auxiliary" "-A" "unused"
[00:43:19] ------------------------------------------
[00:43:19] 
[00:43:19] ------------------------------------------
[00:43:19] stderr:
[00:43:19] stderr:
[00:43:19] ------------------------------------------
[00:43:19] {"message":"`v` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n