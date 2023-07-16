\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-types-ICE-when-projecting-out-of-err.rs","byte_start":850,"byte_end":851,"line_start":33,"line_end":33,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    r = r + a;","highlight_start":11,"highlight_end":12}],"label":"the trait `Add<A>` is not implemented for `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0277]: the trait bound `(): Add<A>` is not satisfied\n  --> /checkout/src/test/ui/associated-types-ICE-when-projecting-out-of-err.rs:33:11\n   |\nLL |     r = r + a;\n   |           ^ the trait `Add<A>` is not implemented for `()`\n\n"}
[00:50:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:15] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/associated-types-ICE-when-projecting-out-of-err.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/associated-types-in-ambiguous-context.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types-in-ambiguous-context.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types-in-ambiguous-context/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types-in-ambiguous-context/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"ambiguous associated type","code":{"code":"E0223","explanation":"\nAn attempt was made to retrieve an associated type, but the type was ambiguous.\nFor example:\n\n