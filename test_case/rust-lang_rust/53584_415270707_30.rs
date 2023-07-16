\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generic/generic-impl-more-params-with-defaults.rs","byte_start":653,"byte_end":682,"line_start":23,"line_end":23,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    Vec::<isize, Heap, bool>::new();","highlight_start":5,"highlight_end":34}],"label":"expected at most 2 type argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of type arguments: expected at most 2, found 3\n  --> /checkout/src/test/ui/generic/generic-impl-more-params-with-defaults.rs:23:5\n   |\nLL |     Vec::<isize, Heap, bool>::new();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected at most 2 type argument\n\n"}
[00:46:53] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:53] {"message":"For more information about this error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0107`.\n"}
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] thread '[ui] ui/generic/generic-impl-more-params-with-defaults.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:53] 
[00:46:53] 
[00:46:53] ---- [ui] ui/generic/generic-type-less-params-with-defaults.rs stdout ----
[00:46:53] diff of stderr:
[00:46:53] 
[00:46:53] - error[E0243]:ng number of lifetime arguments:\n                        //        expected 1, found 0\n    f::<'static, 'b>() // error: wrong number of lifetime arguments:\n                        //        expected 0, found 2\n}\n