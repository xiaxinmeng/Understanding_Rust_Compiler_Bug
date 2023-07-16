plain
diff of stderr:

-error: this `if` has the same function call as a previous `if`
-  --> $DIR/same_functions_in_if_condition.rs:37:15
+error[E0741]: `main::E` must implement `ConstParamTy` to be used as the type of a const generic parameter
    |
-LL |     } else if function() {
-   |               ^^^^^^^^^^
-   |               ^^^^^^^^^^
+LL |     fn generic<const P: E>() -> bool {
    |
-note: same as this
-  --> $DIR/same_functions_in_if_condition.rs:36:8
+help: add `#[derive(ConstParamTy)]` to the struct
+help: add `#[derive(ConstParamTy)]` to the struct
    |
-LL |     if function() {
-   |        ^^^^^^^^^^
-note: the lint level is defined here
-  --> $DIR/same_functions_in_if_condition.rs:2:9
+LL +     #[derive(ConstParamTy)]
+LL |     enum E {
    |
-LL | #![deny(clippy::same_functions_in_if_condition)]
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 
-error: this `if` has the same function call as a previous `if`
-  --> $DIR/same_functions_in_if_condition.rs:42:15
-  --> $DIR/same_functions_in_if_condition.rs:42:15
-   |
-LL |     } else if fn_arg(a) {
-   |
-note: same as this
-  --> $DIR/same_functions_in_if_condition.rs:41:8
-   |
-   |
-LL |     if fn_arg(a) {
+error: aborting due to previous error
 
-error: this `if` has the same function call as a previous `if`
-  --> $DIR/same_functions_in_if_condition.rs:47:15
-  --> $DIR/same_functions_in_if_condition.rs:47:15
-   |
-LL |     } else if obj.method() {
-   |
-note: same as this
-  --> $DIR/same_functions_in_if_condition.rs:46:8
-   |
-   |
-LL |     if obj.method() {
-
-error: this `if` has the same function call as a previous `if`
-  --> $DIR/same_functions_in_if_condition.rs:52:15
-   |
-   |
-LL |     } else if obj.method_arg(a) {
-   |
-note: same as this
-  --> $DIR/same_functions_in_if_condition.rs:51:8
-   |
-   |
-LL |     if obj.method_arg(a) {
-
-error: this `if` has the same function call as a previous `if`
-  --> $DIR/same_functions_in_if_condition.rs:59:15
-   |
-   |
-LL |     } else if v.pop().is_none() {
-   |
-note: same as this
-  --> $DIR/same_functions_in_if_condition.rs:57:8
-   |
-   |
-LL |     if v.pop().is_none() {
-
-error: this `if` has the same function call as a previous `if`
-  --> $DIR/same_functions_in_if_condition.rs:64:15
-   |
-   |
-LL |     } else if v.len() == 42 {
-   |
-note: same as this
-  --> $DIR/same_functions_in_if_condition.rs:62:8
-   |
-   |
-LL |     if v.len() == 42 {
-
-error: aborting due to 6 previous errors
-
+For more information about this error, try `rustc --explain E0741`.
---
To only update this specific test, also pass `--test-args same_functions_in_if_condition.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/same_functions_in_if_condition.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/same_functions_in_if_condition.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-781eafa059bcf7f7.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/same_functions_in_if_condition.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`main::E` must implement `ConstParamTy` to be used as the type of a const generic parameter","code":{"code":"E0741","explanation":"A non-structural-match type was used as the type of a const generic parameter.\n\nErroneous code example:\n\n