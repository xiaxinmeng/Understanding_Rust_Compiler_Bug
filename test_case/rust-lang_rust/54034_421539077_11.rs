\nuse std::cell::RefCell;\n\n#[derive(Clone, Copy)] // we implement the Copy trait\nstruct TheDarkKnight;\n\nimpl TheDarkKnight {\n    fn nothing_is_true(self) {}\n}\n\nfn mfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms.rs:8:36\n   |\nLL |         VecWrapper::A(v) if { drop(v); false } => 1,\n   |                                    ^ cannot move out of borrowed content\n\n"}
[00:55:50] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:55:50] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[00:55:50] ------------------------------------------
[00:55:50] 
[00:55:50] 
[00:55:50] thread '[ui] ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:55:50] 
[00:55:50] ---- [ui] ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm.rs stdout ----
[00:55:50] 
[00:55:50] 1 error[E0507]: cannot move out of borrowed content
[00:55:50] 1 error[E0507]: cannot move out of borrowed content
[00:55:50] -   --> $DIR/rfc-reject-double-move-in-first-arm.rs:19:30
[00:55:50] +   --> $DIR/rfc-reject-double-move-in-first-arm.rs:9:30
[00:55:50] 3    |
[00:55:50] 4 LL |         A { a: v } if { drop(v); true } => v,
[00:55:50] 5    |                              ^ cannot move out of borrowed content
[00:55:50] 
[00:55:50] The actual stderr differed from the expected stderr.
[00:55:50] The actual stderr differed from the expected stderr.
[00:55:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm/rfc-reject-double-move-in-first-arm.stderr
[00:55:50] To update references, rerun the tests and pass the `--bless` flag
[00:55:50] To only update this specific test, also pass `--test-args rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm.rs`
[00:55:50] error: 1 errors occurred comparing output.
[00:55:50] status: exit code: 1
[00:55:50] status: exit code: 1
[00:55:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm/auxiliary" "-A" "unused"
[00:55:50] ------------------------------------------
[00:55:50] 
[00:55:50] ------------------------------------------
[00:55:50] stderr:
[00:55:50] stderr:
[00:55:50] ------------------------------------------
[00:55:50] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n