\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/constructor-lifetime-args.rs","byte_start":1432,"byte_end":1439,"line_start":34,"line_end":34,"column_start":30,"column_end":37,"is_primary":true,"text":[{"text":"    E::V::<'static, 'static, 'static>(&0);","highlight_start":30,"highlight_end":37}],"label":"unexpected lifetime argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of lifetime arguments: expected 2, found 3\n  --> /checkout/src/test/ui/constructor-lifetime-args.rs:34:30\n   |\nLL |     E::V::<'static, 'static, 'static>(&0);\n   |                              ^^^^^^^ unexpected lifetime argument\n\n"}
[00:47:09] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:47:09] {"message":"For more information about this error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0107`.\n"}
[00:47:09] ------------------------------------------
[00:47:09] 
[00:47:09] thread '[ui] ui/constructor-lifetime-args.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:47:09] 
[00:47:09] 
[00:47:09] ---- [ui] ui/generic/generic-arg-mismatch-recover.rs stdout ----
[00:47:09] diff of stderr:
[00:47:09] 
[00:4 C }\n\nstruct Baz<'a> {\n    foo: Foo<'a>, // error: wrong number of lifetime arguments:\n                  //        expected 2, found 1\n    bar: Bar<'a>, // error: wrong number of lifetime arguments:\n                  //        expected 0, found 1\n}\n