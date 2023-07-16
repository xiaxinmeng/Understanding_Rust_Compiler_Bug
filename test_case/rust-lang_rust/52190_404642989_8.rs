\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":1933,"byte_end":1935,"line_start":64,"line_end":64,"column_start":24,"column_end":26,"is_primary":false,"text":[{"text":"    let c1 = to_fn_mut(|| set(&mut *x.f));","highlight_start":24,"highlight_end":26}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":1976,"byte_end":1978,"line_start":65,"line_end":65,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"    let c2 = to_fn_mut(|| set(&mut *x.f));","       - previous borrow occurs due to use of `x` in closure\n   |                        |\n   |                        first mutable borrow occurs here\nLL |     let c2 = to_fn_mut(|| set(&mut *x.f));\n   |                        ^^           - borrow occurs due to use of `x` in closure\n   |                        |\n   |                        second mutable borrow occurs here\n...\nLL | }\n   | - first borrow ends here\n\n"}
[00:44:22] thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/librustc_data_structures/indexed_vec.rs:505:32
[00:44:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:22] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:44:22] {"message":"For more information about this error, try `rustc --explain E0499`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0499`.\n"}
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
[00ed here
[00ed here
[00:44:22] - 
[00:44:22] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |     if let [.., _, ref from_end4, ref from_end3, _, ref from_end1] = *s {
[00:44:22] -    |                    ------------- immutable borrow occurs here
[00:44:22] - ...
[00:44:22] - LL |         if let [_, ref mut from_begin1, ..] = *s { //~ERROR
[00:44:22] -    |                    ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:44:22] - LL |             nop(&[from_begin1, from_end1, from_end3, from_end4]);
[00:44:22] -    |                                                      --------- borrow later used here
[00:44:22] - 
[00:44:22] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |     if let [.., _, ref from_end4, ref from_end3, _, ref from_end1] = *s {
[00:44:22] -    |                                   ------------- immutable borrow occurs here
[00:44:22] - ...
[00:44:22] - LL |         if let [_, _, ref mut from_begin2, ..] = *s { //~ERROR
[00:44:22] -    |                       ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:44:22] - LL |             nop(&[from_begin2, from_end1, from_end3, from_end4]);
[00:44:22] -    |                                           --------- borrow later used here
[00:44:22] - 
[00:44:22] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |     if let [.., _, ref from_end4, ref from_end3, _, ref from_end1] = *s {
[00:44:22] -    |                                   ------------- immutable borrow occurs here
[00:44:22] - ...
[00:44:22] - LL |         if let [_, _, _, ref mut from_begin3, ..] = *s { //~ERROR
[00:44:22] -    |                          ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:44:22] - LL |             nop(&[from_begin3, from_end1, from_end3, from_end4]);
[00:44:22] -    |                                           --------- borrow later used here
[00:44:22] - 
[00:44:22] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |     if let [ref from_begin0, ref from_begin1, _, ref from_begin3, _, ..] = *s {
[00:44:22] -    |                                                  --------------- immutable borrow occurs here
[00:44:22] - ...
[00:44:22] - LL |         if let [.., ref mut from_end2, _] = *s { //~ERROR
[00:44:22] -    |                     ^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:44:22] - LL |             nop(&[from_begin0, from_begin1, from_begin3, from_end2]);
[00:44:22] -    |                                             ----------- borrow later used here
[00:44:22] - 
[00:44:22] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |     if  try `rustc --explain E0502`.
[00:44:22] 
[00:44:22] 
[00:44:22] 
[00:44:22] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-slice-pattern-element-loan/borrowck-slice-pattern-element-loan.stderr`: No such file or directory (os error 2)
[00:44:22] 
[00:44:22] ---- [ui] ui/borrowck/issue-41962.rs stdout ----
[00:44:22] diff of stderr:
[00:44:22] 
[00:44:22] 
[00:44:22] 16    |
[00:44:22] 17    = note: move occurs because the value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait
[00:44:22] 18 
[00:44:22] - error[E0382]: use of moved value: `maybe` (Mir)
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |           if let Some(thing) = maybe {
[00:44:22] -    |           ^           ----- value moved here
[00:44:22] -    |  _________|
[00:44:22] -    | |
[00:44:22] - LL | |         }
[00:44:22] -    | |_________^ value used here after move
[00:44:22] -    |
[00:44:22] -    = note: move occurs because value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait
[00:44:22] - 
[00:44:22] - error[E0382]: borrow of moved value: `maybe` (Mir)
[00:44:22] -    |
[00:44:22] -    |
[00:44:22] - LL |           if let Some(thing) = maybe {
[00:44:22] -    |           ^           ----- value moved here
[00:44:22] -    |  _________|
[00:44:22] -    | |
[00:44:22] - LL | |         }
[00:44:22] -    | |_________^ value borrowed here afs` flag
[00:44:22] To only update this specific test, also pass `--test-args borrowck/issue-41962.rs`
[00:44:22] error: 1 errors occurred comparing output.
[00:44:22] status: exit code: 101
[00:44:22] status: exit code: 101
[00:44:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-41962.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-41962/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-41962/auxiliary" "-A" "unused"
[00:44:22] ------------------------------------------
[00:44:22] 
[00:44:22] ------------------------------------------
[00:44:22] stderr:
[00:44:22] stderr:
[00:44:22] ------------------------------------------
[00:44:22] {"message":"use of partially moved value: `maybe` (Ast)","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n