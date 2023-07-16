plain
normalized stderr:
error: this operation will panic at runtime
  --> $DIR/ice-5497.rs:9:22
   |
LL |     const OOB: i32 = [1][1] + T::OOB;
   |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1
   = note: `#[deny(unconditional_panic)]` on by default

error: aborting due to previous error

---
To only update this specific test, also pass `--test-args crashes/ice-5497.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-5497.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-5497.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-5497.stage-id.aux"
------------------------------------------

------------------------------------------
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
stderr:
------------------------------------------
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-5497.rs","byte_start":172,"byte_end":178,"line_start":9,"line_end":9,"column_start":22,"column_end":28,"is_primary":true,"text":[{"text":"    const OOB: i32 = [1][1] + T::OOB;","highlight_start":22,"highlight_end":28}],"label":"index out of bounds: the length is 1 but the index is 1","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(unconditional_panic)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/crashes/ice-5497.rs:9:22\n   |\nLL |     const OOB: i32 = [1][1] + T::OOB;\n   |                      ^^^^^^ index out of bounds: the length is 1 but the index is 1\n   |\n   = note: `#[deny(unconditional_panic)]` on by default\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
