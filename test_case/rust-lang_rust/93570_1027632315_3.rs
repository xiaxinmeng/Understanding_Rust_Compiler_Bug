\n\nThis error occurs when an expression was used in a place where the compiler\nexpected an expression of a different type. It can occur in several cases, the\nmost common being when calling a function and passing an argument which has a\ndifferent type than the matching type in the function declaration.\n"},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-6250.rs","byte_start":231,"byte_end":268,"line_start":12,"line_end":12,"column_start":9,"column_end":46,"is_primary":true,"text":[{"text":"        Some(reference) = cache.data.get(key) {","highlight_start":9,"highlight_end":46}],"label":"expected `bool`, found `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0308]: mismatched types\n  --> tests/ui/crashes/ice-6250.rs:12:9\n   |\nLL |         Some(reference) = cache.data.get(key) {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`\n\n"}
{"message":"Some errors have detailed explanations: E0308, E0601.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"Some errors have detailed explanations: E0308, E0601.\n"}
{"message":"For more information about an error, try `rustc --explain E0308`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0308`.\n"}

------------------------------------------
------------------------------------------

diff of stderr:

 error[E0601]: `main` function not found in crate `ice_6251`
-  --> $DIR/ice-6251.rs:4:1
+  --> $DIR/ice-6251.rs:6:2
    |
    |
-LL | / fn bug<T>() -> impl Iterator<Item = [(); { |x: [u8]| x }]> {
-LL | |     std::iter::empty()
-LL | | }
-   | |_^ consider adding a `main` function to `$DIR/ice-6251.rs`
+LL | }
+   |  ^ consider adding a `main` function to `$DIR/ice-6251.rs`
 error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
   --> $DIR/ice-6251.rs:4:45
    |
    |
 LL | fn bug<T>() -> impl Iterator<Item = [(); { |x: [u8]| x }]> {
    |
    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
    = help: unsized fn params are gated as an unstable feature
    = help: unsized fn params are gated as an unstable feature
 help: function arguments must have a statically known size, borrowed types always have a known size
    |
 LL | fn bug<T>() -> impl Iterator<Item = [(); { |x: &[u8]| x }]> {
 
 error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
   --> $DIR/ice-6251.rs:4:54
    |
    |
 LL | fn bug<T>() -> impl Iterator<Item = [(); { |x: [u8]| x }]> {
    |
    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
    = note: the return type of a function must have a statically known size
 
 
 error[E0308]: mismatched types
   --> $DIR/ice-6251.rs:4:44
    |
 LL | fn bug<T>() -> impl Iterator<Item = [(); { |x: [u8]| x }]> {
    |
    = note: expected type `usize`
    = note: expected type `usize`
            found closure `[closure@$DIR/ice-6251.rs:4:44: 4:55]`
 error: aborting due to 4 previous errors
 
 Some errors have detailed explanations: E0277, E0308, E0601.
 For more information about an error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args crashes/ice-6251.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-6251.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6251.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-6688bfb3a0699fc9.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-84536a848ae0c873.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-b682a5a8a9c64b20.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6251.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`main` function not found in crate `ice_6251`","code":{"code":"E0601","explanation":"No `main` function was found in a binary crate.\n\nTo fix this error, add a `main` function:\n\n