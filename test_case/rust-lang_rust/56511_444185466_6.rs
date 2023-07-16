\n\nMust always be called with exactly two arguments, e.g. `f(2, \"test\")`.\n\nNote that Rust does not have a notion of optional function arguments or\nvariadic functions (except for its C-FFI).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_plugin_attr.rs","byte_start":2341,"byte_end":2347,"line_start":71,"line_end":71,"column_start":15,"column_end":21,"is_primary":true,"text":[{"text":"    trait_def.expand(cx, mitem, item, push)","highlight_start":15,"highlight_end":21}],"label":"expected 3 parameters","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0061]: this function takes 3 parameters but 4 parameters were supplied\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_plugin_attr.rs:71:15\n   |\nLL |     trait_def.expand(cx, mitem, item, push)\n   |               ^^^^^^ expected 3 parameters\n\n"}
[01:00:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:10] {"message":"For more information about this error, try `rustc --explain E0061`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0061`.\n"}
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] thread '[run-pass] run-pass-fulldeps/derive-totalsum-attr.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:00:10] 
[01:00:10] 
[01:00:10] ---- [run-pass] run-pass-fulldeps/derive-totalsum.rs stdout ----
[01:00:10] 
[01:00:10] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_plugin.rs" failed to compile: 
[01:00:10] status: exit code: 1
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/custom_derive_plugin.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/derive-totalsum/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/derive-totalsum/auxiliary"
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] ------------------------------------------
[01:00:10] stderr:
[01:00:10] stderr:
[01:00:10] ------------------------------------------
[01:00:10] {"message":"this function takes 3 parameters but 4 parameters were supplied","code":{"code":"E0061","explanation":"\nThe number of arguments passed to a function must match the number of arguments\nspecified in the function signature.\n\nFor example, a function like:\n\n