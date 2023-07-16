\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0509.rs","byte_start":700,"byte_end":717,"line_start":26,"line_end":26,"column_start":23,"column_end":40,"is_primary":true,"text":[{"text":"    let fancy_field = drop_struct.fancy; //~ ERROR E0509","highlplicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:52:46] ---- [ui (nll)] ui/issues/issue-17718-static-move.rs stdout ----
[00:52:46] diff of stderr:
[00:52:46] 
[00:52:46] 5    |              ^^^
---
[00:52:46] 11 
[00:52:46] 
[00:52:46] 
[00:52:46] The actual stderr differed from the expected stderr.
[00:52:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-static-move.nll/issue-17718-static-move.nll.stderr
[00:52:46] To update references, rerun the tests and pass the `--bless` flag
[00:52:46] To only update this specific test, also pass `--test-args issues/issue-17718-static-move.rs`
[00:52:46] error: 1 errors occurred comparing output.
[00:52:46] status: exit code: 1
[00:52:46] status: exit code: 1
[00:52:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17718-static-move.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-static-move.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-static-move.nll/auxiliary" "-A" "unused"
[00:52:46] ------------------------------------------
[00:52:46] 
[00:52:46] ------------------------------------------
[00:52:46] stderr:
[00:52:46] stderr:
[00:52:46] ------------------------------------------
[00:52:46] {"message":"cannot move out of static item","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n