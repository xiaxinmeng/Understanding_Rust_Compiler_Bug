\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-4036.rs","byte_start":686,"byte_end":690,"line_start":21,"line_end":21,"column_start":17,"column_end":21,"is_primary":true,"text":[{"text":"use serialize::{json, Decodable};","highlight_start":17,"highlight_end":21}],"label":"no `json` in the root","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved import `serialize::json`\n  --> /checkout/src/test/run-pass-fulldeps/issue-4036.rs:21:17\n   |\nLL | use serialize::{json, Decodable};\n   |                 ^^^^ no `json` in the root\n\n"}
[01:00:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:10] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] thread '[run-pass] run-pass-fulldeps/issue-4036.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:00:10] 
[01:00:10] 
[01:00:10] ---- [run-pass] run-pass-fulldeps/issue-40663.rs stdout ----
[01:00:10] 
[01:00:10] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_plugin.rs" failed to compile: 
[01:00:10] status: exit code: 1
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_plugin.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-40663/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-40663/auxiliary"
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] ------------------------------------------
[01:00:10] stderr:
[01:00:10] stderr:
[01:00:10] ------------------------------------------
[01:00:10] {"message":"this function takes 3 parameters but 4 parameters were supplied","code":{"code":"E0061","explanation":"\nThe number of arguments passed to a function must match the number of arguments\nspecified in the function signature.\n\nFor example, a function like:\n\n