\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/methods/method-call-lifetime-args-fail.rs","byte_start":3170,"byte_end":3189,"line_start":73,"line_end":73,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    S::early::<'static>(S);","highlight_start":5,"highlight_end":24}],"label":"expected 2 lifetime arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0107]: wrong number of lifetime arguments: expected 2, found 1\n  --> /checkout/src/test/ui/methods/method-call-lifetime-args-fail.rs:73:5\n   |\nLL |     S::early::<'static>(S);\n   |     ^^^^[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:53] {"message":"For more information about this error, try `rustc --explain E0107`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0107`.\n"}
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] thread '[ui] ui/seq-args.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:53] 
[00:46:53] 
[00:46:53] ---- [ui] ui/structs/structure-constructor-type-mismatch.rs stdout ----
[00:46:53] diff of stderr:
[00:46:53] 
[00:46:53] 70    = note: expected type `f32`
[00:46:53] 71               found type `{integer}`
[00:46:53] - error[E0244]: wrong number of type arguments: expected 0, found 1
[00:46:53] + error[E0107]: wrong number of type arguments: expected 0, found 1
[00:46:53] 74   --> $DIR/structure-constructor-type-mismatch.rs:58:24
[00:46:53] 75    |
[00:46:53] 75    |
[00:46:53] 76 LL |     let pt3 = PointF::<i32> { //~ ERROR wrong number of type arguments
[00:46:53] 100    = note: expected type `f32`
[00:46:53] 100    = note: expected type `f32`
[00:46:53] 101               found type `{integer}`
[00:46:53] - error[E0244]: wrong number of type arguments: expected 0, found 1
[00:46:53] + error[E0107]: wrong number of type arguments: expected 0, found 1
[00:46:53] 104   --> $DIR/structure-constructor-type-mismatch.rs:64:18
[00:46:53] 105    |
[00:46:53] 105    |
[00:46:53] 106 LL |         PointF::<u32> { .. } => {} //~ ERROR wrong number of type arguments
[00:46:53] 135 
[00:46:53] 136 error: aborting due to 13 previous errors
[00:46:53] -------------
[00:46:53] -------------
[00:46:53] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n