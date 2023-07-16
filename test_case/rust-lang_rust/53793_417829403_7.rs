\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-enum-not-wf.rs","byte_start":765,"byte_end":787,"line_start":28,"line_end":28,"column_start":18,"column_end":40,"is_primary":true,"text":[{"text":"    Ref1Variant1(RequireOutlives<'a, T>) //~ ERROR the parameter type `T` may not live long enough","highlight_start":18,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...so that the type `T` will meet its required lifetime bounds","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-enum-not-wf.rs","byte_start":765,"byte_end":787,"line_start":28,"line_end":28,"column_start":18,"column_end":40,"is_primary":true,"text":[{"text":"    Ref1Variant1(RequireOutlives<'a, T>) //~ ERROR the parameter type `T` may not live long enough","highlight_start":18,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider adding an explicit lifetime bound `T: 'a`...","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-enum-not-wf.rs","byte_star7:21] 3    |
[00:47:21] 4 LL | enum Ref1<'a, T> {
[00:47:21] 5    |               - help: consider adding an explicit lifetime bound `T: 'a`...
[00:47:21] 7    |                  ^^^^^^^^^^^^^^^^^^^^^^
[00:47:21] 8    |
[00:47:21] 8    |
[00:47:21] 9 note: ...so that the type `T` will meet its required lifetime bounds
[00:47:21] +   --> $DIR/regions-enum-not-wf.rs:28:18
[00:47:21] 11    |
[00:47:21] 11    |
[00:47:21] 12 LL |     Ref1Variant1(RequireOutlives<'a, T>) //~ ERROR the parameter type `T` may not live long enough
[00:47:21] 
[00:47:21] 14 
[00:47:21] 14 
[00:47:21] 15 error[E0309]: the parameter type `T` may not live long enough
[00:47:21] +   --> $DIR/regions-enum-not-wf.rs:33:25
[00:47:21] 17    |
[00:47:21] 17    |
[00:47:21] 18 LL | enum Ref2<'a, T> {
[00:47:21] 19    |               - help: consider adding an explicit lifetime bound `T: 'a`...
[00:47:21] 22    |                         ^^^^^^^^^^^^^^^^^^^^^^
[00:47:21] 23    |
[00:47:21] 23    |
[00:47:21] 24 note: ...so that the type `T` will meet its required lifetime bounds
[00:47:21] +   --> $DIR/regions-enum-not-wf.rs:33:25
[00:47:21] 26    |
[00:47:21] 26    |
[00:47:21] 27 LL |     Ref2Variant2(isize, RequireOutlives<'a, T>), //~ ERROR the parameter type `T` may not live long enough
[00:47:21] 
[00:47:21] 29 
[00:47:21] 29 
[00:47:21] 30 error[E0309]: the parameter type `T` may not live long enough
[00:47:21] -   --> $DIR/regions-enum-not-wf.he parameter type `T` may not live long enough","code":{"code":"E0309","explanation":"\nTypes in type definitions have lifetimes associated with them that represent\nhow long the data stored within them is guaranteed to be live. This lifetime\nmust be as long as the data needs to be alive, and missing the constraint that\ndenotes this will cause this error.\n\n