plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---

---- compile_test stdout ----
diff of stderr:

 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:5:28
    |
 LL | const CONST_NOT_OK: Option<BTreeMap<String, ()>> = None;
    |
    |
    = note: `-D clippy::zero-sized-map-values` implied by `-D warnings`
    = help: consider using a set instead
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:8:30
    |
 LL | static STATIC_NOT_OK: Option<BTreeMap<String, ()>> = None;
    |                              ^^^^^^^^^^^^^^^^^^^^
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:11:17
    |
 LL | type NotOkMap = BTreeMap<String, ()>;
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:15:11
    |
 LL |     NotOk(BTreeMap<String, ()>),
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:20:13
    |
 LL |     not_ok: BTreeMap<String, ()>,
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:22:22
    |
 LL |     also_not_ok: Vec<BTreeMap<usize, ()>>,
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:30:30
    |
 LL |     fn weird_map(&self, map: BTreeMap<usize, ()>);
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:38:25
    |
 LL |     fn not_ok(&self) -> BTreeMap<String, ()> {
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:55:14
    |
 LL | fn test(map: BTreeMap<String, ()>, key: &str) -> BTreeMap<String, ()> {
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:55:50
    |
 LL | fn test(map: BTreeMap<String, ()>, key: &str) -> BTreeMap<String, ()> {
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:64:35
    |
 LL |     let _: BTreeMap<String, ()> = BTreeMap::new();
-   |                                   ^^^^^^^^^^^^^
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:64:12
    |
 LL |     let _: BTreeMap<String, ()> = BTreeMap::new();
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_btreemap_values.rs:67:12
    |
 LL |     let _: BTreeMap<_, _> = std::iter::empty::<(String, ())>().collect();
    |
    = help: consider using a set instead
 
 error: aborting due to 13 previous errors
 error: aborting due to 13 previous errors
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/zero_sized_btreemap_values.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args zero_sized_btreemap_values.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/zero_sized_btreemap_values.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/zero_sized_btreemap_values.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f0962f37786a4888.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-cd40a3b060be150c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b396e06d7d6d4d75.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-db6d918eacac1474.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-4cd0b4140eeb9c8d.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/zero_sized_btreemap_values.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":156,"byte_end":176,"line_start":5,"line_end":5,"column_start":28,"column_end":48,"is_primary":true,"text":[{"text":"const CONST_NOT_OK: Option<BTreeMap<String, ()>> = None;","highlight_start":28,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::zero-sized-map-values` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:5:28\n   |\nLL | const CONST_NOT_OK: Option<BTreeMap<String, ()>> = None;\n   |                            ^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::zero-sized-map-values` implied by `-D warnings`\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":274,"byte_end":294,"line_start":8,"line_end":8,"column_start":30,"column_end":50,"is_primary":true,"text":[{"text":"static STATIC_NOT_OK: Option<BTreeMap<String, ()>> = None;","highlight_start":30,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:8:30\n   |\nLL | static STATIC_NOT_OK: Option<BTreeMap<String, ()>> = None;\n   |                              ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":359,"byte_end":379,"line_start":11,"line_end":11,"column_start":17,"column_end":37,"is_primary":true,"text":[{"text":"type NotOkMap = BTreeMap<String, ()>;","highlight_start":17,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:11:17\n   |\nLL | type NotOkMap = BTreeMap<String, ()>;\n   |                 ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":441,"byte_end":461,"line_start":15,"line_end":15,"column_start":11,"column_end":31,"is_primary":true,"text":[{"text":"    NotOk(BTreeMap<String, ()>),","highlight_start":11,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:15:11\n   |\nLL |     NotOk(BTreeMap<String, ()>),\n   |           ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":526,"byte_end":546,"line_start":20,"line_end":20,"column_start":13,"column_end":33,"is_primary":true,"text":[{"text":"    not_ok: BTreeMap<String, ()>,","highlight_start":13,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:20:13\n   |\nLL |     not_ok: BTreeMap<String, ()>,\n   |             ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":570,"byte_end":589,"line_start":22,"line_end":22,"column_start":22,"column_end":41,"is_primary":true,"text":[{"text":"    also_not_ok: Vec<BTreeMap<usize, ()>>,","highlight_start":22,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:22:22\n   |\nLL |     also_not_ok: Vec<BTreeMap<usize, ()>>,\n   |                      ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":702,"byte_end":721,"line_start":30,"line_end":30,"column_start":30,"column_end":49,"is_primary":true,"text":[{"text":"    fn weird_map(&self, map: BTreeMap<usize, ()>);","highlight_start":30,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:30:30\n   |\nLL |     fn weird_map(&self, map: BTreeMap<usize, ()>);\n   |                              ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":832,"byte_end":852,"line_start":38,"line_end":38,"column_start":25,"column_end":45,"is_primary":true,"text":[{"text":"    fn not_ok(&self) -> BTreeMap<String, ()> {","highlight_start":25,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:38:25\n   |\nLL |     fn not_ok(&self) -> BTreeMap<String, ()> {\n   |                         ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":1104,"byte_end":1124,"line_start":55,"line_end":55,"column_start":14,"column_end":34,"is_primary":true,"text":[{"text":"fn test(map: BTreeMap<String, ()>, key: &str) -> BTreeMap<String, ()> {","highlight_start":14,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:55:14\n   |\nLL | fn test(map: BTreeMap<String, ()>, key: &str) -> BTreeMap<String, ()> {\n   |              ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":1140,"byte_end":1160,"line_start":55,"line_end":55,"column_start":50,"column_end":70,"is_primary":true,"text":[{"text":"fn test(map: BTreeMap<String, ()>, key: &str) -> BTreeMap<String, ()> {","highlight_start":50,"highlight_end":70}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:55:50\n   |\nLL | fn test(map: BTreeMap<String, ()>, key: &str) -> BTreeMap<String, ()> {\n   |                                                  ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":1320,"byte_end":1328,"line_start":64,"line_end":64,"column_start":35,"column_end":43,"is_primary":true,"text":[{"text":"    let _: BTreeMap<String, ()> = BTreeMap::new();","highlight_start":35,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:64:35\n   |\nLL |     let _: BTreeMap<String, ()> = BTreeMap::new();\n   |                                   ^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":1297,"byte_end":1317,"line_start":64,"line_end":64,"column_start":12,"column_end":32,"is_primary":true,"text":[{"text":"    let _: BTreeMap<String, ()> = BTreeMap::new();","highlight_start":12,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:64:12\n   |\nLL |     let _: BTreeMap<String, ()> = BTreeMap::new();\n   |            ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_btreemap_values.rs","byte_start":1403,"byte_end":1417,"line_start":67,"line_end":67,"column_start":12,"column_end":26,"is_primary":true,"text":[{"text":"    let _: BTreeMap<_, _> = std::iter::empty::<(String, ())>().collect();","highlight_start":12,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_btreemap_values.rs:67:12\n   |\nLL |     let _: BTreeMap<_, _> = std::iter::empty::<(String, ())>().collect();\n   |            ^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:5:28
    |
 LL | const CONST_NOT_OK: Option<HashMap<String, ()>> = None;
    |
    |
    = note: `-D clippy::zero-sized-map-values` implied by `-D warnings`
    = help: consider using a set instead
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:8:30
    |
 LL | static STATIC_NOT_OK: Option<HashMap<String, ()>> = None;
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:11:17
    |
 LL | type NotOkMap = HashMap<String, ()>;
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:15:11
    |
 LL |     NotOk(HashMap<String, ()>),
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:20:13
    |
 LL |     not_ok: HashMap<String, ()>,
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:22:22
    |
 LL |     also_not_ok: Vec<HashMap<usize, ()>>,
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:30:30
    |
 LL |     fn weird_map(&self, map: HashMap<usize, ()>);
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:38:25
    |
 LL |     fn not_ok(&self) -> HashMap<String, ()> {
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:55:14
    |
 LL | fn test(map: HashMap<String, ()>, key: &str) -> HashMap<String, ()> {
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:55:49
    |
 LL | fn test(map: HashMap<String, ()>, key: &str) -> HashMap<String, ()> {
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:64:34
    |
 LL |     let _: HashMap<String, ()> = HashMap::new();
-   |                                  ^^^^^^^^^^^^
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:64:12
    |
 LL |     let _: HashMap<String, ()> = HashMap::new();
    |
    = help: consider using a set instead
 
 
 error: map with zero-sized value type
   --> $DIR/zero_sized_hashmap_values.rs:67:12
    |
 LL |     let _: HashMap<_, _> = std::iter::empty::<(String, ())>().collect();
    |
    = help: consider using a set instead
 
 error: aborting due to 13 previous errors
 error: aborting due to 13 previous errors
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/zero_sized_hashmap_values.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args zero_sized_hashmap_values.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/zero_sized_hashmap_values.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/zero_sized_hashmap_values.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f0962f37786a4888.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-cd40a3b060be150c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b396e06d7d6d4d75.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-db6d918eacac1474.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-4cd0b4140eeb9c8d.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/zero_sized_hashmap_values.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":154,"byte_end":173,"line_start":5,"line_end":5,"column_start":28,"column_end":47,"is_primary":true,"text":[{"text":"const CONST_NOT_OK: Option<HashMap<String, ()>> = None;","highlight_start":28,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::zero-sized-map-values` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:5:28\n   |\nLL | const CONST_NOT_OK: Option<HashMap<String, ()>> = None;\n   |                            ^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::zero-sized-map-values` implied by `-D warnings`\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":270,"byte_end":289,"line_start":8,"line_end":8,"column_start":30,"column_end":49,"is_primary":true,"text":[{"text":"static STATIC_NOT_OK: Option<HashMap<String, ()>> = None;","highlight_start":30,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:8:30\n   |\nLL | static STATIC_NOT_OK: Option<HashMap<String, ()>> = None;\n   |                              ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":353,"byte_end":372,"line_start":11,"line_end":11,"column_start":17,"column_end":36,"is_primary":true,"text":[{"text":"type NotOkMap = HashMap<String, ()>;","highlight_start":17,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:11:17\n   |\nLL | type NotOkMap = HashMap<String, ()>;\n   |                 ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":433,"byte_end":452,"line_start":15,"line_end":15,"column_start":11,"column_end":30,"is_primary":true,"text":[{"text":"    NotOk(HashMap<String, ()>),","highlight_start":11,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:15:11\n   |\nLL |     NotOk(HashMap<String, ()>),\n   |           ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":516,"byte_end":535,"line_start":20,"line_end":20,"column_start":13,"column_end":32,"is_primary":true,"text":[{"text":"    not_ok: HashMap<String, ()>,","highlight_start":13,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:20:13\n   |\nLL |     not_ok: HashMap<String, ()>,\n   |             ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":559,"byte_end":577,"line_start":22,"line_end":22,"column_start":22,"column_end":40,"is_primary":true,"text":[{"text":"    also_not_ok: Vec<HashMap<usize, ()>>,","highlight_start":22,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:22:22\n   |\nLL |     also_not_ok: Vec<HashMap<usize, ()>>,\n   |                      ^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":690,"byte_end":708,"line_start":30,"line_end":30,"column_start":30,"column_end":48,"is_primary":true,"text":[{"text":"    fn weird_map(&self, map: HashMap<usize, ()>);","highlight_start":30,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:30:30\n   |\nLL |     fn weird_map(&self, map: HashMap<usize, ()>);\n   |                              ^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":818,"byte_end":837,"line_start":38,"line_end":38,"column_start":25,"column_end":44,"is_primary":true,"text":[{"text":"    fn not_ok(&self) -> HashMap<String, ()> {","highlight_start":25,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:38:25\n   |\nLL |     fn not_ok(&self) -> HashMap<String, ()> {\n   |                         ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":1087,"byte_end":1106,"line_start":55,"line_end":55,"column_start":14,"column_end":33,"is_primary":true,"text":[{"text":"fn test(map: HashMap<String, ()>, key: &str) -> HashMap<String, ()> {","highlight_start":14,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:55:14\n   |\nLL | fn test(map: HashMap<String, ()>, key: &str) -> HashMap<String, ()> {\n   |              ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":1122,"byte_end":1141,"line_start":55,"line_end":55,"column_start":49,"column_end":68,"is_primary":true,"text":[{"text":"fn test(map: HashMap<String, ()>, key: &str) -> HashMap<String, ()> {","highlight_start":49,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:55:49\n   |\nLL | fn test(map: HashMap<String, ()>, key: &str) -> HashMap<String, ()> {\n   |                                                 ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":1298,"byte_end":1305,"line_start":64,"line_end":64,"column_start":34,"column_end":41,"is_primary":true,"text":[{"text":"    let _: HashMap<String, ()> = HashMap::new();","highlight_start":34,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:64:34\n   |\nLL |     let _: HashMap<String, ()> = HashMap::new();\n   |                                  ^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":1276,"byte_end":1295,"line_start":64,"line_end":64,"column_start":12,"column_end":31,"is_primary":true,"text":[{"text":"    let _: HashMap<String, ()> = HashMap::new();","highlight_start":12,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:64:12\n   |\nLL |     let _: HashMap<String, ()> = HashMap::new();\n   |            ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}
{"message":"map with zero-sized value type","code":{"code":"clippy::zero_sized_map_values","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_sized_hashmap_values.rs","byte_start":1378,"byte_end":1391,"line_start":67,"line_end":67,"column_start":12,"column_end":25,"is_primary":true,"text":[{"text":"    let _: HashMap<_, _> = std::iter::empty::<(String, ())>().collect();","highlight_start":12,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a set instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: map with zero-sized value type\n  --> tests/ui/zero_sized_hashmap_values.rs:67:12\n   |\nLL |     let _: HashMap<_, _> = std::iter::empty::<(String, ())>().collect();\n   |            ^^^^^^^^^^^^^\n   |\n   = help: consider using a set instead\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:18:21
    |
 LL |         fn new() -> Foo {
    |                     ^^^ help: use the applicable keyword: `Self`
    |
    = note: `-D clippy::use-self` implied by `-D warnings`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:19:13
    |
 LL |             Foo {}
 LL |             Foo {}
    |             ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:21:22
    |
 LL |         fn test() -> Foo {
 LL |         fn test() -> Foo {
    |                      ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:22:13
    |
 LL |             Foo::new()
 LL |             Foo::new()
    |             ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:27:25
    |
 LL |         fn default() -> Foo {
 LL |         fn default() -> Foo {
    |                         ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:28:13
    |
 LL |             Foo::new()
 LL |             Foo::new()
    |             ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:93:24
    |
    |
 LL |         fn bad(foos: &[Foo]) -> impl Iterator<Item = &Foo> {
    |                        ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:93:55
    |
    |
 LL |         fn bad(foos: &[Foo]) -> impl Iterator<Item = &Foo> {
    |                                                       ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:108:13
    |
 LL |             TS(0)
 LL |             TS(0)
    |             ^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:143:29
    |
 LL |                 fn bar() -> Bar {
 LL |                 fn bar() -> Bar {
    |                             ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:144:21
    |
    |
 LL |                     Bar { foo: Foo {} }
    |                     ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:155:21
    |
    |
 LL |         fn baz() -> Foo {
    |                     ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:156:13
    |
 LL |             Foo {}
 LL |             Foo {}
    |             ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:173:21
    |
    |
 LL |             let _ = Enum::B(42);
    |                     ^^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:174:21
    |
    |
 LL |             let _ = Enum::C { field: true };
    |                     ^^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:175:21
    |
    |
 LL |             let _ = Enum::A;
    |                     ^^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:217:13
    |
    |
 LL |             nested::A::fun_1();
    |             ^^^^^^^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:218:13
    |
    |
 LL |             nested::A::A;
    |             ^^^^^^^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:220:13
    |
    |
 LL |             nested::A {};
    |             ^^^^^^^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:239:13
    |
 LL |             TestStruct::from_something()
 LL |             TestStruct::from_something()
    |             ^^^^^^^^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:253:25
    |
    |
 LL |         async fn g() -> S {
    |                         ^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:254:13
    |
 LL |             S {}
 LL |             S {}
    |             ^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:258:16
    |
    |
 LL |             &p[S::A..S::B]
    |                ^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:258:22
    |
    |
 LL |             &p[S::A..S::B]
    |                      ^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:281:29
    |
    |
 LL |         fn foo(value: T) -> Foo<T> {
    |                             ^^^^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:282:13
    |
 LL |             Foo { value }
 LL |             Foo { value }
    |             ^^^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:319:21
    |
    |
 LL |         type From = T::From;
-   |                     ^^^^^^^ help: use the applicable keyword: `Self`
+   |                     ^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:320:19
    |
    |
 LL |         type To = T::To;
-   |                   ^^^^^ help: use the applicable keyword: `Self`
+   |                   ^ help: use the applicable keyword: `Self`
 error: unnecessary structure name repetition
   --> $DIR/use_self.rs:453:13
    |
    |
 LL |             A::new::<submod::B>(submod::B {})
    |             ^ help: use the applicable keyword: `Self`
 error: aborting due to 29 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/use_self.stage-id.stderr

 // run-rustfix
 // edition:2018
 // edition:2018
 // aux-build:proc_macro_derive.rs
 
 #![warn(clippy::use_self)]
 #![allow(dead_code)]
 #![allow(clippy::should_implement_trait, clippy::upper_case_acronyms, clippy::from_over_into)]
 #[macro_use]
 extern crate proc_macro_derive;
 
 fn main() {}
 fn main() {}
 
 mod use_self {
     struct Foo {}
 
     impl Foo {
         fn new() -> Self {
             Self {}
         fn test() -> Self {
             Self::new()
         }
     }
---
     }
 
     struct Foo {}
 
     impl Foo {
         use_self_expand!(); // Should not lint in local macros
 
 
     #[derive(StructAUseSelf)] // Should not lint in derives
     struct A;
 
 mod nesting {
     struct Foo {}
     impl Foo {
     impl Foo {
         fn foo() {
             #[allow(unused_imports)]
             use self::Foo; // Can't use Self here
             struct Bar {
                 foo: Foo, // Foo != Self
 
             impl Bar {
                 fn bar() -> Self {
                 fn bar() -> Self {
                     Self { foo: Foo {} }
             }
 
 
             // Can't use Self here
             fn baz() -> Foo {
                 Foo {}
         }
 
         // Should lint here
         // Should lint here
         fn baz() -> Self {
             Self {}
     }
 
 
     enum Enum {
         A,
         B(u64),
         C { field: bool },
     impl Enum {
         fn method() {
         fn method() {
             #[allow(unused_imports)]
             use self::Enum::*; // Issue 3425
             static STATIC: Enum = Enum::A; // Can't use Self as type
 
         fn method2() {
         fn method2() {
             let _ = Self::B(42);
             let _ = Self::C { field: true };
             let _ = Self::A;
     }
 }
 
 mod issue3410 {
 mod issue3410 {
 
     struct A;
     struct B;
 
     trait Trait<T> {
         fn a(v: T) -> Self;
 
 
     impl Trait<Vec<A>> for Vec<B> {
         fn a(_: Vec<A>) -> Self {
             unimplemented!()
     }
 
 
     impl<T> Trait<Vec<A>> for Vec<T>
     where
         T: Trait<B>,
     {
         fn a(v: Vec<A>) -> Self {
             <Vec<B>>::a(v).into_iter().map(Trait::a).collect()
     }
 }
 
 
 #[allow(clippy::no_effect, path_statements)]
 mod rustfix {
     mod nested {
         pub struct A {}
 
 
     impl nested::A {
         const A: bool = true;
 
         fn fun_1() {}
 
         fn fun_2() {
             Self::fun_1();
             Self::A;
             Self {};
         }
     }
 }
 }
 
 mod issue3567 {
     struct TestStruct {}
     impl TestStruct {
         fn from_something() -> Self {
             Self {}
     }
 
     trait Test {
         fn test() -> TestStruct;
---
     impl S {
         const A: usize = 0;
         const B: usize = 1;
 
         async fn g() -> Self {
             Self {}
 
 
         fn f<'a>(&self, p: &'a [u8]) -> &'a [u8] {
             &p[Self::A..Self::B]
     }
 
     trait T {
     trait T {
         fn f<'a>(&self, p: &'a [u8]) -> &'a [u8];
 
 
     impl T for Range<u8> {
         fn f<'a>(&self, p: &'a [u8]) -> &'a [u8] {
             &p[0..1]
     }
 }
 
 // reused from #1997
 // reused from #1997
 mod generics {
     struct Foo<T> {
         value: T,
 
 
     impl<T> Foo<T> {
         // `Self` is applicable here
         fn foo(value: T) -> Self {
             Self { value }
 
 
         // `Cannot` use `Self` as a return type as the generic types are different
         fn bar(value: i32) -> Foo<i32> {
             Foo { value }
     }
 }
 
 mod issue4140 {
 mod issue4140 {
     pub struct Error<From, To> {
         _from: From,
         _too: To,
 
 
     pub trait From<T> {
         type From;
         type To;
 
         fn from(value: T) -> Self;
 
 
     pub trait TryFrom<T>
     where
         Self: Sized,
         type From;
         type To;
 
 
         fn try_from(value: T) -> Result<Self, Error<Self::From, Self::To>>;
 
 
     impl<F, T> TryFrom<F> for T
     where
         T: From<F>,
-        type From = Self;
-        type To = Self;
-        type To = Self;
+        type From = Self::From;
+        type To = Self::To;
 
         fn try_from(value: F) -> Result<Self, Error<Self::From, Self::To>> {
             Ok(From::from(value))
     }
 
 
     impl From<bool> for i64 {
         type From = bool;
         type To = Self;
 
         fn from(value: bool) -> Self {
             if value { 100 } else { 0 }
     }
 }
 
 mod issue2843 {
 mod issue2843 {
     trait Foo {
         type Bar;
     }
 
     impl Foo for usize {
         type Bar = u8;
 
 
     impl<T: Foo> Foo for Option<T> {
         type Bar = Option<T::Bar>;
 }
 
 mod issue3859 {
     pub struct Foo;
     pub struct Foo;
     pub struct Bar([usize; 3]);
     impl Foo {
         pub const BAR: usize = 3;
 
         pub fn foo() {
         pub fn foo() {
             const _X: usize = Foo::BAR;
             // const _Y: usize = Self::BAR;
     }
 }
 
 mod issue4305 {
 mod issue4305 {
     trait Foo: 'static {}
     struct Bar;
 
 
     impl Foo for Bar {}
 
     impl<T: Foo> From<T> for Box<dyn Foo> {
         fn from(t: T) -> Self {
             Box::new(t)
     }
 }
 
 mod lint_at_item_level {
 mod lint_at_item_level {
     struct Foo {}
 
     #[allow(clippy::use_self)]
     impl Foo {
         fn new() -> Foo {
             Foo {}
     }
 
 
     #[allow(clippy::use_self)]
     impl Default for Foo {
         fn default() -> Foo {
             Foo::new()
     }
 }
 
 mod lint_at_impl_item_level {
 mod lint_at_impl_item_level {
     struct Foo {}
 
     impl Foo {
         #[allow(clippy::use_self)]
         fn new() -> Foo {
             Foo {}
     }
 
     impl Default for Foo {
     impl Default for Foo {
         #[allow(clippy::use_self)]
         fn default() -> Foo {
             Foo::new()
     }
 }
 
 mod issue4734 {
 mod issue4734 {
     #[repr(C, packed)]
     pub struct X {
         pub x: u32,
 
 
     impl From<X> for u32 {
         fn from(c: X) -> Self {
             unsafe { core::mem::transmute(c) }
     }
 }
 
 mod nested_paths {
 mod nested_paths {
     use std::convert::Into;
     mod submod {
         pub struct B {}
         pub struct C {}
 
         impl Into<C> for B {
             fn into(self) -> C {
                 C {}
         }
     }
 
 
     struct A<T> {
         t: T,
 
 
     impl<T> A<T> {
         fn new<V: Into<T>>(v: V) -> Self {
             Self { t: Into::into(v) }
     }
 
 
     impl A<submod::C> {
         fn test() -> Self {
             Self::new::<submod::B>(submod::B {})
     }
 }
 
 mod issue6818 {
 mod issue6818 {
     #[derive(serde::Deserialize)]
     struct A {
         a: i32,
     }
 }
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/use_self.stage-id.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args use_self.rs`
error: 2 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/use_self.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/use_self.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f0962f37786a4888.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-cd40a3b060be150c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b396e06d7d6d4d75.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-db6d918eacac1474.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-4cd0b4140eeb9c8d.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-0487049019a34248/out/test_build_base/use_self.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":339,"byte_end":342,"line_start":18,"line_end":18,"column_start":21,"column_end":24,"is_primary":true,"text":[{"text":"        fn new() -> Foo {","highlight_start":21,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::use-self` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":339,"byte_end":342,"line_start":18,"line_end":18,"column_start":21,"column_end":24,"is_primary":true,"text":[{"text":"        fn new() -> Foo {","highlight_start":21,"highlight_end":24}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:18:21\n   |\nLL |         fn new() -> Foo {\n   |                     ^^^ help: use the applicable keyword: `Self`\n   |\n   = note: `-D clippy::use-self` implied by `-D warnings`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":357,"byte_end":360,"line_start":19,"line_end":19,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo {}","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":357,"byte_end":360,"line_start":19,"line_end":19,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo {}","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:19:13\n   |\nLL |             Foo {}\n   |             ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":395,"byte_end":398,"line_start":21,"line_end":21,"column_start":22,"column_end":25,"is_primary":true,"text":[{"text":"        fn test() -> Foo {","highlight_start":22,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":395,"byte_end":398,"line_start":21,"line_end":21,"column_start":22,"column_end":25,"is_primary":true,"text":[{"text":"        fn test() -> Foo {","highlight_start":22,"highlight_end":25}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:21:22\n   |\nLL |         fn test() -> Foo {\n   |                      ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":413,"byte_end":416,"line_start":22,"line_end":22,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo::new()","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":413,"byte_end":416,"line_start":22,"line_end":22,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo::new()","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:22:13\n   |\nLL |             Foo::new()\n   |             ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":492,"byte_end":495,"line_start":27,"line_end":27,"column_start":25,"column_end":28,"is_primary":true,"text":[{"text":"        fn default() -> Foo {","highlight_start":25,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":492,"byte_end":495,"line_start":27,"line_end":27,"column_start":25,"column_end":28,"is_primary":true,"text":[{"text":"        fn default() -> Foo {","highlight_start":25,"highlight_end":28}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:27:25\n   |\nLL |         fn default() -> Foo {\n   |                         ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":510,"byte_end":513,"line_start":28,"line_end":28,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo::new()","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":510,"byte_end":513,"line_start":28,"line_end":28,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo::new()","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:28:13\n   |\nLL |             Foo::new()\n   |             ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":1754,"byte_end":1757,"line_start":93,"line_end":93,"column_start":24,"column_end":27,"is_primary":true,"text":[{"text":"        fn bad(foos: &[Foo]) -> impl Iterator<Item = &Foo> {","highlight_start":24,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":1754,"byte_end":1757,"line_start":93,"line_end":93,"column_start":24,"column_end":27,"is_primary":true,"text":[{"text":"        fn bad(foos: &[Foo]) -> impl Iterator<Item = &Foo> {","highlight_start":24,"highlight_end":27}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:93:24\n   |\nLL |         fn bad(foos: &[Foo]) -> impl Iterator<Item = &Foo> {\n   |                        ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":1785,"byte_end":1788,"line_start":93,"line_end":93,"column_start":55,"column_end":58,"is_primary":true,"text":[{"text":"        fn bad(foos: &[Foo]) -> impl Iterator<Item = &Foo> {","highlight_start":55,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":1785,"byte_end":1788,"line_start":93,"line_end":93,"column_start":55,"column_end":58,"is_primary":true,"text":[{"text":"        fn bad(foos: &[Foo]) -> impl Iterator<Item = &Foo> {","highlight_start":55,"highlight_end":58}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:93:55\n   |\nLL |         fn bad(foos: &[Foo]) -> impl Iterator<Item = &Foo> {\n   |                                                       ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2035,"byte_end":2037,"line_start":108,"line_end":108,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"            TS(0)","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2035,"byte_end":2037,"line_start":108,"line_end":108,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"            TS(0)","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:108:13\n   |\nLL |             TS(0)\n   |             ^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2675,"byte_end":2678,"line_start":143,"line_end":143,"column_start":29,"column_end":32,"is_primary":true,"text":[{"text":"                fn bar() -> Bar {","highlight_start":29,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2675,"byte_end":2678,"line_start":143,"line_end":143,"column_start":29,"column_end":32,"is_primary":true,"text":[{"text":"                fn bar() -> Bar {","highlight_start":29,"highlight_end":32}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:143:29\n   |\nLL |                 fn bar() -> Bar {\n   |                             ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2701,"byte_end":2704,"line_start":144,"line_end":144,"column_start":21,"column_end":24,"is_primary":true,"text":[{"text":"                    Bar { foo: Foo {} }","highlight_start":21,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2701,"byte_end":2704,"line_start":144,"line_end":144,"column_start":21,"column_end":24,"is_primary":true,"text":[{"text":"                    Bar { foo: Foo {} }","highlight_start":21,"highlight_end":24}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:144:21\n   |\nLL |                     Bar { foo: Foo {} }\n   |                     ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2915,"byte_end":2918,"line_start":155,"line_end":155,"column_start":21,"column_end":24,"is_primary":true,"text":[{"text":"        fn baz() -> Foo {","highlight_start":21,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2915,"byte_end":2918,"line_start":155,"line_end":155,"column_start":21,"column_end":24,"is_primary":true,"text":[{"text":"        fn baz() -> Foo {","highlight_start":21,"highlight_end":24}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:155:21\n   |\nLL |         fn baz() -> Foo {\n   |                     ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2933,"byte_end":2936,"line_start":156,"line_end":156,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo {}","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":2933,"byte_end":2936,"line_start":156,"line_end":156,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo {}","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:156:13\n   |\nLL |             Foo {}\n   |             ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":3276,"byte_end":3280,"line_start":173,"line_end":173,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"            let _ = Enum::B(42);","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":3276,"byte_end":3280,"line_start":173,"line_end":173,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"            let _ = Enum::B(42);","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:173:21\n   |\nLL |             let _ = Enum::B(42);\n   |                     ^^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":3309,"byte_end":3313,"line_start":174,"line_end":174,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"            let _ = Enum::C { field: true };","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":3309,"byte_end":3313,"line_start":174,"line_end":174,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"            let _ = Enum::C { field: true };","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:174:21\n   |\nLL |             let _ = Enum::C { field: true };\n   |                     ^^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":3354,"byte_end":3358,"line_start":175,"line_end":175,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"            let _ = Enum::A;","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":3354,"byte_end":3358,"line_start":175,"line_end":175,"column_start":21,"column_end":25,"is_primary":true,"text":[{"text":"            let _ = Enum::A;","highlight_start":21,"highlight_end":25}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:175:21\n   |\nLL |             let _ = Enum::A;\n   |                     ^^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4005,"byte_end":4014,"line_start":217,"line_end":217,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"            nested::A::fun_1();","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4005,"byte_end":4014,"line_start":217,"line_end":217,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"            nested::A::fun_1();","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:217:13\n   |\nLL |             nested::A::fun_1();\n   |             ^^^^^^^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4037,"byte_end":4046,"line_start":218,"line_end":218,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"            nested::A::A;","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4037,"byte_end":4046,"line_start":218,"line_end":218,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"            nested::A::A;","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:218:13\n   |\nLL |             nested::A::A;\n   |             ^^^^^^^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4064,"byte_end":4073,"line_start":220,"line_end":220,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"            nested::A {};","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4064,"byte_end":4073,"line_start":220,"line_end":220,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"            nested::A {};","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:220:13\n   |\nLL |             nested::A {};\n   |             ^^^^^^^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4369,"byte_end":4379,"line_start":239,"line_end":239,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"            TestStruct::from_something()","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4369,"byte_end":4379,"line_start":239,"line_end":239,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"            TestStruct::from_something()","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:239:13\n   |\nLL |             TestStruct::from_something()\n   |             ^^^^^^^^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4586,"byte_end":4587,"line_start":253,"line_end":253,"column_start":25,"column_end":26,"is_primary":true,"text":[{"text":"        async fn g() -> S {","highlight_start":25,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4586,"byte_end":4587,"line_start":253,"line_end":253,"column_start":25,"column_end":26,"is_primary":true,"text":[{"text":"        async fn g() -> S {","highlight_start":25,"highlight_end":26}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:253:25\n   |\nLL |         async fn g() -> S {\n   |                         ^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4602,"byte_end":4603,"line_start":254,"line_end":254,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"            S {}","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4602,"byte_end":4603,"line_start":254,"line_end":254,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"            S {}","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:254:13\n   |\nLL |             S {}\n   |             ^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4684,"byte_end":4685,"line_start":258,"line_end":258,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"            &p[S::A..S::B]","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4684,"byte_end":4685,"line_start":258,"line_end":258,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"            &p[S::A..S::B]","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:258:16\n   |\nLL |             &p[S::A..S::B]\n   |                ^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4690,"byte_end":4691,"line_start":258,"line_end":258,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"            &p[S::A..S::B]","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":4690,"byte_end":4691,"line_start":258,"line_end":258,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"            &p[S::A..S::B]","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:258:22\n   |\nLL |             &p[S::A..S::B]\n   |                      ^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":5069,"byte_end":5075,"line_start":281,"line_end":281,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"        fn foo(value: T) -> Foo<T> {","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":5069,"byte_end":5075,"line_start":281,"line_end":281,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"        fn foo(value: T) -> Foo<T> {","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:281:29\n   |\nLL |         fn foo(value: T) -> Foo<T> {\n   |                             ^^^^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":5090,"byte_end":5093,"line_start":282,"line_end":282,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo { value }","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":5090,"byte_end":5093,"line_start":282,"line_end":282,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"            Foo { value }","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:282:13\n   |\nLL |             Foo { value }\n   |             ^^^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":5752,"byte_end":5753,"line_start":319,"line_end":319,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"        type From = T::From;","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":5752,"byte_end":5753,"line_start":319,"line_end":319,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"        type From = T::From;","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:319:21\n   |\nLL |         type From = T::From;\n   |                     ^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":5779,"byte_end":5780,"line_start":320,"line_end":320,"column_start":19,"column_end":20,"is_primary":true,"text":[{"text":"        type To = T::To;","highlight_start":19,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":5779,"byte_end":5780,"line_start":320,"line_end":320,"column_start":19,"column_end":20,"is_primary":true,"text":[{"text":"        type To = T::To;","highlight_start":19,"highlight_end":20}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:320:19\n   |\nLL |         type To = T::To;\n   |                   ^ help: use the applicable keyword: `Self`\n\n"}
{"message":"unnecessary structure name repetition","code":{"code":"clippy::use_self","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":7970,"byte_end":7971,"line_start":453,"line_end":453,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"            A::new::<submod::B>(submod::B {})","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the applicable keyword","code":null,"level":"help","spans":[{"file_name":"tests/ui/use_self.rs","byte_start":7970,"byte_end":7971,"line_start":453,"line_end":453,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"            A::new::<submod::B>(submod::B {})","highlight_start":13,"highlight_end":14}],"label":null,"suggested_replacement":"Self","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary structure name repetition\n  --> tests/ui/use_self.rs:453:13\n   |\nLL |             A::new::<submod::B>(submod::B {})\n   |             ^ help: use the applicable keyword: `Self`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
