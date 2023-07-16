plain

---- compile_test stdout ----
diff of stderr:

-error[E0658]: destructuring assignments are unstable
-  --> $DIR/ice-6250.rs:12:25
-   |
-LL |         Some(reference) = cache.data.get(key) {
-   |         --------------- ^
-   |         |
-   |         cannot assign to this expression
-   |
-   = note: see issue #71126 <https://github.com/rust-lang/rust/issues/71126> for more information
-   = help: add `#![feature(destructuring_assignment)]` to the crate attributes to enable
-
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
 LL |     for reference in vec![1, 2, 3] {
 ...
 ...
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
-error: aborting due to 4 previous errors
+error: aborting due to 3 previous errors
 
-Some errors have detailed explanations: E0308, E0601, E0658.
---
To only update this specific test, also pass `--test-args crashes/ice-6250.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-6250.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6250.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6250.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`main` function not found in crate `ice_6250`","code":{"code":"E0601","explanation":"No `main` function was found in a binary crate.\n\nTo fix this error, add a `main` function:\n\n