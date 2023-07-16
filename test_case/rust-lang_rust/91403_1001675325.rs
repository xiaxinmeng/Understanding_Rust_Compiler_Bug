plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f8abed9ed48bace6be0087bcd44ed534e239b8d8 and 874ca9631dfff708710de25ef4aff79a5e57d9c2
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
---- compile_test stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/manual_async_fn.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/manual_async_fn.stage-id" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/manual_async_fn.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_async_fn.fixed","byte_start":1520,"byte_end":1576,"line_start":83,"line_end":83,"column_start":1,"column_end":57,"is_primary":true,"text":[{"text":"async fn explicit<'a, 'b>(_: &'a i32, _: &'b i32) -> i32 { 42 }","highlight_start":1,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-lifetimes` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/manual_async_fn.fixed:83:1\n   |\nLL | async fn explicit<'a, 'b>(_: &'a i32, _: &'b i32) -> i32 { 42 }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:
error: test failed, to rerun pass '--test compile-test'

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
   --> $DIR/needless_lifetimes.rs:31:1
    |
 LL | async fn func<'a>(args: &[&'a str]) -> Option<&'a str> {
+   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
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
 
 error: aborting due to 26 previous errors
 
 
---
To only update this specific test, also pass `--test-args needless_lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/needless_lifetimes.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/needless_lifetimes.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/needless_lifetimes.stage-id.aux"
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

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
