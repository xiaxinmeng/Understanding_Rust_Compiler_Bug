plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0b42deaccc2cbe17a68067aa5fdb76104369e1fd and 39f25e9939a8fd941df1c48cf8ab59d799aa6d97
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
---- compile_test stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/manual_async_fn.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/manual_async_fn.stage-id" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/manual_async_fn.stage-id.aux"
error: test failed, to rerun pass '--test compile-test'
------------------------------------------

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_async_fn.fixed","byte_start":1520,"byte_end":1576,"line_start":83,"line_end":83,"column_start":1,"column_end":57,"is_primary":true,"text":[{"text":"async fn explicit<'a, 'b>(_: &'a i32, _: &'b i32) -> i32 { 42 }","highlight_start":1,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-lifetimes` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/manual_async_fn.fixed:83:1\n   |\nLL | async fn explicit<'a, 'b>(_: &'a i32, _: &'b i32) -> i32 { 42 }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:4:1
    |
 LL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}
    |
    |
    = note: `-D clippy::needless-lifetimes` implied by `-D warnings`
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:6:1
    |
 LL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:16:1
    |
 LL | fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
+  --> $DIR/needless_lifetimes.rs:31:1
+   |
+LL | async fn func<'a>(args: &[&'a str]) -> Option<&'a str> {
+
+
+error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:50:1
    |
 LL | fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:55:1
    |
 LL | fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:67:1
    |
 LL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:91:1
    |
 LL | fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:121:5
    |
 LL |     fn self_and_out<'s>(&'s self) -> &'s u8 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:130:5
    |
 LL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:149:1
    |
 LL | fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:179:1
    |
 LL | fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:185:1
    |
 LL | fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:204:1
    |
 LL | fn named_input_elided_output<'a>(_arg: &'a str) -> &str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:212:1
    |
 LL | fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:248:1
    |
 LL | fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:255:9
    |
 LL |         fn needless_lt<'a>(x: &'a u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:259:9
    |
 LL |         fn needless_lt<'a>(_x: &'a u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:272:9
    |
 LL |         fn baz<'a>(&'a self) -> impl Foo + 'a {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:301:5
    |
 LL |     fn impl_trait_elidable_nested_named_lifetimes<'a>(i: &'a i32, f: impl for<'b> Fn(&'b i32) -> &'b i32) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:304:5
    |
 LL |     fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:313:5
    |
 LL |     fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:325:5
    |
 LL |     fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:340:5
    |
 LL |     fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:353:5
    |
 LL |     fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:356:5
    |
 LL |     fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {
 
-error: aborting due to 25 previous errors
+error: aborting due to 26 previous errors
 
---
To only update this specific test, also pass `--test-args needless_lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/needless_lifetimes.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/needless_lifetimes.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/needless_lifetimes.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":128,"byte_end":190,"line_start":4,"line_end":4,"column_start":1,"column_end":63,"is_primary":true,"text":[{"text":"fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-lifetimes` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:4:1\n   |\nLL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":194,"byte_end":266,"line_start":6,"line_end":6,"column_start":1,"column_end":73,"is_primary":true,"text":[{"text":"fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}","highlight_start":1,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:6:1\n   |\nLL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":522,"byte_end":568,"line_start":16,"line_end":16,"column_start":1,"column_end":47,"is_primary":true,"text":[{"text":"fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {","highlight_start":1,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:16:1\n   |\nLL | fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":831,"byte_end":885,"line_start":31,"line_end":31,"column_start":1,"column_end":55,"is_primary":true,"text":[{"text":"async fn func<'a>(args: &[&'a str]) -> Option<&'a str> {","highlight_start":1,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:31:1\n   |\nLL | async fn func<'a>(args: &[&'a str]) -> Option<&'a str> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1231,"byte_end":1295,"line_start":50,"line_end":50,"column_start":1,"column_end":65,"is_primary":true,"text":[{"text":"fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:50:1\n   |\nLL | fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1351,"byte_end":1425,"line_start":55,"line_end":55,"column_start":1,"column_end":75,"is_primary":true,"text":[{"text":"fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>","highlight_start":1,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:55:1\n   |\nLL | fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1579,"byte_end":1632,"line_start":67,"line_end":67,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:67:1\n   |\nLL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":2041,"byte_end":2099,"line_start":91,"line_end":91,"column_start":1,"column_end":59,"is_primary":true,"text":[{"text":"fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>","highlight_start":1,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:91:1\n   |\nLL | fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":2633,"byte_end":2672,"line_start":121,"line_end":121,"column_start":5,"column_end":44,"is_primary":true,"text":[{"text":"    fn self_and_out<'s>(&'s self) -> &'s u8 {","highlight_start":5,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:121:5\n   |\nLL |     fn self_and_out<'s>(&'s self) -> &'s u8 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":2828,"byte_end":2882,"line_start":130,"line_end":130,"column_start":5,"column_end":59,"is_primary":true,"text":[{"text":"    fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}","highlight_start":5,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:130:5\n   |\nLL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":3303,"byte_end":3350,"line_start":149,"line_end":149,"column_start":1,"column_end":48,"is_primary":true,"text":[{"text":"fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {","highlight_start":1,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:149:1\n   |\nLL | fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4155,"byte_end":4210,"line_start":179,"line_end":179,"column_start":1,"column_end":56,"is_primary":true,"text":[{"text":"fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:179:1\n   |\nLL | fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4267,"byte_end":4318,"line_start":185,"line_end":185,"column_start":1,"column_end":52,"is_primary":true,"text":[{"text":"fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {","highlight_start":1,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:185:1\n   |\nLL | fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4799,"byte_end":4854,"line_start":204,"line_end":204,"column_start":1,"column_end":56,"is_primary":true,"text":[{"text":"fn named_input_elided_output<'a>(_arg: &'a str) -> &str {","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:204:1\n   |\nLL | fn named_input_elided_output<'a>(_arg: &'a str) -> &str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4963,"byte_end":5028,"line_start":212,"line_end":212,"column_start":1,"column_end":66,"is_primary":true,"text":[{"text":"fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {","highlight_start":1,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:212:1\n   |\nLL | fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5651,"byte_end":5700,"line_start":248,"line_end":248,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {","highlight_start":1,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:248:1\n   |\nLL | fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5818,"byte_end":5848,"line_start":255,"line_end":255,"column_start":9,"column_end":39,"is_primary":true,"text":[{"text":"        fn needless_lt<'a>(x: &'a u8) {}","highlight_start":9,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:255:9\n   |\nLL |         fn needless_lt<'a>(x: &'a u8) {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5893,"byte_end":5924,"line_start":259,"line_end":259,"column_start":9,"column_end":40,"is_primary":true,"text":[{"text":"        fn needless_lt<'a>(_x: &'a u8) {}","highlight_start":9,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:259:9\n   |\nLL |         fn needless_lt<'a>(_x: &'a u8) {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":6092,"byte_end":6129,"line_start":272,"line_end":272,"column_start":9,"column_end":46,"is_primary":true,"text":[{"text":"        fn baz<'a>(&'a self) -> impl Foo + 'a {","highlight_start":9,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:272:9\n   |\nLL |         fn baz<'a>(&'a self) -> impl Foo + 'a {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":6839,"byte_end":6951,"line_start":301,"line_end":301,"column_start":5,"column_end":117,"is_primary":true,"text":[{"text":"    fn impl_trait_elidable_nested_named_lifetimes<'a>(i: &'a i32, f: impl for<'b> Fn(&'b i32) -> &'b i32) -> &'a i32 {","highlight_start":5,"highlight_end":117}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:301:5\n   |\nLL |     fn impl_trait_elidable_nested_named_lifetimes<'a>(i: &'a i32, f: impl for<'b> Fn(&'b i32) -> &'b i32) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":6977,"byte_end":7079,"line_start":304,"line_end":304,"column_start":5,"column_end":107,"is_primary":true,"text":[{"text":"    fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {","highlight_start":5,"highlight_end":107}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:304:5\n   |\nLL |     fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7226,"byte_end":7300,"line_start":313,"line_end":313,"column_start":5,"column_end":79,"is_primary":true,"text":[{"text":"    fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {","highlight_start":5,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:313:5\n   |\nLL |     fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7476,"byte_end":7538,"line_start":325,"line_end":325,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:325:5\n   |\nLL |     fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7850,"byte_end":7920,"line_start":340,"line_end":340,"column_start":5,"column_end":75,"is_primary":true,"text":[{"text":"    fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {","highlight_start":5,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:340:5\n   |\nLL |     fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":8171,"byte_end":8240,"line_start":353,"line_end":353,"column_start":5,"column_end":74,"is_primary":true,"text":[{"text":"    fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {","highlight_start":5,"highlight_end":74}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:353:5\n   |\nLL |     fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":8268,"byte_end":8327,"line_start":356,"line_end":356,"column_start":5,"column_end":64,"is_primary":true,"text":[{"text":"    fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {","highlight_start":5,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:356:5\n   |\nLL |     fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: unneeded unit return type
-  --> $DIR/unused_unit.rs:19:28
+  --> $DIR/unused_unit.rs:19:58
    |
 LL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()
-   |                            ^^^^^^ help: remove the `-> ()`
+   |                                                          ^^^^^^ help: remove the `-> ()`
 note: the lint level is defined here
   --> $DIR/unused_unit.rs:12:9
    |
    |
 LL | #![deny(clippy::unused_unit)]
 
 error: unneeded unit return type
-  --> $DIR/unused_unit.rs:20:18
+  --> $DIR/unused_unit.rs:19:28
+  --> $DIR/unused_unit.rs:19:28
    |
-LL |     where G: Fn() -> () {
-   |                  ^^^^^^ help: remove the `-> ()`
+LL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()
+   |                            ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
-  --> $DIR/unused_unit.rs:19:58
+  --> $DIR/unused_unit.rs:20:18
    |
    |
-LL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()
-   |                                                          ^^^^^^ help: remove the `-> ()`
+LL |     where G: Fn() -> () {
+   |                  ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:21:26
    |
    |
 LL |         let _y: &dyn Fn() -> () = &f;
    |                          ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:28:18
    |
 LL |     fn into(self) -> () {
---
 
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:34:29
    |
 LL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)
    |                             ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:36:19
    |
    |
 LL |         G: FnMut() -> (),
    |                   ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:37:16
    |
    |
 LL |         H: Fn() -> ();
    |                ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:41:29
    |
    |
 LL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)
    |                             ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:43:19
    |
    |
 LL |         G: FnMut() -> (),
    |                   ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:44:16
    |
    |
 LL |         H: Fn() -> () {}
    |                ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:47:17
    |
    |
 LL | fn return_unit() -> () { () }
    |                 ^^^^^^ help: remove the `-> ()`
 error: unneeded unit expression
   --> $DIR/unused_unit.rs:47:26
    |
    |
 LL | fn return_unit() -> () { () }
    |                          ^^ help: remove the final `()`
 error: unneeded `()`
   --> $DIR/unused_unit.rs:57:14
    |
 LL |         break();
---
 
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:76:10
    |
 LL | fn test()->(){}
    |          ^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:79:11
    |
    |
 LL | fn test2() ->(){}
    |           ^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:82:11
    |
    |
 LL | fn test3()-> (){}
    |           ^^^^^ help: remove the `-> ()`
 error: aborting due to 19 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/unused_unit.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused_unit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unused_unit.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/unused_unit.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/unused_unit.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":611,"byte_end":617,"line_start":19,"line_end":19,"column_start":58,"column_end":64,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":58,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":413,"byte_end":432,"line_start":12,"line_end":12,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"#![deny(clippy::unused_unit)]","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":611,"byte_end":617,"line_start":19,"line_end":19,"column_start":58,"column_end":64,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":58,"highlight_end":64}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:19:58\n   |\nLL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()\n   |                                                          ^^^^^^ help: remove the `-> ()`\n   |\nnote: the lint level is defined here\n  --> tests/ui/unused_unit.rs:12:9\n   |\nLL | #![deny(clippy::unused_unit)]\n   |         ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":581,"byte_end":587,"line_start":19,"line_end":19,"column_start":28,"column_end":34,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":28,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":581,"byte_end":587,"line_start":19,"line_end":19,"column_start":28,"column_end":34,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":28,"highlight_end":34}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:19:28\n   |\nLL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()\n   |                            ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":635,"byte_end":641,"line_start":20,"line_end":20,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    where G: Fn() -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":635,"byte_end":641,"line_start":20,"line_end":20,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    where G: Fn() -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:20:18\n   |\nLL |     where G: Fn() -> () {\n   |                  ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":669,"byte_end":675,"line_start":21,"line_end":21,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"        let _y: &dyn Fn() -> () = &f;","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":669,"byte_end":675,"line_start":21,"line_end":21,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"        let _y: &dyn Fn() -> () = &f;","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:21:26\n   |\nLL |         let _y: &dyn Fn() -> () = &f;\n   |                          ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":830,"byte_end":836,"line_start":28,"line_end":28,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    fn into(self) -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":830,"byte_end":836,"line_start":28,"line_end":28,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    fn into(self) -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:28:18\n   |\nLL |     fn into(self) -> () {\n   |                  ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit expression","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":847,"byte_end":849,"line_start":29,"line_end":29,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        ()","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the final `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":847,"byte_end":849,"line_start":29,"line_end":29,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        ()","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit expression\n  --> tests/ui/unused_unit.rs:29:9\n   |\nLL |         ()\n   |         ^^ help: remove the final `()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":901,"byte_end":907,"line_start":34,"line_end":34,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":901,"byte_end":907,"line_start":34,"line_end":34,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:34:29\n   |\nLL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)\n   |                             ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":971,"byte_end":977,"line_start":36,"line_end":36,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":971,"byte_end":977,"line_start":36,"line_end":36,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:36:19\n   |\nLL |         G: FnMut() -> (),\n   |                   ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":994,"byte_end":1000,"line_start":37,"line_end":37,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> ();","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":994,"byte_end":1000,"line_start":37,"line_end":37,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> ();","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:37:16\n   |\nLL |         H: Fn() -> ();\n   |                ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1058,"byte_end":1064,"line_start":41,"line_end":41,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1058,"byte_end":1064,"line_start":41,"line_end":41,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:41:29\n   |\nLL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)\n   |                             ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1128,"byte_end":1134,"line_start":43,"line_end":43,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1128,"byte_end":1134,"line_start":43,"line_end":43,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:43:19\n   |\nLL |         G: FnMut() -> (),\n   |                   ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1151,"byte_end":1157,"line_start":44,"line_end":44,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> () {}","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1151,"byte_end":1157,"line_start":44,"line_end":44,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> () {}","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:44:16\n   |\nLL |         H: Fn() -> () {}\n   |                ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1180,"byte_end":1186,"line_start":47,"line_end":47,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1180,"byte_end":1186,"line_start":47,"line_end":47,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:47:17\n   |\nLL | fn return_unit() -> () { () }\n   |                 ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit expression","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1189,"byte_end":1191,"line_start":47,"line_end":47,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the final `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1189,"byte_end":1191,"line_start":47,"line_end":47,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit expression\n  --> tests/ui/unused_unit.rs:47:26\n   |\nLL | fn return_unit() -> () { () }\n   |                          ^^ help: remove the final `()`\n\n"}
{"message":"unneeded `()`","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1419,"byte_end":1421,"line_start":57,"line_end":57,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        break();","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1419,"byte_end":1421,"line_start":57,"line_end":57,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        break();","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `()`\n  --> tests/ui/unused_unit.rs:57:14\n   |\nLL |         break();\n   |              ^^ help: remove the `()`\n\n"}
{"message":"unneeded `()`","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1439,"byte_end":1441,"line_start":59,"line_end":59,"column_start":11,"column_end":13,"is_primary":true,"text":[{"text":"    return();","highlight_start":11,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1439,"byte_end":1441,"line_start":59,"line_end":59,"column_start":11,"column_end":13,"is_primary":true,"text":[{"text":"    return();","highlight_start":11,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `()`\n  --> tests/ui/unused_unit.rs:59:11\n   |\nLL |     return();\n   |           ^^ help: remove the `()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1701,"byte_end":1705,"line_start":76,"line_end":76,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"fn test()->(){}","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1701,"byte_end":1705,"line_start":76,"line_end":76,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"fn test()->(){}","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:76:10\n   |\nLL | fn test()->(){}\n   |          ^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1736,"byte_end":1741,"line_start":79,"line_end":79,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test2() ->(){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1736,"byte_end":1741,"line_start":79,"line_end":79,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test2() ->(){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:79:11\n   |\nLL | fn test2() ->(){}\n   |           ^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1772,"byte_end":1777,"line_start":82,"line_end":82,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test3()-> (){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1772,"byte_end":1777,"line_start":82,"line_end":82,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test3()-> (){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:82:11\n   |\nLL | fn test3()-> (){}\n   |           ^^^^^ help: remove the `-> ()`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
