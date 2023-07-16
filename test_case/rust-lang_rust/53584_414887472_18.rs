\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generic/generic-type-more-params-with-defaults.rs","byte_start":581,"byte_end":603,"line_start":19,"line_end":19,"column_start":12,"column_end":34,"is_primary":true,"text":[{"text":"    let _: Vec<isize, Heap, bool>;","highlight_start":12,"highlight_end":34}],"label":"expected at most 2 type argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of type arguments: expected at most 2, found 3\n  --> /checkout/src/test/ui/generic/generic-type-more-params-with-defaults.rs:19:12\n   |\nLL |     let _: Vec<isize, Heap, bool>;\n   |            ^^^^^^^^^^^^^^^^^^^^^^ expected at most 2 type argument\n\n"}
[00:47:09] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:09] {"message":"For more information about this error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0107`.\n"}
[00:47:09] ------------------------------------------
[00:47:09] 
[00:47:09] thread '[ui] ui/generic/generic-type-more-params-with-defaults.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:47:09] 
[00:47:09] 
[00:47:09] ---- [ui] ui/issue-53251.rs stdout ----
[00:47:09] diff of stderr:
[00:47:09] 
[00:47:09] - error[E0087]: wrong number of type arguments: expected 0, fime arguments:\n                  //        expected 0, found 1\n}\n