plain

---- compile_test stdout ----
diff of stderr:

 error[E0601]: `main` function not found in crate `ice_6251`
   --> $DIR/ice-6251.rs:4:1
    |
 LL | / fn bug<T>() -> impl Iterator<Item = [(); { |x: [u8]| x }]> {
 LL | |     std::iter::empty()
 LL | | }
    | |_^ consider adding a `main` function to `$DIR/ice-6251.rs`
 
 error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
   --> $DIR/ice-6251.rs:4:45
    |
 LL | fn bug<T>() -> impl Iterator<Item = [(); { |x: [u8]| x }]> {
    |                                             ^ doesn't have a size known at compile-time
    |
    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
    = help: unsized fn params are gated as an unstable feature
 help: function arguments must have a statically known size, borrowed types always have a known size
    |
-LL | fn bug<T>() -> impl Iterator<Item = [(); { |&x: [u8]| x }]> {
-   |                                             ^
+LL | fn bug<T>() -> impl Iterator<Item = [(); { |x: &[u8]| x }]> {
 
 
 error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
   --> $DIR/ice-6251.rs:4:54
    |
 LL | fn bug<T>() -> impl Iterator<Item = [(); { |x: [u8]| x }]> {
    |                                                      ^ doesn't have a size known at compile-time
    |
    = help: the trait `std::marker::Sized` is not implemented for `[u8]`
    = note: the return type of a function must have a statically known size
 error[E0308]: mismatched types
   --> $DIR/ice-6251.rs:4:44
    |
    |
 LL | fn bug<T>() -> impl Iterator<Item = [(); { |x: [u8]| x }]> {
    |                                            ^^^^^^^^^^^ expected `usize`, found closure
    = note: expected type `usize`
    = note: expected type `usize`
            found closure `[closure@$DIR/ice-6251.rs:4:44: 4:55]`
 error: aborting due to 4 previous errors
 
 Some errors have detailed explanations: E0277, E0308, E0601.
 For more information about an error, try `rustc --explain E0277`.
 For more information about an error, try `rustc --explain E0277`.
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/crashes/ice-6251.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crashes/ice-6251.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/crashes/ice-6251.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/crashes/ice-6251.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/crashes/ice-6251.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`main` function not found in crate `ice_6251`","code":{"code":"E0601","explanation":"No `main` function was found in a binary crate.\n\nTo fix this error, add a `main` function:\n\n