plain

---- compile_test stdout ----
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
   --> $DIR/needless_lifetimes.rs:45:1
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
 LL | fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:50:1
    |
 LL | fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:62:1
    |
 LL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:86:1
    |
 LL | fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:116:5
    |
 LL |     fn self_and_out<'s>(&'s self) -> &'s u8 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:125:5
    |
 LL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:144:1
    |
 LL | fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:174:1
    |
 LL | fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:180:1
    |
 LL | fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:199:1
    |
 LL | fn named_input_elided_output<'a>(_arg: &'a str) -> &str {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:207:1
    |
 LL | fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:243:1
    |
 LL | fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:250:9
    |
 LL |         fn needless_lt<'a>(x: &'a u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:254:9
    |
 LL |         fn needless_lt<'a>(_x: &'a u8) {}
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:267:9
    |
 LL |         fn baz<'a>(&'a self) -> impl Foo + 'a {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:296:5
    |
 LL |     fn impl_trait_elidable_nested_named_lifetimes<'a>(i: &'a i32, f: impl for<'b> Fn(&'b i32) -> &'b i32) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:299:5
    |
 LL |     fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:308:5
    |
 LL |     fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:320:5
    |
 LL |     fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:335:5
    |
 LL |     fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:348:5
    |
 LL |     fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {
 
 
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:351:5
    |
 LL |     fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {
 
-error: aborting due to 25 previous errors
-error: aborting due to 25 previous errors
+error: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
+  --> $DIR/needless_lifetimes.rs:174:40
+   |
+LL | fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {
+   |
+   |
+   = note: `-D dyn-drop` implied by `-D warnings`
+error: aborting due to 26 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_lifetimes.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args needless_lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/needless_lifetimes.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_lifetimes.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-a05ee8fc41b4648a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-11da7531132c7401.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-70309b46dc6386a4.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-3496925cd229d067.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-eb1b439e42ed31e0.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_lifetimes.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":118,"byte_end":180,"line_start":4,"line_end":4,"column_start":1,"column_end":63,"is_primary":true,"text":[{"text":"fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-lifetimes` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:4:1\n   |\nLL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":184,"byte_end":256,"line_start":6,"line_end":6,"column_start":1,"column_end":73,"is_primary":true,"text":[{"text":"fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}","highlight_start":1,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:6:1\n   |\nLL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":512,"byte_end":558,"line_start":16,"line_end":16,"column_start":1,"column_end":47,"is_primary":true,"text":[{"text":"fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {","highlight_start":1,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:16:1\n   |\nLL | fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1103,"byte_end":1167,"line_start":45,"line_end":45,"column_start":1,"column_end":65,"is_primary":true,"text":[{"text":"fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:45:1\n   |\nLL | fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1223,"byte_end":1297,"line_start":50,"line_end":50,"column_start":1,"column_end":75,"is_primary":true,"text":[{"text":"fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>","highlight_start":1,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:50:1\n   |\nLL | fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1451,"byte_end":1504,"line_start":62,"line_end":62,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:62:1\n   |\nLL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":1913,"byte_end":1971,"line_start":86,"line_end":86,"column_start":1,"column_end":59,"is_primary":true,"text":[{"text":"fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>","highlight_start":1,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:86:1\n   |\nLL | fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":2505,"byte_end":2544,"line_start":116,"line_end":116,"column_start":5,"column_end":44,"is_primary":true,"text":[{"text":"    fn self_and_out<'s>(&'s self) -> &'s u8 {","highlight_start":5,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:116:5\n   |\nLL |     fn self_and_out<'s>(&'s self) -> &'s u8 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":2700,"byte_end":2754,"line_start":125,"line_end":125,"column_start":5,"column_end":59,"is_primary":true,"text":[{"text":"    fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}","highlight_start":5,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:125:5\n   |\nLL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":3175,"byte_end":3222,"line_start":144,"line_end":144,"column_start":1,"column_end":48,"is_primary":true,"text":[{"text":"fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {","highlight_start":1,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:144:1\n   |\nLL | fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4027,"byte_end":4082,"line_start":174,"line_end":174,"column_start":1,"column_end":56,"is_primary":true,"text":[{"text":"fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:174:1\n   |\nLL | fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4139,"byte_end":4190,"line_start":180,"line_end":180,"column_start":1,"column_end":52,"is_primary":true,"text":[{"text":"fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {","highlight_start":1,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:180:1\n   |\nLL | fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4671,"byte_end":4726,"line_start":199,"line_end":199,"column_start":1,"column_end":56,"is_primary":true,"text":[{"text":"fn named_input_elided_output<'a>(_arg: &'a str) -> &str {","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:199:1\n   |\nLL | fn named_input_elided_output<'a>(_arg: &'a str) -> &str {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4835,"byte_end":4900,"line_start":207,"line_end":207,"column_start":1,"column_end":66,"is_primary":true,"text":[{"text":"fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {","highlight_start":1,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:207:1\n   |\nLL | fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5523,"byte_end":5572,"line_start":243,"line_end":243,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {","highlight_start":1,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:243:1\n   |\nLL | fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5690,"byte_end":5720,"line_start":250,"line_end":250,"column_start":9,"column_end":39,"is_primary":true,"text":[{"text":"        fn needless_lt<'a>(x: &'a u8) {}","highlight_start":9,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:250:9\n   |\nLL |         fn needless_lt<'a>(x: &'a u8) {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5765,"byte_end":5796,"line_start":254,"line_end":254,"column_start":9,"column_end":40,"is_primary":true,"text":[{"text":"        fn needless_lt<'a>(_x: &'a u8) {}","highlight_start":9,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:254:9\n   |\nLL |         fn needless_lt<'a>(_x: &'a u8) {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":5964,"byte_end":6001,"line_start":267,"line_end":267,"column_start":9,"column_end":46,"is_primary":true,"text":[{"text":"        fn baz<'a>(&'a self) -> impl Foo + 'a {","highlight_start":9,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:267:9\n   |\nLL |         fn baz<'a>(&'a self) -> impl Foo + 'a {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":6711,"byte_end":6823,"line_start":296,"line_end":296,"column_start":5,"column_end":117,"is_primary":true,"text":[{"text":"    fn impl_trait_elidable_nested_named_lifetimes<'a>(i: &'a i32, f: impl for<'b> Fn(&'b i32) -> &'b i32) -> &'a i32 {","highlight_start":5,"highlight_end":117}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:296:5\n   |\nLL |     fn impl_trait_elidable_nested_named_lifetimes<'a>(i: &'a i32, f: impl for<'b> Fn(&'b i32) -> &'b i32) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":6849,"byte_end":6951,"line_start":299,"line_end":299,"column_start":5,"column_end":107,"is_primary":true,"text":[{"text":"    fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {","highlight_start":5,"highlight_end":107}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:299:5\n   |\nLL |     fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7098,"byte_end":7172,"line_start":308,"line_end":308,"column_start":5,"column_end":79,"is_primary":true,"text":[{"text":"    fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {","highlight_start":5,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:308:5\n   |\nLL |     fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7348,"byte_end":7410,"line_start":320,"line_end":320,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:320:5\n   |\nLL |     fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":7722,"byte_end":7792,"line_start":335,"line_end":335,"column_start":5,"column_end":75,"is_primary":true,"text":[{"text":"    fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {","highlight_start":5,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:335:5\n   |\nLL |     fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":8043,"byte_end":8112,"line_start":348,"line_end":348,"column_start":5,"column_end":74,"is_primary":true,"text":[{"text":"    fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {","highlight_start":5,"highlight_end":74}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:348:5\n   |\nLL |     fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":8140,"byte_end":8199,"line_start":351,"line_end":351,"column_start":5,"column_end":64,"is_primary":true,"text":[{"text":"    fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {","highlight_start":5,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/needless_lifetimes.rs:351:5\n   |\nLL |     fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped","code":{"code":"dyn_drop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_lifetimes.rs","byte_start":4066,"byte_end":4070,"line_start":174,"line_end":174,"column_start":40,"column_end":44,"is_primary":true,"text":[{"text":"fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {","highlight_start":40,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D dyn-drop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped\n  --> tests/ui/needless_lifetimes.rs:174:40\n   |\nLL | fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {\n   |                                        ^^^^\n   |\n   = note: `-D dyn-drop` implied by `-D warnings`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
