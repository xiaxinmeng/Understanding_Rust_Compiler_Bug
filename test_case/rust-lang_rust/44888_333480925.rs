
[00:35:42] ---- [ui] ui/regions-fn-subtyping-return-static.rs stdout ----
[00:35:42] 	normalized stderr:
[00:35:42] error[E0308]: mismatched types
[00:35:42]   --> $DIR/regions-fn-subtyping-return-static.rs:51:12
[00:35:42]    |
[00:35:42] 51 |     want_F(bar); //~ ERROR E0308
[00:35:42]    |            ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
[00:35:42]    |
[00:35:42]    = note: expected type `for<'cx> fn(&'cx S) -> &'cx S`
[00:35:42]               found type `for<'a> fn(&'a S) -> &S {bar::<'_>}`
[00:35:42] 
[00:35:42] error: aborting due to previous error
[00:35:42] 
[00:35:42] 
[00:35:42] 
[00:35:42] expected stderr:
[00:35:42] error[E0308]: mismatched types
[00:35:42]   --> $DIR/regions-fn-subtyping-return-static.rs:51:12
[00:35:42]    |
[00:35:42] 51 |     want_F(bar); //~ ERROR E0308
[00:35:42]    |            ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
[00:35:42]    |
[00:35:42]    = note: expected type `for<'cx> fn(&'cx S) -> &'cx S`
[00:35:42]               found type `fn(&'a S) -> &S {bar::<'_>}`
[00:35:42] 
[00:35:42] error: aborting due to previous error
[00:35:42] 
[00:35:42] 
[00:35:42] 
[00:35:42] diff of stderr:
[00:35:42] 
[00:35:42]  error[E0308]: mismatched types
[00:35:42]    --> $DIR/regions-fn-subtyping-return-static.rs:51:12
[00:35:42]     |
[00:35:42]  51 |     want_F(bar); //~ ERROR E0308
[00:35:42]     |            ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
[00:35:42]     |
[00:35:42]     = note: expected type `for<'cx> fn(&'cx S) -> &'cx S`
[00:35:42] -              found type `fn(&'a S) -> &S {bar::<'_>}`
[00:35:42] +              found type `for<'a> fn(&'a S) -> &S {bar::<'_>}`
[00:35:42]  
[00:35:42]  error: aborting due to previous error
