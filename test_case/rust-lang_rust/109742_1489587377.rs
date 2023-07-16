plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---

 error: use of a disallowed method `regex::Regex::new`
   --> $DIR/conf_disallowed_methods.rs:33:14
    |
 LL |     let re = Regex::new(r"ab.*c").unwrap();
    |
    |
    = note: `-D clippy::disallowed-methods` implied by `-D warnings`
 error: use of a disallowed method `regex::Regex::is_match`
   --> $DIR/conf_disallowed_methods.rs:34:5
    |
    |
 LL |     re.is_match("abc");
    |
    = note: no matching allowed (from clippy.toml)
 
 error: use of a disallowed method `std::iter::Iterator::sum`
 error: use of a disallowed method `std::iter::Iterator::sum`
error: test failed, to rerun pass `--test compile-test`
   --> $DIR/conf_disallowed_methods.rs:37:5
    |
 LL |     a.iter().sum::<i32>();
 
 error: use of a disallowed method `slice::sort_unstable`
   --> $DIR/conf_disallowed_methods.rs:39:5
    |
    |
 LL |     a.sort_unstable();
    |     ^^^^^^^^^^^^^^^^^
 
 error: use of a disallowed method `f32::clamp`
   --> $DIR/conf_disallowed_methods.rs:41:13
    |
 LL |     let _ = 2.0f32.clamp(3.0f32, 4.0f32);
 
+error: use of a disallowed method `f32::clamp`
+  --> $DIR/conf_disallowed_methods.rs:42:13
+   |
+   |
+LL |     let _ = 2.0f64.clamp(3.0f64, 4.0f64);
+
 error: use of a disallowed method `regex::Regex::new`
   --> $DIR/conf_disallowed_methods.rs:44:61
    |
    |
 LL |     let indirect: fn(&str) -> Result<Regex, regex::Error> = Regex::new;
 
 error: use of a disallowed method `f32::clamp`
   --> $DIR/conf_disallowed_methods.rs:47:28
    |
    |
 LL |     let in_call = Box::new(f32::clamp);
 
 error: use of a disallowed method `regex::Regex::new`
   --> $DIR/conf_disallowed_methods.rs:48:53
    |
    |
 LL |     let in_method_call = ["^", "$"].into_iter().map(Regex::new);
 
 error: use of a disallowed method `futures::stream::select_all`
   --> $DIR/conf_disallowed_methods.rs:51:31
    |
    |
 LL |     let same_name_as_module = select_all(vec![empty::<()>()]);
 
 error: use of a disallowed method `conf_disallowed_methods::local_fn`
   --> $DIR/conf_disallowed_methods.rs:53:5
    |
    |
 LL |     local_fn();
    |     ^^^^^^^^^^
 
 error: use of a disallowed method `conf_disallowed_methods::local_mod::f`
   --> $DIR/conf_disallowed_methods.rs:54:5
    |
 LL |     local_mod::f();
 
 error: use of a disallowed method `conf_disallowed_methods::Struct::method`
   --> $DIR/conf_disallowed_methods.rs:56:5
    |
    |
 LL |     s.method();
 
 error: use of a disallowed method `conf_disallowed_methods::Trait::provided_method`
   --> $DIR/conf_disallowed_methods.rs:57:5
    |
    |
 LL |     s.provided_method();
    |     ^^^^^^^^^^^^^^^^^^^
 
 error: use of a disallowed method `conf_disallowed_methods::Trait::implemented_method`
   --> $DIR/conf_disallowed_methods.rs:58:5
    |
 LL |     s.implemented_method();
 
-error: aborting due to 14 previous errors
+error: aborting due to 15 previous errors
 
---
To only update this specific test, also pass `--test-args toml_disallowed_methods/conf_disallowed_methods.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-toml" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-toml/toml_disallowed_methods/conf_disallowed_methods.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e5b19e58ae33983e.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-60efa73d6fad7e91.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-e22c3fc74241d5e4.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-23c15a454288152e.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-fc796eef9a515a44.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-40ff94c070351d6c.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-d8aee1681c496322.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--crate-name" "conf_disallowed_methods" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-toml/toml_disallowed_methods/conf_disallowed_methods.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of a disallowed method `regex::Regex::new`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":482,"byte_end":502,"line_start":33,"line_end":33,"column_start":14,"column_end":34,"is_primary":true,"text":[{"text":"    let re = Regex::new(r\"ab.*c\").unwrap();","highlight_start":14,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::disallowed-methods` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of a disallowed method `regex::Regex::new`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:33:14\n   |\nLL |     let re = Regex::new(r\"ab.*c\").unwrap();\n   |              ^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::disallowed-methods` implied by `-D warnings`\n\n"}
{"message":"use of a disallowed method `regex::Regex::is_match`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":517,"byte_end":535,"line_start":34,"line_end":34,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    re.is_match(\"abc\");","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"no matching allowed (from clippy.toml)","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of a disallowed method `regex::Regex::is_match`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:34:5\n   |\nLL |     re.is_match(\"abc\");\n   |     ^^^^^^^^^^^^^^^^^^\n   |\n   = note: no matching allowed (from clippy.toml)\n\n"}
{"message":"use of a disallowed method `std::iter::Iterator::sum`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":576,"byte_end":597,"line_start":37,"line_end":37,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    a.iter().sum::<i32>();","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `std::iter::Iterator::sum`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:37:5\n   |\nLL |     a.iter().sum::<i32>();\n   |     ^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `slice::sort_unstable`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":604,"byte_end":621,"line_start":39,"line_end":39,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    a.sort_unstable();","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `slice::sort_unstable`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:39:5\n   |\nLL |     a.sort_unstable();\n   |     ^^^^^^^^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `f32::clamp`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":636,"byte_end":664,"line_start":41,"line_end":41,"column_start":13,"column_end":41,"is_primary":true,"text":[{"text":"    let _ = 2.0f32.clamp(3.0f32, 4.0f32);","highlight_start":13,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `f32::clamp`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:41:13\n   |\nLL |     let _ = 2.0f32.clamp(3.0f32, 4.0f32);\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `f32::clamp`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":678,"byte_end":706,"line_start":42,"line_end":42,"column_start":13,"column_end":41,"is_primary":true,"text":[{"text":"    let _ = 2.0f64.clamp(3.0f64, 4.0f64);","highlight_start":13,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `f32::clamp`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:42:13\n   |\nLL |     let _ = 2.0f64.clamp(3.0f64, 4.0f64);\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `regex::Regex::new`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":769,"byte_end":779,"line_start":44,"line_end":44,"column_start":61,"column_end":71,"is_primary":true,"text":[{"text":"    let indirect: fn(&str) -> Result<Regex, regex::Error> = Regex::new;","highlight_start":61,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `regex::Regex::new`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:44:61\n   |\nLL |     let indirect: fn(&str) -> Result<Regex, regex::Error> = Regex::new;\n   |                                                             ^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `f32::clamp`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":846,"byte_end":856,"line_start":47,"line_end":47,"column_start":28,"column_end":38,"is_primary":true,"text":[{"text":"    let in_call = Box::new(f32::clamp);","highlight_start":28,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `f32::clamp`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:47:28\n   |\nLL |     let in_call = Box::new(f32::clamp);\n   |                            ^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `regex::Regex::new`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":911,"byte_end":921,"line_start":48,"line_end":48,"column_start":53,"column_end":63,"is_primary":true,"text":[{"text":"    let in_method_call = [\"^\", \"$\"].into_iter().map(Regex::new);","highlight_start":53,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `regex::Regex::new`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:48:53\n   |\nLL |     let in_method_call = [\"^\", \"$\"].into_iter().map(Regex::new);\n   |                                                     ^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `futures::stream::select_all`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":1046,"byte_end":1077,"line_start":51,"line_end":51,"column_start":31,"column_end":62,"is_primary":true,"text":[{"text":"    let same_name_as_module = select_all(vec![empty::<()>()]);","highlight_start":31,"highlight_end":62}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `futures::stream::select_all`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:51:31\n   |\nLL |     let same_name_as_module = select_all(vec![empty::<()>()]);\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `conf_disallowed_methods::local_fn`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":1084,"byte_end":1094,"line_start":53,"line_end":53,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    local_fn();","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `conf_disallowed_methods::local_fn`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:53:5\n   |\nLL |     local_fn();\n   |     ^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `conf_disallowed_methods::local_mod::f`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":1100,"byte_end":1114,"line_start":54,"line_end":54,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    local_mod::f();","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `conf_disallowed_methods::local_mod::f`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:54:5\n   |\nLL |     local_mod::f();\n   |     ^^^^^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `conf_disallowed_methods::Struct::method`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":1140,"byte_end":1150,"line_start":56,"line_end":56,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    s.method();","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `conf_disallowed_methods::Struct::method`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:56:5\n   |\nLL |     s.method();\n   |     ^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `conf_disallowed_methods::Trait::provided_method`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":1156,"byte_end":1175,"line_start":57,"line_end":57,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    s.provided_method();","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `conf_disallowed_methods::Trait::provided_method`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:57:5\n   |\nLL |     s.provided_method();\n   |     ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"use of a disallowed method `conf_disallowed_methods::Trait::implemented_method`","code":{"code":"clippy::disallowed_methods","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs","byte_start":1181,"byte_end":1203,"line_start":58,"line_end":58,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    s.implemented_method();","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: use of a disallowed method `conf_disallowed_methods::Trait::implemented_method`\n  --> /checkout/src/tools/clippy/tests/ui-toml/toml_disallowed_methods/conf_disallowed_methods.rs:58:5\n   |\nLL |     s.implemented_method();\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', src/tools/clippy/tests/compile-test.rs:238:22
