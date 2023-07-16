\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs","byte_start":1017,"byte_end":1020,"line_start":33,"line_end":33,"column_start":25,"column_end":28,"is_primary":true,"text":[{"text":"            (self.func)(arg) //~ ERROR cannot borrow","highlight_start":25,"highlight_end":28}],"label":"mutable borrow starts here in previous iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the lifetime 'a as defined on the impl at 17:6...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs","byte_start":602,"byte_end":604,"line_start":17,"line_end":17,"column_start":6,"column_end":8,"is_primary":true,"text":[{"text":"impl<'a, T : 'a> FuncWrapper<'a, T> {","highlight_start":6,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider extracting this into a `let`-binding","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs","byte_start":1005,"byte_end":1021,"line_start":33,"line_end":33,"column_start":13,"column_end":29,"is_primary":true,"text":[{"text":"            (self.func)(arg) //~ ERROR cannot borrow","highlight_start":13,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0499]: cannot borrow `*arg` as mutable more than once at a time\n  --> /checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs:33:25\n   |\nLL |             (self.func)(arg) //~ ERROR cannot borrow\n   |                         ^^^ mutable borrow starts here in previous iteration of loop\n   |\nnote: borrowed value must be valid for the lifetime 'a as defined on the impl at 17:6...\n  --> /checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs:17:6\n   |\nLL | impl<'a, T : 'a> FuncWrapper<'a, T> {\n   |      ^^\nnote: consider extracting this into a `let`-binding\n  --> /checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs:33:13\n   |\nLL |             (self.func)(arg) //~ ERROR cannot borrow\n   |             ^^^^^^^^^^^^^^^^\n\n"}
[00:51:17] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:51:17] {"message":"For more information about this error, try `rustc --explain E0499`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0499`.\n"}
[00:51:17] ------------------------------------------
[00:51:17] 
[00:51:17] thread '[ui (nll)] ui/borrowck/mut-borrow-in-loop.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:51:17] 
[00:51:17] 
[00:51:17] ---- [ui (nll)] ui/codemap_tests/one_line.rs stdout ----
[00:51:17] diff of stderr:
[00:51:17] 
[00:51:17] 7    |     |      second mutable borrow occurs here
[00:51:17] 8    |     first mutable borrow occurs here
[00:51:17] 9    |     borrow later used here
[00:51:17] +    |
[00:51:17] + note: consider extracting this into a `let`-binding
[00:51:17] +   --> $DIR/one_line.rs:13:12
[00:51:17] +    |
[00:51:17] + LL |     v.push(v.pop().unwrap()); //~ ERROR cannot borrow
[00:51:17] 10 
[00:51:17] 11 error: aborting due to previous error
[00:51:17] 12 
[00:51:17] 
[00:51:17] 
[00:51:17] 
[00:51:17] The actual stderr differed from the expected stderr.
[00:51:17] Actual stderr saved to /checkout/o-lang.org/stable/book/references-and-borrowing.html for more\ninformation. Example:\n\n\n