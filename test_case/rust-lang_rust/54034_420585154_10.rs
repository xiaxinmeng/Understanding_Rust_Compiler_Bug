\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/first-edition/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms.rs","byte_start":174,"byte_end":175,"line_start":8,"line_end":8,"column_start":36,"column_end":37,"is_primary":true,"text":[{"text":"        VecWrapper::A(v) if { drop(v); false } => 1,","highlight_start":36,"highlight_end":37}],"label":"cannot move out of borrowed content","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms.rs:8:36\n   |\nLL |         VecWrapper::A(v) if { drop(v); false } => 1,\n   |                                    ^ cannot move out of borrowed content\n\n"}
[00:51:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:16] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] 
[00:51:16] thread '[ui] ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:51:16] 
[00:51:16] ---- [ui] ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm.rs stdout ----
[00:51:16] 
[00:51:16] 1 error[E0507]: cannot move out of borrowed content
[00:51:16] 1 error[E0507]: cannot move out of borrowed content
[00:51:16] -   --> $DIR/rfc-reject-double-move-in-first-arm.rs:19:30
[00:51:16] +   --> $DIR/rfc-reject-double-move-in-first-arm.rs:9:30
[00:51:16] 3    |
[00:51:16] 4 LL |         A { a: v } if { drop(v); true } => v,
[00:51:16] 5    |                              ^ cannot move out of borrowed content
[00:51:16] 
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm/rfc-reject-double-move-in-first-arm.stderr
[00:51:16] To update references, rerun ``\n\nHere, the `nothing_is_true` method takes the ownership of `self`. However,\n`self` cannot be moved because `.borrow()` only provides an `&TheDarkKnight`,\nwhich is a borrow of the content owned by the `RefCell`. To fix this error,\nyou have three choices:\n\n* Try to avoid moving the variable.\n* Somehow reclaim the ownership.\n* Implement the `Copy` trait on the type.\n\nExamples:\n\n