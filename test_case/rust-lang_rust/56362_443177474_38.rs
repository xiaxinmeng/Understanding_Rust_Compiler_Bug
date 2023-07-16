\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/non-exhaustive/non-exhaustive-match.rs","byte_start":1474,"byte_end":1478,"line_start":43,"line_end":43,"column_start":11,"column_end":15,"is_primary":true,"text":[{"text":"    match *vec { //~ ERROR non-exhaustive patterns: `[]` not covered","highlight_start":11,"highlight_end":15}],"label":"pattern `[]` not covered","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0004]: non-exhaustive patterns: `[]` not covered\n  --> /checkout/"column_start":11,"column_end":15,"is_primary":true,"text":[{"text":"    match *vec { //~ ERROR non-exhaustive patterns: `[_, _, _, _]` not covered","highlight_start":11,"highlight_end":15}],"label":"pattern `[_, _, _, _]` not covered","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0004]: non-exhaustive patterns: `[_, _, _, _]` not covered\n  --> /checkout/src/test/ui/non-exhaustive/non-exhaustive-match.rs:56:11\n   |\nLL |     match *vec { //~ ERROR non-exhaustive patterns: `[_, _, _, _]` not covered\n   |           ^^^^ pattern `[_, _, _, _]` not covered\n\n"}
[00:51:08] {"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}
[00:51:08] {"message":"For more information about this error, try `rustc --explain E0004`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0004`.\n"}
[00:51:08] ------------------------------------------
[00:51:08] 
[00:51:08] thread '[ui] ui/non-exhaustive/non-exhaustive-match.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:51:08] 
[00:51:08] 
[00:51:08] ---- [ui] ui/refutable-pattern-errors.rs stdout ----
[00:51:08] diff of stderr:
[00:51:08] 
[00:51:08] 4 LL | fn func((1, (Some(1), 2..=3)): (isize, (Option<isize>, isize))) { }
[00:51:08] 5    |         ^^^^^^^^^^^^^^^^^^^^^ pattern `(_, _)` not covered
[00:51:08] 6 
[00:51:08] - error[E0005]: refutable pattern in local binding: `(_, _)` not covered
[00:51:08] + e51:08] 
[00:51:08] stderr:
[00:51:08] ------------------------------------------
[00:51:08] ------------------------------------------
[00:51:08] {"message":"refutable pattern in function argument: `(_, _)` not covered","code":{"code":"E0005","explanation":"\nPatterns used to bind names must be irrefutable, that is, they must guarantee\nthat a name will be extracted in all cases. Erroneous code example:\n\n