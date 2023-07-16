\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0087.rs","byte_start":517,"byte_end":520,"line_start":15,"line_end":15,"column_start":11,"column_end":14,"is_primary":true,"text":[{"text":"    foo::<f64>(); //~ ERROR wrong number of type arguments: expected 0, found 1 [E0107]","highlight_start":11,"highlight_end":14}],"label":"unexpected type argument","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of type arguments: expected 0, found 1\n  --> /checkout/src/test/ui/error-codes/E0087.rs:15:11\n   |\nLL |     foo::<f64>(); //~ ERROR wrong number of type arguments: expected 0, found 1 [E0107]\n   |           ^^ 2, found 1 [E0107]","highlight_start":5,"highlight_end":15}],"label":"expected 2 type arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of type arguments: expected 2, found 1\n  --> /checkout/src/test/ui/error-codes/E0089.rs:14:5\n   |\nLL |     foo::<f64>(); //~ ERROR wrong number of type arguments: expected 2, found 1 [E0107]\n   |     ^^^^^^^^^^ expected 2 type arguments\n\n"}
[00:46:53] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:53] {"message":"For more information about this error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0107`.\n"}
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] thread '[ui] ui/error-codes/E0089.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:53] 
[00:46:53] 
[00:46:53] ---- [ui] ui/error-codes/E0090.rs stdout ----
[00:46:53] normalized stderr:
[00:46:53] error[E0107]: wrong number of lifetime arguments: expected 2, found 1
[00:46:53]   --> $DIR/E0090.rs:14:5
[00:46:53]    |
[00:46:53] LL |     foo::<'static>(); //~ ERROR wrong number of lifetime arguments: expected 2, found 1 [E0107]
[00:46:53] 
[00:46:53] error: aborting due to previous error
[00:46:53] 
[00:46:53] For more information about this error, try `rustc --explain E0107`.
[00:46:53] For more information about this error, try `rustc --explain E0107`.
[00:46:53]d 1\n    bar: Bar<'a>, // error: wrong number of lifetime arguments:\n                  //        expected 0, found 1\n}\n