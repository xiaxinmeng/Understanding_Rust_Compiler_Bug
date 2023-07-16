plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7a3d1a5f3dfeaf5177885fedd7db8ecc70670dc1 and 06f9b0827c7551f9e890d7e2b029ecf54dfef3cb
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: mutable key type
    |
    |
 LL | fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {
    |
    |
    = note: `-D clippy::mutable-key-type` implied by `-D warnings`
 
 error: mutable key type
    |
    |
 LL | fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {
 
 
 error: mutable key type
    |
    |
 LL |     let _other: HashMap<Key, bool> = HashMap::new();
 
 
 error: mutable key type
+  --> $DIR/mut_key.rs:45:22
    |
    |
 LL | fn tuples_bad<U>(_m: &mut HashMap<(Key, U), bool>) {}
 
 error: aborting due to 4 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/mut_key.stage-id.stderr
To only update this specific test, also pass `--test-args mut_key.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/mut_key.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/mut_key.stage-id" "-A" "unused" "--emit=metadata" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/mut_key.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"mutable key type","code":{"code":"clippy::mutable_key_type","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/mut_key.rs","byte_start":559,"byte_end":583,"line_start":27,"line_end":27,"column_start":32,"column_end":56,"is_primary":true,"text":[{"text":"fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {","highlight_start":32,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::mutable-key-type` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: mutable key type\n  --> tests/ui/mut_key.rs:27:32\n   |\nLL | fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {\n   |                                ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::mutable-key-type` implied by `-D warnings`\n\n"}
{"message":"mutable key type","code":{"code":"clippy::mutable_key_type","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/mut_key.rs","byte_start":599,"byte_end":611,"line_start":27,"line_end":27,"column_start":72,"column_end":84,"is_primary":true,"text":[{"text":"fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {","highlight_start":72,"highlight_end":84}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: mutable key type\n  --> tests/ui/mut_key.rs:27:72\n   |\nLL | fn should_not_take_this_arg(m: &mut HashMap<Key, usize>, _n: usize) -> HashSet<Key> {\n   |                                                                        ^^^^^^^^^^^^\n\n"}
{"message":"mutable key type","code":{"code":"clippy::mutable_key_type","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/mut_key.rs","byte_start":618,"byte_end":666,"line_start":28,"line_end":28,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    let _other: HashMap<Key, bool> = HashMap::new();","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: mutable key type\n  --> tests/ui/mut_key.rs:28:5\n   |\nLL |     let _other: HashMap<Key, bool> = HashMap::new();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"mutable key type","code":{"code":"clippy::mutable_key_type","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/mut_key.rs","byte_start":1008,"byte_end":1036,"line_start":45,"line_end":45,"column_start":22,"column_end":50,"is_primary":true,"text":[{"text":"fn tuples_bad<U>(_m: &mut HashMap<(Key, U), bool>) {}","highlight_start":22,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: mutable key type\n  --> tests/ui/mut_key.rs:45:22\n   |\nLL | fn tuples_bad<U>(_m: &mut HashMap<(Key, U), bool>) {}\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
