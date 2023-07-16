\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-issue-14498.rs","byte_start":2103,"byte_end":2105,"line_start":70,"line_end":70,"column_start":13,"column_end":15,"is_primary":false,"text":[{"text":"    let p = &y;","highlight_start":13,"highlight_end":15}],"label":"borrow of `y**` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-issue-14498.rs","byte_start":2130,"byte_end":2137,"line_start":72,"line_end":72,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    **y = 2; //[ast]~ ERROR cannot assign to `**y` because it is borrowed","highlight_start":5,"highlight_end":12}],"label":"assignment to borrowed `y**` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-issue-14498.rs","byte_start":2284,"byte_end":2285,"line_start":74,"line_end":74,"column_start":10,"column_end":11,"is_primary":false,"text":[{"text":"    drop(p);","highlight_start":10,"highlight_end":11}],"label":"borrow later used here",uard
[00:48:10] +    |                 cannot move out of `foo*`, as it is immutable for the pattern guard
[00:48:10] 19    |
[00:48:10] 19    |
[00:48:10] 20    = note: variables bound in patterns are immutable until the end of the pattern guard
[00:48:10] 
[00:48:10] The actual stderr differed from the expected stderr.
[00:48:10] The actual stderr differed from the expected stderr.
[00:48:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/borrowck-migrate-to-nll.edition.stderr
[00:48:10] To update references, rerun the tests and pass the `--bless` flag
[00:48:10] To only update this specific test, also pass `--test-args borrowck/borrowck-migrate-to-nll.rs`
[00:48:10] 
[00:48:10] error in revision `edition`: 1 errors occurred comparing output.
[00:48:10] status: exit code: 0
[00:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "edition" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/auxiliary" "-A" "unused"
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] stderr:
[00:48:10] stderr:
[00:48:10] ------------------------------------------
[00:48:10] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n