\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-box.rs","byte_start":648,"byte_end":663,"line_start":20,"line_end":20,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"use serialize::json;","highlight_start":5,"highlight_end":20}],"label":"no `json` in the root","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved import `serialize::json`\n  --> /checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-box.rs:20:5\n   |\nLL | use serialize::json;\n   |     ^^^^^^^^^^^^^^^ no `json` in the root\n\n"}
[01:00:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:10] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] thread '[run-pass] run-pass-fulldeps/deriving-encodable-decodable-box.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:00:10] 
[01:00:10] 
[01:00:10] ---- [run-pass] run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs stdout ----
[01:00:10] 
[01:00:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:00:10] error: test compilation failed although it shouldn't!
[01:00:10] status: exit code: 1
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell/auxiliary"
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] ------------------------------------------
[01:00:10] stderr:
[01:00:10] stderr:
[01:00:10] ------------------------------------------
[01:00:10] {"message":"unresolved import `serialize::json`","code":{"code[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:10] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] thread '[run-pass] run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:00:10] 
[01:00:10] 
[01:00:10] ---- [run-pass] run-pass-fulldeps/extern-mod-syntax.rs stdout ----
[01:00:10] 
[01:00:10] error: test compilation failed although it shouldn't!
[01:00:10] status: exit code: 1
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/extern-mod-syntax.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/extern-mod-syntax/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/extern-mod-syntax/auxiliary"
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] ------------------------------------------
[01:00:10] stderr:
[01:00:10] stderr:
[01:00:10] ------------------------------------------
[01:00:10] {"message":"unresolved import `serialize::json`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n