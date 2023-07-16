compile_fail,E0008\nm,"line_start":33,"line_end":33,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"        A { a: v } if *v == 42 => v,","highlight_start":16,"highlight_end":17}],"label":"moves value into pattern guard","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0008]: cannot bind by-move into a pattern guard\n  --> /checkout/src/test/ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.rs:33:16\n   |\nLL |         A { a: v } if *v == 42 => v,\n   |                ^ moves value into pattern guard\n\n"}
[00:51:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:16] {"message":"For more information about this error, try `rustc --explain E0008`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0008`.\n"}
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] 
[00:51:16] thread '[ui] ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.rs#no_gate' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:51:16] 
[00:51:16] ---- [ui] ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms.rs stdout ----
[00:51:16] 
[00:51:16] 1 error[E0507]: cannot move out of borrowed content
[00:51:16] 1 error[E0507]: cannot move out of borrowed content
[00:51:16] -   --> $DIR/rfc-reject-double-move-across-arms.rs:18:36
[00:51:16] +   --> $DIR/rfc-reject-double-move-across-arms.rs:8:36
[00:51:16] 3    |
[00:51:16] 4 LL |         VecWrapper::A(v) if { drop(v); false } => 1,
[00:51:16] 5    |                                    ^ cannot move out of borrowed content
[00:51:16] 
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms/rfc-reject-double-move-across-arms.stderr
[00:51:16] To update references, rerun the tests and pass the `--bless` flag
[00:51:16] To only update this specific test, also pass `--test-args rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms.rs`
[00:51:16] error: 1 errors occurred comparing output.
[00:51:16] status: exit code: 1
[00:51:16] status: exit code: 1
[00:51:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms/auxiliary" "-A" "unused"
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] ------------------------------------------
[00:51:16] stderr:
[00:51:16] stderr:
[00:51:16] ------------------------------------------
[00:51:16] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n