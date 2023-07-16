plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling chalk-derive v0.55.0
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0599]: no method named `find_path` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:698:32
    |
698 |     let found_str = json_value.find_path(&["dog", "cat", "mouse"]);
    |                                ^^^^^^^^^ method not found in `rustc_serialize::json::Json`

error[E0599]: no method named `is_object` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:721:24
    |
721 |     assert!(json_value.is_object());
    |                        ^^^^^^^^^ help: there is an associated function with a similar name: `as_object`

error[E0599]: no method named `is_array` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:734:24
    |
734 |     assert!(json_value.is_array());
    |                        ^^^^^^^^ help: there is an associated function with a similar name: `as_array`

error[E0599]: no method named `is_string` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:748:24
    |
748 |     assert!(json_value.is_string());
    |                        ^^^^^^^^^ help: there is an associated function with a similar name: `as_string`

error[E0599]: no method named `is_number` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:762:24
    |
762 |     assert!(json_value.is_number());
    |                        ^^^^^^^^^ method not found in `rustc_serialize::json::Json`

error[E0599]: no method named `is_i64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:768:24
    |
768 |     assert!(json_value.is_i64());
    |                        ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_i64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:771:25
    |
771 |     assert!(!json_value.is_i64());
    |                         ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_i64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:774:25
    |
774 |     assert!(!json_value.is_i64());
    |                         ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_u64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:780:24
    |
780 |     assert!(json_value.is_u64());
    |                        ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_u64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:783:25
    |
783 |     assert!(!json_value.is_u64());
    |                         ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_u64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:786:25
    |
786 |     assert!(!json_value.is_u64());
    |                         ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_f64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:792:25
    |
792 |     assert!(!json_value.is_f64());
    |                         ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_f64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:795:25
    |
795 |     assert!(!json_value.is_f64());
    |                         ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_f64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:798:24
    |
798 |     assert!(json_value.is_f64());
    |                        ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_f64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:801:24
    |
801 |     assert!(json_value.is_f64());
    |                        ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `as_i64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:807:31
    |
807 |     let json_num = json_value.as_i64();
    |                               ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `as_f64` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:821:31
    |
821 |     let json_num = json_value.as_f64();
    |                               ^^^^^^ help: there is an associated function with a similar name: `as_u64`

error[E0599]: no method named `is_boolean` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:828:24
    |
828 |     assert!(json_value.is_boolean());
    |                        ^^^^^^^^^^ help: there is an associated function with a similar name: `as_boolean`

error[E0599]: no method named `is_null` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:842:24
    |
842 |     assert!(json_value.is_null());
    |                        ^^^^^^^ method not found in `rustc_serialize::json::Json`

error[E0599]: no method named `as_null` found for enum `rustc_serialize::json::Json` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:848:32
    |
848 |     let json_null = json_value.as_null();
    |                                ^^^^^^^ method not found in `rustc_serialize::json::Json`
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0599]: no method named `indent` found for struct `AsPrettyJson<'_, rustc_serialize::json::Json>` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:913:63
    |
913 |         write!(&mut writer, "{}", json::as_pretty_json(&json).indent(i)).unwrap();
    |                                                               ^^^^^^ private field, not a method

error[E0599]: no method named `is_equal_to` found for reference `&Stack` in the current scope
   --> compiler/rustc_serialize/tests/json.rs:987:28
    |
987 |         if !parser.stack().is_equal_to(expected_stack) {
    |                            ^^^^^^^^^^^ method not found in `&Stack`
error: aborting due to 22 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_serialize`
error: could not compile `rustc_serialize`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: associated function is never used: `find_path`
    --> compiler/rustc_serialize/src/json.rs:1199:8
     |
1199 |     fn find_path<'a>(&'a self, keys: &[&str]) -> Option<&'a Json> {
     |
     |
     = note: `-D dead-code` implied by `-D warnings`

error: associated function is never used: `is_object`
    --> compiler/rustc_serialize/src/json.rs:1230:8
1230 |     fn is_object(&self) -> bool {
     |        ^^^^^^^^^


error: associated function is never used: `is_array`
    --> compiler/rustc_serialize/src/json.rs:1245:8
     |
1245 |     fn is_array(&self) -> bool {


error: associated function is never used: `is_string`
    --> compiler/rustc_serialize/src/json.rs:1260:8
     |
1260 |     fn is_string(&self) -> bool {


error: associated function is never used: `is_number`
    --> compiler/rustc_serialize/src/json.rs:1275:8
1275 |     fn is_number(&self) -> bool {
     |        ^^^^^^^^^


error: associated function is never used: `is_i64`
    --> compiler/rustc_serialize/src/json.rs:1281:8
     |
1281 |     fn is_i64(&self) -> bool {


error: associated function is never used: `is_u64`
    --> compiler/rustc_serialize/src/json.rs:1287:8
     |
1287 |     fn is_u64(&self) -> bool {


error: associated function is never used: `is_f64`
    --> compiler/rustc_serialize/src/json.rs:1293:8
     |
1293 |     fn is_f64(&self) -> bool {


error: associated function is never used: `as_i64`
    --> compiler/rustc_serialize/src/json.rs:1300:8
     |
1300 |     fn as_i64(&self) -> Option<i64> {


error: associated function is never used: `as_f64`
    --> compiler/rustc_serialize/src/json.rs:1321:8
     |
1321 |     fn as_f64(&self) -> Option<f64> {


error: associated function is never used: `is_boolean`
    --> compiler/rustc_serialize/src/json.rs:1332:8
     |
1332 |     fn is_boolean(&self) -> bool {

error: associated function is never used: `is_null`
    --> compiler/rustc_serialize/src/json.rs:1347:8
     |
     |
1347 |     fn is_null(&self) -> bool {


error: associated function is never used: `as_null`
    --> compiler/rustc_serialize/src/json.rs:1354:8
     |
1354 |     fn as_null(&self) -> Option<()> {


error: associated function is never used: `indent`
    --> compiler/rustc_serialize/src/json.rs:2766:8
     |
2766 |     fn indent(mut self, indent: usize) -> AsPrettyJson<'a, T> {

error: aborting due to 14 previous errors

error: build failed
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_attr" "-p" "rustc_macros" "-p" "rustc_lexer" "-p" "rustc_parse_format" "-p" "rustc_index" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_hir" "-p" "rustc_save_analysis" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_error_codes" "-p" "rustc_data_structures" "-p" "rustc_serialize" "-p" "rustc_typeck" "-p" "rustc_feature" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_errors" "-p" "rustc_hir_pretty" "-p" "rustc_target" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_ast" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_resolve" "-p" "rustc_query_impl" "-p" "rustc_ast_lowering" "-p" "rustc_symbol_mangling" "-p" "rustc_incremental" "-p" "rustc_ast_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:56
