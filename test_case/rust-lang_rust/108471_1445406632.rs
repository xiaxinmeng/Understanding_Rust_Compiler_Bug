plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
    |
 LL |     0;
    |     ^^
    |
    = note: `-D clippy::no-effect` implied by `-D warnings`
 error: statement with no effect
   --> $DIR/no_effect.rs:93:5
    |
 LL |     s2;
---
 
 error: statement with no effect
   --> $DIR/no_effect.rs:96:5
    |
 LL |     Struct { field: 0 };
 
 error: statement with no effect
   --> $DIR/no_effect.rs:97:5
    |
    |
 LL |     Struct { ..s };
 
 error: statement with no effect
   --> $DIR/no_effect.rs:98:5
    |
    |
 LL |     Union { a: 0 };
 
 error: statement with no effect
   --> $DIR/no_effect.rs:99:5
    |
    |
 LL |     Enum::Tuple(0);
    |     ^^^^^^^^^^^^^^^
 
 error: statement with no effect
   --> $DIR/no_effect.rs:100:5
    |
 LL |     Enum::Struct { field: 0 };
 
 error: statement with no effect
   --> $DIR/no_effect.rs:101:5
    |
---
 error: statement with no effect
-  --> $DIR/no_effect.rs:112:5
+  --> $DIR/no_effect.rs:111:5
    |
 LL |     [42, 55][1];
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:113:5
+  --> $DIR/no_effect.rs:112:5
+  --> $DIR/no_effect.rs:112:5
    |
 LL |     (42, 55).1;
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:114:5
+  --> $DIR/no_effect.rs:113:5
---
 error: statement with no effect
-  --> $DIR/no_effect.rs:115:5
+  --> $DIR/no_effect.rs:114:5
    |
 LL |     [42; 55][13];
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:117:5
+  --> $DIR/no_effect.rs:116:5
+  --> $DIR/no_effect.rs:116:5
    |
 LL |     || x += 5;
 
 error: statement with no effect
-  --> $DIR/no_effect.rs:119:5
+  --> $DIR/no_effect.rs:118:5
+  --> $DIR/no_effect.rs:118:5
    |
 LL |     FooString { s: s };
 
 
 error: binding to `_` prefixed variable with no side-effect
+  --> $DIR/no_effect.rs:119:5
    |
 LL |     let _unused = 1;
    |     ^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^
    |
    = note: `-D clippy::no-effect-underscore-binding` implied by `-D warnings`
 
 error: binding to `_` prefixed variable with no side-effect
+  --> $DIR/no_effect.rs:120:5
    |
    |
 LL |     let _penguin = || println!("Some helpful closure");
 
 
 error: binding to `_` prefixed variable with no side-effect
+  --> $DIR/no_effect.rs:121:5
    |
    |
 LL |     let _duck = Struct { field: 0 };
 
 
 error: binding to `_` prefixed variable with no side-effect
+  --> $DIR/no_effect.rs:122:5
    |
    |
 LL |     let _cat = [2, 4, 6, 8][2];
 
-error: aborting due to 30 previous errors
+error: aborting due to 29 previous errors
 
---
To only update this specific test, also pass `--test-args no_effect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/no_effect.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/no_effect.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/no_effect.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1659,"byte_end":1661,"line_start":92,"line_end":92,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    0;","highlight_start":5,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::no-effect` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:92:5\n   |\nLL |     0;\n   |     ^^\n   |\n   = note: `-D clippy::no-effect` implied by `-D warnings`\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1666,"byte_end":1669,"line_start":93,"line_end":93,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    s2;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:93:5\n   |\nLL |     s2;\n   |     ^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1674,"byte_end":1679,"line_start":94,"line_end":94,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    Unit;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:94:5\n   |\nLL |     Unit;\n   |     ^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1684,"byte_end":1693,"line_start":95,"line_end":95,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    Tuple(0);","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:95:5\n   |\nLL |     Tuple(0);\n   |     ^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1698,"byte_end":1718,"line_start":96,"line_end":96,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    Struct { field: 0 };","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:96:5\n   |\nLL |     Struct { field: 0 };\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1723,"byte_end":1738,"line_start":97,"line_end":97,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Struct { ..s };","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:97:5\n   |\nLL |     Struct { ..s };\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1743,"byte_end":1758,"line_start":98,"line_end":98,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Union { a: 0 };","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:98:5\n   |\nLL |     Union { a: 0 };\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1763,"byte_end":1778,"line_start":99,"line_end":99,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    Enum::Tuple(0);","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:99:5\n   |\nLL |     Enum::Tuple(0);\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1783,"byte_end":1809,"line_start":100,"line_end":100,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    Enum::Struct { field: 0 };","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:100:5\n   |\nLL |     Enum::Struct { field: 0 };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1814,"byte_end":1820,"line_start":101,"line_end":101,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    5 + 6;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:101:5\n   |\nLL |     5 + 6;\n   |     ^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1825,"byte_end":1830,"line_start":102,"line_end":102,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    *&42;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:102:5\n   |\nLL |     *&42;\n   |     ^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1835,"byte_end":1838,"line_start":103,"line_end":103,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    &6;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:103:5\n   |\nLL |     &6;\n   |     ^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1843,"byte_end":1853,"line_start":104,"line_end":104,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    (5, 6, 7);","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:104:5\n   |\nLL |     (5, 6, 7);\n   |     ^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1858,"byte_end":1861,"line_start":105,"line_end":105,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    ..;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:105:5\n   |\nLL |     ..;\n   |     ^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1866,"byte_end":1870,"line_start":106,"line_end":106,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    5..;","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:106:5\n   |\nLL |     5..;\n   |     ^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1875,"byte_end":1879,"line_start":107,"line_end":107,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    ..5;","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:107:5\n   |\nLL |     ..5;\n   |     ^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1884,"byte_end":1889,"line_start":108,"line_end":108,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    5..6;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:108:5\n   |\nLL |     5..6;\n   |     ^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1894,"byte_end":1900,"line_start":109,"line_end":109,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    5..=6;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:109:5\n   |\nLL |     5..=6;\n   |     ^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1905,"byte_end":1914,"line_start":110,"line_end":110,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    [42, 55];","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:110:5\n   |\nLL |     [42, 55];\n   |     ^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1919,"byte_end":1931,"line_start":111,"line_end":111,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    [42, 55][1];","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:111:5\n   |\nLL |     [42, 55][1];\n   |     ^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1936,"byte_end":1947,"line_start":112,"line_end":112,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    (42, 55).1;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:112:5\n   |\nLL |     (42, 55).1;\n   |     ^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1952,"byte_end":1961,"line_start":113,"line_end":113,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    [42; 55];","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:113:5\n   |\nLL |     [42; 55];\n   |     ^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":1966,"byte_end":1979,"line_start":114,"line_end":114,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    [42; 55][13];","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:114:5\n   |\nLL |     [42; 55][13];\n   |     ^^^^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2003,"byte_end":2013,"line_start":116,"line_end":116,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    || x += 5;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:116:5\n   |\nLL |     || x += 5;\n   |     ^^^^^^^^^^\n\n"}
{"message":"statement with no effect","code":{"code":"clippy::no_effect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2052,"byte_end":2071,"line_start":118,"line_end":118,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    FooString { s: s };","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statement with no effect\n  --> tests/ui/no_effect.rs:118:5\n   |\nLL |     FooString { s: s };\n   |     ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"binding to `_` prefixed variable with no side-effect","code":{"code":"clippy::no_effect_underscore_binding","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2076,"byte_end":2092,"line_start":119,"line_end":119,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    let _unused = 1;","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::no-effect-underscore-binding` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: binding to `_` prefixed variable with no side-effect\n  --> tests/ui/no_effect.rs:119:5\n   |\nLL |     let _unused = 1;\n   |     ^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::no-effect-underscore-binding` implied by `-D warnings`\n\n"}
{"message":"binding to `_` prefixed variable with no side-effect","code":{"code":"clippy::no_effect_underscore_binding","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2097,"byte_end":2148,"line_start":120,"line_end":120,"column_start":5,"column_end":56,"is_primary":true,"text":[{"text":"    let _penguin = || println!(\"Some helpful closure\");","highlight_start":5,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: binding to `_` prefixed variable with no side-effect\n  --> tests/ui/no_effect.rs:120:5\n   |\nLL |     let _penguin = || println!(\"Some helpful closure\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"binding to `_` prefixed variable with no side-effect","code":{"code":"clippy::no_effect_underscore_binding","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2153,"byte_end":2185,"line_start":121,"line_end":121,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    let _duck = Struct { field: 0 };","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: binding to `_` prefixed variable with no side-effect\n  --> tests/ui/no_effect.rs:121:5\n   |\nLL |     let _duck = Struct { field: 0 };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"binding to `_` prefixed variable with no side-effect","code":{"code":"clippy::no_effect_underscore_binding","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/no_effect.rs","byte_start":2190,"byte_end":2217,"line_start":122,"line_end":122,"column_start":5,"column_end":32,"is_primary":true,"text":[{"text":"    let _cat = [2, 4, 6, 8][2];","highlight_start":5,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: binding to `_` prefixed variable with no side-effect\n  --> tests/ui/no_effect.rs:122:5\n   |\nLL |     let _cat = [2, 4, 6, 8][2];\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:51:5
+  --> $DIR/unnecessary_operation.rs:50:5
    |
 LL |     Tuple(get_number());
    |     ^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`
    |
    = note: `-D clippy::unnecessary-operation` implied by `-D warnings`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:52:5
+  --> $DIR/unnecessary_operation.rs:51:5
    |
    |
 LL |     Struct { field: get_number() };
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:53:5
+  --> $DIR/unnecessary_operation.rs:52:5
    |
    |
 LL |     Struct { ..get_struct() };
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_struct();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:54:5
+  --> $DIR/unnecessary_operation.rs:53:5
    |
    |
 LL |     Enum::Tuple(get_number());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:55:5
+  --> $DIR/unnecessary_operation.rs:54:5
    |
    |
 LL |     Enum::Struct { field: get_number() };
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:56:5
+  --> $DIR/unnecessary_operation.rs:55:5
    |
    |
 LL |     5 + get_number();
    |     ^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `5;get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:57:5
+  --> $DIR/unnecessary_operation.rs:56:5
    |
    |
 LL |     *&get_number();
    |     ^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:58:5
+  --> $DIR/unnecessary_operation.rs:57:5
    |
    |
 LL |     &get_number();
    |     ^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`
 
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:59:5
+  --> $DIR/unnecessary_operation.rs:58:5
    |
 LL |     (5, 6, get_number());
    |     ^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `5;6;get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:60:5
+  --> $DIR/unnecessary_operation.rs:59:5
    |
---
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:63:5
+  --> $DIR/unnecessary_operation.rs:61:5
    |
 LL |     5..get_number();
    |     ^^^^^^^^^^^^^^^^ help: statement can be reduced to: `5;get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:64:5
+  --> $DIR/unnecessary_operation.rs:62:5
    |
    |
 LL |     [42, get_number()];
    |     ^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `42;get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:65:5
+  --> $DIR/unnecessary_operation.rs:63:5
    |
    |
 LL |     [42, 55][get_usize()];
    |     ^^^^^^^^^^^^^^^^^^^^^^ help: statement can be written as: `assert!([42, 55].len() > get_usize());`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:66:5
+  --> $DIR/unnecessary_operation.rs:64:5
    |
    |
 LL |     (42, get_number()).1;
    |     ^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `42;get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:67:5
+  --> $DIR/unnecessary_operation.rs:65:5
    |
    |
 LL |     [get_number(); 55];
    |     ^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:68:5
+  --> $DIR/unnecessary_operation.rs:66:5
    |
    |
 LL |     [42; 55][get_usize()];
    |     ^^^^^^^^^^^^^^^^^^^^^^ help: statement can be written as: `assert!([42; 55].len() > get_usize());`
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:69:5
+  --> $DIR/unnecessary_operation.rs:67:5
    |
---
 error: unnecessary operation
-  --> $DIR/unnecessary_operation.rs:72:5
+  --> $DIR/unnecessary_operation.rs:70:5
    |
 LL | /     FooString {
 LL | |         s: String::from("blah"),
 LL | |     };
    | |______^ help: statement can be reduced to: `String::from("blah");`
-error: aborting due to 20 previous errors
+error: aborting due to 19 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_operation.stage-id.stderr
diff of fixed:

 // run-rustfix
 
-#![feature(box_syntax)]
 #![allow(clippy::deref_addrof, dead_code, unused, clippy::no_effect)]
 #![warn(clippy::unnecessary_operation)]
 struct Tuple(i32);
 struct Struct {
     field: i32,
 }
 }
 enum Enum {
     Tuple(i32),
     Struct { field: i32 },
 struct DropStruct {
     field: i32,
 }
 }
 impl Drop for DropStruct {
     fn drop(&mut self) {}
 }
 struct DropTuple(i32);
 impl Drop for DropTuple {
     fn drop(&mut self) {}
 }
 enum DropEnum {
     Tuple(i32),
     Struct { field: i32 },
 }
 impl Drop for DropEnum {
     fn drop(&mut self) {}
 struct FooString {
     s: String,
 }
 
 
 fn get_number() -> i32 {
     0
 }
 
 fn get_usize() -> usize {
     0
 }
 fn get_struct() -> Struct {
     Struct { field: 0 }
 }
 fn get_drop_struct() -> DropStruct {
     DropStruct { field: 0 }
 
 fn main() {
     get_number();
     get_number();
     get_number();
     get_struct();
     get_number();
     get_number();
     5;get_number();
     get_number();
     get_number();
     5;6;get_number();
     get_number();
     get_number();
     get_number();
     5;get_number();
     42;get_number();
     assert!([42, 55].len() > get_usize());
     42;get_number();
     get_number();
     assert!([42; 55].len() > get_usize());
     get_number();
     String::from("blah");
     // Do not warn
     // Do not warn
     DropTuple(get_number());
     DropStruct { field: get_number() };
     DropStruct { field: get_number() };
     DropStruct { ..get_drop_struct() };
     DropEnum::Tuple(get_number());
     DropEnum::Struct { field: get_number() };
     // Issue #9954
     fn one() -> i8 {
         1
     }
     }
     macro_rules! use_expr {
         ($($e:expr),*) => {{ $($e;)* }}
     }
     use_expr!(isize::MIN / -(one() as isize), i8::MIN / -one());
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_operation.stage-id.fixed
To only update this specific test, also pass `--test-args unnecessary_operation.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unnecessary_operation.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_operation.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-504e25b9109e124f.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-ff2c2aaa2e33fd1f.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_operation.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":764,"byte_end":784,"line_start":50,"line_end":50,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    Tuple(get_number());","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::unnecessary-operation` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":764,"byte_end":784,"line_start":50,"line_end":50,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    Tuple(get_number());","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:50:5\n   |\nLL |     Tuple(get_number());\n   |     ^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`\n   |\n   = note: `-D clippy::unnecessary-operation` implied by `-D warnings`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":789,"byte_end":820,"line_start":51,"line_end":51,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    Struct { field: get_number() };","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":789,"byte_end":820,"line_start":51,"line_end":51,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    Struct { field: get_number() };","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:51:5\n   |\nLL |     Struct { field: get_number() };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":825,"byte_end":851,"line_start":52,"line_end":52,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    Struct { ..get_struct() };","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":825,"byte_end":851,"line_start":52,"line_end":52,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    Struct { ..get_struct() };","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":"get_struct();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:52:5\n   |\nLL |     Struct { ..get_struct() };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_struct();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":856,"byte_end":882,"line_start":53,"line_end":53,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    Enum::Tuple(get_number());","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":856,"byte_end":882,"line_start":53,"line_end":53,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    Enum::Tuple(get_number());","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:53:5\n   |\nLL |     Enum::Tuple(get_number());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":887,"byte_end":924,"line_start":54,"line_end":54,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    Enum::Struct { field: get_number() };","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":887,"byte_end":924,"line_start":54,"line_end":54,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    Enum::Struct { field: get_number() };","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:54:5\n   |\nLL |     Enum::Struct { field: get_number() };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":929,"byte_end":946,"line_start":55,"line_end":55,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    5 + get_number();","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":929,"byte_end":946,"line_start":55,"line_end":55,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    5 + get_number();","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":"5;get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:55:5\n   |\nLL |     5 + get_number();\n   |     ^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `5;get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":951,"byte_end":966,"line_start":56,"line_end":56,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    *&get_number();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":951,"byte_end":966,"line_start":56,"line_end":56,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    *&get_number();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:56:5\n   |\nLL |     *&get_number();\n   |     ^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":971,"byte_end":985,"line_start":57,"line_end":57,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    &get_number();","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":971,"byte_end":985,"line_start":57,"line_end":57,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    &get_number();","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:57:5\n   |\nLL |     &get_number();\n   |     ^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":990,"byte_end":1011,"line_start":58,"line_end":58,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    (5, 6, get_number());","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":990,"byte_end":1011,"line_start":58,"line_end":58,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    (5, 6, get_number());","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":"5;6;get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:58:5\n   |\nLL |     (5, 6, get_number());\n   |     ^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `5;6;get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1016,"byte_end":1031,"line_start":59,"line_end":59,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    get_number()..;","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1016,"byte_end":1031,"line_start":59,"line_end":59,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    get_number()..;","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:59:5\n   |\nLL |     get_number()..;\n   |     ^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1036,"byte_end":1051,"line_start":60,"line_end":60,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    ..get_number();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1036,"byte_end":1051,"line_start":60,"line_end":60,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    ..get_number();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:60:5\n   |\nLL |     ..get_number();\n   |     ^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1056,"byte_end":1072,"line_start":61,"line_end":61,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    5..get_number();","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1056,"byte_end":1072,"line_start":61,"line_end":61,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    5..get_number();","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":"5;get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:61:5\n   |\nLL |     5..get_number();\n   |     ^^^^^^^^^^^^^^^^ help: statement can be reduced to: `5;get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1077,"byte_end":1096,"line_start":62,"line_end":62,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    [42, get_number()];","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1077,"byte_end":1096,"line_start":62,"line_end":62,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    [42, get_number()];","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":"42;get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:62:5\n   |\nLL |     [42, get_number()];\n   |     ^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `42;get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1101,"byte_end":1123,"line_start":63,"line_end":63,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    [42, 55][get_usize()];","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be written as","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1101,"byte_end":1123,"line_start":63,"line_end":63,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    [42, 55][get_usize()];","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":"assert!([42, 55].len() > get_usize());","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:63:5\n   |\nLL |     [42, 55][get_usize()];\n   |     ^^^^^^^^^^^^^^^^^^^^^^ help: statement can be written as: `assert!([42, 55].len() > get_usize());`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1128,"byte_end":1149,"line_start":64,"line_end":64,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    (42, get_number()).1;","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1128,"byte_end":1149,"line_start":64,"line_end":64,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    (42, get_number()).1;","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":"42;get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:64:5\n   |\nLL |     (42, get_number()).1;\n   |     ^^^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `42;get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1154,"byte_end":1173,"line_start":65,"line_end":65,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    [get_number(); 55];","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1154,"byte_end":1173,"line_start":65,"line_end":65,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    [get_number(); 55];","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:65:5\n   |\nLL |     [get_number(); 55];\n   |     ^^^^^^^^^^^^^^^^^^^ help: statement can be reduced to: `get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1178,"byte_end":1200,"line_start":66,"line_end":66,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    [42; 55][get_usize()];","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be written as","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1178,"byte_end":1200,"line_start":66,"line_end":66,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    [42; 55][get_usize()];","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":"assert!([42; 55].len() > get_usize());","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:66:5\n   |\nLL |     [42; 55][get_usize()];\n   |     ^^^^^^^^^^^^^^^^^^^^^^ help: statement can be written as: `assert!([42; 55].len() > get_usize());`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1205,"byte_end":1234,"line_start":67,"line_end":69,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    {","highlight_start":5,"highlight_end":6},{"text":"        get_number()","highlight_start":1,"highlight_end":21},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1205,"byte_end":1234,"line_start":67,"line_end":69,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    {","highlight_start":5,"highlight_end":6},{"text":"        get_number()","highlight_start":1,"highlight_end":21},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"get_number();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:67:5\n   |\nLL | /     {\nLL | |         get_number()\nLL | |     };\n   | |______^ help: statement can be reduced to: `get_number();`\n\n"}
{"message":"unnecessary operation","code":{"code":"clippy::unnecessary_operation","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1239,"byte_end":1290,"line_start":70,"line_end":72,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    FooString {","highlight_start":5,"highlight_end":16},{"text":"        s: String::from(\"blah\"),","highlight_start":1,"highlight_end":33},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"statement can be reduced to","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_operation.rs","byte_start":1239,"byte_end":1290,"line_start":70,"line_end":72,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    FooString {","highlight_start":5,"highlight_end":16},{"text":"        s: String::from(\"blah\"),","highlight_start":1,"highlight_end":33},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"String::from(\"blah\");","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary operation\n  --> tests/ui/unnecessary_operation.rs:70:5\n   |\nLL | /     FooString {\nLL | |         s: String::from(\"blah\"),\nLL | |     };\n   | |______^ help: statement can be reduced to: `String::from(\"blah\");`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/lib.rs:111:22
