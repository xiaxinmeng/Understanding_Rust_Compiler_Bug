\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs","byte_start":4406,"byte_end":4411,"line_start":115,"line_end":115,"column_start":22,"column_end":27,"is_primary":true,"text":[{"text":"                word.ident.segments.last().unwrap().ident","highlight_start":22,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0609]: no field `ident` on type `&syntax::ast::MetaItem`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs:115:22\n   |\nLL |                 word.ident.segments.last().unwrap().ident\n   |                      ^^^^^\n\n"}
[01:00:47] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:47] {"message":"For more information about this error, try `rustc --explain E0609`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0609`.\n"}
[01:00:47] ------------------------------------------
[01:00:47] 
[01:00:47] thread '[run-pass] run-pass-fulldeps/macro-crate-does-hygiene-work.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:00:47] 
[01:00:47] 
[01:00:47] ---- [run-pass] run-pass-fulldeps/macro-crate-multi-decorator-literals.rs stdout ----
[01:00:47] 
[01:00:47] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" failed to compile: 
[01:00:47] status: exit code: 1
[01:00:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/macro_crate_test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown!(\"{}\", s.foo); // error: no field `foo` on type `StructWithFields`\n