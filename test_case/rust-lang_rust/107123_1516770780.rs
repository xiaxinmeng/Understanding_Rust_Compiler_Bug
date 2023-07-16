plain
    |
 LL |     0;
    |     ^^
    |
    = note: `-D clippy::no-effect` implied by `-D warnings`
 error: statement with no effect
-  --> $DIR/no_effect.rs:98:5
error: test failed, to rerun pass `--test compile-test`
+  --> $DIR/no_effect.rs:103:5
---
 error: statement with no effect
-  --> $DIR/no_effect.rs:102:5
+  --> $DIR/no_effect.rs:107:5
    |
 LL |     Struct { ..s };
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:103:5
+  --> $DIR/no_effect.rs:108:5
+  --> $DIR/no_effect.rs:108:5
    |
 LL |     Union { a: 0 };
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:104:5
+  --> $DIR/no_effect.rs:109:5
---
 error: statement with no effect
-  --> $DIR/no_effect.rs:116:5
+  --> $DIR/no_effect.rs:121:5
    |
 LL |     [42, 55][1];
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:117:5
+  --> $DIR/no_effect.rs:122:5
+  --> $DIR/no_effect.rs:122:5
    |
 LL |     (42, 55).1;
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:118:5
+  --> $DIR/no_effect.rs:123:5
---
 error: statement with no effect
-  --> $DIR/no_effect.rs:119:5
+  --> $DIR/no_effect.rs:124:5
    |
 LL |     [42; 55][13];
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:121:5
+  --> $DIR/no_effect.rs:126:5
+  --> $DIR/no_effect.rs:126:5
    |
 LL |     || x += 5;
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:123:5
+  --> $DIR/no_effect.rs:128:5
+  --> $DIR/no_effect.rs:128:5
    |
 LL |     FooString { s: s };
 
 
 error: binding to `_` prefixed variable with no side-effect
+  --> $DIR/no_effect.rs:129:5
    |
 LL |     let _unused = 1;
    |     ^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^
    |
    = note: `-D clippy::no-effect-underscore-binding` implied by `-D warnings`
 
 error: binding to `_` prefixed variable with no side-effect
+  --> $DIR/no_effect.rs:130:5
    |
    |
 LL |     let _penguin = || println!("Some helpful closure");
 
 
 error: binding to `_` prefixed variable with no side-effect
+  --> $DIR/no_effect.rs:131:5
    |
    |
 LL |     let _duck = Struct { field: 0 };
 
 
 error: binding to `_` prefixed variable with no side-effect
+  --> $DIR/no_effect.rs:132:5
    |
    |
 LL |     let _cat = [2, 4, 6, 8][2];
 
 error: aborting due to 29 previous errors
 
 
---
To only update this specific test, also pass `--test-args no_effect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/no_effect.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/no_effect.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-8bc4c00ed7c71629.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-7f0c26842d997632.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-b83448bd4610d9d2.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-62999cf098a4f169.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/no_effect.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1881,"byte_end":1883,"line_start":102,"line_end":102,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    0;","highlight_start":5,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::no-effect` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:102:5\n   |\nLL |     0;\n   |     ^^\n   |\n   = note: `-D clippy::no-effect` implied by `-D warnings`\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1888,"byte_end":1891,"line_start":103,"line_end":103,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    s2;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:103:5\n   |\nLL |     s2;\n   |     ^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1896,"byte_end":1901,"line_start":104,"line_end":104,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    Unit;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:104:5\n   |\nLL |     Unit;\n   |     ^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1906,"byte_end":1915,"line_start":105,"line_end":105,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    Tuple(0);","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:105:5\n   |\nLL |     Tuple(0);\n   |     ^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1920,"byte_end":1940,"line_start":106,"line_end":106,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    Struct { field: 0 };","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:106:5\n   |\nLL |     Struct { field: 0 };\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1945,"byte_end":1960,"line_start":107,"line_end":107,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Struct { ..s };","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:107:5\n   |\nLL |     Struct { ..s };\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1965,"byte_end":1980,"line_start":108,"line_end":108,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Union { a: 0 };","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:108:5\n   |\nLL |     Union { a: 0 };\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1985,"byte_end":2000,"line_start":109,"line_end":109,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Enum::Tuple(0);","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:109:5\n   |\nLL |     Enum::Tuple(0);\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2005,"byte_end":2031,"line_start":110,"line_end":110,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    Enum::Struct { field: 0 };","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:110:5\n   |\nLL |     Enum::Struct { field: 0 };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2036,"byte_end":2042,"line_start":111,"line_end":111,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    5 + 6;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:111:5\n   |\nLL |     5 + 6;\n   |     ^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2047,"byte_end":2052,"line_start":112,"line_end":112,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    *&42;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:112:5\n   |\nLL |     *&42;\n   |     ^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2057,"byte_end":2060,"line_start":113,"line_end":113,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    &6;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:113:5\n   |\nLL |     &6;\n   |     ^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2065,"byte_end":2075,"line_start":114,"line_end":114,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    (5, 6, 7);","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:114:5\n   |\nLL |     (5, 6, 7);\n   |     ^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2080,"byte_end":2083,"line_start":115,"line_end":115,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    ..;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:115:5\n   |\nLL |     ..;\n   |     ^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2088,"byte_end":2092,"line_start":116,"line_end":116,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    5..;","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:116:5\n   |\nLL |     5..;\n   |     ^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2097,"byte_end":2101,"line_start":117,"line_end":117,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    ..5;","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:117:5\n   |\nLL |     ..5;\n   |     ^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2106,"byte_end":2111,"line_start":118,"line_end":118,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    5..6;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:118:5\n   |\nLL |     5..6;\n   |     ^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2116,"byte_end":2122,"line_start":119,"line_end":119,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    5..=6;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:119:5\n   |\nLL |     5..=6;\n   |     ^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2127,"byte_end":2136,"line_start":120,"line_end":120,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    [42, 55];","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:120:5\n   |\nLL |     [42, 55];\n   |     ^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2141,"byte_end":2153,"line_start":121,"line_end":121,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    [42, 55][1];","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:121:5\n   |\nLL |     [42, 55][1];\n   |     ^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2158,"byte_end":2169,"line_start":122,"line_end":122,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    (42, 55).1;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:122:5\n   |\nLL |     (42, 55).1;\n   |     ^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2174,"byte_end":2183,"line_start":123,"line_end":123,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    [42; 55];","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:123:5\n   |\nLL |     [42; 55];\n   |     ^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2188,"byte_end":2201,"line_start":124,"line_end":124,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    [42; 55][13];","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:124:5\n   |\nLL |     [42; 55][13];\n   |     ^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2225,"byte_end":2235,"line_start":126,"line_end":126,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    || x += 5;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:126:5\n   |\nLL |     || x += 5;\n   |     ^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2274,"byte_end":2293,"line_start":128,"line_end":128,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    FooString { s: s };","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:128:5\n   |\nLL |     FooString { s: s };\n   |     ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"binding to `_` prefixed variable with no side-effect","code":{"code":"clippy::no_effect_underscore_binding","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2298,"byte_end":2314,"line_start":129,"line_end":129,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    let _unused = 1;","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::no-effect-underscore-binding` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: binding to `_` prefixed variable with no side-effect\n  --> tests/ui/no_effect.rs:129:5\n   |\nLL |     let _unused = 1;\n   |     ^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::no-effect-underscore-binding` implied by `-D warnings`\n\n"}
{"message":"binding to `_` prefixed variable with no side-effect","code":{"code":"clippy::no_effect_underscore_binding","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2319,"byte_end":2370,"line_start":130,"line_end":130,"column_start":5,"column_end":56,"is_primary":true,"text":[{"text":"    let _penguin = || println!(\"Some helpful closure\");","highlight_start":5,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: binding to `_` prefixed variable with no side-effect\n  --> tests/ui/no_effect.rs:130:5\n   |\nLL |     let _penguin = || println!(\"Some helpful closure\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"binding to `_` prefixed variable with no side-effect","code":{"code":"clippy::no_effect_underscore_binding","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2375,"byte_end":2407,"line_start":131,"line_end":131,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    let _duck = Struct { field: 0 };","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: binding to `_` prefixed variable with no side-effect\n  --> tests/ui/no_effect.rs:131:5\n   |\nLL |     let _duck = Struct { field: 0 };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"binding to `_` prefixed variable with no side-effect","code":{"code":"clippy::no_effect_underscore_binding","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2412,"byte_end":2439,"line_start":132,"line_end":132,"column_start":5,"column_end":32,"is_primary":true,"text":[{"text":"    let _cat = [2, 4, 6, 8][2];","highlight_start":5,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: binding to `_` prefixed variable with no side-effect\n  --> tests/ui/no_effect.rs:132:5\n   |\nLL |     let _cat = [2, 4, 6, 8][2];\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/lib.rs:111:22
