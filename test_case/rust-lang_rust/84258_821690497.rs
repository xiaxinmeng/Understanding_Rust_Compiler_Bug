plain

---- compile_test stdout ----
diff of stderr:

-error: called `filter(..).flat_map(..)` on an `Iterator`
-  --> $DIR/filter_methods.rs:8:21
+error: unknown lint: `clippy::clippy::let_underscore_drop`
    |
    |
-LL |       let _: Vec<_> = vec![5_i8; 6]
-   |  _____________________^
-LL | |         .into_iter()
-LL | |         .filter(|&x| x == 0)
-LL | |         .flat_map(|x| x.checked_mul(2))
-   | |_______________________________________^
+LL | #![allow(clippy::clippy::let_underscore_drop)]
+   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `clippy::let_underscore_drop`
    |
-   = note: `-D clippy::filter-map` implied by `-D warnings`
-   = help: this is more succinctly expressed by calling `.flat_map(..)` and filtering by returning `iter::empty()`
+   = note: `-D unknown-lints` implied by `-D warnings`
 
-error: called `filter_map(..).flat_map(..)` on an `Iterator`
-  --> $DIR/filter_methods.rs:14:21
+error: unknown lint: `clippy::clippy::let_underscore_drop`
    |
    |
-LL |       let _: Vec<_> = vec![5_i8; 6]
-   |  _____________________^
-LL | |         .into_iter()
-LL | |         .filter_map(|x| x.checked_mul(2))
-LL | |         .flat_map(|x| x.checked_mul(2))
-   | |_______________________________________^
-   |
-   = help: this is more succinctly expressed by calling `.flat_map(..)` and filtering by returning `iter::empty()`
+LL | #![allow(clippy::clippy::let_underscore_drop)]
+   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `clippy::let_underscore_drop`
 
-error: called `filter_map(..).map(..)` on an `Iterator`
-   |
-   |
-LL |       let _: Vec<_> = vec![5_i8; 6]
-   |  _____________________^
-LL | |         .into_iter()
-LL | |         .filter_map(|x| x.checked_mul(2))
-LL | |         .map(|x| x.checked_mul(2))
-   | |__________________________________^
-   |
-   = help: this is more succinctly expressed by only calling `.filter_map(..)` instead
-error: aborting due to 3 previous errors
+error: aborting due to 2 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/filter_methods.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args filter_methods.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/filter_methods.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/filter_methods.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/filter_methods.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unknown lint: `clippy::clippy::let_underscore_drop`","code":{"code":"unknown_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/filter_methods.rs","byte_start":49,"byte_end":84,"line_start":2,"line_end":2,"column_start":10,"column_end":45,"is_primary":true,"text":[{"text":"#![allow(clippy::clippy::let_underscore_drop)]","highlight_start":10,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unknown-lints` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/filter_methods.rs","byte_start":49,"byte_end":84,"line_start":2,"line_end":2,"column_start":10,"column_end":45,"is_primary":true,"text":[{"text":"#![allow(clippy::clippy::let_underscore_drop)]","highlight_start":10,"highlight_end":45}],"label":null,"suggested_replacement":"clippy::let_underscore_drop","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unknown lint: `clippy::clippy::let_underscore_drop`\n  --> tests/ui/filter_methods.rs:2:10\n   |\nLL | #![allow(clippy::clippy::let_underscore_drop)]\n   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `clippy::let_underscore_drop`\n   |\n   = note: `-D unknown-lints` implied by `-D warnings`\n\n"}
{"message":"unknown lint: `clippy::clippy::let_underscore_drop`","code":{"code":"unknown_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/filter_methods.rs","byte_start":49,"byte_end":84,"line_start":2,"line_end":2,"column_start":10,"column_end":45,"is_primary":true,"text":[{"text":"#![allow(clippy::clippy::let_underscore_drop)]","highlight_start":10,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/filter_methods.rs","byte_start":49,"byte_end":84,"line_start":2,"line_end":2,"column_start":10,"column_end":45,"is_primary":true,"text":[{"text":"#![allow(clippy::clippy::let_underscore_drop)]","highlight_start":10,"highlight_end":45}],"label":null,"suggested_replacement":"clippy::let_underscore_drop","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unknown lint: `clippy::clippy::let_underscore_drop`\n  --> tests/ui/filter_methods.rs:2:10\n   |\nLL | #![allow(clippy::clippy::let_underscore_drop)]\n   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `clippy::let_underscore_drop`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
