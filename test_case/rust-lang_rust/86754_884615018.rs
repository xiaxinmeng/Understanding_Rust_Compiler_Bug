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
 LL |         Some(reference) = cache.data.get(key) {
-   |              ^^^^^^^^^
-   |              |
-   |              expected integer, found `&i32`
-   |              help: consider dereferencing the borrow: `*reference`
+   |              ^^^^^^^^^ expected integer, found `&i32`
+help: consider dereferencing the borrow
+   |
+   |
+LL |         Some(*reference) = cache.data.get(key) {
 
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/crashes/ice-6250.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/crashes/ice-6250.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-a05ee8fc41b4648a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-3496925cd229d067.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-70309b46dc6386a4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-eb1b439e42ed31e0.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-11da7531132c7401.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/crashes/ice-6250.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"destructuring assignments are unstable","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n