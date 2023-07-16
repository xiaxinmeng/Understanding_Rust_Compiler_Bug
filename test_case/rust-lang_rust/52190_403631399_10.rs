\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":1933,"byte_end":1935,"line_start":64,"line_end":64,"column_start":24,"column_end":26,"is_primary":false,"text":[{"text":"    let c1 = to_fn_mut(|| set(&mut *x.f));","highlight_start":24,"highlight_end":26}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":1976,"byte_end":1978,"line_start":65,"line_end":65,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"    let c2 = to_fn_mut(|| set(&mut *x.f));","highlight_start":24,"highlight_end":26}],"label":"second mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":2134,"byte_end":2135,"line_start":69,"line_end":69,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"first borrow ends here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":1989,"byte_end":1990,"line_start":65,"line_end":65,"column_start":37,"column_end":38,"is_primary":false,"text":[{"text":"    let c2 = to_fn_mut(|| set(&mut *x.f));","highlight_start":37,"highlight_end":38}],"label":"borrow occurs due to use of `x` in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs","byte_start":1946,"byte_end":1947,"line_start":64,"line_end":64,"column_start":37,"column_end":38,"is_primary":false,"text":[{"text":"    let c1 = to_fn_mut(|| set(&mut *x.f));","highlight_start":37,"highlight_end":38}],"label":"previous borrow occurs due to use of `x` in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0499]: cannot borrow `x` as mutable more than once at a time (Ast)\n  --> /checkout/src/test/ui/borrowck/borrowck-closures-two-mut.rs:65:24\n   |\nLL |     let c1 = to_fn_mut(|| set(&mut *x.f));\n   |                        --           - previous borrow occurs due to use of `x` in closure\n   |                        |\n   |                        first mutable borrow occurs here\nLL |     let c2 = to_fn_mut(|| set(&mut *x.f));\n   |                        ^^           - borrow occurs due to use of `x` in closure\n   |                        |\n   |                        second mutable borrow occurs here\n...\nLL | }\n   | - first borrow ends here\n\n"}
[00:39:51] thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/librustc_data_structures/indexed_vec.rs:505:32
[00:39:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:39:51] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:39:51] {"message":"For more information about this error, try `rustc --explain E0499`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0499`.\n"}
[00:39:51] error: internal compiler error: unexpected panic
[00:39:51] 
[00:39:51] 
[00:39:51] note: the compiler unexpectedly panicked. this is a bug.
[00:39:51] 
[00:39:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:39:51] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:39:51] 
[00:39:51] 
[00:39:51] note: compiler flags: -Z ui-testing -Z unstable-options -Z borrowck=compare -C prefer-dynamic -C rpath
[00:39:51] 
[00:39:51] ------------------------------------------
[00:39:51] 
[00:39:51] thread '[ui] ui/borrowck/borrowck-closures-two-mut.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:39:51] thread '[ui] ui/borrowck/borrowck-closures-two-mut.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:39:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:39:51] 
[00:39:51] ---- [ui] ui/borrowck/borrowck-slice-pattern-element-loan.rs stdout ----
[00:39:51] diff of stderr:
[00:39:51] 
[00:39:51] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     if let [ref first, ref second, ..] = *s {
[00:39:51] -    |                        ---------- immutable borrow occurs here
[00:39:51] - LL |         if let [_, ref mut  second2, ref mut third, ..] = *s { //~ERROR
[00:39:51] -    |                    ^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:39:51] - LL |             nop(&[first, second, second2, third]);
[00:39:51] -    |                          ------ borrow later used here
[00:39:51] - 
[00:39:51] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     if let [.., ref fourth, ref third, _, ref first] = *s {
[00:39:51] -    |                             --------- immutable borrow occurs here
[00:39:51] - LL |         if let [.., ref mut third2, _, _] = *s { //~ERROR
[00:39:51] -    |                     ^^^^^^^^^^^^^^ mutable borrow occurs here
[00:39:51] - LL |             nop(&[first, third, third2, fourth]);
[00:39:51] -    |                          ----- borrow later used here
[00:39:51] - 
[00:39:51] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     if let [.., _, ref from_end4, ref from_end3, _, ref from_end1] = *s {
[00:39:51] -    |                    ------------- immutable borrow occurs here
[00:39:51] - ...
[00:39:51] - LL |         if let [_, ref mut from_begin1, ..] = *s { //~ERROR
[00:39:51] -    |                    ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:39:51] - LL |             nop(&[from_begin1, from_end1, from_end3, from_end4]);
[00:39:51] -    |                                                      --------- borrow later used here
[00:39:51] - 
[00:39:51] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     if let [.., _, ref from_end4, ref from_end3, _, ref from_end1] = *s {
[00:39:51] -    |                                   ------------- immutable borrow occurs here
[00:39:51] - ...
[00:39:51] - LL |         if let [_, _, ref mut from_begin2, ..] = *s { //~ERROR
[00:39:51] -    |                       ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:39:51] - LL |             nop(&[from_begin2, from_end1, from_end3, from_end4]);
[00:39:51] -    |                                           --------- borrow later used here
[00:39:51] - 
[00:39:51] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     if let [.., _, ref from_end4, ref from_end3, _, ref from_end1] = *s {
[00:39:51] -    |                                   ------------- immutable borrow occurs here
[00:39:51] - ...
[00:39:51] - LL |         if let [_, _, _, ref mut from_begin3, ..] = *s { //~ERROR
[00:39:51] -    |                          ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:39:51] - LL |             nop(&[from_begin3, from_end1, from_end3, from_end4]);
[00:39:51] -    |                                           --------- borrow later used here
[00:39:51] - 
[00:39:51] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     if let [ref from_begin0, ref from_begin1, _, ref from_begin3, _, ..] = *s {
[00:39:51] -    |                                                  --------------- immutable borrow occurs here
[00:39:51] - ...
[00:39:51] - LL |         if let [.., ref mut from_end2, _] = *s { //~ERROR
[00:39:51] -    |                     ^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:39:51] - LL |             nop(&[from_begin0, from_begin1, from_begin3, from_end2]);
[00:39:51] -    |                                             ----------- borrow later used here
[00:39:51] - 
[00:39:51] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     if let [ref from_begin0, ref from_begin1, _, ref from_begin3, _, ..] = *s {
[00:39:51] -    |                                                  --------------- immutable borrow occurs here
[00:39:51] - ...
[00:39:51] - LL |         if let [.., ref mut from_end3, _,  _] = *s { //~ERROR
[00:39:51] -    |                     ^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:39:51] - LL |             nop(&[from_begin0, from_begin1, from_begin3, from_end3]);
[00:39:51] -    |                                             ----------- borrow later used here
[00:39:51] - 
[00:39:51] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     if let [ref from_begin0, ref from_begin1, _, ref from_begin3, _, ..] = *s {
[00:39:51] -    |                              --------------- immutable borrow occurs here
[00:39:51] - ...
[00:39:51] - LL |         if let [.., ref mut from_end4, _, _, _] = *s { //~ERROR
[00:39:51] -    |                     ^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[00:39:51] - LL |             nop(&[from_begin0, from_begin1, from_begin3, from_end4]);
[00:39:51] -    |                                ----------- borrow later used here
[00:39:51] - 
[00:39:51] - error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |     if let [ref first, ref second, ..] = *s {
[00:39:51] -    |                        ---------- immutable borrow occurs here
[00:39:51] - LL |         if let [_, ref mut tail..] = *s [00:39:51] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-slice-pattern-element-loan/borrowck-slice-pattern-element-loan.stderr`: No such file or directory (os error 2)
[00:39:51] 
[00:39:51] ---- [ui] ui/borrowck/issue-41962.rs stdout ----
[00:39:51] diff of stderr:
[00:39:51] 
[00:39:51] 
[00:39:51] 16    |
[00:39:51] 17    = note: move occurs because the value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait
[00:39:51] 18 
[00:39:51] - error[E0382]: use of moved value: `maybe` (Mir)
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |           if let Some(thing) = maybe {
[00:39:51] -    |           ^           ----- value moved here
[00:39:51] -    |  _________|
[00:39:51] -    | |
[00:39:51] - LL | |         }
[00:39:51] -    | |_________^ value used here after move
[00:39:51] -    |
[00:39:51] -    = note: move occurs because value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait
[00:39:51] - 
[00:39:51] - error[E0382]: borrow of moved value: `maybe` (Mir)
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |           if let Some(thing) = maybe {
[00:39:51] -    |           ^           ----- value moved here
[00:39:51] -    |  _________|
[00:39:51] -    | |
[00:39:51] - LL | |         }
[00:39:51] -    | |_________^ value borrowed here after move
[00:39:51] -    |
[00:39:51] -    = note: move occurs because value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait
[00:39:51] - 
[00:39:51] - error[E0382]: use of moved value: `maybe` (Mir)
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |         if let Some(thing) = maybe {
[00:39:51] -    |                ^^^^^-----^
[00:39:51] -    |                |    value moved here
[00:39:51] -    |                |    value moved here
[00:39:51] -    |                value used here after move
[00:39:51] -    |
[00:39:51] -    = note: move occurs because value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait
[00:39:51] - 
[00:39:51] - error[E0382]: use of moved value (Mir)
[00:39:51] -    |
[00:39:51] -    |
[00:39:51] - LL |         if let Some(thing) = maybe {
[00:39:51] -    |                     ^^^^^ value moved here in previous iteration of loop
[00:39:51] -    |
[00:39:51] -    = note: move occurs because value has type `std::vec::Vec<bool>`, which does not implement the `Copy` trait
[00:39:51] - error: aborting due to 6 previous errors
[00:39:51] + error: aborting due to 2 previous errors
[00:39:51] 63 
[00:39:51] 64 For more information about this error, try `rustc --explain E0382`.
[00:39:51] 64 For more information about this error, try `rustc --explain E0382`.
[00:39:51] 65 
[00:39:51] 
[00:39:51] 
[00:39:51] The actual stderr differed from the expected stderr.
[00:39:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-41962/issue-41962.stderr
[00:39:51] To update references, rerun the tests and pass the `--bless` flag
[00:39:51] To only update this specific test, also pass `--test-args borrowck/issue-41962.rs`
[00:39:51] error: 1 errors occurred comparing output.
[00:39:51] status: exit code: 101
[00:39:51] status: exit code: 101
[00:39:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-41962.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-41962/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-41962/auxiliary" "-A" "unused"
[00:39:51] ------------------------------------------
[00:39:51] 
[00:39:51] ------------------------------------------
[00:39:51] stderr:
[00:39:51] stderr:
[00:39:51] ------------------------------------------
[00:39:51] {"message":"use of partially moved value: `maybe` (Ast)","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n