plain

---- compile_test stdout ----
diff of stderr:

-error: the function has a cognitive complexity of (28/25)
-  --> $DIR/cognitive_complexity.rs:6:4
error: test failed, to rerun pass `--test compile-test`
+  --> $DIR/cognitive_complexity.rs:327:13
    |
-LL | fn main() {
-LL | fn main() {
-   |    ^^^^
+LL |     let _ = Ok(42)?;
+   |             ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
    |
-   = help: you could split it up into multiple smaller functions
-   = note: `-D clippy::cognitive-complexity` implied by `-D warnings`
-
-error: the function has a cognitive complexity of (7/1)
-  --> $DIR/cognitive_complexity.rs:91:4
+help: consider specifying the generic arguments
    |
-LL | fn kaboom() {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
+LL |     let _ = Ok::<i32, E>(42)?;
 
 
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:149:4
-   |
-LL | fn baa() {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
 
 
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:150:13
-   |
-LL |     let x = || match 99 {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:167:4
-LL | fn bar() {
-   |    ^^^
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:186:4
-LL | fn barr() {
-   |    ^^^^
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (3/1)
-  --> $DIR/cognitive_complexity.rs:196:4
-LL | fn barr2() {
-   |    ^^^^^
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:212:4
-   |
-LL | fn barrr() {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (3/1)
-  --> $DIR/cognitive_complexity.rs:222:4
-   |
-LL | fn barrr2() {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:238:4
-   |
-LL | fn barrrr() {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (3/1)
-  --> $DIR/cognitive_complexity.rs:248:4
-   |
-LL | fn barrrr2() {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:264:4
-LL | fn cake() {
-   |    ^^^^
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (4/1)
-  --> $DIR/cognitive_complexity.rs:274:8
-   |
-LL | pub fn read_file(input_path: &str) -> String {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:305:4
-   |
-LL | fn void(void: Void) {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (8/1)
-  --> $DIR/cognitive_complexity.rs:356:4
-   |
-LL | fn early_ret() -> i32 {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:377:13
-   |
-LL |     let x = |a: i32, b: i32| -> i32 {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:390:8
-   |
-LL |     fn moo(&self) {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:399:14
-LL |     async fn a() {
-   |              ^
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-
-error: the function has a cognitive complexity of (2/1)
-  --> $DIR/cognitive_complexity.rs:406:22
-   |
-LL |         pub async fn async_method() {
-   |
-   |
-   = help: you could split it up into multiple smaller functions
-error: aborting due to 19 previous errors
-
+For more information about this error, try `rustc --explain E0282`.
 
---
To only update this specific test, also pass `--test-args cognitive_complexity.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/cognitive_complexity.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/cognitive_complexity.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/cognitive_complexity.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"type annotations needed","code":{"code":"E0282","explanation":"The compiler could not infer a type and asked for a type annotation.\n\nErroneous code example:\n\n