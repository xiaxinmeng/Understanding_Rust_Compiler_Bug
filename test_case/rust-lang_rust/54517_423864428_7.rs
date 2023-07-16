\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs","byte_start":297,"byte_end":302,"line_start":18,"line_end":18,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    Bench;","highlight_start":5,"highlight_end":10}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0425]: cannot find value `Bench` in this scope\n  --> /checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs:18:5\n   |\nLL |     Bench;\n   |     ^^^^^ not found in this scope\n\n"}
[01:10:40] {"message":"cannot find value `NonExistent` in this scope","code":{"code":"E0425","explanation":"\nAn unresolved name was used.\n\nErroneous code examptin-attrs.rs`
[01:10:40] error: 1 errors occurred comparing output.
[01:10:40] status: exit code: 1
[01:10:40] status: exit code: 1
[01:10:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs/auxiliary" "-A" "unused"
[01:10:40] ------------------------------------------
[01:10:40] 
[01:10:40] ------------------------------------------
[01:10:40] stderr:
[01:10:40] stderr:
[01:10:40] ------------------------------------------
[01:10:40] {"message":"the name `Test` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n