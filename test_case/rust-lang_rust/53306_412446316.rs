plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:50] 
[00:43:50] running 2252 tests
[00:43:53] .....................................................F..............................................
[00:44:01] .........i..........................................................................................
[00:44:03] ....................................................................................................
[00:44:05] ....................................................................................................
[00:44:07] ....................................................................................................
---
[00:44:44] .................................i..................................................................
[00:44:47] ....................................................................................................
[00:44:51] ....................................................................................................
44:56] 42 
[00:44:56] 43 error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:44:56] 
[00:44:56] 45    |
[00:44:56] 45    |
[00:44:56] 46 LL |     if let [.., _, ref from_end4, ref from_end3, _, ref from_end1] = *s {
[00:44:56] +    |                                   ------------- immutable borrow occurs here
[00:44:56] 48 ...
[00:44:56] 48 ...
[00:44:56] 49 LL |         if let [_, _, _, ref mut from_begin3, ..] = *s { //~ERROR
[00:44:56] 
[00:44:56] 
[00:44:56] 51 LL |             nop(&[from_begin3, from_end1, from_end3, from_end4]);
[00:44:56] +    |                                           --------- borrow later used here
[00:44:56] 53 
[00:44:56] 53 
[00:44:56] 54 error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable
[00:44:56] 
[00:44:56] 
[00:44:56] The actual stderr differed from the expected stderr.
[00:44:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-slice-pattern-element-loan/borrowck-slice-pattern-element-loan.stderr
[00:44:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-slice-pattern-element-loan/borrowck-slice-pattern-element-loan.stderr
[00:44:56] To update references, rerun the tests and pass the `--bless` flag
[00:44:56] To only update this specific test, also pass `--test-args borrowck/borrowck-slice-pattern-element-loan.rs`
[00:44:56] 
[20,"highlight_end":33}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-slice-pattern-element-loan.rs","byte_start":1823,"byte_end":1832,"line_start":56,"line_end":56,"column_start":54,"column_end":63,"is_primary":false,"text":[{"text":"            nop(&[from_begin1, from_end1, from_end3, from_end4]);","highlight_start":54,"highlight_end":63}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `s[..]` as mutable because it is also borrowed as immutable\n  --> /checkout/src/test/ui/borrowck/borrowck-slice-pattern-element-loan.rs:55:20\n   |\nLL |     if let [.., _, ref from_end4, ref from_end3, _, ref from_end1] = *s {\n   |                    ------------- immutable borrow occurs here\n...\nLL |         if let [_, ref mut from_begin1, ..] = *s { //~ERROR\n   |                    ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here\nLL |             nop(&[from_begin1, from_end1, from_end3, from_end4]);\n   |                                                      --------- borrow later used here\n\n"}
[00:44:56] {"message":"cannot borrow `s[..]` as mutable because it is also borrowed as immutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n