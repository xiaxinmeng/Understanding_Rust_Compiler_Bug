plain

 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
   --> $DIR/macro_use_imports.rs:25:5
    |
 LL |     #[macro_use]
    |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::inner::nested::string_add;`
    |
    = note: `-D clippy::macro-use-imports` implied by `-D warnings`
 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
   --> $DIR/macro_use_imports.rs:21:5
    |
 LL |     #[macro_use]
 LL |     #[macro_use]
    |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mini_mac::ClippyMiniMacroTest;`
 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
   --> $DIR/macro_use_imports.rs:23:5
    |
 LL |     #[macro_use]
 LL |     #[macro_use]
    |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{inner::mut_mut, inner::try_err};`
 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
error: test failed, to rerun pass `--test compile-test`
   --> $DIR/macro_use_imports.rs:19:5
    |
    |
 LL |     #[macro_use]
-   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{pub_macro, function_macro, ty_macro, inner_mod_macro, pub_in_private_macro};`
+   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};`
 error: aborting due to 4 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/macro_use_imports.stage-id.stderr
diff of fixed:

 // aux-build:macro_rules.rs
 // aux-build:macro_use_helper.rs
 // aux-build:proc_macro_derive.rs
 // run-rustfix
 // ignore-32bit
 #![feature(lint_reasons)]
 #![feature(lint_reasons)]
 #![allow(unused_imports, unreachable_code, unused_variables, dead_code, unused_attributes)]
 #![allow(clippy::single_component_path_imports)]
 #![warn(clippy::macro_use_imports)]
 #[macro_use]
 extern crate macro_use_helper as mac;
 
 #[macro_use]
 #[macro_use]
 extern crate proc_macro_derive as mini_mac;
 
 mod a {
-    use mac::{pub_macro, function_macro, ty_macro, inner_mod_macro, pub_in_private_macro};
+    use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};
     use mac;
     use mini_mac::ClippyMiniMacroTest;
     use mini_mac;
     use mac::{inner::mut_mut, inner::try_err};
     use mac::inner;
     use mac::inner::nested::string_add;
     use mac::inner::nested;
 
     #[derive(ClippyMiniMacroTest)]
     struct Test;
     fn test() {
         pub_macro!();
         pub_macro!();
         inner_mod_macro!();
         pub_in_private_macro!(_var);
         function_macro!();
         let v: ty_macro!() = Vec::default();
         inner::try_err!();
         inner::mut_mut!();
         inner::mut_mut!();
         nested::string_add!();
 }
 
 
 // issue #7015, ICE due to calling `module_children` with local `DefId`
 use a as b;
 
 fn main() {}
 
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/macro_use_imports.stage-id.fixed
To only update this specific test, also pass `--test-args macro_use_imports.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/macro_use_imports.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/macro_use_imports.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f9e171d447eb61b6.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-23c15a454288152e.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-fc796eef9a515a44.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-d8aee1681c496322.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-e22c3fc74241d5e4.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/macro_use_imports.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":555,"byte_end":567,"line_start":25,"line_end":25,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::macro-use-imports` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":555,"byte_end":567,"line_start":25,"line_end":25,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mac::inner::nested::string_add;","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:25:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::inner::nested::string_add;`\n   |\n   = note: `-D clippy::macro-use-imports` implied by `-D warnings`\n\n"}
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":483,"byte_end":495,"line_start":21,"line_end":21,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":483,"byte_end":495,"line_start":21,"line_end":21,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mini_mac::ClippyMiniMacroTest;","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:21:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mini_mac::ClippyMiniMacroTest;`\n\n"}
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":518,"byte_end":530,"line_start":23,"line_end":23,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":518,"byte_end":530,"line_start":23,"line_end":23,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mac::{inner::mut_mut, inner::try_err};","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:23:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{inner::mut_mut, inner::try_err};`\n\n"}
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":453,"byte_end":465,"line_start":19,"line_end":19,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":453,"byte_end":465,"line_start":19,"line_end":19,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:19:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/lib.rs:111:22
