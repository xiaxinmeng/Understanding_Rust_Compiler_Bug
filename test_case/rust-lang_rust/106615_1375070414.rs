plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between a377893da2cd7124e5a18c7116cbb70e16dd5541 and 92da50f6d7cc6885a82eaa72a9327e1db5863e3a
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

-error: methods called `as_*` usually take `self` by reference or `self` by mutable reference
+error: incorrect number of parameters for the `start` lang item
+  --> $DIR/def_id_nocore.rs:18:1
    |
    |
-LL |     pub fn as_ref(self) -> &'static str {
-   |                   ^^^^
+LL | fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize {
    |
-   = help: consider choosing a less ambiguous name
-   = help: consider choosing a less ambiguous name
-   = note: `-D clippy::wrong-self-convention` implied by `-D warnings`
+   = note: the `start` lang item should have four parameters, but found 3
+   = note: the `start` lang item should have the signature `fn(fn() -> T, isize, *const *const u8, u8) -> isize`
 
 error: aborting due to previous error
 
 
---
To only update this specific test, also pass `--test-args def_id_nocore.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/def_id_nocore.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/def_id_nocore.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-6626e77b87cd93c1.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-23187159d3b7d3bc.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-6bd734a538ea3818.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a230f7c0a2c9e71f.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-b766ad73a57a34f7.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-ce4f9359ffd56cec.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/def_id_nocore.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"incorrect number of parameters for the `start` lang item","code":null,"level":"error","spans":[{"file_name":"tests/ui/def_id_nocore.rs","byte_start":280,"byte_end":357,"line_start":18,"line_end":18,"column_start":1,"column_end":78,"is_primary":true,"text":[{"text":"fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize {","highlight_start":1,"highlight_end":78}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the `start` lang item should have four parameters, but found 3","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the `start` lang item should have the signature `fn(fn() -> T, isize, *const *const u8, u8) -> isize`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: incorrect number of parameters for the `start` lang item\n  --> tests/ui/def_id_nocore.rs:18:1\n   |\nLL | fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: the `start` lang item should have four parameters, but found 3\n   = note: the `start` lang item should have the signature `fn(fn() -> T, isize, *const *const u8, u8) -> isize`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.9.0/src/lib.rs:111:22
