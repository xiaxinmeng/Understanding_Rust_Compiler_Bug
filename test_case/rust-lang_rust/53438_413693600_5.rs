\n\nNote: This error is usually hidden by E0301 or E0302. This may change in the\nfuture.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-1.rs","byte_start":1245,"byte_end":1247,"line_start":33,"line_end":33,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"            (|| { *x = None; drop(force_fn_once); })();","highlight_start":14,"highlight_end":16}],"label":"cannot mutably borrow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-1.rs","byte_start":1105,"byte_end":1106,"line_start":29,"line_end":29,"column_start":11,"column_end":12,"is_primary":false,"text":[{"text":"    match x {","highlight_start":11,"highlight_end":12}],"label":"value is immutable in match guard","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-1.rs","byte_start":1251,"byte_end":1252,"line_start":33,"line_end":33,"column_start":20,"column_end":21,"is_primary":false,"text":[{"text":"            (|| { *x = None; drop(force_fn_once); })();","highlight_start":20,"highlight_end":21}],"label":"borrow occurs due to use of `x` in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0510]: cannot mutably borrow `x` in match guard\n  --> /checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-1.rs:33:14\n   |\nLL |     match x {\n   |           - value is immutable in match guard\n...\nLL |             (|| { *x = None; drop(force_fn_once); })();\n   |              ^^    - borrow occurs due to use of `x` in closure\n   |              |\n   |              cannot mutably borrow\n\n"}
[00:48:18] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:18] {"message":"For more information about this error, try `rustc --explain E0510`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0510`.\n"}
[00:48:18] ------------------------------------------
[00:48:18] 
[00:48:18] thread '[ui] ui/issues/issue-27282-mutate-before-diverging-arm-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:48:18] 
[00:48:18] 
[00:48:18] ---- [ui] ui/issues/issue-27282-mutate-before-diverging-arm-2.rs stdout ----
[00:48:18] diff of stderr:
[00:48:18] 
[00:48:18] 2   --> $DIR/issue-27282-mutate-before-diverging-arm-2.rs:38:18
[00:48:18] 3    |
[00:48:18] 4 LL |     match x {
[00:48:18] -    |           |
[00:48:18] -    |           value is immutable due to match
[00:48:18] -    |           borrow later used here
[00:48:18] -    |           borrow later used here
[00:48:18] +    |           - value is immutable in match guard
[00:48:18] 9 ...
[00:48:18] 10 LL |                 (|| { *x = None; drop(force_fn_once); })();
[00:48:18] 11    |                  ^^    - borrow occurs due to use of `x` in closure
[00:48:18] 
[00:48:18] The actual stderr differed from the expected stderr.
[00:48:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27282-mutate-before-diverging-arm-2/issue-27282-mutate-before-diverging-arm-2.stderr
[00:48:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27282-mutate-before-diverging-arm-2/issue-27282-mutate-before-diverging-arm-2.stderr
[00:48:18] To update references, rerun the tests and pass the `--bless` flag
[00:48:18] To only update this specific test, also pass `--test-args issues/issue-27282-mutate-before-diverging-arm-2.rs`
[00:48:18] error: 1 errors occurred comparing output.
[00:48:18] status: exit code: 1
[00:48:18] status: exit code: 1
[00:48:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27282-mutate-before-diverging-arm-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27282-mutate-before-diverging-arm-2/auxiliary" "-A" "unused"
[00:48:18] ------------------------------------------
[00:48:18] 
[00:48:18] ------------------------------------------
[00:48:18] stderr:
[00:48:18] stderr:
[00:48:18] ------------------------------------------
[00:48:18] {"message":"cannot mutably borrow `x` in match guard","code":{"code":"E0510","explanation":"\nCannot mutate place in this match guard.\n\nWhen matching on a variable it cannot be mutated in the match guards, as this\ncould cause the match to be non-exhaustive:\n\n