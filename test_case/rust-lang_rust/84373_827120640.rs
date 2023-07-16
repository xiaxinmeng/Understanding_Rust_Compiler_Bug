plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---

---- compile_test stdout ----
diff of stderr:

 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
+  --> $DIR/macro_use_imports.rs:24:5
    |
 LL |     #[macro_use]
 LL |     #[macro_use]
-   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};`
+   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::inner::nested::string_add;`
    |
    = note: `-D clippy::macro-use-imports` implied by `-D warnings`
 
 
 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
+  --> $DIR/macro_use_imports.rs:18:5
    |
 LL |     #[macro_use]
 LL |     #[macro_use]
-   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mini_mac::ClippyMiniMacroTest;`
+   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};`
 
 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
+  --> $DIR/macro_use_imports.rs:20:5
    |
 LL |     #[macro_use]
 LL |     #[macro_use]
-   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{inner::foofoo, inner::try_err};`
+   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mini_mac::ClippyMiniMacroTest;`
 
 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
+  --> $DIR/macro_use_imports.rs:22:5
    |
 LL |     #[macro_use]
 LL |     #[macro_use]
-   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::inner::nested::string_add;`
+   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{inner::foofoo, inner::try_err};`
 error: aborting due to 4 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/macro_use_imports.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macro_use_imports.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/macro_use_imports.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/macro_use_imports.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/macro_use_imports.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":533,"byte_end":545,"line_start":24,"line_end":24,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::macro-use-imports` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":533,"byte_end":545,"line_start":24,"line_end":24,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mac::inner::nested::string_add;","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:24:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::inner::nested::string_add;`\n   |\n   = note: `-D clippy::macro-use-imports` implied by `-D warnings`\n\n"}
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":431,"byte_end":443,"line_start":18,"line_end":18,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":431,"byte_end":443,"line_start":18,"line_end":18,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:18:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};`\n\n"}
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":461,"byte_end":473,"line_start":20,"line_end":20,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":461,"byte_end":473,"line_start":20,"line_end":20,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mini_mac::ClippyMiniMacroTest;","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:20:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mini_mac::ClippyMiniMacroTest;`\n\n"}
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":496,"byte_end":508,"line_start":22,"line_end":22,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":496,"byte_end":508,"line_start":22,"line_end":22,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mac::{inner::foofoo, inner::try_err};","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:22:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{inner::foofoo, inner::try_err};`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL | const C: Option<Option<i32>> = None;
    |
 note: the lint level is defined here
   --> $DIR/option_option.rs:1:9
    |
    |
 LL | #![deny(clippy::option_option)]
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL | static S: Option<Option<i32>> = None;
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL | fn input(_: Option<Option<u8>>) {}
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL | fn output() -> Option<Option<u8>> {
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL | fn output_nested() -> Vec<Option<Option<u8>>> {
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL | fn output_nested_nested() -> Option<Option<Option<u8>>> {
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
 LL |     x: Option<Option<u8>>,
    |        ^^^^^^^^^^^^^^^^^^
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL |     fn struct_fn() -> Option<Option<u8>> {
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
 LL |     fn trait_fn() -> Option<Option<u8>>;
    |                      ^^^^^^^^^^^^^^^^^^
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL |     Tuple(Option<Option<u8>>),
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL |     Struct { x: Option<Option<u8>> },
 
 
 error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
    |
    |
 LL |         foo: Option<Option<Cow<'a, str>>>,
 
-error: aborting due to 12 previous errors
-error: aborting due to 12 previous errors
+error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
+   |
+   |
+LL |         foo: Option<Option<Cow<'a, str>>>,
+
+
+error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases
+   |
+   |
+LL |         foo: Option<Option<Cow<'a, str>>>,
+
+error: aborting due to 14 previous errors
 
 
---
To only update this specific test, also pass `--test-args option_option.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/option_option.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/option_option.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/option_option.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":79,"byte_end":98,"line_start":4,"line_end":4,"column_start":10,"column_end":29,"is_primary":true,"text":[{"text":"const C: Option<Option<i32>> = None;","highlight_start":10,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":8,"byte_end":29,"line_start":1,"line_end":1,"column_start":9,"column_end":30,"is_primary":true,"text":[{"text":"#![deny(clippy::option_option)]","highlight_start":9,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:4:10\n   |\nLL | const C: Option<Option<i32>> = None;\n   |          ^^^^^^^^^^^^^^^^^^^\n   |\nnote: the lint level is defined here\n  --> tests/ui/option_option.rs:1:9\n   |\nLL | #![deny(clippy::option_option)]\n   |         ^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":117,"byte_end":136,"line_start":5,"line_end":5,"column_start":11,"column_end":30,"is_primary":true,"text":[{"text":"static S: Option<Option<i32>> = None;","highlight_start":11,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:5:11\n   |\nLL | static S: Option<Option<i32>> = None;\n   |           ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":158,"byte_end":176,"line_start":7,"line_end":7,"column_start":13,"column_end":31,"is_primary":true,"text":[{"text":"fn input(_: Option<Option<u8>>) {}","highlight_start":13,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:7:13\n   |\nLL | fn input(_: Option<Option<u8>>) {}\n   |             ^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":197,"byte_end":215,"line_start":9,"line_end":9,"column_start":16,"column_end":34,"is_primary":true,"text":[{"text":"fn output() -> Option<Option<u8>> {","highlight_start":16,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:9:16\n   |\nLL | fn output() -> Option<Option<u8>> {\n   |                ^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":256,"byte_end":274,"line_start":13,"line_end":13,"column_start":27,"column_end":45,"is_primary":true,"text":[{"text":"fn output_nested() -> Vec<Option<Option<u8>>> {","highlight_start":27,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:13:27\n   |\nLL | fn output_nested() -> Vec<Option<Option<u8>>> {\n   |                           ^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":373,"byte_end":399,"line_start":18,"line_end":18,"column_start":30,"column_end":56,"is_primary":true,"text":[{"text":"fn output_nested_nested() -> Option<Option<Option<u8>>> {","highlight_start":30,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:18:30\n   |\nLL | fn output_nested_nested() -> Option<Option<Option<u8>>> {\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":437,"byte_end":455,"line_start":23,"line_end":23,"column_start":8,"column_end":26,"is_primary":true,"text":[{"text":"    x: Option<Option<u8>>,","highlight_start":8,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:23:8\n   |\nLL |     x: Option<Option<u8>>,\n   |        ^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":496,"byte_end":514,"line_start":27,"line_end":27,"column_start":23,"column_end":41,"is_primary":true,"text":[{"text":"    fn struct_fn() -> Option<Option<u8>> {","highlight_start":23,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:27:23\n   |\nLL |     fn struct_fn() -> Option<Option<u8>> {\n   |                       ^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":574,"byte_end":592,"line_start":33,"line_end":33,"column_start":22,"column_end":40,"is_primary":true,"text":[{"text":"    fn trait_fn() -> Option<Option<u8>>;","highlight_start":22,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:33:22\n   |\nLL |     fn trait_fn() -> Option<Option<u8>>;\n   |                      ^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":619,"byte_end":637,"line_start":37,"line_end":37,"column_start":11,"column_end":29,"is_primary":true,"text":[{"text":"    Tuple(Option<Option<u8>>),","highlight_start":11,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:37:11\n   |\nLL |     Tuple(Option<Option<u8>>),\n   |           ^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":656,"byte_end":674,"line_start":38,"line_end":38,"column_start":17,"column_end":35,"is_primary":true,"text":[{"text":"    Struct { x: Option<Option<u8>> },","highlight_start":17,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:38:17\n   |\nLL |     Struct { x: Option<Option<u8>> },\n   |                 ^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":1480,"byte_end":1508,"line_start":79,"line_end":79,"column_start":14,"column_end":42,"is_primary":true,"text":[{"text":"        foo: Option<Option<Cow<'a, str>>>,","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:79:14\n   |\nLL |         foo: Option<Option<Cow<'a, str>>>,\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":1480,"byte_end":1508,"line_start":79,"line_end":79,"column_start":14,"column_end":42,"is_primary":true,"text":[{"text":"        foo: Option<Option<Cow<'a, str>>>,","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:79:14\n   |\nLL |         foo: Option<Option<Cow<'a, str>>>,\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases","code":{"code":"clippy::option_option","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_option.rs","byte_start":1480,"byte_end":1508,"line_start":79,"line_end":79,"column_start":14,"column_end":42,"is_primary":true,"text":[{"text":"        foo: Option<Option<Cow<'a, str>>>,","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: consider using `Option<T>` instead of `Option<Option<T>>` or a custom enum if you need to distinguish all 3 cases\n  --> tests/ui/option_option.rs:79:14\n   |\nLL |         foo: Option<Option<Cow<'a, str>>>,\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
