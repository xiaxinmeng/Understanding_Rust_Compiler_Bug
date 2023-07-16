plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c3c0f80d6081092faff801542dd82f0e2420152b and 7c635ff4f6a4aa030049b6c4cc3a1352bb604797
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: methods called `as_*` usually take `self` by reference or `self` by mutable reference
+  --> $DIR/def_id_nocore.rs:27:19
    |
    |
 LL |     pub fn as_ref(self) -> &'static str {
    |
    |
    = note: `-D clippy::wrong-self-convention` implied by `-D warnings`
    = help: consider choosing a less ambiguous name
 error: aborting due to previous error
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/def_id_nocore.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args def_id_nocore.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/def_id_nocore.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/def_id_nocore.stage-id" "-A" "unused" "--emit=metadata" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/def_id_nocore.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"methods called `as_*` usually take `self` by reference or `self` by mutable reference","code":{"code":"clippy::wrong_self_convention","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/def_id_nocore.rs","byte_start":401,"byte_end":405,"line_start":27,"line_end":27,"column_start":19,"column_end":23,"is_primary":true,"text":[{"text":"    pub fn as_ref(self) -> &'static str {","highlight_start":19,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::wrong-self-convention` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider choosing a less ambiguous name","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: methods called `as_*` usually take `self` by reference or `self` by mutable reference\n  --> tests/ui/def_id_nocore.rs:27:19\n   |\nLL |     pub fn as_ref(self) -> &'static str {\n   |                   ^^^^\n   |\n   = note: `-D clippy::wrong-self-convention` implied by `-D warnings`\n   = help: consider choosing a less ambiguous name\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
