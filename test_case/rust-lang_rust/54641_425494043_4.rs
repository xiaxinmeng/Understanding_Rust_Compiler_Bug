\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-struct-not-wf.rs","byte_start":857,"byte_end":878,"line_start":35,"line_end":35,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    type Out = &'a &'b T;","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the pointer is valid for the lifetime 'a as defined on the impl at 34:6","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-struct-not-wf.rs","byte_start":814,"byte_end":816,"line_start":34,"line_end":34,"column_start":6,"column_end":8,"is_primary":true,"text":[{"text":"impl<'a, 'b, T> Trait1<'a, 'b, T> for u32 {","highlight_start":6,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"but the referenced data is only valid for the lifetime 'b as defined on the impl at 34:10","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-struct-not-wf.rs","byte_start":818,"byte_end":820,"line_start":34,"line_end":34,"column_start":10,"column_end":12,"is_primary":true,"text":[{"text":"impl<'a, 'b, T> Trait1<'a, 'b, T> for u32 {","highlight_start":10,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0491]: in type `&'a &'b T`, reference has a longer lifetime than the data it references\n  --> /checkout/src/test/ui/regions/regions-struct-not-wf.rs:35:5\n   |\nLL |     type Out = &'a &'b T;\n   |     ^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: the pointer is valid for the lifetime 'a as defined on the impl at 34:6\n  --> /checkout/src/test/ui/regions/regions-struct-not-wf.rs:34:6\n   |\nLL | impl<'a, 'b, T> Trait1<'a, 'b, T> for u32 {\n   |      ^^\nnote: but the referenced data is only valid for the lifetime 'b as defined on the impl at 34:10\n  --> /checkout/src/test/ui/regions/regions-struct-not-wf.rs:34:10\n   |\nLL | impl<'a, 'b, T> Trait1<'a, 'b, T> for u32 {\n   |          ^^\n\n"}
[00:52:11] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:11] {"message":"For more information about this error, try `rustc --explain E0491`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0491`.\n"}
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] thread '[ui] ui/regions/regions-struct-not-wf.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:52:11] 
[00:52:11] 
[00:52:11] ---- [ui] ui/rfc-2093-infer-outlives/dont-infer-static.rs stdout ----
[00:52:11] 
[00:52:11] error: ui test compiled successfully!
[00:52:11] status: exit code: 0
[00:52:11] command: "inux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/regions-enum-not-wf/auxiliary" "-A" "unused"
[00:52:11] ------------------------------------------
[00:52:11] 
[00:52:11] ------------------------------------------
[00:52:11] stderr:
---
[00:52:11] 
[00:52:11] ---- [ui] ui/rfc-2093-infer-outlives/regions-struct-not-wf.rs stdout ----
[00:52:11] diff of stderr:
[00:52:11] 
[00:52:11] - error[E0309]: the parameter type `T` may not live long enough
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | impl<'a, T> Trait<'a, T> for usize {
[00:52:11] -    |          - help: consider adding an explicit lifetime bound `T: 'a`...
[00:52:11] - LL |     type Out = &'a T;
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - note: ...so that the reference type `&'a T` does not outlive the data it points at
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL |     type Out = &'a T;
[00:52:11] - 
[00:52:11] - 
[00:52:11] - error[E0309]: the parameter type `T` may not live long enough
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL | impl<'a, T> Trait<'a, T> for u32 {
[00:52:11] -    |          - help: consider adding an explicit lifetime bound `T: 'a`...
[00:52:11] - LL |     type Out = RefOk<'a, T>;
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - note: ...so that the type `T` will meet its required lifetime bounds
[00:52:11] -    |
[00:52:11] -    |
[00:52:11] - LL |     type Out = RefOk<'a, T>;
[00:52:11] - 
[00:52:11] - 
[00:52:11] 29 error[E0491]: in type `&'a &'b T`, reference has a longer lifetime than the data it references
[00:52:11] 31    |
[00:52:11] 
[00:52:11] 
[00:52:11] 43 LL | impl<'a, 'b, T> Trait1<'a, 'b, T> for u32 {
[00:52:11] 45 
[00:52:11] - error: aborting due to 3 previous errors
[00:52:11] + error: aborting due to previous error
[00:52:11] 47 
[00:52:11] 47 
[00:52:11] - Some errors occurred: E0309, E0491.
[00:52:11] - For more information about an error, try `rustc --explain E0309`.
[00:52:11] + For more information about this error, try `rustc --explain E0491`.
[00:52:11] 50 
[00:52:11] 
[00:52:11] 
[00:52:11] The actual stderr differed from the expected stderr.
[00:52:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/regions-struct-not-wf/regions-struct-not-wf.stderr
[00:52:11] To update references, rerun the tests and pass the `--bless` flag
[00:52:11] To only update this specific test, also pass `--test-args rfc-2093-infer-outlives/regions-struct-not-wf.rs`
[00:52:11] error: 1 errors occurred comparing output.
[00:52:11] status: exit code: 1
[00:52:11] status: exit code: 1
e Output = &'a T; // compile error E0491\n}\n