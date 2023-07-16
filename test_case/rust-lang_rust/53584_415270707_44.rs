\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-3214.rs","byte_start":600,"byte_end":601,"line_start":16,"line_end":16,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"    impl<T> Drop for foo<T> {","highlight_start":26,"highlight_end":27}],"label":"unexpected type argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of type arguments: expected 0, found 1\n  --> /checkout/src/test/ui/issues/issue-3214.rs:16:26\n   |\nLL |     impl<T> Drop for foo<T> {\n   |                          ^ unexpected type argument\n\n"}
[00:46:53] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:53] {"message":"Some errors occurred: E0107, E0401.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0107, E0401.\n"}
[00:46:53] {"message":"For more information about an error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0107`.\n"}
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] thread '[ui] ui/issues/issue-3214.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:53] 
[00:46:53] 
[00:46:53] ---- [ui] ui/methods/method-call-lifetime-args-fail.rs stdout ----
[00:46:53] diff of stderr:
[00:46:53] 
[00:46:53] - error[E0090]: wrong number of lifetime arguments: expected 2, foun,"column_end":12,"is_primary":true,"text":[{"text":"    S.early::<'static>();","highlight_start":7,"highlight_end":12}],"label":"expected 2 lifetime arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of lifetime arguments: expected 2, found 1\n  --> /checkout/src/test/ui/methods/method-call-lifetime-args-fail.rs:26:7\n   |\nLL |     S.early::<'static>();\n   |       ^^^^^ expected 2 lifetime arguments\n\n"}
[00:46:53] {"message":"wrong number of lifetime arguments: expected 2, found 3","code":{"code":"E0107","explanation":"\nThis error means that an incorrect number of type or lifetime parameters\nwere provided:\n\n