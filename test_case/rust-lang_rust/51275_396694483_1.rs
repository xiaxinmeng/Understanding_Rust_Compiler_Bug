\n\nIn here, `x` isn't mutable, so light_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider changing this to be mutable","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-31424.rs","byte_start":758,"byte_end":762,"line_start":22,"line_end":22,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    fn bar(self: &mut Self) {","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":"mut self","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable item `self` as mutable\n  --> /checkout/src/test/ui/did_you_mean/issue-31424.rs:23:9\n   |\nLL |     fn bar(self: &mut Self) {\n   |            ---- help: consider changing this to be mutable: `mut self`\nLL |         (&mut self).bar(); //~ ERROR cannot borrow\n   |         ^^^^^^^^^^^ cannot borrow as mutable\n\n"}
[00:51:44] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:44] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:51:44] ------------------------------------------
[00:51:44] 
[00:51:44] thread '[ui (nll)] ui/did_you_mean/issue-31424.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:51:44] 
[00:51:44] 
[00:51:44] ---- [ui (nll)] ui/trivial-bounds-inconsistent-copy-reborrow.rs stdout ----
[00:51:44] diff of stderr:
[00:51:44] 
[00:51:44] 1 error[E0596]: cannot borrow immutable item `**t` as mutable
[00:51:44] 3    |
[00:51:44] 3    |
[00:51:44] + LL | fn reborrow_mut<'a>(t: &'a &'a mut i32) -> &'a mut i32 where &'a mut i32: Copy {
[00:51:44] +    |                        --------------- help: consider changing this to be a mutable reference: `&mut &mut i32`
[00:51:44] 4 LL |     *t //~ ERROR
[00:51:44] 5    |     ^^ cannot borrow as mutable
[00:51:44] -    |
[00:51:44] -    = note: the value which is causing this path not to be mutable is...: `*t`
[00:51:44] 8 
[00:51:44] 9 error[E0596]: cannot borrow immutable item `**t` as mutable
[00:51:44] 
[00:51:44] 11    |
[00:51:44] 11    |
[00:51:44] + LL | fn copy_reborrow_mut<'a>(t: &'a &'a mut i32) -> &'a mut i32 where &'a mut i32: Copy {
[00:51:44] +    |                             --------------- help: consider changing this to be a mutable reference: `&mut &mut i32`
[00:51:44] 12 LL |     {*t} //~ ERROR
[00:51:44] 13    |      ^^ cannot borrow as mutable
[00:51:44] -    |
[00:51:44] -    = note: the value which is causing this path not to be mutable is...: `*t`
[00:51:44] 17 error: aborting due to 2 previous errors
[00:51:44] 18 
[00:51:44] 
[00:51:44] 
[00:51:44] 
[00:51:44] The actual stderr differed from the expected stderr.
[00:51:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-inconsistent-copy-reborrow.nll/trivial-bounds-inconsistent-copy-reborrow.nll.stderr
[00:51:44] To update references, rerun the tests and pass the `--bless` flag
[00:51:44] To only update this specific test, also pass `--test-args trivial-bounds-inconsistent-copy-reborrow.rs`
[00:51:44] error: 1 errors occurred comparing output.
[00:51:44] status: exit code: 101
[00:51:44] status: exit code: 101
[00:51:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds-inconsistent-copy-reborrow.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-inconsistent-copy-reborrow.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-inconsistent-copy-reborrow.nll/auxiliary" "-A" "unused"
[00:51:44] ------------------------------------------
[00:51:44] 
[00:51:44] ------------------------------------------
[00:51:44] stderr:
[00:51:44] stderr:
[00:51:44] ------------------------------------------
[00:51:44] {"message":"cannot borrow immutable item `**t` as mutable","code":{"code":"E0596","explanation":"\nThis error occurs because you tried to mutably borrow a non-mutable variable.\n\nExample of erroneous code:\n\n