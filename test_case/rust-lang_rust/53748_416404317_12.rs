\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0404.rs","byte_start":553,"byte_end":556,"line_start":18,"line_end":18,"column_start":11,"column_end":14,"is_primary":true,"text":[{"text":"fn baz<T: Foo>(_: T) {} //~ ERROR E0404","highlight_start":11,"highlight_end":14}],"label":"not a tr
[00:47:57] 
[00:47:57] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:57] status: exit code: 101
[00:47:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0405.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0405/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0405/auxiliary" "-A" "unused"
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] stderr:
[00:47:57] stderr:
[00:47:57] ------------------------------------------
[00:47:57] {"message":"cannot find trait `SomeTrait` in this scope","code":{"code":"E0405","explanation":"\nThe code refers to a trait that is not in scope.\n\nErroneous code example:\n\n