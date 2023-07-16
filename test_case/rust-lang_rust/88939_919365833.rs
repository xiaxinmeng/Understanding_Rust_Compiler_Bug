plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ec9a1bdc4586eec99acbe34df3717b3fd1277b06 and f54d20ab77cbdcf6fbb859bfb35530ae832d4bab
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
   Compiling stable_deref_trait v1.2.0
   Compiling cpuid-bool v0.1.2
   Compiling unicode-width v0.1.8
   Compiling unic-char-range v0.9.0
   Compiling unic-common v0.9.0
   Compiling serde_derive v1.0.125
   Compiling termcolor v1.1.2
   Compiling serde v1.0.125
---
   Compiling thread_local v1.0.1
   Compiling sharded-slab v0.1.1
   Compiling itertools v0.9.0
   Compiling getopts v0.2.21
   Compiling unic-char-property v0.9.0
   Compiling unic-ucd-version v0.9.0
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling num-traits v0.2.12
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling generic-array v0.14.4
   Compiling unicode-normalization v0.1.13
   Compiling unic-emoji-char v0.9.0
   Compiling stacker v0.1.14
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
   Compiling ena v0.14.0
---
   Compiling stable_deref_trait v1.2.0
   Compiling opaque-debug v0.3.0
   Compiling cpuid-bool v0.1.2
   Compiling unicode-width v0.1.8
   Compiling unic-common v0.9.0
   Compiling unic-char-range v0.9.0
   Compiling serde_derive v1.0.125
   Compiling termcolor v1.1.2
   Compiling serde v1.0.125
   Compiling annotate-snippets v0.8.0
---
   Compiling tracing-core v0.1.17
   Compiling sharded-slab v0.1.1
   Compiling thread_local v1.0.1
   Compiling itertools v0.9.0
   Compiling unic-ucd-version v0.9.0
   Compiling getopts v0.2.21
   Compiling unic-char-property v0.9.0
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling generic-array v0.14.4
   Compiling generic-array v0.14.4
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling unicode-normalization v0.1.13
   Compiling unic-emoji-char v0.9.0
   Compiling psm v0.1.16
   Compiling stacker v0.1.14
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
   Compiling rustc_lexer v0.1.0 (/checkout/compiler/rustc_lexer)
---

---- compile_test stdout ----
diff of stderr:

-error: methods called `as_*` usually take `self` by reference or `self` by mutable reference
-  --> $DIR/def_id_nocore.rs:26:19
+error[E0718]: `start` language item must be applied to a function with 1 generic argument
    |
    |
-LL |     pub fn as_ref(self) -> &'static str {
-   |                   ^^^^
-   |
-   = note: `-D clippy::wrong-self-convention` implied by `-D warnings`
-   = help: consider choosing a less ambiguous name
+LL | #[lang = "start"]
+   | ^^^^^^^^^^^^^^^^^
+LL | #[start]
+LL | fn start(_argc: isize, _argv: *const *const u8) -> isize {
+   |         - this function has 0 generic arguments
 error: aborting due to previous error
 
+For more information about this error, try `rustc --explain E0718`.
 
---
To only update this specific test, also pass `--test-args def_id_nocore.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/def_id_nocore.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/def_id_nocore.stage-id" "-A" "unused" "--emit=metadata" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/def_id_nocore.stage-id.aux"
error: test failed, to rerun pass '--test compile-test'
------------------------------------------

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
{"message":"`start` language item must be applied to a function with 1 generic argument","code":{"code":"E0718","explanation":"A `#[lang = \"..\"]` attribute was placed on the wrong item type.\n\nErroneous code example:\n\n