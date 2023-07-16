\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-27282-move-match-input-into-guard.rs","byte_start":980,"byte_end":982,"line_start":26,"line_end":26,"column_start":17,"column_end":19,"is_primary":false,"text":[{"text":"        _ if { (|| { let bar = b; *bar = false; })();","highlight_start":17,"highlight_end":19}],"label":"value moved into closure here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-27282-move-match-input-into-guard.rs","byte_start":995,"byte_end":996,"line_start":26,"line_end":26,"column_start":32,"column_end":33,"is_primary":false,"text":[{"text":"        _ if { (|| { let bar = b; *bar = false; })();","highlight_start":32,"highlight_end":33}],"label":"variable moved due to use in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-27282-move-match-input-into-guard.rs","byte_start":1068,"byte_end":1072,"line_start":28,"line_end":28,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"        &mut true => { println!(\"You might think we should get here\"); },","highlight_start":14,"highlight_end":18}],"label":"value used here after move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0382]: use of moved value: `*b`\n  --> /checkout/src/test/ui/issues/issue-27282-move-match-input-into-guard.rs:28:14\n   |\nLL |         _ if { (|| { let bar = b; *bar = false; })();\n   |                 --             - variable moved due to use in closure\n   |                 |\n   |                 value moved into closure here\nLL |                      false } => { },\nLL |         &mut true => { println!(\"You might think we should get here\"); },\n   |              ^^^^ value used here after move\n\n"}
[00:48:18] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:18] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[00:48:18] ------------------------------------------
[00:48:18] 
[00:48:18] thread '[ui] ui/issues/issue-27282-move-match-input-into-guard.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:48:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:18] 
[00:48:18] ---- [ui] ui/issues/issue-27282-mutate-before-diverging-arm-1.rs stdout ----
[00:48:18] diff of stderr:
[00:48:18] 
[00:48:18] 2   --> $DIR/issue-27282-mutate-before-diverging-arm-1.rs:33:14
[00:48:18] 3    |
[00:48:18] 4 LL |     match x {
[00:48:18] -    |           |
[00:48:18] -    |           value is immutable due to match
[00:48:18] -    |           borrow later used here
[00:48:18] -    |           borrow later used here
[00:48:18] +    |           - value is immutable in match guard
[00:48:18] 9 ...
[00:48:18] 10 LL |             (|| { *x = None; drop(force_fn_once); })();
[00:48:18] 11    |              ^^    - borrow occurs due to use of `x` in closure
[00:48:18] 
[00:48:18] The actual stderr differed from the expected stderr.
[00:48:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27282-mutate-before-diverging-arm-1/issue-27282-mutate-before-diverging-arm-1.stderr
[00:48:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27282-mutate-before-diverging-arm-1/issue-27282-mutate-before-diverging-arm-1.stderr
[00:48:18] To update references, rerun the tests and pass the `--bless` flag
[00:48:18] To only update this specific test, also pass `--test-args issues/issue-27282-mutate-before-diverging-arm-1.rs`
[00:48:18] error: 1 errors occurred comparing output.
[00:48:18] status: exit code: 1
[00:48:18] status: exit code: 1
[00:48:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27282-mutate-before-diverging-arm-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27282-mutate-before-diverging-arm-1/auxiliary" "-A" "unused"
[00:48:18] ------------------------------------------
[00:48:18] 
[00:48:18] ------------------------------------------
[00:48:18] stderr:
[00:48:18] stderr:
[00:48:18] ------------------------------------------
[00:48:18] {"message":"cannot mutably borrow `x` in match guard","code":{"code":"E0510","explanation":"\nCannot mutate place in this match guard.\n\nWhen matching on a variable it cannot be mutated in the match guards, as this\ncould cause the match to be non-exhaustive:\n\n