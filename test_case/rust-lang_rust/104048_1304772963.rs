plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e30fb6a26f1ac406a346cbf79b41c92e84703a28 and 2f02b94ef967466f6cc8ab47d289218998923c82
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

-error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
-  --> $DIR/needless_lifetimes_impl_trait.rs:15:5
-   |
-LL |     fn baz<'a>(&'a self) -> impl Foo + 'a {
-   |
-note: the lint level is defined here
-  --> $DIR/needless_lifetimes_impl_trait.rs:1:9
-   |
-   |
-LL | #![deny(clippy::needless_lifetimes)]
-
-error: aborting due to previous error
-
-
---
To only update this specific test, also pass `--test-args crashes/needless_lifetimes_impl_trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/needless_lifetimes_impl_trait.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/needless_lifetimes_impl_trait.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/needless_lifetimes_impl_trait.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:11:1
    |
 LL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}
error: test failed, to rerun pass `--test compile-test`
    |
    |
    = note: `-D clippy::needless-lifetimes` implied by `-D warnings`
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:13:1
    |
 LL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:23:1
    |
 LL | fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:57:1
    |
 LL | fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:62:1
    |
 LL | fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:74:1
    |
 LL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:98:1
    |
 LL | fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:128:5
    |
 LL |     fn self_and_out<'s>(&'s self) -> &'s u8 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:137:5
    |
 LL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:156:1
    |
 LL | fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {
 
+error: this lifetime isn't used in the function definition
+  --> $DIR/needless_lifetimes.rs:161:34
+   |
+   |
+LL | fn struct_with_lt2<'a>(_foo: &'a Foo) -> &'a str {
+   |
+   |
+   = note: `-D clippy::extra-unused-lifetimes` implied by `-D warnings`
+error: this lifetime isn't used in the function definition
+  --> $DIR/needless_lifetimes.rs:180:39
+   |
+   |
+LL | fn trait_obj_elided<'a>(_arg: &'a dyn WithLifetime) -> &'a str {
+
+
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:186:1
    |
 LL | fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:192:1
    |
 LL | fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {
 
+error: this lifetime isn't used in the function definition
+  --> $DIR/needless_lifetimes.rs:197:33
+   |
+   |
+LL | fn alias_with_lt2<'a>(_foo: &'a FooAlias) -> &'a str {
+
+
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:211:1
    |
 LL | fn named_input_elided_output<'a>(_arg: &'a str) -> &str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:219:1
    |
 LL | fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:255:1
    |
 LL | fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:262:9
    |
 LL |         fn needless_lt<'a>(x: &'a u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:266:9
    |
 LL |         fn needless_lt<'a>(_x: &'a u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
-  --> $DIR/needless_lifetimes.rs:279:9
-   |
-LL |         fn baz<'a>(&'a self) -> impl Foo + 'a {
-
-
-error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:311:5
    |
 LL |     fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:320:5
    |
 LL |     fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:332:5
    |
 LL |     fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:347:5
    |
 LL |     fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:360:5
    |
 LL |     fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:363:5
    |
 LL |     fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:385:9
    |
 LL |         fn implicit<'a>(&'a self) -> &'a () {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:388:9
    |
 LL |         fn implicit_mut<'a>(&'a mut self) -> &'a () {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:399:9
    |
 LL |         fn lifetime_elsewhere<'a>(self: Box<Self>, here: &'a ()) -> &'a () {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:405:9
    |
 LL |         fn implicit<'a>(&'a self) -> &'a ();
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:406:9
    |
 LL |         fn implicit_provided<'a>(&'a self) -> &'a () {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:415:9
    |
 LL |         fn lifetime_elsewhere<'a>(self: Box<Self>, here: &'a ()) -> &'a ();
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:416:9
    |
 LL |         fn lifetime_elsewhere_provided<'a>(self: Box<Self>, here: &'a ()) -> &'a () {
 
-error: aborting due to 31 previous errors
+error: aborting due to 33 previous errors
 
---
To only update this specific test, also pass `--test-args needless_lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/needless_lifetimes.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_lifetimes.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_lifetimes.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":194,"byte_end":256,"line_start":11,"line_end":11,"column_start":1,"column_end":63,"is_primary":true,"text":[{"text":"fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-lifetimes` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:11:1\n   |\nLL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":260,"byte_end":332,"line_start":13,"line_end":13,"column_start":1,"column_end":73,"is_primary":true,"text":[{"text":"fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}","highlight_start":1,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:13:1\n   |\nLL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":588,"byte_end":634,"line_start":23,"line_end":23,"column_start":1,"column_end":47,"is_primary":true,"text":[{"text":"fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {","highlight_start":1,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:23:1\n   |\nLL | fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1297,"byte_end":1361,"line_start":57,"line_end":57,"column_start":1,"column_end":65,"is_primary":true,"text":[{"text":"fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:57:1\n   |\nLL | fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1417,"byte_end":1491,"line_start":62,"line_end":62,"column_start":1,"column_end":75,"is_primary":true,"text":[{"text":"fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>","highlight_start":1,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:62:1\n   |\nLL | fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1645,"byte_end":1698,"line_start":74,"line_end":74,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:74:1\n   |\nLL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":2107,"byte_end":2165,"line_start":98,"line_end":98,"column_start":1,"column_end":59,"is_primary":true,"text":[{"text":"fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>","highlight_start":1,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:98:1\n   |\nLL | fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":2699,"byte_end":2738,"line_start":128,"line_end":128,"column_start":5,"column_end":44,"is_primary":true,"text":[{"text":"    fn self_and_out<'s>(&'s self) -> &'s u8 {","highlight_start":5,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:128:5\n   |\nLL |     fn self_and_out<'s>(&'s self) -> &'s u8 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":2894,"byte_end":2948,"line_start":137,"line_end":137,"column_start":5,"column_end":59,"is_primary":true,"text":[{"text":"    fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}","highlight_start":5,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:137:5\n   |\nLL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":3369,"byte_end":3416,"line_start":156,"line_end":156,"column_start":1,"column_end":48,"is_primary":true,"text":[{"text":"fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {","highlight_start":1,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:156:1\n   |\nLL | fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"this lifetime isn't used in the function definition","code":{"code":"clippy::extra_unused_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":3557,"byte_end":3560,"line_start":161,"line_end":161,"column_start":34,"column_end":37,"is_primary":true,"text":[{"text":"fn struct_with_lt2<'a>(_foo: &'a Foo) -> &'a str {","highlight_start":34,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::extra-unused-lifetimes` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this lifetime isn't used in the function definition\n  --> tests/ui/needless_lifetimes.rs:161:34\n   |\nLL | fn struct_with_lt2<'a>(_foo: &'a Foo) -> &'a str {\n   |                                  ^^^\n   |\n   = note: `-D clippy::extra-unused-lifetimes` implied by `-D warnings`\n\n"}
{"message":"this lifetime isn't used in the function definition","code":{"code":"clippy::extra_unused_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4056,"byte_end":4068,"line_start":180,"line_end":180,"column_start":39,"column_end":51,"is_primary":true,"text":[{"text":"fn trait_obj_elided<'a>(_arg: &'a dyn WithLifetime) -> &'a str {","highlight_start":39,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this lifetime isn't used in the function definition\n  --> tests/ui/needless_lifetimes.rs:180:39\n   |\nLL | fn trait_obj_elided<'a>(_arg: &'a dyn WithLifetime) -> &'a str {\n   |                                       ^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4221,"byte_end":4276,"line_start":186,"line_end":186,"column_start":1,"column_end":56,"is_primary":true,"text":[{"text":"fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:186:1\n   |\nLL | fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4333,"byte_end":4384,"line_start":192,"line_end":192,"column_start":1,"column_end":52,"is_primary":true,"text":[{"text":"fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {","highlight_start":1,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:192:1\n   |\nLL | fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"this lifetime isn't used in the function definition","code":{"code":"clippy::extra_unused_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4529,"byte_end":4537,"line_start":197,"line_end":197,"column_start":33,"column_end":41,"is_primary":true,"text":[{"text":"fn alias_with_lt2<'a>(_foo: &'a FooAlias) -> &'a str {","highlight_start":33,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this lifetime isn't used in the function definition\n  --> tests/ui/needless_lifetimes.rs:197:33\n   |\nLL | fn alias_with_lt2<'a>(_foo: &'a FooAlias) -> &'a str {\n   |                                 ^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4865,"byte_end":4920,"line_start":211,"line_end":211,"column_start":1,"column_end":56,"is_primary":true,"text":[{"text":"fn named_input_elided_output<'a>(_arg: &'a str) -> &str {","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:211:1\n   |\nLL | fn named_input_elided_output<'a>(_arg: &'a str) -> &str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5029,"byte_end":5094,"line_start":219,"line_end":219,"column_start":1,"column_end":66,"is_primary":true,"text":[{"text":"fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {","highlight_start":1,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:219:1\n   |\nLL | fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5717,"byte_end":5766,"line_start":255,"line_end":255,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {","highlight_start":1,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:255:1\n   |\nLL | fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5884,"byte_end":5914,"line_start":262,"line_end":262,"column_start":9,"column_end":39,"is_primary":true,"text":[{"text":"        fn needless_lt<'a>(x: &'a u8) {}","highlight_start":9,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:262:9\n   |\nLL |         fn needless_lt<'a>(x: &'a u8) {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5959,"byte_end":5990,"line_start":266,"line_end":266,"column_start":9,"column_end":40,"is_primary":true,"text":[{"text":"        fn needless_lt<'a>(_x: &'a u8) {}","highlight_start":9,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:266:9\n   |\nLL |         fn needless_lt<'a>(_x: &'a u8) {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7041,"byte_end":7143,"line_start":311,"line_end":311,"column_start":5,"column_end":107,"is_primary":true,"text":[{"text":"    fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {","highlight_start":5,"highlight_end":107}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:311:5\n   |\nLL |     fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7290,"byte_end":7364,"line_start":320,"line_end":320,"column_start":5,"column_end":79,"is_primary":true,"text":[{"text":"    fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {","highlight_start":5,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:320:5\n   |\nLL |     fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7540,"byte_end":7602,"line_start":332,"line_end":332,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:332:5\n   |\nLL |     fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7914,"byte_end":7984,"line_start":347,"line_end":347,"column_start":5,"column_end":75,"is_primary":true,"text":[{"text":"    fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {","highlight_start":5,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:347:5\n   |\nLL |     fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":8235,"byte_end":8304,"line_start":360,"line_end":360,"column_start":5,"column_end":74,"is_primary":true,"text":[{"text":"    fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {","highlight_start":5,"highlight_end":74}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:360:5\n   |\nLL |     fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":8332,"byte_end":8391,"line_start":363,"line_end":363,"column_start":5,"column_end":64,"is_primary":true,"text":[{"text":"    fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {","highlight_start":5,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:363:5\n   |\nLL |     fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":8720,"byte_end":8755,"line_start":385,"line_end":385,"column_start":9,"column_end":44,"is_primary":true,"text":[{"text":"        fn implicit<'a>(&'a self) -> &'a () {","highlight_start":9,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:385:9\n   |\nLL |         fn implicit<'a>(&'a self) -> &'a () {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":8792,"byte_end":8835,"line_start":388,"line_end":388,"column_start":9,"column_end":52,"is_primary":true,"text":[{"text":"        fn implicit_mut<'a>(&'a mut self) -> &'a () {","highlight_start":9,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:388:9\n   |\nLL |         fn implicit_mut<'a>(&'a mut self) -> &'a () {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":9047,"byte_end":9113,"line_start":399,"line_end":399,"column_start":9,"column_end":75,"is_primary":true,"text":[{"text":"        fn lifetime_elsewhere<'a>(self: Box<Self>, here: &'a ()) -> &'a () {","highlight_start":9,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:399:9\n   |\nLL |         fn lifetime_elsewhere<'a>(self: Box<Self>, here: &'a ()) -> &'a () {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":9173,"byte_end":9208,"line_start":405,"line_end":405,"column_start":9,"column_end":44,"is_primary":true,"text":[{"text":"        fn implicit<'a>(&'a self) -> &'a ();","highlight_start":9,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:405:9\n   |\nLL |         fn implicit<'a>(&'a self) -> &'a ();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":9218,"byte_end":9262,"line_start":406,"line_end":406,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        fn implicit_provided<'a>(&'a self) -> &'a () {","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:406:9\n   |\nLL |         fn implicit_provided<'a>(&'a self) -> &'a () {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":9449,"byte_end":9515,"line_start":415,"line_end":415,"column_start":9,"column_end":75,"is_primary":true,"text":[{"text":"        fn lifetime_elsewhere<'a>(self: Box<Self>, here: &'a ()) -> &'a ();","highlight_start":9,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:415:9\n   |\nLL |         fn lifetime_elsewhere<'a>(self: Box<Self>, here: &'a ()) -> &'a ();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":9525,"byte_end":9600,"line_start":416,"line_end":416,"column_start":9,"column_end":84,"is_primary":true,"text":[{"text":"        fn lifetime_elsewhere_provided<'a>(self: Box<Self>, here: &'a ()) -> &'a () {","highlight_start":9,"highlight_end":84}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:416:9\n   |\nLL |         fn lifetime_elsewhere_provided<'a>(self: Box<Self>, here: &'a ()) -> &'a () {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
    |
    |
 LL | fn do_vec(x: &Vec<i64>) {
    |              ^^^^^^^^^ help: change this to: `&[i64]`
    |
    = note: `-D clippy::ptr-arg` implied by `-D warnings`
 
 error: writing `&mut Vec` instead of `&mut [_]` involves a new object where a slice will do
    |
    |
 LL | fn do_vec_mut(x: &mut Vec<i64>) {
    |                  ^^^^^^^^^^^^^ help: change this to: `&mut [i64]`
 
 error: writing `&String` instead of `&str` involves a new object where a slice will do
    |
    |
 LL | fn do_str(x: &String) {
    |              ^^^^^^^ help: change this to: `&str`
 
 error: writing `&mut String` instead of `&mut str` involves a new object where a slice will do
    |
    |
 LL | fn do_str_mut(x: &mut String) {
    |                  ^^^^^^^^^^^ help: change this to: `&mut str`
 
 error: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do
    |
    |
 LL | fn do_path(x: &PathBuf) {
    |               ^^^^^^^^ help: change this to: `&Path`
 
 error: writing `&mut PathBuf` instead of `&mut Path` involves a new object where a slice will do
    |
    |
 LL | fn do_path_mut(x: &mut PathBuf) {
    |                   ^^^^^^^^^^^^ help: change this to: `&mut Path`
 
 error: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
    |
    |
 LL |     fn do_vec(x: &Vec<i64>);
    |                  ^^^^^^^^^ help: change this to: `&[i64]`
 
 error: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
    |
    |
 LL | fn cloned(x: &Vec<u8>) -> Vec<u8> {
    |
 help: change this to
    |
    |
 LL ~ fn cloned(x: &[u8]) -> Vec<u8> {
 LL ~     let e = x.to_owned();
 LL |     let f = e.clone(); // OK
 LL |     let g = x;
 LL ~     let h = g.to_owned();
 LL |     let i = (e).clone();
 LL ~     x.to_owned()
 
 
 error: writing `&String` instead of `&str` involves a new object where a slice will do
    |
    |
 LL | fn str_cloned(x: &String) -> String {
    |
 help: change this to
    |
    |
 LL ~ fn str_cloned(x: &str) -> String {
 LL ~     let a = x.to_owned();
 LL ~     let b = x.to_owned();
 LL |     let c = b.clone();
 LL |     let d = a.clone().clone().clone();
 LL ~     x.to_owned()
 
 
 error: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do
    |
    |
 LL | fn path_cloned(x: &PathBuf) -> PathBuf {
    |
 help: change this to
    |
    |
 LL ~ fn path_cloned(x: &Path) -> PathBuf {
 LL ~     let a = x.to_path_buf();
 LL ~     let b = x.to_path_buf();
 LL |     let c = b.clone();
 LL |     let d = a.clone().clone().clone();
 LL ~     x.to_path_buf()
 
 
 error: writing `&String` instead of `&str` involves a new object where a slice will do
    |
    |
 LL | fn false_positive_capacity(x: &Vec<u8>, y: &String) {
    |
 help: change this to
    |
    |
 LL ~ fn false_positive_capacity(x: &Vec<u8>, y: &str) {
 LL |     let a = x.capacity();
 LL ~     let b = y.to_owned();
 LL ~     let c = y;
 
 error: using a reference to `Cow` is not recommended
   --> $DIR/ptr_arg.rs:88:25
    |
    |
 LL | fn test_cow_with_ref(c: &Cow<[i32]>) {}
    |                         ^^^^^^^^^^^ help: change this to: `&[i32]`
+error: this lifetime isn't used in the function definition
+  --> $DIR/ptr_arg.rs:90:19
+   |
+   |
+LL | fn test_cow(c: Cow<[i32]>) {
+   |
+   |
+   = note: `-D clippy::extra-unused-lifetimes` implied by `-D warnings`
+
 error: writing `&String` instead of `&str` involves a new object where a slice will do
    |
    |
 LL |     fn some_allowed(#[allow(clippy::ptr_arg)] _v: &Vec<u32>, _s: &String) {}
    |                                                                  ^^^^^^^ help: change this to: `&str`
 
 error: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
    |
    |
 LL |     fn foo_vec(vec: &Vec<u8>) {
    |
 help: change this to
    |
    |
 LL ~     fn foo_vec(vec: &[u8]) {
 LL ~         let _ = vec.to_owned().pop();
 LL ~         let _ = vec.to_owned().clone();
 
 
 error: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do
    |
    |
 LL |     fn foo_path(path: &PathBuf) {
    |
 help: change this to
    |
    |
 LL ~     fn foo_path(path: &Path) {
 LL ~         let _ = path.to_path_buf().pop();
 LL ~         let _ = path.to_path_buf().clone();
 
 
 error: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do
    |
    |
 LL |     fn foo_str(str: &PathBuf) {
    |
 help: change this to
    |
    |
 LL ~     fn foo_str(str: &Path) {
 LL ~         let _ = str.to_path_buf().pop();
 LL ~         let _ = str.to_path_buf().clone();
 
 
 error: writing `&mut Vec` instead of `&mut [_]` involves a new object where a slice will do
    |
    |
 LL | fn mut_vec_slice_methods(v: &mut Vec<u32>) {
    |                             ^^^^^^^^^^^^^ help: change this to: `&mut [u32]`
 
 error: writing `&mut Vec` instead of `&mut [_]` involves a new object where a slice will do
    |
    |
 LL | fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {
    |                 ^^^^^^^^^^^^^ help: change this to: `&mut [u32]`
 
 error: writing `&mut String` instead of `&mut str` involves a new object where a slice will do
    |
    |
 LL | fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {
    |                                   ^^^^^^^^^^^ help: change this to: `&mut str`
 
 error: writing `&mut PathBuf` instead of `&mut Path` involves a new object where a slice will do
    |
    |
 LL | fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {
    |                                                   ^^^^^^^^^^^^ help: change this to: `&mut Path`
-error: aborting due to 20 previous errors
+error: aborting due to 21 previous errors
 
 
---
To only update this specific test, also pass `--test-args ptr_arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/ptr_arg.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/ptr_arg.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/ptr_arg.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"writing `&Vec` instead of `&[_]` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":196,"byte_end":205,"line_start":8,"line_end":8,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"fn do_vec(x: &Vec<i64>) {","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::ptr-arg` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":196,"byte_end":205,"line_start":8,"line_end":8,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"fn do_vec(x: &Vec<i64>) {","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":"&[i64]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&Vec` instead of `&[_]` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:8:14\n   |\nLL | fn do_vec(x: &Vec<i64>) {\n   |              ^^^^^^^^^ help: change this to: `&[i64]`\n   |\n   = note: `-D clippy::ptr-arg` implied by `-D warnings`\n\n"}
{"message":"writing `&mut Vec` instead of `&mut [_]` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":248,"byte_end":261,"line_start":12,"line_end":12,"column_start":18,"column_end":31,"is_primary":true,"text":[{"text":"fn do_vec_mut(x: &mut Vec<i64>) {","highlight_start":18,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":248,"byte_end":261,"line_start":12,"line_end":12,"column_start":18,"column_end":31,"is_primary":true,"text":[{"text":"fn do_vec_mut(x: &mut Vec<i64>) {","highlight_start":18,"highlight_end":31}],"label":null,"suggested_replacement":"&mut [i64]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&mut Vec` instead of `&mut [_]` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:12:18\n   |\nLL | fn do_vec_mut(x: &mut Vec<i64>) {\n   |                  ^^^^^^^^^^^^^ help: change this to: `&mut [i64]`\n\n"}
{"message":"writing `&String` instead of `&str` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":300,"byte_end":307,"line_start":16,"line_end":16,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"fn do_str(x: &String) {","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":300,"byte_end":307,"line_start":16,"line_end":16,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"fn do_str(x: &String) {","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":"&str","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&String` instead of `&str` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:16:14\n   |\nLL | fn do_str(x: &String) {\n   |              ^^^^^^^ help: change this to: `&str`\n\n"}
{"message":"writing `&mut String` instead of `&mut str` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":357,"byte_end":368,"line_start":20,"line_end":20,"column_start":18,"column_end":29,"is_primary":true,"text":[{"text":"fn do_str_mut(x: &mut String) {","highlight_start":18,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":357,"byte_end":368,"line_start":20,"line_end":20,"column_start":18,"column_end":29,"is_primary":true,"text":[{"text":"fn do_str_mut(x: &mut String) {","highlight_start":18,"highlight_end":29}],"label":null,"suggested_replacement":"&mut str","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&mut String` instead of `&mut str` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:20:18\n   |\nLL | fn do_str_mut(x: &mut String) {\n   |                  ^^^^^^^^^^^ help: change this to: `&mut str`\n\n"}
{"message":"writing `&PathBuf` instead of `&Path` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":415,"byte_end":423,"line_start":24,"line_end":24,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"fn do_path(x: &PathBuf) {","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":415,"byte_end":423,"line_start":24,"line_end":24,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"fn do_path(x: &PathBuf) {","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":"&Path","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:24:15\n   |\nLL | fn do_path(x: &PathBuf) {\n   |               ^^^^^^^^ help: change this to: `&Path`\n\n"}
{"message":"writing `&mut PathBuf` instead of `&mut Path` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":474,"byte_end":486,"line_start":28,"line_end":28,"column_start":19,"column_end":31,"is_primary":true,"text":[{"text":"fn do_path_mut(x: &mut PathBuf) {","highlight_start":19,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":474,"byte_end":486,"line_start":28,"line_end":28,"column_start":19,"column_end":31,"is_primary":true,"text":[{"text":"fn do_path_mut(x: &mut PathBuf) {","highlight_start":19,"highlight_end":31}],"label":null,"suggested_replacement":"&mut Path","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&mut PathBuf` instead of `&mut Path` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:28:19\n   |\nLL | fn do_path_mut(x: &mut PathBuf) {\n   |                   ^^^^^^^^^^^^ help: change this to: `&mut Path`\n\n"}
{"message":"writing `&Vec` instead of `&[_]` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":577,"byte_end":586,"line_start":36,"line_end":36,"column_start":18,"column_end":27,"is_primary":true,"text":[{"text":"    fn do_vec(x: &Vec<i64>);","highlight_start":18,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":577,"byte_end":586,"line_start":36,"line_end":36,"column_start":18,"column_end":27,"is_primary":true,"text":[{"text":"    fn do_vec(x: &Vec<i64>);","highlight_start":18,"highlight_end":27}],"label":null,"suggested_replacement":"&[i64]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&Vec` instead of `&[_]` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:36:18\n   |\nLL |     fn do_vec(x: &Vec<i64>);\n   |                  ^^^^^^^^^ help: change this to: `&[i64]`\n\n"}
{"message":"writing `&Vec` instead of `&[_]` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":793,"byte_end":801,"line_start":49,"line_end":49,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"fn cloned(x: &Vec<u8>) -> Vec<u8> {","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":793,"byte_end":801,"line_start":49,"line_end":49,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"fn cloned(x: &Vec<u8>) -> Vec<u8> {","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":"&[u8]","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":828,"byte_end":837,"line_start":50,"line_end":50,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"    let e = x.clone();","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"x.to_owned()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":895,"byte_end":904,"line_start":53,"line_end":53,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"    let h = g.clone();","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"g.to_owned()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":935,"byte_end":944,"line_start":55,"line_end":55,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    x.clone()","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":"x.to_owned()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&Vec` instead of `&[_]` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:49:14\n   |\nLL | fn cloned(x: &Vec<u8>) -> Vec<u8> {\n   |              ^^^^^^^^\n   |\nhelp: change this to\n   |\nLL ~ fn cloned(x: &[u8]) -> Vec<u8> {\nLL ~     let e = x.to_owned();\nLL |     let f = e.clone(); // OK\nLL |     let g = x;\nLL ~     let h = g.to_owned();\nLL |     let i = (e).clone();\nLL ~     x.to_owned()\n   |\n\n"}
{"message":"writing `&String` instead of `&str` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":965,"byte_end":972,"line_start":58,"line_end":58,"column_start":18,"column_end":25,"is_primary":true,"text":[{"text":"fn str_cloned(x: &String) -> String {","highlight_start":18,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":965,"byte_end":972,"line_start":58,"line_end":58,"column_start":18,"column_end":25,"is_primary":true,"text":[{"text":"fn str_cloned(x: &String) -> String {","highlight_start":18,"highlight_end":25}],"label":null,"suggested_replacement":"&str","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":998,"byte_end":1007,"line_start":59,"line_end":59,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"    let a = x.clone();","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"x.to_owned()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":1021,"byte_end":1030,"line_start":60,"line_end":60,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"    let b = x.clone();","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"x.to_owned()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":1098,"byte_end":1107,"line_start":63,"line_end":63,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    x.clone()","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":"x.to_owned()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&String` instead of `&str` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:58:18\n   |\nLL | fn str_cloned(x: &String) -> String {\n   |                  ^^^^^^^\n   |\nhelp: change this to\n   |\nLL ~ fn str_cloned(x: &str) -> String {\nLL ~     let a = x.to_owned();\nLL ~     let b = x.to_owned();\nLL |     let c = b.clone();\nLL |     let d = a.clone().clone().clone();\nLL ~     x.to_owned()\n   |\n\n"}
{"message":"writing `&PathBuf` instead of `&Path` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":1129,"byte_end":1137,"line_start":66,"line_end":66,"column_start":19,"column_end":27,"is_primary":true,"text":[{"text":"fn path_cloned(x: &PathBuf) -> PathBuf {","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":1129,"byte_end":1137,"line_start":66,"line_end":66,"column_start":19,"column_end":27,"is_primary":true,"text":[{"text":"fn path_cloned(x: &PathBuf) -> PathBuf {","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":"&Path","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":1164,"byte_end":1173,"line_start":67,"line_end":67,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"    let a = x.clone();","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"x.to_path_buf()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":1187,"byte_end":1196,"line_start":68,"line_end":68,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"    let b = x.clone();","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"x.to_path_buf()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":1264,"byte_end":1273,"line_start":71,"line_end":71,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    x.clone()","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":"x.to_path_buf()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:66:19\n   |\nLL | fn path_cloned(x: &PathBuf) -> PathBuf {\n   |                   ^^^^^^^^\n   |\nhelp: change this to\n   |\nLL ~ fn path_cloned(x: &Path) -> PathBuf {\nLL ~     let a = x.to_path_buf();\nLL ~     let b = x.to_path_buf();\nLL |     let c = b.clone();\nLL |     let d = a.clone().clone().clone();\nLL ~     x.to_path_buf()\n   |\n\n"}
{"message":"writing `&String` instead of `&str` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":1320,"byte_end":1327,"line_start":74,"line_end":74,"column_start":44,"column_end":51,"is_primary":true,"text":[{"text":"fn false_positive_capacity(x: &Vec<u8>, y: &String) {","highlight_start":44,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":1320,"byte_end":1327,"line_start":74,"line_end":74,"column_start":44,"column_end":51,"is_primary":true,"text":[{"text":"fn false_positive_capacity(x: &Vec<u8>, y: &String) {","highlight_start":44,"highlight_end":51}],"label":null,"suggested_replacement":"&str","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":1369,"byte_end":1378,"line_start":76,"line_end":76,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"    let b = y.clone();","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"y.to_owned()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":1392,"byte_end":1402,"line_start":77,"line_end":77,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let c = y.as_str();","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":"y","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&String` instead of `&str` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:74:44\n   |\nLL | fn false_positive_capacity(x: &Vec<u8>, y: &String) {\n   |                                            ^^^^^^^\n   |\nhelp: change this to\n   |\nLL ~ fn false_positive_capacity(x: &Vec<u8>, y: &str) {\nLL |     let a = x.capacity();\nLL ~     let b = y.to_owned();\nLL ~     let c = y;\n   |\n\n"}
{"message":"using a reference to `Cow` is not recommended","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":1588,"byte_end":1599,"line_start":88,"line_end":88,"column_start":25,"column_end":36,"is_primary":true,"text":[{"text":"fn test_cow_with_ref(c: &Cow<[i32]>) {}","highlight_start":25,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":1588,"byte_end":1599,"line_start":88,"line_end":88,"column_start":25,"column_end":36,"is_primary":true,"text":[{"text":"fn test_cow_with_ref(c: &Cow<[i32]>) {}","highlight_start":25,"highlight_end":36}],"label":null,"suggested_replacement":"&[i32]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using a reference to `Cow` is not recommended\n  --> tests/ui/ptr_arg.rs:88:25\n   |\nLL | fn test_cow_with_ref(c: &Cow<[i32]>) {}\n   |                         ^^^^^^^^^^^ help: change this to: `&[i32]`\n\n"}
{"message":"this lifetime isn't used in the function definition","code":{"code":"clippy::extra_unused_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":1623,"byte_end":1624,"line_start":90,"line_end":90,"column_start":19,"column_end":20,"is_primary":true,"text":[{"text":"fn test_cow(c: Cow<[i32]>) {","highlight_start":19,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::extra-unused-lifetimes` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this lifetime isn't used in the function definition\n  --> tests/ui/ptr_arg.rs:90:19\n   |\nLL | fn test_cow(c: Cow<[i32]>) {\n   |                   ^\n   |\n   = note: `-D clippy::extra-unused-lifetimes` implied by `-D warnings`\n\n"}
{"message":"writing `&String` instead of `&str` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":2296,"byte_end":2303,"line_start":117,"line_end":117,"column_start":66,"column_end":73,"is_primary":true,"text":[{"text":"    fn some_allowed(#[allow(clippy::ptr_arg)] _v: &Vec<u32>, _s: &String) {}","highlight_start":66,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":2296,"byte_end":2303,"line_start":117,"line_end":117,"column_start":66,"column_end":73,"is_primary":true,"text":[{"text":"    fn some_allowed(#[allow(clippy::ptr_arg)] _v: &Vec<u32>, _s: &String) {}","highlight_start":66,"highlight_end":73}],"label":null,"suggested_replacement":"&str","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&String` instead of `&str` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:117:66\n   |\nLL |     fn some_allowed(#[allow(clippy::ptr_arg)] _v: &Vec<u32>, _s: &String) {}\n   |                                                                  ^^^^^^^ help: change this to: `&str`\n\n"}
{"message":"writing `&Vec` instead of `&[_]` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":3059,"byte_end":3067,"line_start":146,"line_end":146,"column_start":21,"column_end":29,"is_primary":true,"text":[{"text":"    fn foo_vec(vec: &Vec<u8>) {","highlight_start":21,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":3059,"byte_end":3067,"line_start":146,"line_end":146,"column_start":21,"column_end":29,"is_primary":true,"text":[{"text":"    fn foo_vec(vec: &Vec<u8>) {","highlight_start":21,"highlight_end":29}],"label":null,"suggested_replacement":"&[u8]","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":3087,"byte_end":3098,"line_start":147,"line_end":147,"column_start":17,"column_end":28,"is_primary":true,"text":[{"text":"        let _ = vec.clone().pop();","highlight_start":17,"highlight_end":28}],"label":null,"suggested_replacement":"vec.to_owned()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":3122,"byte_end":3133,"line_start":148,"line_end":148,"column_start":17,"column_end":28,"is_primary":true,"text":[{"text":"        let _ = vec.clone().clone();","highlight_start":17,"highlight_end":28}],"label":null,"suggested_replacement":"vec.to_owned()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&Vec` instead of `&[_]` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:146:21\n   |\nLL |     fn foo_vec(vec: &Vec<u8>) {\n   |                     ^^^^^^^^\n   |\nhelp: change this to\n   |\nLL ~     fn foo_vec(vec: &[u8]) {\nLL ~         let _ = vec.to_owned().pop();\nLL ~         let _ = vec.to_owned().clone();\n   |\n\n"}
{"message":"writing `&PathBuf` instead of `&Path` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":3172,"byte_end":3180,"line_start":151,"line_end":151,"column_start":23,"column_end":31,"is_primary":true,"text":[{"text":"    fn foo_path(path: &PathBuf) {","highlight_start":23,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":3172,"byte_end":3180,"line_start":151,"line_end":151,"column_start":23,"column_end":31,"is_primary":true,"text":[{"text":"    fn foo_path(path: &PathBuf) {","highlight_start":23,"highlight_end":31}],"label":null,"suggested_replacement":"&Path","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":3200,"byte_end":3212,"line_start":152,"line_end":152,"column_start":17,"column_end":29,"is_primary":true,"text":[{"text":"        let _ = path.clone().pop();","highlight_start":17,"highlight_end":29}],"label":null,"suggested_replacement":"path.to_path_buf()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":3236,"byte_end":3248,"line_start":153,"line_end":153,"column_start":17,"column_end":29,"is_primary":true,"text":[{"text":"        let _ = path.clone().clone();","highlight_start":17,"highlight_end":29}],"label":null,"suggested_replacement":"path.to_path_buf()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:151:23\n   |\nLL |     fn foo_path(path: &PathBuf) {\n   |                       ^^^^^^^^\n   |\nhelp: change this to\n   |\nLL ~     fn foo_path(path: &Path) {\nLL ~         let _ = path.to_path_buf().pop();\nLL ~         let _ = path.to_path_buf().clone();\n   |\n\n"}
{"message":"writing `&PathBuf` instead of `&Path` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":3285,"byte_end":3293,"line_start":156,"line_end":156,"column_start":21,"column_end":29,"is_primary":true,"text":[{"text":"    fn foo_str(str: &PathBuf) {","highlight_start":21,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":3285,"byte_end":3293,"line_start":156,"line_end":156,"column_start":21,"column_end":29,"is_primary":true,"text":[{"text":"    fn foo_str(str: &PathBuf) {","highlight_start":21,"highlight_end":29}],"label":null,"suggested_replacement":"&Path","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":3313,"byte_end":3324,"line_start":157,"line_end":157,"column_start":17,"column_end":28,"is_primary":true,"text":[{"text":"        let _ = str.clone().pop();","highlight_start":17,"highlight_end":28}],"label":null,"suggested_replacement":"str.to_path_buf()","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/ptr_arg.rs","byte_start":3348,"byte_end":3359,"line_start":158,"line_end":158,"column_start":17,"column_end":28,"is_primary":true,"text":[{"text":"        let _ = str.clone().clone();","highlight_start":17,"highlight_end":28}],"label":null,"suggested_replacement":"str.to_path_buf()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:156:21\n   |\nLL |     fn foo_str(str: &PathBuf) {\n   |                     ^^^^^^^^\n   |\nhelp: change this to\n   |\nLL ~     fn foo_str(str: &Path) {\nLL ~         let _ = str.to_path_buf().pop();\nLL ~         let _ = str.to_path_buf().clone();\n   |\n\n"}
{"message":"writing `&mut Vec` instead of `&mut [_]` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":3406,"byte_end":3419,"line_start":162,"line_end":162,"column_start":29,"column_end":42,"is_primary":true,"text":[{"text":"fn mut_vec_slice_methods(v: &mut Vec<u32>) {","highlight_start":29,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":3406,"byte_end":3419,"line_start":162,"line_end":162,"column_start":29,"column_end":42,"is_primary":true,"text":[{"text":"fn mut_vec_slice_methods(v: &mut Vec<u32>) {","highlight_start":29,"highlight_end":42}],"label":null,"suggested_replacement":"&mut [u32]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&mut Vec` instead of `&mut [_]` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:162:29\n   |\nLL | fn mut_vec_slice_methods(v: &mut Vec<u32>) {\n   |                             ^^^^^^^^^^^^^ help: change this to: `&mut [u32]`\n\n"}
{"message":"writing `&mut Vec` instead of `&mut [_]` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":4545,"byte_end":4558,"line_start":224,"line_end":224,"column_start":17,"column_end":30,"is_primary":true,"text":[{"text":"fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {","highlight_start":17,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":4545,"byte_end":4558,"line_start":224,"line_end":224,"column_start":17,"column_end":30,"is_primary":true,"text":[{"text":"fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {","highlight_start":17,"highlight_end":30}],"label":null,"suggested_replacement":"&mut [u32]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&mut Vec` instead of `&mut [_]` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:224:17\n   |\nLL | fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {\n   |                 ^^^^^^^^^^^^^ help: change this to: `&mut [u32]`\n\n"}
{"message":"writing `&mut String` instead of `&mut str` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":4563,"byte_end":4574,"line_start":224,"line_end":224,"column_start":35,"column_end":46,"is_primary":true,"text":[{"text":"fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {","highlight_start":35,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":4563,"byte_end":4574,"line_start":224,"line_end":224,"column_start":35,"column_end":46,"is_primary":true,"text":[{"text":"fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {","highlight_start":35,"highlight_end":46}],"label":null,"suggested_replacement":"&mut str","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&mut String` instead of `&mut str` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:224:35\n   |\nLL | fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {\n   |                                   ^^^^^^^^^^^ help: change this to: `&mut str`\n\n"}
{"message":"writing `&mut PathBuf` instead of `&mut Path` involves a new object where a slice will do","code":{"code":"clippy::ptr_arg","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":4579,"byte_end":4591,"line_start":224,"line_end":224,"column_start":51,"column_end":63,"is_primary":true,"text":[{"text":"fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {","highlight_start":51,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change this to","code":null,"level":"help","spans":[{"file_name":"tests/ui/ptr_arg.rs","byte_start":4579,"byte_end":4591,"line_start":224,"line_end":224,"column_start":51,"column_end":63,"is_primary":true,"text":[{"text":"fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {","highlight_start":51,"highlight_end":63}],"label":null,"suggested_replacement":"&mut Path","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: writing `&mut PathBuf` instead of `&mut Path` involves a new object where a slice will do\n  --> tests/ui/ptr_arg.rs:224:51\n   |\nLL | fn dyn_trait(a: &mut Vec<u32>, b: &mut String, c: &mut PathBuf) {\n   |                                                   ^^^^^^^^^^^^ help: change this to: `&mut Path`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.9.0/src/lib.rs:111:22
