plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4e8fb743ccbec27344b2dd42de7057f41d4ebfdd and ee3cfa1518001d9e25caf128e366ab51dd3881ca
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
To only update this specific test, also pass `--test-args fs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletestSia442" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestSia442/fs.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestSia442/fs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestSia442/portable-simd.stage-id.stderr
To only update this specific test, also pass `--test-args portable-simd.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/portable-simd.rs" "-L" "/tmp/compiletestSia442" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestSia442/portable-simd.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestSia442/portable-simd.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
   --> $DIR/macro_use_imports.rs:18:5
    |
 LL |     #[macro_use]
-   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};`
+   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{pub_macro, function_macro, ty_macro, inner_mod_macro, pub_in_private_macro};`
    |
    = note: `-D clippy::macro-use-imports` implied by `-D warnings`
 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
   --> $DIR/macro_use_imports.rs:20:5
    |
 LL |     #[macro_use]
 LL |     #[macro_use]
    |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mini_mac::ClippyMiniMacroTest;`
 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
   --> $DIR/macro_use_imports.rs:22:5
    |
 LL |     #[macro_use]
 LL |     #[macro_use]
    |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{inner::foofoo, inner::try_err};`
 error: `macro_use` attributes are no longer needed in the Rust 2018 edition
   --> $DIR/macro_use_imports.rs:24:5
    |
 LL |     #[macro_use]
 LL |     #[macro_use]
    |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::inner::nested::string_add;`
 error: aborting due to 4 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/macro_use_imports.stage-id.stderr
diff of fixed:

 // aux-build:macro_rules.rs
 // aux-build:macro_use_helper.rs
 // aux-build:proc_macro_derive.rs
 // run-rustfix
 // ignore-32bit
 
 #![allow(unused_imports, unreachable_code, unused_variables, dead_code, unused_attributes)]
 #![allow(clippy::single_component_path_imports)]
 #![warn(clippy::macro_use_imports)]
 #[macro_use]
 extern crate macro_use_helper as mac;
 
 #[macro_use]
 #[macro_use]
 extern crate proc_macro_derive as mini_mac;
 
 mod a {
-    use mac::{pub_macro, inner_mod_macro, function_macro, ty_macro, pub_in_private_macro};
+    use mac::{pub_macro, function_macro, ty_macro, inner_mod_macro, pub_in_private_macro};
     use mac;
     use mini_mac::ClippyMiniMacroTest;
     use mini_mac;
     use mac::{inner::foofoo, inner::try_err};
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
         inner::try_err!();
         inner::foofoo!();
         nested::string_add!();
 }
 
 
 // issue #7015, ICE due to calling `module_children` with local `DefId`
 use a as b;
 
 fn main() {}
 
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/macro_use_imports.stage-id.fixed
To only update this specific test, also pass `--test-args macro_use_imports.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/macro_use_imports.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/macro_use_imports.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-b682a5a8a9c64b20.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-6688bfb3a0699fc9.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-84536a848ae0c873.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/macro_use_imports.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":427,"byte_end":439,"line_start":18,"line_end":18,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::macro-use-imports` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":427,"byte_end":439,"line_start":18,"line_end":18,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mac::{pub_macro, function_macro, ty_macro, inner_mod_macro, pub_in_private_macro};","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:18:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{pub_macro, function_macro, ty_macro, inner_mod_macro, pub_in_private_macro};`\n   |\n   = note: `-D clippy::macro-use-imports` implied by `-D warnings`\n\n"}
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":457,"byte_end":469,"line_start":20,"line_end":20,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":457,"byte_end":469,"line_start":20,"line_end":20,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mini_mac::ClippyMiniMacroTest;","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:20:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mini_mac::ClippyMiniMacroTest;`\n\n"}
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":492,"byte_end":504,"line_start":22,"line_end":22,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":492,"byte_end":504,"line_start":22,"line_end":22,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mac::{inner::foofoo, inner::try_err};","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:22:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::{inner::foofoo, inner::try_err};`\n\n"}
{"message":"`macro_use` attributes are no longer needed in the Rust 2018 edition","code":{"code":"clippy::macro_use_imports","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":529,"byte_end":541,"line_start":24,"line_end":24,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the attribute and import the macro directly, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/macro_use_imports.rs","byte_start":529,"byte_end":541,"line_start":24,"line_end":24,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    #[macro_use]","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"use mac::inner::nested::string_add;","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `macro_use` attributes are no longer needed in the Rust 2018 edition\n  --> tests/ui/macro_use_imports.rs:24:5\n   |\nLL |     #[macro_use]\n   |     ^^^^^^^^^^^^ help: remove the attribute and import the macro directly, try: `use mac::inner::nested::string_add;`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
