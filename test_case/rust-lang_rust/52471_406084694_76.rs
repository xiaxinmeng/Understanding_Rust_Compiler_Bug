\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/blind-item-item-shadow.rs","byte_start":501,"byte_end":509,"line_start":13,"line_end":13,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use foo::foo;","highlight_start":5,"highlight_end":13}],"label":"`foo` reimported here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/blind-item-item-shadow.rs","byte_start":467,"byte_end":474,"line_start":11,"line_end":11,"column_start":1,"column_end":8,"is_primary":false,"text":[{"text":"mod foo { pub mod foo {  } }","highlight_start":1,"highlight_end":8}],"label":"previous definition of the module `foo` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`foo` must be defined only once in the type namespace of this module","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"You can use `as` to change the binding name of the import","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/blind-item-item-shadow.rs","byte_start":501,"byte_end":509,"line_start":13,"line_end":13,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use foo::foo;","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":"foo::foo as other_foo","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0255]: the name `foo` is defined multiple times\n  --> /checkout/src/test/ui/blind-item-item-shadow.rs:13:5\n   |\nLL | mod foo { pub mod foo {  } }\n   | ------- previous definition of the module `foo` here\nLL | \nLL | use foo::foo;\n   |     ^^^^^^^^ `foo` reimported here\n   |\n   = note: `foo` must be defined only once in the type namespace of this module\nhelp: You can use `as` to change the binding name of the import\n   |\nLL | use foo::foo as other_foo;\n   |     ^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:15] {"message":"For more information about this error, try `rustc --explain E0255`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0255`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/blind-item-item-shadow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/binary-op-on-double-ref.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binary-op-on-double-ref.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binary-op-on-double-ref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binary-op-on-double-ref/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"binary operation `%` cannot be applied to type `&&{integer}`","code":{"code":"E0369","explanation":"\nA binary operation was attempted on a type which doesn't support it.\nErroneous code example:\n\n