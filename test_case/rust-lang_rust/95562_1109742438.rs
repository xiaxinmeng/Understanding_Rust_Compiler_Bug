plain
   Compiling miri v0.1.0 (/checkout/src/tools/miri)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/tools/miri/src/machine.rs:545:29
     |
545  |         let attrs = ecx.tcx.get_attrs(def_id);
     |                             ^^^^^^^^^-------- an argument of type `Symbol` is missing
note: associated function defined here
    --> /checkout/compiler/rustc_middle/src/ty/mod.rs:2179:12
     |
     |
2179 |     pub fn get_attrs(self, did: DefId, attr: Symbol) -> ty::Attributes<'tcx> {
help: provide the argument
     |
     |
545  |         let attrs = ecx.tcx.get_attrs(def_id, {Symbol});

error[E0308]: mismatched types
    --> src/tools/miri/src/machine.rs:546:73
     |
     |
546  |         let link_name = match ecx.tcx.sess.first_attr_value_str_by_name(&attrs, sym::link_name) {
     |                                            ---------------------------- ^^^^^^ expected slice, found opaque type
     |                                            arguments to this function are incorrect
     |
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:1925:29
     |
     |
1925 | pub type Attributes<'tcx> = impl Iterator<Item = &'tcx ast::Attribute>;
     |
     |
     = note: expected reference `&[Attribute]`
                found reference `&rustc_middle::ty::Attributes<'_>`
    --> /checkout/compiler/rustc_session/src/session.rs:1074:12
     |
1074 |     pub fn first_attr_value_str_by_name(
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/tools/miri/src/shims/foreign_items.rs:237:30
     |
237  |         let attrs = this.tcx.get_attrs(def_id);
     |                              ^^^^^^^^^-------- an argument of type `Symbol` is missing
note: associated function defined here
    --> /checkout/compiler/rustc_middle/src/ty/mod.rs:2179:12
     |
     |
2179 |     pub fn get_attrs(self, did: DefId, attr: Symbol) -> ty::Attributes<'tcx> {
help: provide the argument
     |
     |
237  |         let attrs = this.tcx.get_attrs(def_id, {Symbol});

error[E0308]: mismatched types
    --> src/tools/miri/src/shims/foreign_items.rs:241:43
     |
     |
241  |             .first_attr_value_str_by_name(&attrs, sym::link_name)
     |              ---------------------------- ^^^^^^ expected slice, found opaque type
     |              arguments to this function are incorrect
     |
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:1925:29
     |
     |
1925 | pub type Attributes<'tcx> = impl Iterator<Item = &'tcx ast::Attribute>;
     |
     |
     = note: expected reference `&[Attribute]`
                found reference `&rustc_middle::ty::Attributes<'_>`
    --> /checkout/compiler/rustc_session/src/session.rs:1074:12
     |
1074 |     pub fn first_attr_value_str_by_name(
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
---

---- compile_test stdout ----
diff of stderr:

 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
 LL |     u <= 0;
    |     ^^^^^^
    |
    |
    = note: `-D clippy::absurd-extreme-comparisons` implied by `-D warnings`
    = help: because `0` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `u == 0` instead
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     u <= Z;
    |
    |
    = help: because `Z` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `u == Z` instead
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
 LL |     u < Z;
    |     ^^^^^
    |
    |
    = help: because `Z` is the minimum value for this type, this comparison is always false
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     Z >= u;
    |
    |
    = help: because `Z` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `Z == u` instead
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     Z > u;
    |
    |
    = help: because `Z` is the minimum value for this type, this comparison is always false
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     u > u32::MAX;
    |
    |
    = help: because `u32::MAX` is the maximum value for this type, this comparison is always false
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     u >= u32::MAX;
    |
    |
    = help: because `u32::MAX` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `u == u32::MAX` instead
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
 LL |     u32::MAX < u;
    |     ^^^^^^^^^^^^
    |
    |
    = help: because `u32::MAX` is the maximum value for this type, this comparison is always false
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     u32::MAX <= u;
    |
    |
    = help: because `u32::MAX` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `u32::MAX == u` instead
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
 LL |     1-1 > u;
    |     ^^^^^^^
    |
    |
    = help: because `1-1` is the minimum value for this type, this comparison is always false
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     u >= !0;
    |
    |
    = help: because `!0` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `u == !0` instead
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     u <= 12 - 2*6;
    |
    |
    = help: because `12 - 2*6` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `u == 12 - 2*6` instead
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     i < -127 - 1;
    |
    |
    = help: because `-127 - 1` is the minimum value for this type, this comparison is always false
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     i8::MAX >= i;
    |
    |
    = help: because `i8::MAX` is the maximum value for this type, this comparison is always true
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     3-7 < i32::MIN;
    |
    |
    = help: because `i32::MIN` is the minimum value for this type, this comparison is always false
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
    |
 LL |     b >= true;
    |
    |
    = help: because `true` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `b == true` instead
 
 error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
    |
 LL |     false > b;
    |     ^^^^^^^^^
    |
    |
    = help: because `false` is the minimum value for this type, this comparison is always false
 
 error: <-comparison of unit values detected. This will always be false
    |
    |
 LL |     () < {};
    |
    |
    = note: `#[deny(clippy::unit_cmp)]` on by default
-error: aborting due to 18 previous errors
-error: aborting due to 18 previous errors
+error: re-implementing `PartialEq::ne` is unnecessary
+   |
+   |
+LL | #[derive(PartialEq, PartialOrd)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
+error: aborting due to 19 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/absurd-extreme-comparisons.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args absurd-extreme-comparisons.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/absurd-extreme-comparisons.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/absurd-extreme-comparisons.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/absurd-extreme-comparisons.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":259,"byte_end":265,"line_start":14,"line_end":14,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    u <= 0;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::absurd-extreme-comparisons` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"because `0` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `u == 0` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:14:5\n   |\nLL |     u <= 0;\n   |     ^^^^^^\n   |\n   = note: `-D clippy::absurd-extreme-comparisons` implied by `-D warnings`\n   = help: because `0` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `u == 0` instead\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":271,"byte_end":277,"line_start":15,"line_end":15,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    u <= Z;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `Z` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `u == Z` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:15:5\n   |\nLL |     u <= Z;\n   |     ^^^^^^\n   |\n   = help: because `Z` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `u == Z` instead\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":283,"byte_end":288,"line_start":16,"line_end":16,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    u < Z;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `Z` is the minimum value for this type, this comparison is always false","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:16:5\n   |\nLL |     u < Z;\n   |     ^^^^^\n   |\n   = help: because `Z` is the minimum value for this type, this comparison is always false\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":294,"byte_end":300,"line_start":17,"line_end":17,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    Z >= u;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `Z` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `Z == u` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:17:5\n   |\nLL |     Z >= u;\n   |     ^^^^^^\n   |\n   = help: because `Z` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `Z == u` instead\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":306,"byte_end":311,"line_start":18,"line_end":18,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    Z > u;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `Z` is the minimum value for this type, this comparison is always false","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:18:5\n   |\nLL |     Z > u;\n   |     ^^^^^\n   |\n   = help: because `Z` is the minimum value for this type, this comparison is always false\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":317,"byte_end":329,"line_start":19,"line_end":19,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    u > u32::MAX;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `u32::MAX` is the maximum value for this type, this comparison is always false","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:19:5\n   |\nLL |     u > u32::MAX;\n   |     ^^^^^^^^^^^^\n   |\n   = help: because `u32::MAX` is the maximum value for this type, this comparison is always false\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":335,"byte_end":348,"line_start":20,"line_end":20,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    u >= u32::MAX;","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `u32::MAX` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `u == u32::MAX` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:20:5\n   |\nLL |     u >= u32::MAX;\n   |     ^^^^^^^^^^^^^\n   |\n   = help: because `u32::MAX` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `u == u32::MAX` instead\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":354,"byte_end":366,"line_start":21,"line_end":21,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    u32::MAX < u;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `u32::MAX` is the maximum value for this type, this comparison is always false","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:21:5\n   |\nLL |     u32::MAX < u;\n   |     ^^^^^^^^^^^^\n   |\n   = help: because `u32::MAX` is the maximum value for this type, this comparison is always false\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":372,"byte_end":385,"line_start":22,"line_end":22,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    u32::MAX <= u;","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `u32::MAX` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `u32::MAX == u` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:22:5\n   |\nLL |     u32::MAX <= u;\n   |     ^^^^^^^^^^^^^\n   |\n   = help: because `u32::MAX` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `u32::MAX == u` instead\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":391,"byte_end":398,"line_start":23,"line_end":23,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    1-1 > u;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `1-1` is the minimum value for this type, this comparison is always false","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:23:5\n   |\nLL |     1-1 > u;\n   |     ^^^^^^^\n   |\n   = help: because `1-1` is the minimum value for this type, this comparison is always false\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":404,"byte_end":411,"line_start":24,"line_end":24,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    u >= !0;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `!0` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `u == !0` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:24:5\n   |\nLL |     u >= !0;\n   |     ^^^^^^^\n   |\n   = help: because `!0` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `u == !0` instead\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":417,"byte_end":430,"line_start":25,"line_end":25,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    u <= 12 - 2*6;","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `12 - 2*6` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `u == 12 - 2*6` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:25:5\n   |\nLL |     u <= 12 - 2*6;\n   |     ^^^^^^^^^^^^^\n   |\n   = help: because `12 - 2*6` is the minimum value for this type, the case where the two sides are not equal never occurs, consider using `u == 12 - 2*6` instead\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":455,"byte_end":467,"line_start":27,"line_end":27,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    i < -127 - 1;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `-127 - 1` is the minimum value for this type, this comparison is always false","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:27:5\n   |\nLL |     i < -127 - 1;\n   |     ^^^^^^^^^^^^\n   |\n   = help: because `-127 - 1` is the minimum value for this type, this comparison is always false\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":473,"byte_end":485,"line_start":28,"line_end":28,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    i8::MAX >= i;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `i8::MAX` is the maximum value for this type, this comparison is always true","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:28:5\n   |\nLL |     i8::MAX >= i;\n   |     ^^^^^^^^^^^^\n   |\n   = help: because `i8::MAX` is the maximum value for this type, this comparison is always true\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":491,"byte_end":505,"line_start":29,"line_end":29,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    3-7 < i32::MIN;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `i32::MIN` is the minimum value for this type, this comparison is always false","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:29:5\n   |\nLL |     3-7 < i32::MIN;\n   |     ^^^^^^^^^^^^^^\n   |\n   = help: because `i32::MIN` is the minimum value for this type, this comparison is always false\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":530,"byte_end":539,"line_start":31,"line_end":31,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    b >= true;","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `true` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `b == true` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:31:5\n   |\nLL |     b >= true;\n   |     ^^^^^^^^^\n   |\n   = help: because `true` is the maximum value for this type, the case where the two sides are not equal never occurs, consider using `b == true` instead\n\n"}
{"message":"this comparison involving the minimum or maximum element for this type contains a case that is always true or always false","code":{"code":"clippy::absurd_extreme_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":545,"byte_end":554,"line_start":32,"line_end":32,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    false > b;","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"because `false` is the minimum value for this type, this comparison is always false","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false\n  --> tests/ui/absurd-extreme-comparisons.rs:32:5\n   |\nLL |     false > b;\n   |     ^^^^^^^^^\n   |\n   = help: because `false` is the minimum value for this type, this comparison is always false\n\n"}
{"message":"<-comparison of unit values detected. This will always be false","code":{"code":"clippy::unit_cmp","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":620,"byte_end":627,"line_start":35,"line_end":35,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    () < {};","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(clippy::unit_cmp)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: <-comparison of unit values detected. This will always be false\n  --> tests/ui/absurd-extreme-comparisons.rs:35:5\n   |\nLL |     () < {};\n   |     ^^^^^^^\n   |\n   = note: `#[deny(clippy::unit_cmp)]` on by default\n\n"}
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":691,"byte_end":700,"line_start":40,"line_end":40,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(PartialEq, PartialOrd)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/absurd-extreme-comparisons.rs","byte_start":691,"byte_end":700,"line_start":40,"line_end":40,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(PartialEq, PartialOrd)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/absurd-extreme-comparisons.rs:40:10\n   |\nLL | #[derive(PartialEq, PartialOrd)]\n   |          ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: variable appears on both sides of an assignment operation
    |
    |
 LL |     a += a + 1;
    |
    |
    = note: `-D clippy::misrefactored-assign-op` implied by `-D warnings`
 help: did you mean `a = a + 1` or `a = a + a + 1`? Consider replacing it with
 LL |     a += 1;
    |     ~~~~~~
 help: or
    |
    |
 LL |     a = a + a + 1;
 
 
 error: variable appears on both sides of an assignment operation
    |
    |
 LL |     a += 1 + a;
    |
    |
 help: did you mean `a = a + 1` or `a = a + 1 + a`? Consider replacing it with
 LL |     a += 1;
    |     ~~~~~~
 help: or
    |
    |
 LL |     a = a + 1 + a;
 
 
 error: variable appears on both sides of an assignment operation
    |
    |
 LL |     a -= a - 1;
    |
    |
 help: did you mean `a = a - 1` or `a = a - (a - 1)`? Consider replacing it with
    |
 LL |     a -= 1;
 help: or
    |
    |
 LL |     a = a - (a - 1);
 
 
 error: variable appears on both sides of an assignment operation
    |
    |
 LL |     a *= a * 99;
    |
    |
 help: did you mean `a = a * 99` or `a = a * a * 99`? Consider replacing it with
    |
 LL |     a *= 99;
 help: or
    |
    |
 LL |     a = a * a * 99;
 
 
 error: variable appears on both sides of an assignment operation
    |
    |
 LL |     a *= 42 * a;
    |
    |
 help: did you mean `a = a * 42` or `a = a * 42 * a`? Consider replacing it with
    |
 LL |     a *= 42;
 help: or
    |
    |
 LL |     a = a * 42 * a;
 
 
 error: variable appears on both sides of an assignment operation
    |
    |
 LL |     a /= a / 2;
    |
    |
 help: did you mean `a = a / 2` or `a = a / (a / 2)`? Consider replacing it with
    |
 LL |     a /= 2;
 help: or
    |
    |
 LL |     a = a / (a / 2);
 
 
 error: variable appears on both sides of an assignment operation
    |
    |
 LL |     a %= a % 5;
    |
    |
 help: did you mean `a = a % 5` or `a = a % (a % 5)`? Consider replacing it with
    |
 LL |     a %= 5;
 help: or
    |
    |
 LL |     a = a % (a % 5);
 
 
 error: variable appears on both sides of an assignment operation
    |
    |
 LL |     a &= a & 1;
    |
    |
 help: did you mean `a = a & 1` or `a = a & a & 1`? Consider replacing it with
    |
 LL |     a &= 1;
 help: or
    |
    |
 LL |     a = a & a & 1;
 
 
 error: variable appears on both sides of an assignment operation
    |
    |
 LL |     a *= a * a;
    |
    |
 help: did you mean `a = a * a` or `a = a * a * a`? Consider replacing it with
    |
 LL |     a *= a;
 help: or
    |
    |
 LL |     a = a * a * a;
 
 
+error: re-implementing `PartialEq::ne` is unnecessary
+   |
+   |
+LL | #[derive(Copy, Clone, Debug, PartialEq)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
 error: manual implementation of an assign operation
   --> $DIR/assign_ops2.rs:50:5
    |
    |
 LL |     buf = buf + cows.clone();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `buf += cows.clone()`
    |
    = note: `-D clippy::assign-op-pattern` implied by `-D warnings`
-error: aborting due to 10 previous errors
+error: aborting due to 11 previous errors
 
 
---
To only update this specific test, also pass `--test-args assign_ops2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/assign_ops2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/assign_ops2.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/assign_ops2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"variable appears on both sides of an assignment operation","code":{"code":"clippy::misrefactored_assign_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":132,"byte_end":142,"line_start":5,"line_end":5,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a += a + 1;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::misrefactored-assign-op` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"did you mean `a = a + 1` or `a = a + a + 1`? Consider replacing it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":132,"byte_end":142,"line_start":5,"line_end":5,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a += a + 1;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a += 1","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":132,"byte_end":142,"line_start":5,"line_end":5,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a += a + 1;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a = a + a + 1","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable appears on both sides of an assignment operation\n  --> tests/ui/assign_ops2.rs:5:5\n   |\nLL |     a += a + 1;\n   |     ^^^^^^^^^^\n   |\n   = note: `-D clippy::misrefactored-assign-op` implied by `-D warnings`\nhelp: did you mean `a = a + 1` or `a = a + a + 1`? Consider replacing it with\n   |\nLL |     a += 1;\n   |     ~~~~~~\nhelp: or\n   |\nLL |     a = a + a + 1;\n   |     ~~~~~~~~~~~~~\n\n"}
{"message":"variable appears on both sides of an assignment operation","code":{"code":"clippy::misrefactored_assign_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":148,"byte_end":158,"line_start":6,"line_end":6,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a += 1 + a;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `a = a + 1` or `a = a + 1 + a`? Consider replacing it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":148,"byte_end":158,"line_start":6,"line_end":6,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a += 1 + a;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a += 1","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":148,"byte_end":158,"line_start":6,"line_end":6,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a += 1 + a;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a = a + 1 + a","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable appears on both sides of an assignment operation\n  --> tests/ui/assign_ops2.rs:6:5\n   |\nLL |     a += 1 + a;\n   |     ^^^^^^^^^^\n   |\nhelp: did you mean `a = a + 1` or `a = a + 1 + a`? Consider replacing it with\n   |\nLL |     a += 1;\n   |     ~~~~~~\nhelp: or\n   |\nLL |     a = a + 1 + a;\n   |     ~~~~~~~~~~~~~\n\n"}
{"message":"variable appears on both sides of an assignment operation","code":{"code":"clippy::misrefactored_assign_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":164,"byte_end":174,"line_start":7,"line_end":7,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a -= a - 1;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `a = a - 1` or `a = a - (a - 1)`? Consider replacing it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":164,"byte_end":174,"line_start":7,"line_end":7,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a -= a - 1;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a -= 1","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":164,"byte_end":174,"line_start":7,"line_end":7,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a -= a - 1;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a = a - (a - 1)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable appears on both sides of an assignment operation\n  --> tests/ui/assign_ops2.rs:7:5\n   |\nLL |     a -= a - 1;\n   |     ^^^^^^^^^^\n   |\nhelp: did you mean `a = a - 1` or `a = a - (a - 1)`? Consider replacing it with\n   |\nLL |     a -= 1;\n   |     ~~~~~~\nhelp: or\n   |\nLL |     a = a - (a - 1);\n   |     ~~~~~~~~~~~~~~~\n\n"}
{"message":"variable appears on both sides of an assignment operation","code":{"code":"clippy::misrefactored_assign_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":180,"byte_end":191,"line_start":8,"line_end":8,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    a *= a * 99;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `a = a * 99` or `a = a * a * 99`? Consider replacing it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":180,"byte_end":191,"line_start":8,"line_end":8,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    a *= a * 99;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"a *= 99","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":180,"byte_end":191,"line_start":8,"line_end":8,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    a *= a * 99;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"a = a * a * 99","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable appears on both sides of an assignment operation\n  --> tests/ui/assign_ops2.rs:8:5\n   |\nLL |     a *= a * 99;\n   |     ^^^^^^^^^^^\n   |\nhelp: did you mean `a = a * 99` or `a = a * a * 99`? Consider replacing it with\n   |\nLL |     a *= 99;\n   |     ~~~~~~~\nhelp: or\n   |\nLL |     a = a * a * 99;\n   |     ~~~~~~~~~~~~~~\n\n"}
{"message":"variable appears on both sides of an assignment operation","code":{"code":"clippy::misrefactored_assign_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":197,"byte_end":208,"line_start":9,"line_end":9,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    a *= 42 * a;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `a = a * 42` or `a = a * 42 * a`? Consider replacing it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":197,"byte_end":208,"line_start":9,"line_end":9,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    a *= 42 * a;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"a *= 42","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":197,"byte_end":208,"line_start":9,"line_end":9,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    a *= 42 * a;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"a = a * 42 * a","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable appears on both sides of an assignment operation\n  --> tests/ui/assign_ops2.rs:9:5\n   |\nLL |     a *= 42 * a;\n   |     ^^^^^^^^^^^\n   |\nhelp: did you mean `a = a * 42` or `a = a * 42 * a`? Consider replacing it with\n   |\nLL |     a *= 42;\n   |     ~~~~~~~\nhelp: or\n   |\nLL |     a = a * 42 * a;\n   |     ~~~~~~~~~~~~~~\n\n"}
{"message":"variable appears on both sides of an assignment operation","code":{"code":"clippy::misrefactored_assign_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":214,"byte_end":224,"line_start":10,"line_end":10,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a /= a / 2;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `a = a / 2` or `a = a / (a / 2)`? Consider replacing it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":214,"byte_end":224,"line_start":10,"line_end":10,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a /= a / 2;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a /= 2","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":214,"byte_end":224,"line_start":10,"line_end":10,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a /= a / 2;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a = a / (a / 2)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable appears on both sides of an assignment operation\n  --> tests/ui/assign_ops2.rs:10:5\n   |\nLL |     a /= a / 2;\n   |     ^^^^^^^^^^\n   |\nhelp: did you mean `a = a / 2` or `a = a / (a / 2)`? Consider replacing it with\n   |\nLL |     a /= 2;\n   |     ~~~~~~\nhelp: or\n   |\nLL |     a = a / (a / 2);\n   |     ~~~~~~~~~~~~~~~\n\n"}
{"message":"variable appears on both sides of an assignment operation","code":{"code":"clippy::misrefactored_assign_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":230,"byte_end":240,"line_start":11,"line_end":11,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a %= a % 5;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `a = a % 5` or `a = a % (a % 5)`? Consider replacing it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":230,"byte_end":240,"line_start":11,"line_end":11,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a %= a % 5;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a %= 5","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":230,"byte_end":240,"line_start":11,"line_end":11,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a %= a % 5;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a = a % (a % 5)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable appears on both sides of an assignment operation\n  --> tests/ui/assign_ops2.rs:11:5\n   |\nLL |     a %= a % 5;\n   |     ^^^^^^^^^^\n   |\nhelp: did you mean `a = a % 5` or `a = a % (a % 5)`? Consider replacing it with\n   |\nLL |     a %= 5;\n   |     ~~~~~~\nhelp: or\n   |\nLL |     a = a % (a % 5);\n   |     ~~~~~~~~~~~~~~~\n\n"}
{"message":"variable appears on both sides of an assignment operation","code":{"code":"clippy::misrefactored_assign_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":246,"byte_end":256,"line_start":12,"line_end":12,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a &= a & 1;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `a = a & 1` or `a = a & a & 1`? Consider replacing it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":246,"byte_end":256,"line_start":12,"line_end":12,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a &= a & 1;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a &= 1","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":246,"byte_end":256,"line_start":12,"line_end":12,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a &= a & 1;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a = a & a & 1","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable appears on both sides of an assignment operation\n  --> tests/ui/assign_ops2.rs:12:5\n   |\nLL |     a &= a & 1;\n   |     ^^^^^^^^^^\n   |\nhelp: did you mean `a = a & 1` or `a = a & a & 1`? Consider replacing it with\n   |\nLL |     a &= 1;\n   |     ~~~~~~\nhelp: or\n   |\nLL |     a = a & a & 1;\n   |     ~~~~~~~~~~~~~\n\n"}
{"message":"variable appears on both sides of an assignment operation","code":{"code":"clippy::misrefactored_assign_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":262,"byte_end":272,"line_start":13,"line_end":13,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a *= a * a;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `a = a * a` or `a = a * a * a`? Consider replacing it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":262,"byte_end":272,"line_start":13,"line_end":13,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a *= a * a;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a *= a","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":262,"byte_end":272,"line_start":13,"line_end":13,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    a *= a * a;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"a = a * a * a","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable appears on both sides of an assignment operation\n  --> tests/ui/assign_ops2.rs:13:5\n   |\nLL |     a *= a * a;\n   |     ^^^^^^^^^^\n   |\nhelp: did you mean `a = a * a` or `a = a * a * a`? Consider replacing it with\n   |\nLL |     a *= a;\n   |     ~~~~~~\nhelp: or\n   |\nLL |     a = a * a * a;\n   |     ~~~~~~~~~~~~~\n\n"}
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":554,"byte_end":563,"line_start":27,"line_end":27,"column_start":30,"column_end":39,"is_primary":true,"text":[{"text":"#[derive(Copy, Clone, Debug, PartialEq)]","highlight_start":30,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assign_ops2.rs","byte_start":554,"byte_end":563,"line_start":27,"line_end":27,"column_start":30,"column_end":39,"is_primary":false,"text":[{"text":"#[derive(Copy, Clone, Debug, PartialEq)]","highlight_start":30,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/assign_ops2.rs:27:30\n   |\nLL | #[derive(Copy, Clone, Debug, PartialEq)]\n   |                              ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"manual implementation of an assign operation","code":{"code":"clippy::assign_op_pattern","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":988,"byte_end":1012,"line_start":50,"line_end":50,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    buf = buf + cows.clone();","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::assign-op-pattern` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"replace it with","code":null,"level":"help","spans":[{"file_name":"tests/ui/assign_ops2.rs","byte_start":988,"byte_end":1012,"line_start":50,"line_end":50,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    buf = buf + cows.clone();","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":"buf += cows.clone()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: manual implementation of an assign operation\n  --> tests/ui/assign_ops2.rs:50:5\n   |\nLL |     buf = buf + cows.clone();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `buf += cows.clone()`\n   |\n   = note: `-D clippy::assign-op-pattern` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: re-implementing `PartialEq::ne` is unnecessary
+  --> $DIR/ice-2774.rs:5:14
+   |
+LL | #[derive(Eq, PartialEq, Debug, Hash)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
+
 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/ice-2774.rs:15:1
    |
 LL | pub fn add_barfoos_to_foos<'a>(bars: &HashSet<&'a Bar>) {
    |
    |
    = note: `-D clippy::needless-lifetimes` implied by `-D warnings`
-error: aborting due to previous error
+error: aborting due to 2 previous errors
 
 
---
To only update this specific test, also pass `--test-args crashes/ice-2774.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-2774.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-2774.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-2774.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-2774.rs","byte_start":81,"byte_end":90,"line_start":5,"line_end":5,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"#[derive(Eq, PartialEq, Debug, Hash)]","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/crashes/ice-2774.rs","byte_start":81,"byte_end":90,"line_start":5,"line_end":5,"column_start":14,"column_end":23,"is_primary":false,"text":[{"text":"#[derive(Eq, PartialEq, Debug, Hash)]","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/crashes/ice-2774.rs:5:14\n   |\nLL | #[derive(Eq, PartialEq, Debug, Hash)]\n   |              ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)","code":{"code":"clippy::needless_lifetimes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-2774.rs","byte_start":290,"byte_end":346,"line_start":15,"line_end":15,"column_start":1,"column_end":57,"is_primary":true,"text":[{"text":"pub fn add_barfoos_to_foos<'a>(bars: &HashSet<&'a Bar>) {","highlight_start":1,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-lifetimes` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)\n  --> tests/ui/crashes/ice-2774.rs:15:1\n   |\nLL | pub fn add_barfoos_to_foos<'a>(bars: &HashSet<&'a Bar>) {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: to use a constant of type `Foo` in a pattern, `Foo` must be annotated with `#[derive(PartialEq, Eq)]`
   --> $DIR/ice-6254.rs:12:9
    |
 LL |         FOO_REF_REF => {},
    |
    |
    = note: `-D indirect-structural-match` implied by `-D warnings`
    = note: for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/62411>
 
-error: aborting due to previous error
-error: aborting due to previous error
+error: re-implementing `PartialEq::ne` is unnecessary
+  --> $DIR/ice-6254.rs:5:10
+   |
+LL | #[derive(PartialEq)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
+error: aborting due to 2 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-6254.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crashes/ice-6254.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-6254.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-6254.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-6254.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"to use a constant of type `Foo` in a pattern, `Foo` must be annotated with `#[derive(PartialEq, Eq)]`","code":{"code":"indirect_structural_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-6254.rs","byte_start":411,"byte_end":422,"line_start":12,"line_end":12,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        FOO_REF_REF => {},","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D indirect-structural-match` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/62411>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: to use a constant of type `Foo` in a pattern, `Foo` must be annotated with `#[derive(PartialEq, Eq)]`\n  --> tests/ui/crashes/ice-6254.rs:12:9\n   |\nLL |         FOO_REF_REF => {},\n   |         ^^^^^^^^^^^\n   |\n   = note: `-D indirect-structural-match` implied by `-D warnings`\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/62411>\n\n"}
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-6254.rs","byte_start":217,"byte_end":226,"line_start":5,"line_end":5,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(PartialEq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/crashes/ice-6254.rs","byte_start":217,"byte_end":226,"line_start":5,"line_end":5,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(PartialEq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/crashes/ice-6254.rs:5:10\n   |\nLL | #[derive(PartialEq)]\n   |          ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: re-implementing `PartialEq::ne` is unnecessary
+  --> $DIR/erasing_op.rs:11:26
+   |
+LL | #[derive(Clone, Default, PartialEq, Eq, Hash)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
+
 error: this operation will always return zero. This is likely not the intended outcome
   --> $DIR/erasing_op.rs:37:5
 LL |     x * 0;
    |     ^^^^^
    |
    |
    = note: `-D clippy::erasing-op` implied by `-D warnings`
 
 error: this operation will always return zero. This is likely not the intended outcome
   --> $DIR/erasing_op.rs:38:5
 LL |     0 & x;
    |     ^^^^^
 
 
 error: this operation will always return zero. This is likely not the intended outcome
   --> $DIR/erasing_op.rs:39:5
 LL |     0 / x;
    |     ^^^^^
 
 
 error: this operation will always return zero. This is likely not the intended outcome
   --> $DIR/erasing_op.rs:41:5
 LL |     0 * Vec1 { x: 5 };
    |     ^^^^^^^^^^^^^^^^^
 
 
 error: this operation will always return zero. This is likely not the intended outcome
   --> $DIR/erasing_op.rs:42:5
 LL |     Vec1 { x: 5 } * 0;
    |     ^^^^^^^^^^^^^^^^^
 
-error: aborting due to 5 previous errors
---
To only update this specific test, also pass `--test-args erasing_op.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/erasing_op.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/erasing_op.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/erasing_op.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/erasing_op.rs","byte_start":190,"byte_end":199,"line_start":11,"line_end":11,"column_start":26,"column_end":35,"is_primary":true,"text":[{"text":"#[derive(Clone, Default, PartialEq, Eq, Hash)]","highlight_start":26,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/erasing_op.rs","byte_start":190,"byte_end":199,"line_start":11,"line_end":11,"column_start":26,"column_end":35,"is_primary":false,"text":[{"text":"#[derive(Clone, Default, PartialEq, Eq, Hash)]","highlight_start":26,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/erasing_op.rs:11:26\n   |\nLL | #[derive(Clone, Default, PartialEq, Eq, Hash)]\n   |                          ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"this operation will always return zero. This is likely not the intended outcome","code":{"code":"clippy::erasing_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/erasing_op.rs","byte_start":635,"byte_end":640,"line_start":37,"line_end":37,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    x * 0;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::erasing-op` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this operation will always return zero. This is likely not the intended outcome\n  --> tests/ui/erasing_op.rs:37:5\n   |\nLL |     x * 0;\n   |     ^^^^^\n   |\n   = note: `-D clippy::erasing-op` implied by `-D warnings`\n\n"}
{"message":"this operation will always return zero. This is likely not the intended outcome","code":{"code":"clippy::erasing_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/erasing_op.rs","byte_start":646,"byte_end":651,"line_start":38,"line_end":38,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    0 & x;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will always return zero. This is likely not the intended outcome\n  --> tests/ui/erasing_op.rs:38:5\n   |\nLL |     0 & x;\n   |     ^^^^^\n\n"}
{"message":"this operation will always return zero. This is likely not the intended outcome","code":{"code":"clippy::erasing_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/erasing_op.rs","byte_start":657,"byte_end":662,"line_start":39,"line_end":39,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    0 / x;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will always return zero. This is likely not the intended outcome\n  --> tests/ui/erasing_op.rs:39:5\n   |\nLL |     0 / x;\n   |     ^^^^^\n\n"}
{"message":"this operation will always return zero. This is likely not the intended outcome","code":{"code":"clippy::erasing_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/erasing_op.rs","byte_start":748,"byte_end":765,"line_start":41,"line_end":41,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    0 * Vec1 { x: 5 };","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will always return zero. This is likely not the intended outcome\n  --> tests/ui/erasing_op.rs:41:5\n   |\nLL |     0 * Vec1 { x: 5 };\n   |     ^^^^^^^^^^^^^^^^^\n\n"}
{"message":"this operation will always return zero. This is likely not the intended outcome","code":{"code":"clippy::erasing_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/erasing_op.rs","byte_start":771,"byte_end":788,"line_start":42,"line_end":42,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    Vec1 { x: 5 } * 0;","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will always return zero. This is likely not the intended outcome\n  --> tests/ui/erasing_op.rs:42:5\n   |\nLL |     Vec1 { x: 5 } * 0;\n   |     ^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: re-implementing `PartialEq::ne` is unnecessary
+  --> $DIR/equatable_if_let.rs:8:10
+   |
+LL | #[derive(PartialEq)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
+
+error: re-implementing `PartialEq::ne` is unnecessary
+  --> $DIR/equatable_if_let.rs:16:10
+   |
+LL | #[derive(PartialEq)]
+   |
+   |
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:49:8
    |
    |
 LL |     if let 2 = a {}
    |        ^^^^^^^^^ help: try: `a == 2`
    |
    = note: `-D clippy::equatable-if-let` implied by `-D warnings`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:50:8
    |
    |
 LL |     if let Ordering::Greater = a.cmp(&b) {}
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `a.cmp(&b) == Ordering::Greater`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:51:8
    |
 LL |     if let Some(2) = c {}
 LL |     if let Some(2) = c {}
    |        ^^^^^^^^^^^^^^^ help: try: `c == Some(2)`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:52:8
    |
    |
 LL |     if let Struct { a: 2, b: false } = d {}
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `d == (Struct { a: 2, b: false })`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:53:8
    |
    |
 LL |     if let Enum::TupleVariant(32, 64) = e {}
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `e == Enum::TupleVariant(32, 64)`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:54:8
    |
    |
 LL |     if let Enum::RecordVariant { a: 64, b: 32 } = e {}
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `e == (Enum::RecordVariant { a: 64, b: 32 })`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:55:8
    |
    |
 LL |     if let Enum::UnitVariant = e {}
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `e == Enum::UnitVariant`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:56:8
    |
    |
 LL |     if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) {}
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `(e, &d) == (Enum::UnitVariant, &Struct { a: 2, b: false })`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:66:8
    |
    |
 LL |     if let NotStructuralEq::A = g {}
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `g == NotStructuralEq::A`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:68:8
error: test failed, to rerun pass '--test compile-test'
    |
    |
 LL |     if let Some(NotStructuralEq::A) = Some(g) {}
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `Some(g) == Some(NotStructuralEq::A)`
 error: this pattern matching can be expressed using equality
   --> $DIR/equatable_if_let.rs:75:8
    |
    |
 LL |     if let m1!(x) = "abc" {
    |        ^^^^^^^^^^^^^^^^^^ help: try: `"abc" == m1!(x)`
-error: aborting due to 11 previous errors
+error: aborting due to 13 previous errors
 
 
---
To only update this specific test, also pass `--test-args equatable_if_let.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/equatable_if_let.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/equatable_if_let.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/equatable_if_let.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":125,"byte_end":134,"line_start":8,"line_end":8,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(PartialEq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/equatable_if_let.rs","byte_start":125,"byte_end":134,"line_start":8,"line_end":8,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(PartialEq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/equatable_if_let.rs:8:10\n   |\nLL | #[derive(PartialEq)]\n   |          ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":267,"byte_end":276,"line_start":16,"line_end":16,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(PartialEq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/equatable_if_let.rs","byte_start":267,"byte_end":276,"line_start":16,"line_end":16,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(PartialEq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/equatable_if_let.rs:16:10\n   |\nLL | #[derive(PartialEq)]\n   |          ^^^^^^^^^\n   |\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":723,"byte_end":732,"line_start":49,"line_end":49,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if let 2 = a {}","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::equatable-if-let` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":723,"byte_end":732,"line_start":49,"line_end":49,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if let 2 = a {}","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":"a == 2","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:49:8\n   |\nLL |     if let 2 = a {}\n   |        ^^^^^^^^^ help: try: `a == 2`\n   |\n   = note: `-D clippy::equatable-if-let` implied by `-D warnings`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":743,"byte_end":776,"line_start":50,"line_end":50,"column_start":8,"column_end":41,"is_primary":true,"text":[{"text":"    if let Ordering::Greater = a.cmp(&b) {}","highlight_start":8,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":743,"byte_end":776,"line_start":50,"line_end":50,"column_start":8,"column_end":41,"is_primary":true,"text":[{"text":"    if let Ordering::Greater = a.cmp(&b) {}","highlight_start":8,"highlight_end":41}],"label":null,"suggested_replacement":"a.cmp(&b) == Ordering::Greater","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:50:8\n   |\nLL |     if let Ordering::Greater = a.cmp(&b) {}\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `a.cmp(&b) == Ordering::Greater`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":787,"byte_end":802,"line_start":51,"line_end":51,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if let Some(2) = c {}","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":787,"byte_end":802,"line_start":51,"line_end":51,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if let Some(2) = c {}","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":"c == Some(2)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:51:8\n   |\nLL |     if let Some(2) = c {}\n   |        ^^^^^^^^^^^^^^^ help: try: `c == Some(2)`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":813,"byte_end":846,"line_start":52,"line_end":52,"column_start":8,"column_end":41,"is_primary":true,"text":[{"text":"    if let Struct { a: 2, b: false } = d {}","highlight_start":8,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":813,"byte_end":846,"line_start":52,"line_end":52,"column_start":8,"column_end":41,"is_primary":true,"text":[{"text":"    if let Struct { a: 2, b: false } = d {}","highlight_start":8,"highlight_end":41}],"label":null,"suggested_replacement":"d == (Struct { a: 2, b: false })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:52:8\n   |\nLL |     if let Struct { a: 2, b: false } = d {}\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `d == (Struct { a: 2, b: false })`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":857,"byte_end":891,"line_start":53,"line_end":53,"column_start":8,"column_end":42,"is_primary":true,"text":[{"text":"    if let Enum::TupleVariant(32, 64) = e {}","highlight_start":8,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":857,"byte_end":891,"line_start":53,"line_end":53,"column_start":8,"column_end":42,"is_primary":true,"text":[{"text":"    if let Enum::TupleVariant(32, 64) = e {}","highlight_start":8,"highlight_end":42}],"label":null,"suggested_replacement":"e == Enum::TupleVariant(32, 64)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:53:8\n   |\nLL |     if let Enum::TupleVariant(32, 64) = e {}\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `e == Enum::TupleVariant(32, 64)`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":902,"byte_end":946,"line_start":54,"line_end":54,"column_start":8,"column_end":52,"is_primary":true,"text":[{"text":"    if let Enum::RecordVariant { a: 64, b: 32 } = e {}","highlight_start":8,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":902,"byte_end":946,"line_start":54,"line_end":54,"column_start":8,"column_end":52,"is_primary":true,"text":[{"text":"    if let Enum::RecordVariant { a: 64, b: 32 } = e {}","highlight_start":8,"highlight_end":52}],"label":null,"suggested_replacement":"e == (Enum::RecordVariant { a: 64, b: 32 })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:54:8\n   |\nLL |     if let Enum::RecordVariant { a: 64, b: 32 } = e {}\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `e == (Enum::RecordVariant { a: 64, b: 32 })`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":957,"byte_end":982,"line_start":55,"line_end":55,"column_start":8,"column_end":33,"is_primary":true,"text":[{"text":"    if let Enum::UnitVariant = e {}","highlight_start":8,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":957,"byte_end":982,"line_start":55,"line_end":55,"column_start":8,"column_end":33,"is_primary":true,"text":[{"text":"    if let Enum::UnitVariant = e {}","highlight_start":8,"highlight_end":33}],"label":null,"suggested_replacement":"e == Enum::UnitVariant","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:55:8\n   |\nLL |     if let Enum::UnitVariant = e {}\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `e == Enum::UnitVariant`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":993,"byte_end":1054,"line_start":56,"line_end":56,"column_start":8,"column_end":69,"is_primary":true,"text":[{"text":"    if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) {}","highlight_start":8,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":993,"byte_end":1054,"line_start":56,"line_end":56,"column_start":8,"column_end":69,"is_primary":true,"text":[{"text":"    if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) {}","highlight_start":8,"highlight_end":69}],"label":null,"suggested_replacement":"(e, &d) == (Enum::UnitVariant, &Struct { a: 2, b: false })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:56:8\n   |\nLL |     if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) {}\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `(e, &d) == (Enum::UnitVariant, &Struct { a: 2, b: false })`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1273,"byte_end":1299,"line_start":66,"line_end":66,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"    if let NotStructuralEq::A = g {}","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1273,"byte_end":1299,"line_start":66,"line_end":66,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"    if let NotStructuralEq::A = g {}","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":"g == NotStructuralEq::A","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:66:8\n   |\nLL |     if let NotStructuralEq::A = g {}\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `g == NotStructuralEq::A`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1356,"byte_end":1394,"line_start":68,"line_end":68,"column_start":8,"column_end":46,"is_primary":true,"text":[{"text":"    if let Some(NotStructuralEq::A) = Some(g) {}","highlight_start":8,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1356,"byte_end":1394,"line_start":68,"line_end":68,"column_start":8,"column_end":46,"is_primary":true,"text":[{"text":"    if let Some(NotStructuralEq::A) = Some(g) {}","highlight_start":8,"highlight_end":46}],"label":null,"suggested_replacement":"Some(g) == Some(NotStructuralEq::A)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:68:8\n   |\nLL |     if let Some(NotStructuralEq::A) = Some(g) {}\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `Some(g) == Some(NotStructuralEq::A)`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1480,"byte_end":1498,"line_start":75,"line_end":75,"column_start":8,"column_end":26,"is_primary":true,"text":[{"text":"    if let m1!(x) = \"abc\" {","highlight_start":8,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1480,"byte_end":1498,"line_start":75,"line_end":75,"column_start":8,"column_end":26,"is_primary":true,"text":[{"text":"    if let m1!(x) = \"abc\" {","highlight_start":8,"highlight_end":26}],"label":null,"suggested_replacement":"\"abc\" == m1!(x)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:75:8\n   |\nLL |     if let m1!(x) = \"abc\" {\n   |        ^^^^^^^^^^^^^^^^^^ help: try: `\"abc\" == m1!(x)`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this match arm has an identical body to the `_` wildcard arm
    |
    |
 LL |         Abc::A => 0,
    |
    |
    = note: `-D clippy::match-same-arms` implied by `-D warnings`
    = help: or try changing either arm body
 note: `_` wildcard arm here
    |
    |
 LL |         _ => 0, //~ ERROR match arms have same body
 
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         (1, .., 3) => 42,
    |         |
    |         |
    |         help: try merging the arm patterns: `(1, .., 3) | (.., 3)`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms.rs:18:9
    |
    |
 LL |         (.., 3) => 42, //~ ERROR match arms have same body
 
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         51 => 1, //~ ERROR match arms have same body
    |         --^^^^^
    |         |
    |         help: try merging the arm patterns: `51 | 42`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms.rs:23:9
    |
    |
 LL |         42 => 1,
    |         ^^^^^^^
 
 error: this match arm has an identical body to another arm
    |
 LL |         41 => 2,
    |         --^^^^^
    |         |
    |         |
    |         help: try merging the arm patterns: `41 | 52`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms.rs:26:9
    |
    |
 LL |         52 => 2, //~ ERROR match arms have same body
 
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         2 => 2, //~ ERROR 2nd matched arms have same body
    |         -^^^^^
    |         |
    |         help: try merging the arm patterns: `2 | 1`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms.rs:31:9
    |
    |
 LL |         1 => 2,
    |         ^^^^^^
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         3 => 2, //~ ERROR 3rd matched arms have same body
    |         -^^^^^
    |         |
    |         help: try merging the arm patterns: `3 | 1`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms.rs:31:9
    |
    |
 LL |         1 => 2,
    |         ^^^^^^
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         2 => 2, //~ ERROR 2nd matched arms have same body
    |         -^^^^^
    |         |
    |         help: try merging the arm patterns: `2 | 3`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms.rs:33:9
    |
    |
 LL |         3 => 2, //~ ERROR 3rd matched arms have same body
 
 
+error: re-implementing `PartialEq::ne` is unnecessary
+   |
+   |
+LL |     #[derive(PartialEq, PartialOrd, Eq, Ord)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
+
 error: this match arm has an identical body to another arm
    |
    |
 LL |                 CommandInfo::External { name, .. } => name.to_string(),
    |                 |
    |                 |
    |                 help: try merging the arm patterns: `CommandInfo::External { name, .. } | CommandInfo::BuiltIn { name, .. }`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms.rs:49:17
    |
    |
 LL |                 CommandInfo::BuiltIn { name, .. } => name.to_string(),
 
-error: aborting due to 8 previous errors
+error: aborting due to 9 previous errors
 
---
To only update this specific test, also pass `--test-args match_same_arms.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/match_same_arms.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/match_same_arms.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/match_same_arms.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this match arm has an identical body to the `_` wildcard arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":132,"byte_end":143,"line_start":11,"line_end":11,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        Abc::A => 0,","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::match-same-arms` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`_` wildcard arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":174,"byte_end":180,"line_start":13,"line_end":13,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        _ => 0, //~ ERROR match arms have same body","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try removing the arm","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":132,"byte_end":143,"line_start":11,"line_end":11,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        Abc::A => 0,","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to the `_` wildcard arm\n  --> tests/ui/match_same_arms.rs:11:9\n   |\nLL |         Abc::A => 0,\n   |         ^^^^^^^^^^^ help: try removing the arm\n   |\n   = note: `-D clippy::match-same-arms` implied by `-D warnings`\n   = help: or try changing either arm body\nnote: `_` wildcard arm here\n  --> tests/ui/match_same_arms.rs:13:9\n   |\nLL |         _ => 0, //~ ERROR match arms have same body\n   |         ^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":256,"byte_end":272,"line_start":17,"line_end":17,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        (1, .., 3) => 42,","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":282,"byte_end":295,"line_start":18,"line_end":18,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"        (.., 3) => 42, //~ ERROR match arms have same body","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":256,"byte_end":266,"line_start":17,"line_end":17,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"        (1, .., 3) => 42,","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":"(1, .., 3) | (.., 3)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms.rs:17:9\n   |\nLL |         (1, .., 3) => 42,\n   |         ----------^^^^^^\n   |         |\n   |         help: try merging the arm patterns: `(1, .., 3) | (.., 3)`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms.rs:18:9\n   |\nLL |         (.., 3) => 42, //~ ERROR match arms have same body\n   |         ^^^^^^^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":405,"byte_end":412,"line_start":24,"line_end":24,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        51 => 1, //~ ERROR match arms have same body","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":388,"byte_end":395,"line_start":23,"line_end":23,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        42 => 1,","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":405,"byte_end":407,"line_start":24,"line_end":24,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        51 => 1, //~ ERROR match arms have same body","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"51 | 42","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms.rs:24:9\n   |\nLL |         51 => 1, //~ ERROR match arms have same body\n   |         --^^^^^\n   |         |\n   |         help: try merging the arm patterns: `51 | 42`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms.rs:23:9\n   |\nLL |         42 => 1,\n   |         ^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":458,"byte_end":465,"line_start":25,"line_end":25,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        41 => 2,","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":475,"byte_end":482,"line_start":26,"line_end":26,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        52 => 2, //~ ERROR match arms have same body","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":458,"byte_end":460,"line_start":25,"line_end":25,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        41 => 2,","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"41 | 52","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms.rs:25:9\n   |\nLL |         41 => 2,\n   |         --^^^^^\n   |         |\n   |         help: try merging the arm patterns: `41 | 52`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms.rs:26:9\n   |\nLL |         52 => 2, //~ ERROR match arms have same body\n   |         ^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":591,"byte_end":597,"line_start":32,"line_end":32,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        2 => 2, //~ ERROR 2nd matched arms have same body","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":575,"byte_end":581,"line_start":31,"line_end":31,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        1 => 2,","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":591,"byte_end":592,"line_start":32,"line_end":32,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        2 => 2, //~ ERROR 2nd matched arms have same body","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"2 | 1","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms.rs:32:9\n   |\nLL |         2 => 2, //~ ERROR 2nd matched arms have same body\n   |         -^^^^^\n   |         |\n   |         help: try merging the arm patterns: `2 | 1`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms.rs:31:9\n   |\nLL |         1 => 2,\n   |         ^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":649,"byte_end":655,"line_start":33,"line_end":33,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        3 => 2, //~ ERROR 3rd matched arms have same body","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":575,"byte_end":581,"line_start":31,"line_end":31,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        1 => 2,","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":649,"byte_end":650,"line_start":33,"line_end":33,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        3 => 2, //~ ERROR 3rd matched arms have same body","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"3 | 1","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms.rs:33:9\n   |\nLL |         3 => 2, //~ ERROR 3rd matched arms have same body\n   |         -^^^^^\n   |         |\n   |         help: try merging the arm patterns: `3 | 1`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms.rs:31:9\n   |\nLL |         1 => 2,\n   |         ^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":591,"byte_end":597,"line_start":32,"line_end":32,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        2 => 2, //~ ERROR 2nd matched arms have same body","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":649,"byte_end":655,"line_start":33,"line_end":33,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        3 => 2, //~ ERROR 3rd matched arms have same body","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":591,"byte_end":592,"line_start":32,"line_end":32,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        2 => 2, //~ ERROR 2nd matched arms have same body","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"2 | 3","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms.rs:32:9\n   |\nLL |         2 => 2, //~ ERROR 2nd matched arms have same body\n   |         -^^^^^\n   |         |\n   |         help: try merging the arm patterns: `2 | 3`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms.rs:33:9\n   |\nLL |         3 => 2, //~ ERROR 3rd matched arms have same body\n   |         ^^^^^^\n\n"}
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":770,"byte_end":779,"line_start":40,"line_end":40,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"    #[derive(PartialEq, PartialOrd, Eq, Ord)]","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/match_same_arms.rs","byte_start":770,"byte_end":779,"line_start":40,"line_end":40,"column_start":14,"column_end":23,"is_primary":false,"text":[{"text":"    #[derive(PartialEq, PartialOrd, Eq, Ord)]","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/match_same_arms.rs:40:14\n   |\nLL |     #[derive(PartialEq, PartialOrd, Eq, Ord)]\n   |              ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":1129,"byte_end":1183,"line_start":50,"line_end":50,"column_start":17,"column_end":71,"is_primary":true,"text":[{"text":"                CommandInfo::External { name, .. } => name.to_string(),","highlight_start":17,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":1058,"byte_end":1111,"line_start":49,"line_end":49,"column_start":17,"column_end":70,"is_primary":true,"text":[{"text":"                CommandInfo::BuiltIn { name, .. } => name.to_string(),","highlight_start":17,"highlight_end":70}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":1129,"byte_end":1163,"line_start":50,"line_end":50,"column_start":17,"column_end":51,"is_primary":true,"text":[{"text":"                CommandInfo::External { name, .. } => name.to_string(),","highlight_start":17,"highlight_end":51}],"label":null,"suggested_replacement":"CommandInfo::External { name, .. } | CommandInfo::BuiltIn { name, .. }","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms.rs:50:17\n   |\nLL |                 CommandInfo::External { name, .. } => name.to_string(),\n   |                 ----------------------------------^^^^^^^^^^^^^^^^^^^^\n   |                 |\n   |                 help: try merging the arm patterns: `CommandInfo::External { name, .. } | CommandInfo::BuiltIn { name, .. }`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms.rs:49:17\n   |\nLL |                 CommandInfo::BuiltIn { name, .. } => name.to_string(),\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
error: re-implementing `PartialEq::ne` is unnecessary
   |
   |
LL | #[derive(PartialEq, Eq)]
   |
   |
   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error



---
To only update this specific test, also pass `--test-args missing_const_for_fn/cant_be_const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/missing_const_for_fn/cant_be_const.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_const_for_fn/cant_be_const.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_const_for_fn/cant_be_const.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/cant_be_const.rs","byte_start":1477,"byte_end":1486,"line_start":65,"line_end":65,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(PartialEq, Eq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/missing_const_for_fn/cant_be_const.rs","byte_start":1477,"byte_end":1486,"line_start":65,"line_end":65,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(PartialEq, Eq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/missing_const_for_fn/cant_be_const.rs:65:10\n   |\nLL | #[derive(PartialEq, Eq)]\n   |          ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: re-implementing `PartialEq::ne` is unnecessary
-   |
-   |
-LL | /     fn ne(&self, _: &Foo) -> bool {
-LL | |         false
-LL | |     }
-   | |_____^
-   |
-   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
-error: aborting due to previous error
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/partialeq_ne_impl.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args partialeq_ne_impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/partialeq_ne_impl.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/partialeq_ne_impl.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/partialeq_ne_impl.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
 LL | /     match x {
 LL | /     match x {
 LL | |         Some(y) => {
 LL | |             println!("{:?}", y);
 LL | |         },
 LL | |         _ => (),
 LL | |     };
    |
    |
    = note: `-D clippy::single-match` implied by `-D warnings`
    |
    |
 LL ~     if let Some(y) = x {
 LL +         println!("{:?}", y);
 LL ~     };
 
 
 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
 LL | /     match x {
 LL | |         // Note the missing block braces.
 LL | |         // Note the missing block braces.
 LL | |         // We suggest `if let Some(y) = x { .. }` because the macro
 LL | |         // is expanded before we can do anything.
 LL | |         Some(y) => println!("{:?}", y),
 LL | |         _ => (),
 LL | |     }
    | |_____^ help: try this: `if let Some(y) = x { println!("{:?}", y) }`
 
 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
 LL | /     match z {
 LL | /     match z {
 LL | |         (2..=3, 7..=9) => dummy(),
 LL | |         _ => {},
 LL | |     };
    | |_____^ help: try this: `if let (2..=3, 7..=9) = z { dummy() }`
 
 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
 LL | /     match y {
 LL | /     match y {
 LL | |         Ok(y) => dummy(),
 LL | |         Err(..) => (),
 LL | |     };
    | |_____^ help: try this: `if let Ok(y) = y { dummy() }`
 
 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
 LL | /     match c {
 LL | /     match c {
 LL | |         Cow::Borrowed(..) => dummy(),
 LL | |         Cow::Owned(..) => (),
 LL | |     };
    | |_____^ help: try this: `if let Cow::Borrowed(..) = c { dummy() }`
 
 error: you seem to be trying to use `match` for an equality check. Consider using `if`
    |
 LL | /     match x {
 LL | /     match x {
 LL | |         "test" => println!(),
 LL | |         _ => (),
 LL | |     }
    | |_____^ help: try this: `if x == "test" { println!() }`
 
+error: re-implementing `PartialEq::ne` is unnecessary
+   |
+   |
+LL |     #[derive(PartialEq, Eq)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
+
 error: you seem to be trying to use `match` for an equality check. Consider using `if`
    |
 LL | /     match x {
 LL | /     match x {
 LL | |         Foo::A => println!(),
 LL | |         _ => (),
 LL | |     }
    | |_____^ help: try this: `if x == Foo::A { println!() }`
 
 error: you seem to be trying to use `match` for an equality check. Consider using `if`
    |
 LL | /     match x {
 LL | /     match x {
 LL | |         FOO_C => println!(),
 LL | |         _ => (),
 LL | |     }
    | |_____^ help: try this: `if x == FOO_C { println!() }`
 
 error: you seem to be trying to use `match` for an equality check. Consider using `if`
    |
 LL | /     match &&x {
 LL | /     match &&x {
 LL | |         Foo::A => println!(),
 LL | |         _ => (),
 LL | |     }
    | |_____^ help: try this: `if x == Foo::A { println!() }`
 
 error: you seem to be trying to use `match` for an equality check. Consider using `if`
    |
 LL | /     match &x {
 LL | /     match &x {
 LL | |         Foo::A => println!(),
 LL | |         _ => (),
 LL | |     }
    | |_____^ help: try this: `if x == &Foo::A { println!() }`
 
 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
 LL | /     match x {
 LL | /     match x {
 LL | |         Bar::A => println!(),
 LL | |         _ => (),
 LL | |     }
    | |_____^ help: try this: `if let Bar::A = x { println!() }`
 
 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
 LL | /     match x {
 LL | /     match x {
 LL | |         None => println!(),
 LL | |         _ => (),
 LL | |     };
    | |_____^ help: try this: `if let None = x { println!() }`
 
 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
 LL | /     match x {
 LL | /     match x {
 LL | |         (Some(_), _) => {},
 LL | |         (None, _) => {},
 LL | |     }
    | |_____^ help: try this: `if let (Some(_), _) = x {}`
 
 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
 LL | /     match x {
 LL | /     match x {
 LL | |         (Some(E::V), _) => todo!(),
 LL | |         (_, _) => {},
 LL | |     }
    | |_____^ help: try this: `if let (Some(E::V), _) = x { todo!() }`
 
 error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
    |
    |
 LL | /     match (Some(42), Some(E::V), Some(42)) {
 LL | |         (.., Some(E::V), _) => {},
 LL | |         (..) => {},
 LL | |     }
    | |_____^ help: try this: `if let (.., Some(E::V), _) = (Some(42), Some(E::V), Some(42)) {}`
-error: aborting due to 15 previous errors
+error: aborting due to 16 previous errors
 
 
---
To only update this specific test, also pass `--test-args single_match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/single_match.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/single_match.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/single_match.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":95,"byte_end":192,"line_start":8,"line_end":13,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        Some(y) => {","highlight_start":1,"highlight_end":21},{"text":"            println!(\"{:?}\", y);","highlight_start":1,"highlight_end":33},{"text":"        },","highlight_start":1,"highlight_end":11},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::single-match` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":95,"byte_end":192,"line_start":8,"line_end":13,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        Some(y) => {","highlight_start":1,"highlight_end":21},{"text":"            println!(\"{:?}\", y);","highlight_start":1,"highlight_end":33},{"text":"        },","highlight_start":1,"highlight_end":11},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let Some(y) = x {\n        println!(\"{:?}\", y);\n    }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:8:5\n   |\nLL | /     match x {\nLL | |         Some(y) => {\nLL | |             println!(\"{:?}\", y);\nLL | |         },\nLL | |         _ => (),\nLL | |     };\n   | |_____^\n   |\n   = note: `-D clippy::single-match` implied by `-D warnings`\nhelp: try this\n   |\nLL ~     if let Some(y) = x {\nLL +         println!(\"{:?}\", y);\nLL ~     };\n   |\n\n"}
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":222,"byte_end":454,"line_start":16,"line_end":22,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        // Note the missing block braces.","highlight_start":1,"highlight_end":42},{"text":"        // We suggest `if let Some(y) = x { .. }` because the macro","highlight_start":1,"highlight_end":68},{"text":"        // is expanded before we can do anything.","highlight_start":1,"highlight_end":50},{"text":"        Some(y) => println!(\"{:?}\", y),","highlight_start":1,"highlight_end":40},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":222,"byte_end":454,"line_start":16,"line_end":22,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        // Note the missing block braces.","highlight_start":1,"highlight_end":42},{"text":"        // We suggest `if let Some(y) = x { .. }` because the macro","highlight_start":1,"highlight_end":68},{"text":"        // is expanded before we can do anything.","highlight_start":1,"highlight_end":50},{"text":"        Some(y) => println!(\"{:?}\", y),","highlight_start":1,"highlight_end":40},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let Some(y) = x { println!(\"{:?}\", y) }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:16:5\n   |\nLL | /     match x {\nLL | |         // Note the missing block braces.\nLL | |         // We suggest `if let Some(y) = x { .. }` because the macro\nLL | |         // is expanded before we can do anything.\nLL | |         Some(y) => println!(\"{:?}\", y),\nLL | |         _ => (),\nLL | |     }\n   | |_____^ help: try this: `if let Some(y) = x { println!(\"{:?}\", y) }`\n\n"}
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":484,"byte_end":551,"line_start":25,"line_end":28,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match z {","highlight_start":5,"highlight_end":14},{"text":"        (2..=3, 7..=9) => dummy(),","highlight_start":1,"highlight_end":35},{"text":"        _ => {},","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":484,"byte_end":551,"line_start":25,"line_end":28,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match z {","highlight_start":5,"highlight_end":14},{"text":"        (2..=3, 7..=9) => dummy(),","highlight_start":1,"highlight_end":35},{"text":"        _ => {},","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let (2..=3, 7..=9) = z { dummy() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:25:5\n   |\nLL | /     match z {\nLL | |         (2..=3, 7..=9) => dummy(),\nLL | |         _ => {},\nLL | |     };\n   | |_____^ help: try this: `if let (2..=3, 7..=9) = z { dummy() }`\n\n"}
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1080,"byte_end":1144,"line_start":59,"line_end":62,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match y {","highlight_start":5,"highlight_end":14},{"text":"        Ok(y) => dummy(),","highlight_start":1,"highlight_end":26},{"text":"        Err(..) => (),","highlight_start":1,"highlight_end":23},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1080,"byte_end":1144,"line_start":59,"line_end":62,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match y {","highlight_start":5,"highlight_end":14},{"text":"        Ok(y) => dummy(),","highlight_start":1,"highlight_end":26},{"text":"        Err(..) => (),","highlight_start":1,"highlight_end":23},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let Ok(y) = y { dummy() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:59:5\n   |\nLL | /     match y {\nLL | |         Ok(y) => dummy(),\nLL | |         Err(..) => (),\nLL | |     };\n   | |_____^ help: try this: `if let Ok(y) = y { dummy() }`\n\n"}
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1183,"byte_end":1266,"line_start":66,"line_end":69,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match c {","highlight_start":5,"highlight_end":14},{"text":"        Cow::Borrowed(..) => dummy(),","highlight_start":1,"highlight_end":38},{"text":"        Cow::Owned(..) => (),","highlight_start":1,"highlight_end":30},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1183,"byte_end":1266,"line_start":66,"line_end":69,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match c {","highlight_start":5,"highlight_end":14},{"text":"        Cow::Borrowed(..) => dummy(),","highlight_start":1,"highlight_end":38},{"text":"        Cow::Owned(..) => (),","highlight_start":1,"highlight_end":30},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let Cow::Borrowed(..) = c { dummy() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:66:5\n   |\nLL | /     match c {\nLL | |         Cow::Borrowed(..) => dummy(),\nLL | |         Cow::Owned(..) => (),\nLL | |     };\n   | |_____^ help: try this: `if let Cow::Borrowed(..) = c { dummy() }`\n\n"}
{"message":"you seem to be trying to use `match` for an equality check. Consider using `if`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1518,"byte_end":1580,"line_start":87,"line_end":90,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        \"test\" => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1518,"byte_end":1580,"line_start":87,"line_end":90,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        \"test\" => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == \"test\" { println!() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for an equality check. Consider using `if`\n  --> tests/ui/single_match.rs:87:5\n   |\nLL | /     match x {\nLL | |         \"test\" => println!(),\nLL | |         _ => (),\nLL | |     }\n   | |_____^ help: try this: `if x == \"test\" { println!() }`\n\n"}
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1595,"byte_end":1604,"line_start":92,"line_end":92,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"    #[derive(PartialEq, Eq)]","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/single_match.rs","byte_start":1595,"byte_end":1604,"line_start":92,"line_end":92,"column_start":14,"column_end":23,"is_primary":false,"text":[{"text":"    #[derive(PartialEq, Eq)]","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/single_match.rs:92:14\n   |\nLL |     #[derive(PartialEq, Eq)]\n   |              ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"you seem to be trying to use `match` for an equality check. Consider using `if`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1695,"byte_end":1757,"line_start":100,"line_end":103,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        Foo::A => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1695,"byte_end":1757,"line_start":100,"line_end":103,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        Foo::A => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == Foo::A { println!() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for an equality check. Consider using `if`\n  --> tests/ui/single_match.rs:100:5\n   |\nLL | /     match x {\nLL | |         Foo::A => println!(),\nLL | |         _ => (),\nLL | |     }\n   | |_____^ help: try this: `if x == Foo::A { println!() }`\n\n"}
{"message":"you seem to be trying to use `match` for an equality check. Consider using `if`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1797,"byte_end":1858,"line_start":106,"line_end":109,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        FOO_C => println!(),","highlight_start":1,"highlight_end":29},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1797,"byte_end":1858,"line_start":106,"line_end":109,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        FOO_C => println!(),","highlight_start":1,"highlight_end":29},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == FOO_C { println!() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for an equality check. Consider using `if`\n  --> tests/ui/single_match.rs:106:5\n   |\nLL | /     match x {\nLL | |         FOO_C => println!(),\nLL | |         _ => (),\nLL | |     }\n   | |_____^ help: try this: `if x == FOO_C { println!() }`\n\n"}
{"message":"you seem to be trying to use `match` for an equality check. Consider using `if`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1864,"byte_end":1928,"line_start":111,"line_end":114,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match &&x {","highlight_start":5,"highlight_end":16},{"text":"        Foo::A => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1864,"byte_end":1928,"line_start":111,"line_end":114,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match &&x {","highlight_start":5,"highlight_end":16},{"text":"        Foo::A => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == Foo::A { println!() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for an equality check. Consider using `if`\n  --> tests/ui/single_match.rs:111:5\n   |\nLL | /     match &&x {\nLL | |         Foo::A => println!(),\nLL | |         _ => (),\nLL | |     }\n   | |_____^ help: try this: `if x == Foo::A { println!() }`\n\n"}
{"message":"you seem to be trying to use `match` for an equality check. Consider using `if`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1950,"byte_end":2013,"line_start":117,"line_end":120,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match &x {","highlight_start":5,"highlight_end":15},{"text":"        Foo::A => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":1950,"byte_end":2013,"line_start":117,"line_end":120,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match &x {","highlight_start":5,"highlight_end":15},{"text":"        Foo::A => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == &Foo::A { println!() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for an equality check. Consider using `if`\n  --> tests/ui/single_match.rs:117:5\n   |\nLL | /     match &x {\nLL | |         Foo::A => println!(),\nLL | |         _ => (),\nLL | |     }\n   | |_____^ help: try this: `if x == &Foo::A { println!() }`\n\n"}
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":2269,"byte_end":2331,"line_start":134,"line_end":137,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        Bar::A => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":2269,"byte_end":2331,"line_start":134,"line_end":137,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        Bar::A => println!(),","highlight_start":1,"highlight_end":30},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let Bar::A = x { println!() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:134:5\n   |\nLL | /     match x {\nLL | |         Bar::A => println!(),\nLL | |         _ => (),\nLL | |     }\n   | |_____^ help: try this: `if let Bar::A = x { println!() }`\n\n"}
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":2391,"byte_end":2451,"line_start":142,"line_end":145,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        None => println!(),","highlight_start":1,"highlight_end":28},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":2391,"byte_end":2451,"line_start":142,"line_end":145,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        None => println!(),","highlight_start":1,"highlight_end":28},{"text":"        _ => (),","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let None = x { println!() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:142:5\n   |\nLL | /     match x {\nLL | |         None => println!(),\nLL | |         _ => (),\nLL | |     };\n   | |_____^ help: try this: `if let None = x { println!() }`\n\n"}
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":2884,"byte_end":2952,"line_start":164,"line_end":167,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        (Some(_), _) => {},","highlight_start":1,"highlight_end":28},{"text":"        (None, _) => {},","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":2884,"byte_end":2952,"line_start":164,"line_end":167,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        (Some(_), _) => {},","highlight_start":1,"highlight_end":28},{"text":"        (None, _) => {},","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let (Some(_), _) = x {}","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:164:5\n   |\nLL | /     match x {\nLL | |         (Some(_), _) => {},\nLL | |         (None, _) => {},\nLL | |     }\n   | |_____^ help: try this: `if let (Some(_), _) = x {}`\n\n"}
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":2970,"byte_end":3043,"line_start":170,"line_end":173,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        (Some(E::V), _) => todo!(),","highlight_start":1,"highlight_end":36},{"text":"        (_, _) => {},","highlight_start":1,"highlight_end":22},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":2970,"byte_end":3043,"line_start":170,"line_end":173,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match x {","highlight_start":5,"highlight_end":14},{"text":"        (Some(E::V), _) => todo!(),","highlight_start":1,"highlight_end":36},{"text":"        (_, _) => {},","highlight_start":1,"highlight_end":22},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let (Some(E::V), _) = x { todo!() }","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:170:5\n   |\nLL | /     match x {\nLL | |         (Some(E::V), _) => todo!(),\nLL | |         (_, _) => {},\nLL | |     }\n   | |_____^ help: try this: `if let (Some(E::V), _) = x { todo!() }`\n\n"}
{"message":"you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`","code":{"code":"clippy::single_match","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":3061,"byte_end":3162,"line_start":176,"line_end":179,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match (Some(42), Some(E::V), Some(42)) {","highlight_start":5,"highlight_end":45},{"text":"        (.., Some(E::V), _) => {},","highlight_start":1,"highlight_end":35},{"text":"        (..) => {},","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/single_match.rs","byte_start":3061,"byte_end":3162,"line_start":176,"line_end":179,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    match (Some(42), Some(E::V), Some(42)) {","highlight_start":5,"highlight_end":45},{"text":"        (.., Some(E::V), _) => {},","highlight_start":1,"highlight_end":35},{"text":"        (..) => {},","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let (.., Some(E::V), _) = (Some(42), Some(E::V), Some(42)) {}","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`\n  --> tests/ui/single_match.rs:176:5\n   |\nLL | /     match (Some(42), Some(E::V), Some(42)) {\nLL | |         (.., Some(E::V), _) => {},\nLL | |         (..) => {},\nLL | |     }\n   | |_____^ help: try this: `if let (.., Some(E::V), _) = (Some(42), Some(E::V), Some(42)) {}`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: re-implementing `PartialEq::ne` is unnecessary
+   |
+   |
+LL | #[derive(PartialEq)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
+
 error: ==-comparison of unit values detected. This will always be true
    |
 LL |       if {
    |  ________^
 LL | |         true;
 LL | |         true;
 LL | |     } == {
 LL | |         false;
 LL | |     } {}
    |
    |
    = note: `-D clippy::unit-cmp` implied by `-D warnings`
 
 error: >-comparison of unit values detected. This will always be false
    |
 LL |       if {
    |  ________^
 LL | |         true;
 LL | |         true;
 LL | |     } > {
 LL | |         false;
 LL | |     } {}
    | |_____^
 
 error: `assert_eq` of unit values detected. This will always succeed
    |
 LL | /     assert_eq!(
 LL | |         {
 LL | |             true;
 LL | |             true;
 LL | |         },
 ...  |
 LL | |         }
 LL | |     );
    | |_____^
 
 error: `debug_assert_eq` of unit values detected. This will always succeed
    |
 LL | /     debug_assert_eq!(
 LL | |         {
 LL | |             true;
 LL | |             true;
 LL | |         },
 ...  |
 LL | |         }
 LL | |     );
    | |_____^
 
 error: `assert_ne` of unit values detected. This will always fail
    |
 LL | /     assert_ne!(
 LL | |         {
 LL | |             true;
 LL | |             true;
 LL | |         },
 ...  |
 LL | |         }
 LL | |     );
    | |_____^
 
 error: `debug_assert_ne` of unit values detected. This will always fail
    |
 LL | /     debug_assert_ne!(
 LL | |         {
 LL | |             true;
---
To only update this specific test, also pass `--test-args unit_cmp.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unit_cmp.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unit_cmp.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unit_cmp.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unit_cmp.rs","byte_start":97,"byte_end":106,"line_start":4,"line_end":4,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(PartialEq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/unit_cmp.rs","byte_start":97,"byte_end":106,"line_start":4,"line_end":4,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(PartialEq)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/unit_cmp.rs:4:10\n   |\nLL | #[derive(PartialEq)]\n   |          ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"==-comparison of unit values detected. This will always be true","code":{"code":"clippy::unit_cmp","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unit_cmp.rs","byte_start":239,"byte_end":286,"line_start":12,"line_end":16,"column_start":8,"column_end":6,"is_primary":true,"text":[{"text":"    if {","highlight_start":8,"highlight_end":9},{"text":"        true;","highlight_start":1,"highlight_end":14},{"text":"    } == {","highlight_start":1,"highlight_end":11},{"text":"        false;","highlight_start":1,"highlight_end":15},{"text":"    } {}","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::unit-cmp` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: ==-comparison of unit values detected. This will always be true\n  --> tests/ui/unit_cmp.rs:12:8\n   |\nLL |       if {\n   |  ________^\nLL | |         true;\nLL | |     } == {\nLL | |         false;\nLL | |     } {}\n   | |_____^\n   |\n   = note: `-D clippy::unit-cmp` implied by `-D warnings`\n\n"}
{"message":">-comparison of unit values detected. This will always be false","code":{"code":"clippy::unit_cmp","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unit_cmp.rs","byte_start":298,"byte_end":344,"line_start":18,"line_end":22,"column_start":8,"column_end":6,"is_primary":true,"text":[{"text":"    if {","highlight_start":8,"highlight_end":9},{"text":"        true;","highlight_start":1,"highlight_end":14},{"text":"    } > {","highlight_start":1,"highlight_end":10},{"text":"        false;","highlight_start":1,"highlight_end":15},{"text":"    } {}","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: >-comparison of unit values detected. This will always be false\n  --> tests/ui/unit_cmp.rs:18:8\n   |\nLL |       if {\n   |  ________^\nLL | |         true;\nLL | |     } > {\nLL | |         false;\nLL | |     } {}\n   | |_____^\n\n"}
{"message":"`assert_eq` of unit values detected. This will always succeed","code":{"code":"clippy::unit_cmp","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unit_cmp.rs","byte_start":353,"byte_end":448,"line_start":24,"line_end":31,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    assert_eq!(","highlight_start":5,"highlight_end":16},{"text":"        {","highlight_start":1,"highlight_end":10},{"text":"            true;","highlight_start":1,"highlight_end":18},{"text":"        },","highlight_start":1,"highlight_end":11},{"text":"        {","highlight_start":1,"highlight_end":10},{"text":"            false;","highlight_start":1,"highlight_end":19},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `assert_eq` of unit values detected. This will always succeed\n  --> tests/ui/unit_cmp.rs:24:5\n   |\nLL | /     assert_eq!(\nLL | |         {\nLL | |             true;\nLL | |         },\n...  |\nLL | |         }\nLL | |     );\n   | |_____^\n\n"}
{"message":"`debug_assert_eq` of unit values detected. This will always succeed","code":{"code":"clippy::unit_cmp","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unit_cmp.rs","byte_start":454,"byte_end":555,"line_start":32,"line_end":39,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    debug_assert_eq!(","highlight_start":5,"highlight_end":22},{"text":"        {","highlight_start":1,"highlight_end":10},{"text":"            true;","highlight_start":1,"highlight_end":18},{"text":"        },","highlight_start":1,"highlight_end":11},{"text":"        {","highlight_start":1,"highlight_end":10},{"text":"            false;","highlight_start":1,"highlight_end":19},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `debug_assert_eq` of unit values detected. This will always succeed\n  --> tests/ui/unit_cmp.rs:32:5\n   |\nLL | /     debug_assert_eq!(\nLL | |         {\nLL | |             true;\nLL | |         },\n...  |\nLL | |         }\nLL | |     );\n   | |_____^\n\n"}
{"message":"`assert_ne` of unit values detected. This will always fail","code":{"code":"clippy::unit_cmp","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unit_cmp.rs","byte_start":562,"byte_end":657,"line_start":41,"line_end":48,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    assert_ne!(","highlight_start":5,"highlight_end":16},{"text":"        {","highlight_start":1,"highlight_end":10},{"text":"            true;","highlight_start":1,"highlight_end":18},{"text":"        },","highlight_start":1,"highlight_end":11},{"text":"        {","highlight_start":1,"highlight_end":10},{"text":"            false;","highlight_start":1,"highlight_end":19},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `assert_ne` of unit values detected. This will always fail\n  --> tests/ui/unit_cmp.rs:41:5\n   |\nLL | /     assert_ne!(\nLL | |         {\nLL | |             true;\nLL | |         },\n...  |\nLL | |         }\nLL | |     );\n   | |_____^\n\n"}
{"message":"`debug_assert_ne` of unit values detected. This will always fail","code":{"code":"clippy::unit_cmp","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unit_cmp.rs","byte_start":663,"byte_end":764,"line_start":49,"line_end":56,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    debug_assert_ne!(","highlight_start":5,"highlight_end":22},{"text":"        {","highlight_start":1,"highlight_end":10},{"text":"            true;","highlight_start":1,"highlight_end":18},{"text":"        },","highlight_start":1,"highlight_end":11},{"text":"        {","highlight_start":1,"highlight_end":10},{"text":"            false;","highlight_start":1,"highlight_end":19},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `debug_assert_ne` of unit values detected. This will always fail\n  --> tests/ui/unit_cmp.rs:49:5\n   |\nLL | /     debug_assert_ne!(\nLL | |         {\nLL | |             true;\nLL | |         },\n...  |\nLL | |         }\nLL | |     );\n   | |_____^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: use Vec::sort here instead
   --> $DIR/unnecessary_sort_by.rs:14:5
    |
 LL |     vec.sort_by(|a, b| a.cmp(b));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort()`
    |
    = note: `-D clippy::unnecessary-sort-by` implied by `-D warnings`
 error: use Vec::sort here instead
   --> $DIR/unnecessary_sort_by.rs:15:5
    |
    |
 LL |     vec.sort_unstable_by(|a, b| a.cmp(b));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_unstable()`
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:16:5
    |
    |
 LL |     vec.sort_by(|a, b| (a + 5).abs().cmp(&(b + 5).abs()));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_by_key(|a| (a + 5).abs())`
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:17:5
    |
    |
 LL |     vec.sort_unstable_by(|a, b| id(-a).cmp(&id(-b)));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_unstable_by_key(|a| id(-a))`
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:20:5
    |
    |
 LL |     vec.sort_by(|a, b| (b + 5).abs().cmp(&(a + 5).abs()));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_by_key(|b| std::cmp::Reverse((b + 5).abs()))`
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:21:5
    |
    |
 LL |     vec.sort_unstable_by(|a, b| id(-b).cmp(&id(-a)));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_unstable_by_key(|b| std::cmp::Reverse(id(-b)))`
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:31:5
    |
    |
 LL |     vec.sort_by(|a, b| (***a).abs().cmp(&(***b).abs()));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_by_key(|a| (***a).abs())`
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:32:5
    |
    |
 LL |     vec.sort_unstable_by(|a, b| (***a).abs().cmp(&(***b).abs()));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_unstable_by_key(|a| (***a).abs())`
 
+error: re-implementing `PartialEq::ne` is unnecessary
+  --> $DIR/unnecessary_sort_by.rs:47:31
+   |
+LL |     #[derive(PartialOrd, Ord, PartialEq, Eq)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:91:9
    |
    |
 LL |         args.sort_by(|a, b| a.name().cmp(&b.name()));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `args.sort_by_key(|a| a.name())`
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:92:9
    |
    |
 LL |         args.sort_unstable_by(|a, b| a.name().cmp(&b.name()));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `args.sort_unstable_by_key(|a| a.name())`
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:94:9
    |
    |
 LL |         args.sort_by(|a, b| b.name().cmp(&a.name()));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `args.sort_by_key(|b| std::cmp::Reverse(b.name()))`
 error: use Vec::sort_by_key here instead
   --> $DIR/unnecessary_sort_by.rs:95:9
    |
    |
 LL |         args.sort_unstable_by(|a, b| b.name().cmp(&a.name()));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `args.sort_unstable_by_key(|b| std::cmp::Reverse(b.name()))`
-error: aborting due to 12 previous errors
+error: aborting due to 13 previous errors
 
 
---
To only update this specific test, also pass `--test-args unnecessary_sort_by.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unnecessary_sort_by.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_sort_by.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_sort_by.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use Vec::sort here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":233,"byte_end":261,"line_start":14,"line_end":14,"column_start":5,"column_end":33,"is_primary":true,"text":[{"text":"    vec.sort_by(|a, b| a.cmp(b));","highlight_start":5,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::unnecessary-sort-by` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":233,"byte_end":261,"line_start":14,"line_end":14,"column_start":5,"column_end":33,"is_primary":true,"text":[{"text":"    vec.sort_by(|a, b| a.cmp(b));","highlight_start":5,"highlight_end":33}],"label":null,"suggested_replacement":"vec.sort()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort here instead\n  --> tests/ui/unnecessary_sort_by.rs:14:5\n   |\nLL |     vec.sort_by(|a, b| a.cmp(b));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort()`\n   |\n   = note: `-D clippy::unnecessary-sort-by` implied by `-D warnings`\n\n"}
{"message":"use Vec::sort here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":267,"byte_end":304,"line_start":15,"line_end":15,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    vec.sort_unstable_by(|a, b| a.cmp(b));","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":267,"byte_end":304,"line_start":15,"line_end":15,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    vec.sort_unstable_by(|a, b| a.cmp(b));","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":"vec.sort_unstable()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort here instead\n  --> tests/ui/unnecessary_sort_by.rs:15:5\n   |\nLL |     vec.sort_unstable_by(|a, b| a.cmp(b));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_unstable()`\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":310,"byte_end":363,"line_start":16,"line_end":16,"column_start":5,"column_end":58,"is_primary":true,"text":[{"text":"    vec.sort_by(|a, b| (a + 5).abs().cmp(&(b + 5).abs()));","highlight_start":5,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":310,"byte_end":363,"line_start":16,"line_end":16,"column_start":5,"column_end":58,"is_primary":true,"text":[{"text":"    vec.sort_by(|a, b| (a + 5).abs().cmp(&(b + 5).abs()));","highlight_start":5,"highlight_end":58}],"label":null,"suggested_replacement":"vec.sort_by_key(|a| (a + 5).abs())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:16:5\n   |\nLL |     vec.sort_by(|a, b| (a + 5).abs().cmp(&(b + 5).abs()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_by_key(|a| (a + 5).abs())`\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":369,"byte_end":417,"line_start":17,"line_end":17,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    vec.sort_unstable_by(|a, b| id(-a).cmp(&id(-b)));","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":369,"byte_end":417,"line_start":17,"line_end":17,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    vec.sort_unstable_by(|a, b| id(-a).cmp(&id(-b)));","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":"vec.sort_unstable_by_key(|a| id(-a))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:17:5\n   |\nLL |     vec.sort_unstable_by(|a, b| id(-a).cmp(&id(-b)));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_unstable_by_key(|a| id(-a))`\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":547,"byte_end":600,"line_start":20,"line_end":20,"column_start":5,"column_end":58,"is_primary":true,"text":[{"text":"    vec.sort_by(|a, b| (b + 5).abs().cmp(&(a + 5).abs()));","highlight_start":5,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":547,"byte_end":600,"line_start":20,"line_end":20,"column_start":5,"column_end":58,"is_primary":true,"text":[{"text":"    vec.sort_by(|a, b| (b + 5).abs().cmp(&(a + 5).abs()));","highlight_start":5,"highlight_end":58}],"label":null,"suggested_replacement":"vec.sort_by_key(|b| std::cmp::Reverse((b + 5).abs()))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:20:5\n   |\nLL |     vec.sort_by(|a, b| (b + 5).abs().cmp(&(a + 5).abs()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_by_key(|b| std::cmp::Reverse((b + 5).abs()))`\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":606,"byte_end":654,"line_start":21,"line_end":21,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    vec.sort_unstable_by(|a, b| id(-b).cmp(&id(-a)));","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":606,"byte_end":654,"line_start":21,"line_end":21,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    vec.sort_unstable_by(|a, b| id(-b).cmp(&id(-a)));","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":"vec.sort_unstable_by_key(|b| std::cmp::Reverse(id(-b)))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:21:5\n   |\nLL |     vec.sort_unstable_by(|a, b| id(-b).cmp(&id(-a)));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_unstable_by_key(|b| std::cmp::Reverse(id(-b)))`\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":1036,"byte_end":1087,"line_start":31,"line_end":31,"column_start":5,"column_end":56,"is_primary":true,"text":[{"text":"    vec.sort_by(|a, b| (***a).abs().cmp(&(***b).abs()));","highlight_start":5,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":1036,"byte_end":1087,"line_start":31,"line_end":31,"column_start":5,"column_end":56,"is_primary":true,"text":[{"text":"    vec.sort_by(|a, b| (***a).abs().cmp(&(***b).abs()));","highlight_start":5,"highlight_end":56}],"label":null,"suggested_replacement":"vec.sort_by_key(|a| (***a).abs())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:31:5\n   |\nLL |     vec.sort_by(|a, b| (***a).abs().cmp(&(***b).abs()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_by_key(|a| (***a).abs())`\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":1093,"byte_end":1153,"line_start":32,"line_end":32,"column_start":5,"column_end":65,"is_primary":true,"text":[{"text":"    vec.sort_unstable_by(|a, b| (***a).abs().cmp(&(***b).abs()));","highlight_start":5,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":1093,"byte_end":1153,"line_start":32,"line_end":32,"column_start":5,"column_end":65,"is_primary":true,"text":[{"text":"    vec.sort_unstable_by(|a, b| (***a).abs().cmp(&(***b).abs()));","highlight_start":5,"highlight_end":65}],"label":null,"suggested_replacement":"vec.sort_unstable_by_key(|a| (***a).abs())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:32:5\n   |\nLL |     vec.sort_unstable_by(|a, b| (***a).abs().cmp(&(***b).abs()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `vec.sort_unstable_by_key(|a| (***a).abs())`\n\n"}
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":1634,"byte_end":1643,"line_start":47,"line_end":47,"column_start":31,"column_end":40,"is_primary":true,"text":[{"text":"    #[derive(PartialOrd, Ord, PartialEq, Eq)]","highlight_start":31,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":1634,"byte_end":1643,"line_start":47,"line_end":47,"column_start":31,"column_end":40,"is_primary":false,"text":[{"text":"    #[derive(PartialOrd, Ord, PartialEq, Eq)]","highlight_start":31,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/unnecessary_sort_by.rs:47:31\n   |\nLL |     #[derive(PartialOrd, Ord, PartialEq, Eq)]\n   |                               ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":2839,"byte_end":2883,"line_start":91,"line_end":91,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        args.sort_by(|a, b| a.name().cmp(&b.name()));","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":2839,"byte_end":2883,"line_start":91,"line_end":91,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        args.sort_by(|a, b| a.name().cmp(&b.name()));","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":"args.sort_by_key(|a| a.name())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:91:9\n   |\nLL |         args.sort_by(|a, b| a.name().cmp(&b.name()));\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `args.sort_by_key(|a| a.name())`\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":2893,"byte_end":2946,"line_start":92,"line_end":92,"column_start":9,"column_end":62,"is_primary":true,"text":[{"text":"        args.sort_unstable_by(|a, b| a.name().cmp(&b.name()));","highlight_start":9,"highlight_end":62}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":2893,"byte_end":2946,"line_start":92,"line_end":92,"column_start":9,"column_end":62,"is_primary":true,"text":[{"text":"        args.sort_unstable_by(|a, b| a.name().cmp(&b.name()));","highlight_start":9,"highlight_end":62}],"label":null,"suggested_replacement":"args.sort_unstable_by_key(|a| a.name())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:92:9\n   |\nLL |         args.sort_unstable_by(|a, b| a.name().cmp(&b.name()));\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `args.sort_unstable_by_key(|a| a.name())`\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":2975,"byte_end":3019,"line_start":94,"line_end":94,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        args.sort_by(|a, b| b.name().cmp(&a.name()));","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":2975,"byte_end":3019,"line_start":94,"line_end":94,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        args.sort_by(|a, b| b.name().cmp(&a.name()));","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":"args.sort_by_key(|b| std::cmp::Reverse(b.name()))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:94:9\n   |\nLL |         args.sort_by(|a, b| b.name().cmp(&a.name()));\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `args.sort_by_key(|b| std::cmp::Reverse(b.name()))`\n\n"}
{"message":"use Vec::sort_by_key here instead","code":{"code":"clippy::unnecessary_sort_by","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":3029,"byte_end":3082,"line_start":95,"line_end":95,"column_start":9,"column_end":62,"is_primary":true,"text":[{"text":"        args.sort_unstable_by(|a, b| b.name().cmp(&a.name()));","highlight_start":9,"highlight_end":62}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_sort_by.rs","byte_start":3029,"byte_end":3082,"line_start":95,"line_end":95,"column_start":9,"column_end":62,"is_primary":true,"text":[{"text":"        args.sort_unstable_by(|a, b| b.name().cmp(&a.name()));","highlight_start":9,"highlight_end":62}],"label":null,"suggested_replacement":"args.sort_unstable_by_key(|b| std::cmp::Reverse(b.name()))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use Vec::sort_by_key here instead\n  --> tests/ui/unnecessary_sort_by.rs:95:9\n   |\nLL |         args.sort_unstable_by(|a, b| b.name().cmp(&a.name()));\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `args.sort_unstable_by_key(|b| std::cmp::Reverse(b.name()))`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: re-implementing `PartialEq::ne` is unnecessary
+  --> $DIR/wildcard_enum_match_arm.rs:19:34
+   |
+LL | #[derive(Clone, Copy, Debug, Eq, PartialEq)]
+   |
+   |
+   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`
+   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
+
 error: wildcard match will also match any future added variants
   --> $DIR/wildcard_enum_match_arm.rs:42:9
    |
 LL |         _ => eprintln!("Not red"),
    |         ^ help: try this: `Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan`
 note: the lint level is defined here
   --> $DIR/wildcard_enum_match_arm.rs:4:9
    |
    |
 LL | #![deny(clippy::wildcard_enum_match_arm)]
 
 
 error: wildcard match will also match any future added variants
   --> $DIR/wildcard_enum_match_arm.rs:46:9
    |
 LL |         _not_red => eprintln!("Not red"),
    |         ^^^^^^^^ help: try this: `_not_red @ Color::Green | _not_red @ Color::Blue | _not_red @ Color::Rgb(..) | _not_red @ Color::Cyan`
 
 error: wildcard match will also match any future added variants
   --> $DIR/wildcard_enum_match_arm.rs:50:9
    |
 LL |         not_red => format!("{:?}", not_red),
    |         ^^^^^^^ help: try this: `not_red @ Color::Green | not_red @ Color::Blue | not_red @ Color::Rgb(..) | not_red @ Color::Cyan`
 
 error: wildcard match will also match any future added variants
   --> $DIR/wildcard_enum_match_arm.rs:66:9
    |
 LL |         _ => "No red",
    |         ^ help: try this: `Color::Red | Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan`
 
 error: wildcard matches known variants and will also match future added variants
   --> $DIR/wildcard_enum_match_arm.rs:83:9
 LL |         _ => {},
 LL |         _ => {},
    |         ^ help: try this: `ErrorKind::PermissionDenied | _`
 
 error: wildcard matches known variants and will also match future added variants
   --> $DIR/wildcard_enum_match_arm.rs:101:13
 LL |             _ => (),
 LL |             _ => (),
    |             ^ help: try this: `Enum::B | _`
-error: aborting due to 6 previous errors
+error: aborting due to 7 previous errors
 
 
---
To only update this specific test, also pass `--test-args wildcard_enum_match_arm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/wildcard_enum_match_arm.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/wildcard_enum_match_arm.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-96837c1bd81aaeaa.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/wildcard_enum_match_arm.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"re-implementing `PartialEq::ne` is unnecessary","code":{"code":"clippy::partialeq_ne_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":406,"byte_end":415,"line_start":19,"line_end":19,"column_start":34,"column_end":43,"is_primary":true,"text":[{"text":"#[derive(Clone, Copy, Debug, Eq, PartialEq)]","highlight_start":34,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":406,"byte_end":415,"line_start":19,"line_end":19,"column_start":34,"column_end":43,"is_primary":false,"text":[{"text":"#[derive(Clone, Copy, Debug, Eq, PartialEq)]","highlight_start":34,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(PartialEq)]","def_site_span":{"file_name":"/checkout/library/core/src/cmp.rs","byte_start":7685,"byte_end":7716,"line_start":239,"line_end":239,"column_start":1,"column_end":32,"is_primary":false,"text":[{"text":"pub macro PartialEq($item:item) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::partialeq-ne-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: re-implementing `PartialEq::ne` is unnecessary\n  --> tests/ui/wildcard_enum_match_arm.rs:19:34\n   |\nLL | #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n   |                                  ^^^^^^^^^\n   |\n   = note: `-D clippy::partialeq-ne-impl` implied by `-D warnings`\n   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"wildcard match will also match any future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":871,"byte_end":872,"line_start":42,"line_end":42,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => eprintln!(\"Not red\"),","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":60,"byte_end":91,"line_start":4,"line_end":4,"column_start":9,"column_end":40,"is_primary":true,"text":[{"text":"#![deny(clippy::wildcard_enum_match_arm)]","highlight_start":9,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":871,"byte_end":872,"line_start":42,"line_end":42,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => eprintln!(\"Not red\"),","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard match will also match any future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:42:9\n   |\nLL |         _ => eprintln!(\"Not red\"),\n   |         ^ help: try this: `Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan`\n   |\nnote: the lint level is defined here\n  --> tests/ui/wildcard_enum_match_arm.rs:4:9\n   |\nLL | #![deny(clippy::wildcard_enum_match_arm)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"wildcard match will also match any future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":970,"byte_end":978,"line_start":46,"line_end":46,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        _not_red => eprintln!(\"Not red\"),","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":970,"byte_end":978,"line_start":46,"line_end":46,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        _not_red => eprintln!(\"Not red\"),","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":"_not_red @ Color::Green | _not_red @ Color::Blue | _not_red @ Color::Rgb(..) | _not_red @ Color::Cyan","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard match will also match any future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:46:9\n   |\nLL |         _not_red => eprintln!(\"Not red\"),\n   |         ^^^^^^^^ help: try this: `_not_red @ Color::Green | _not_red @ Color::Blue | _not_red @ Color::Rgb(..) | _not_red @ Color::Cyan`\n\n"}
{"message":"wildcard match will also match any future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":1088,"byte_end":1095,"line_start":50,"line_end":50,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        not_red => format!(\"{:?}\", not_red),","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":1088,"byte_end":1095,"line_start":50,"line_end":50,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        not_red => format!(\"{:?}\", not_red),","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":"not_red @ Color::Green | not_red @ Color::Blue | not_red @ Color::Rgb(..) | not_red @ Color::Cyan","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard match will also match any future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:50:9\n   |\nLL |         not_red => format!(\"{:?}\", not_red),\n   |         ^^^^^^^ help: try this: `not_red @ Color::Green | not_red @ Color::Blue | not_red @ Color::Rgb(..) | not_red @ Color::Cyan`\n\n"}
{"message":"wildcard match will also match any future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":1582,"byte_end":1583,"line_start":66,"line_end":66,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => \"No red\",","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":1582,"byte_end":1583,"line_start":66,"line_end":66,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => \"No red\",","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"Color::Red | Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard match will also match any future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:66:9\n   |\nLL |         _ => \"No red\",\n   |         ^ help: try this: `Color::Red | Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan`\n\n"}
{"message":"wildcard matches known variants and will also match future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":2095,"byte_end":2096,"line_start":83,"line_end":83,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => {},","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":2095,"byte_end":2096,"line_start":83,"line_end":83,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => {},","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"ErrorKind::PermissionDenied | _","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard matches known variants and will also match future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:83:9\n   |\nLL |         _ => {},\n   |         ^ help: try this: `ErrorKind::PermissionDenied | _`\n\n"}
{"message":"wildcard matches known variants and will also match future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":2467,"byte_end":2468,"line_start":101,"line_end":101,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"            _ => (),","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":2467,"byte_end":2468,"line_start":101,"line_end":101,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"            _ => (),","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":"Enum::B | _","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard matches known variants and will also match future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:101:13\n   |\nLL |             _ => (),\n   |             ^ help: try this: `Enum::B | _`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
