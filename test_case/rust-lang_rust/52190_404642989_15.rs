\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/issue-41962.rs","byte_start":593,"byte_end":598,"line_start":17,"line_end":17,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"        if let Some(thing) = maybe {","highlight_start":21,"highlight_end":26}],"label":"value moved here in previous iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because the value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `(maybe as std::prelude::v1::Some).0` (Ast)\n  --> /checkout/src/test/ui/borrowck/issue-41962.rs:17:21\n   |\nLL |         if let Some(thing) = maybe {\n   |                     ^^^^^ value moved here in previous iteration of loop\n   |\n   = note: move occurs because the value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait\n\n"}
[00:44:22] thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/librustc_data_structures/indexed_vec.rs:505:32
[00:44:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:22] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:44:22] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[00:44:22] error: internal compiler error: unexpected panic
[00:44:22] 
[00:44:22] 
[00:44:22] note: the compiler unexpectedly panicked. this is a bug.
[00:44:22] 
[00:44:22] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:44:22] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:44:22] 
[00:44:22] 
[00:44:22] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=compare -C prefer-dynamic -C rpath
[00:44:22] 
[00:44:22] ------------------------------------------
[00:44:22] 
[00:44:22] thread '[ui] ui/borrowck/issue-41962.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:44:22] thread '[ui] ui/borrowck/issue-41962.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:44:22] 
[00:44:22] ---- [ui] ui/borrowck/two-phase-method-receivers.rs stdout ----
[00:44:22] 
[00:44:22] error: test compilation failed although it shouldn't!
[00:44:22] status: exit code: 101
[00:44:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-method-receivers.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-method-receivers/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "two-phase-borrows" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-method-receivers/auxiliary" "-A" "unused"
[00:44:22] ------------------------------------------
[00:44:22] 
[00:44:22] ------------------------------------------
[00:44:22] stderr:
[00:44:22] stderr:
[00:44:22] ------------------------------------------
[00:44:22] thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/librustc_data_structures/indexed_vec.rs:505:32
[00:44:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:22] 
[00:44:22] error: internal compiler error: unexpected panic
able borrow occurs here
[00:44:22] -    |     borrow later used here
[00:44:22] - error: aborting due to 2 previous errors
[00:44:22] - 
[00:44:22] - For more information about this error, try `rustc --explain E0499`.
[00:44:22] - 
[00:44:22] - 
[00:44:22] 
[00:44:22] 
[00:44:22] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-multi-mut/two-phase-multi-mut.stderr`: No such file or directory (os error 2)
[00:44:22] 
[00:44:22] ---- [ui] ui/borrowck/two-phase-multiple-activations.rs stdout ----
[00:44:22] 
[00:44:22] 
[00:44:22] error: test compilation failed although it shouldn't!
[00:44:22] status: exit code: 101
[00:44:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-multiple-activations.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-multiple-activations/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "two-phase-borrows" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-multiple-activations/auxiliary" "-A" "unused"
[00:44:22] ------------------------------------------
[00:44:22] 
[00:44:22] ------------------------------------------
[00:44:22] stderr:
[00:44:22] stderr:
[00:44:22] ------------------------------------------
[00:44:22] thread 'main' panicked at 'assertion failed: mid <= len', libcore/slice/mod.rs:875:13
[00:44:22] 
[00:44:22] error: internal compiler error: unexpected panic
[00:44:22] 
[00:44:22] 
[00:44:22] note: the compiler unexpectedly panicked. this is a bug.
[00:44:22] 
[00:44:22] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:44:22] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:44:22] 
[00:44:22] 
[00:44:22] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=mir -Z two-phase-borrows -C prefer-dynamic -C rpath
[00:44:22] 
[00:44:22] ------------------------------------------
[00:44:22] 
[00:44:22] thread '[ui] ui/borrowck/two-phase-multiple-activations.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:44:22] thread '[ui] ui/borrowck/two-phase-multiple-activations.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:44:22] 
[00:44:22] ---- [ui] ui/generator/generator-with-nll.rs stdout ----
[00:44:22] diff of stderr:
[00:44:22] 
[00:44:22] 16 LL |         yield ();
[00:44:22] 17    |         -------- possible yield occurs here
[00:44:22] 18 
[00:44:22] - error[E0626]: borrow may still be in use when generator yields (Mir)
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |         let b = &mut true; //~ ERROR borrow may still be in use when generator yields (Ast)
[00:44:22] -    |                 ^^^^^^^^^
[00:44:22] - LL |         //~^ borrow may still be in use when generator yields (Mir)
[00:44:22] - LL |         yield ();
[00:44:22] -    |         -------- possible yield /test/ui/generator/generator-with-nll.rs:19:23\n   |\nLL |         let _a = &mut true; //~ ERROR borrow may still be in use when generator yields (Ast)\n   |                       ^^^^\n...\nLL |         yield ();\n   |         -------- possible yield occurs here\n\n"}
[00:44:22] {"message":"borrow may still be in use when generator yields (Ast)","code":{"code":"E0626","explanation":"\nThis error occurs because a borrow in a generator persists across a\nyield point.\n\n