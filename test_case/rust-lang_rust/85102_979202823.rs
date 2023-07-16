plain

 error[E0658]: destructuring assignments are unstable
   --> $DIR/ice-6250.rs:12:25
    |
 LL |         Some(reference) = cache.data.get(key) {
    |         --------------- ^
    |         cannot assign to this expression
    |
    = note: see issue #71126 <https://github.com/rust-lang/rust/issues/71126> for more information
    = help: add `#![feature(destructuring_assignment)]` to the crate attributes to enable
    = help: add `#![feature(destructuring_assignment)]` to the crate attributes to enable
 
 error[E0601]: `main` function not found in crate `ice_6250`
   --> $DIR/ice-6250.rs:4:1
    |
 LL | / pub struct Cache {
 LL | |     data: Vec<i32>,
 LL | | }
 LL | |
 LL | |     }
 LL | | }
    | |_^ consider adding a `main` function to `$DIR/ice-6250.rs`
 
 
 error[E0308]: mismatched types
   --> $DIR/ice-6250.rs:12:14
    |
+LL |     for reference in vec![1, 2, 3] {
+...
+...
 LL |         Some(reference) = cache.data.get(key) {
    |              ^^^^^^^^^ expected integer, found `&i32`
 help: consider dereferencing the borrow
    |
    |
 LL |         Some(*reference) = cache.data.get(key) {
 
 error[E0308]: mismatched types
   --> $DIR/ice-6250.rs:12:9
    |
    |
 LL |         Some(reference) = cache.data.get(key) {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
 error: aborting due to 4 previous errors
 
 Some errors have detailed explanations: E0308, E0601, E0658.
 For more information about an error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args crashes/ice-6250.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-6250.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6250.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6250.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"destructuring assignments are unstable","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n