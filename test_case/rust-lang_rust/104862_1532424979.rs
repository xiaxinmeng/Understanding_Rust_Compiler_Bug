plain

 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:12:5
    |
 LL | /     pub fn new() -> Self {
 LL | |         Self { guess: 42 }
 LL | |     }
    |
    |
    = note: `-D clippy::missing-const-for-fn` implied by `-D warnings`
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:16:5
    |
    |
 LL | /     fn const_generic_params<'a, T, const N: usize>(&self, b: &'a [T; N]) -> &'a [T; N] {
 LL | |         b
 LL | |     }
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:22:1
    |
---
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:27:1
    |
 LL | / fn two() -> i32 {
 LL | |     let abc = 2;
 LL | |     abc
 LL | | }
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:33:1
    |
---
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:38:1
    |
 LL | / unsafe fn four() -> i32 {
 LL | |     4
 LL | | }
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:43:1
    |
---
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:51:1
    |
 LL | / fn generic_arr<T: Copy>(t: [T; 1]) -> T {
 LL | |     t[0]
 LL | | }
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:64:9
    |
    |
 LL | /         pub fn b(self, a: &A) -> B {
 LL | |             B
 LL | |         }
 
 error: this could be a `const fn`
-  --> $DIR/could_be_const.rs:73:5
-   |
-   |
-LL | /     fn const_fn_stabilized_before_msrv(byte: u8) {
-LL | |         byte.is_ascii_digit();
-LL | |     }
-
-error: this could be a `const fn`
   --> $DIR/could_be_const.rs:84:1
    |
    |
 LL | / fn msrv_1_46() -> i32 {
 LL | |     46
 LL | | }
 
-error: aborting due to 11 previous errors
+error: aborting due to 10 previous errors
 
---
To only update this specific test, also pass `--test-args missing_const_for_fn/could_be_const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/missing_const_for_fn/could_be_const.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_const_for_fn/could_be_const.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_const_for_fn/could_be_const.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":192,"byte_end":247,"line_start":12,"line_end":14,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        Self { guess: 42 }","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::missing-const-for-fn` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:12:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         Self { guess: 42 }\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::missing-const-for-fn` implied by `-D warnings`\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":253,"byte_end":353,"line_start":16,"line_end":18,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn const_generic_params<'a, T, const N: usize>(&self, b: &'a [T; N]) -> &'a [T; N] {","highlight_start":5,"highlight_end":89},{"text":"        b","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:16:5\n   |\nLL | /     fn const_generic_params<'a, T, const N: usize>(&self, b: &'a [T; N]) -> &'a [T; N] {\nLL | |         b\nLL | |     }\n   | |_____^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":375,"byte_end":400,"line_start":22,"line_end":24,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn one() -> i32 {","highlight_start":1,"highlight_end":18},{"text":"    1","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:22:1\n   |\nLL | / fn one() -> i32 {\nLL | |     1\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":425,"byte_end":469,"line_start":27,"line_end":30,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn two() -> i32 {","highlight_start":1,"highlight_end":18},{"text":"    let abc = 2;","highlight_start":1,"highlight_end":17},{"text":"    abc","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:27:1\n   |\nLL | / fn two() -> i32 {\nLL | |     let abc = 2;\nLL | |     abc\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":507,"byte_end":550,"line_start":33,"line_end":35,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn string() -> String {","highlight_start":1,"highlight_end":24},{"text":"    String::new()","highlight_start":1,"highlight_end":18},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:33:1\n   |\nLL | / fn string() -> String {\nLL | |     String::new()\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":570,"byte_end":603,"line_start":38,"line_end":40,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"unsafe fn four() -> i32 {","highlight_start":1,"highlight_end":26},{"text":"    4","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:38:1\n   |\nLL | / unsafe fn four() -> i32 {\nLL | |     4\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":628,"byte_end":662,"line_start":43,"line_end":45,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn generic<T>(t: T) -> T {","highlight_start":1,"highlight_end":27},{"text":"    t","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:43:1\n   |\nLL | / fn generic<T>(t: T) -> T {\nLL | |     t\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":722,"byte_end":774,"line_start":51,"line_end":53,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn generic_arr<T: Copy>(t: [T; 1]) -> T {","highlight_start":1,"highlight_end":42},{"text":"    t[0]","highlight_start":1,"highlight_end":9},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:51:1\n   |\nLL | / fn generic_arr<T: Copy>(t: [T; 1]) -> T {\nLL | |     t[0]\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":973,"byte_end":1025,"line_start":64,"line_end":66,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        pub fn b(self, a: &A) -> B {","highlight_start":9,"highlight_end":37},{"text":"            B","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:64:9\n   |\nLL | /         pub fn b(self, a: &A) -> B {\nLL | |             B\nLL | |         }\n   | |_________^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":1367,"byte_end":1399,"line_start":84,"line_end":86,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn msrv_1_46() -> i32 {","highlight_start":1,"highlight_end":24},{"text":"    46","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:84:1\n   |\nLL | / fn msrv_1_46() -> i32 {\nLL | |     46\nLL | | }\n   | |_^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: redundant clone
-  --> $DIR/redundant_clone.rs:10:42
+error: this lint expectation is unfulfilled
+  --> $DIR/redundant_clone.rs:34:14
    |
-LL |     let _s = ["lorem", "ipsum"].join(" ").to_string();
-   |                                          ^^^^^^^^^^^^ help: remove this
+LL |     #[expect(clippy::redundant_clone)]
    |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:10:14
-   |
-   |
-LL |     let _s = ["lorem", "ipsum"].join(" ").to_string();
-   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   = note: `-D clippy::redundant-clone` implied by `-D warnings`
+   = note: `-D unfulfilled-lint-expectations` implied by `-D warnings`
-error: redundant clone
-  --> $DIR/redundant_clone.rs:13:15
-   |
-   |
-LL |     let _s = s.clone();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:13:14
-   |
-   |
-LL |     let _s = s.clone();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:16:15
-   |
-   |
-LL |     let _s = s.to_string();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:16:14
-   |
-   |
-LL |     let _s = s.to_string();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:19:15
-   |
-   |
-LL |     let _s = s.to_owned();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:19:14
-   |
-   |
-LL |     let _s = s.to_owned();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:21:42
-   |
-   |
-LL |     let _s = Path::new("/a/b/").join("c").to_owned();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:21:14
-   |
-   |
-LL |     let _s = Path::new("/a/b/").join("c").to_owned();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:23:42
-   |
-   |
-LL |     let _s = Path::new("/a/b/").join("c").to_path_buf();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:23:14
-   |
-   |
-LL |     let _s = Path::new("/a/b/").join("c").to_path_buf();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:25:29
-   |
-   |
-LL |     let _s = OsString::new().to_owned();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:25:14
-   |
-   |
-LL |     let _s = OsString::new().to_owned();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:27:29
-   |
-   |
-LL |     let _s = OsString::new().to_os_string();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:27:14
-   |
-   |
-LL |     let _s = OsString::new().to_os_string();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:38:19
-   |
-   |
-LL |     let _t = tup.0.clone();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:38:14
-   |
-   |
-LL |     let _t = tup.0.clone();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:70:25
-   |
-   |
-LL |     if b { (a.clone(), a.clone()) } else { (Alpha, a) }
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:70:24
-   |
-   |
-LL |     if b { (a.clone(), a.clone()) } else { (Alpha, a) }
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:127:15
-   |
-   |
-LL |     let _s = s.clone();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:127:14
-   |
-   |
-LL |     let _s = s.clone();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:128:15
-   |
-   |
-LL |     let _t = t.clone();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:128:14
-   |
-   |
-LL |     let _t = t.clone();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:138:19
-   |
-   |
-LL |         let _f = f.clone();
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:138:18
-   |
-   |
-LL |         let _f = f.clone();
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:150:14
-   |
-   |
-LL |     let y = x.clone().join("matthias");
-   |
-note: cloned value is neither consumed nor mutated
-  --> $DIR/redundant_clone.rs:150:13
-   |
-   |
-LL |     let y = x.clone().join("matthias");
-
-error: redundant clone
-  --> $DIR/redundant_clone.rs:204:11
-   |
-   |
-LL |     foo(&x.clone(), move || {
error: test failed, to rerun pass `--test compile-test`
Build completed unsuccessfully in 0:02:17
-   |
-note: this value is dropped without further use
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:204:10
-   |
-LL |     foo(&x.clone(), move || {
-
-error: aborting due to 15 previous errors
+error: aborting due to previous error
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_clone.stage-id.stderr
diff of fixed:

 //@run-rustfix
 // rustfix-only-machine-applicable
 #![feature(lint_reasons)]
 #![allow(clippy::drop_non_drop, clippy::implicit_clone, clippy::uninlined_format_args)]
 use std::ffi::OsString;
 use std::path::Path;
 
 fn main() {
 fn main() {
-    let _s = ["lorem", "ipsum"].join(" ");
+    let _s = ["lorem", "ipsum"].join(" ").to_string();
     let s = String::from("foo");
-    let _s = s;
-    let _s = s;
+    let _s = s.clone();
     let s = String::from("foo");
-    let _s = s;
-    let _s = s;
+    let _s = s.to_string();
     let s = String::from("foo");
-    let _s = s;
-    let _s = s;
+    let _s = s.to_owned();
 
-    let _s = Path::new("/a/b/").join("c");
+    let _s = Path::new("/a/b/").join("c").to_owned();
 
-    let _s = Path::new("/a/b/").join("c");
+    let _s = Path::new("/a/b/").join("c").to_path_buf();
-    let _s = OsString::new();
-    let _s = OsString::new();
+    let _s = OsString::new().to_owned();
-    let _s = OsString::new();
-    let _s = OsString::new();
+    let _s = OsString::new().to_os_string();
     // Check that lint level works
     // Check that lint level works
     #[allow(clippy::redundant_clone)]
     let _s = String::new().to_string();
     // Check that lint level works
     // Check that lint level works
     #[expect(clippy::redundant_clone)]
     let _s = String::new().to_string();
 
     let tup = (String::from("foo"),);
-    let _t = tup.0;
+    let _t = tup.0.clone();
 
     let tup_ref = &(String::from("foo"),);
     let _s = tup_ref.0.clone(); // this `.clone()` cannot be removed
     {
         let x = String::new();
         let x = String::new();
         let y = &x;
 
         let _x = x.clone(); // ok; `x` is borrowed by `y`
 
         let _ = y.len();
 
     let x = (String::new(),);
     let x = (String::new(),);
     let _ = Some(String::new()).unwrap_or_else(|| x.0.clone()); // ok; closure borrows `x`
 
     with_branch(Alpha, true);
     cannot_double_move(Alpha);
     cannot_move_from_type_with_drop();
     borrower_propagation();
     not_consumed();
     manually_drop();
     clone_then_move_cloned();
     hashmap_neg();
     false_negative_5707();
     false_negative_5707();
 }
 
 #[derive(Clone)]
 struct Alpha;
 fn with_branch(a: Alpha, b: bool) -> (Alpha, Alpha) {
-    if b { (a.clone(), a) } else { (Alpha, a) }
+    if b { (a.clone(), a.clone()) } else { (Alpha, a) }
 
 
 fn cannot_double_move(a: Alpha) -> (Alpha, Alpha) {
     (a.clone(), a)
 
 
 struct TypeWithDrop {
     x: String,
 
 
 impl Drop for TypeWithDrop {
     fn drop(&mut self) {}
 
 fn cannot_move_from_type_with_drop() -> String {
 fn cannot_move_from_type_with_drop() -> String {
     let s = TypeWithDrop { x: String::new() };
     s.x.clone() // removing this `clone()` summons E0509
 
 fn borrower_propagation() {
     let s = String::new();
     let t = String::new();
     let t = String::new();
 
     {
         fn b() -> bool {
             unimplemented!()
         }
         let _u = if b() { &s } else { &t };
 
         // ok; `s` and `t` are possibly borrowed
         let _s = s.clone();
         let _t = t.clone();
 
     {
     {
         let _u = || s.len();
         let _v = [&t; 32];
         let _s = s.clone(); // ok
         let _t = t.clone(); // ok
 
     {
         let _u = {
         let _u = {
             let u = Some(&s);
             let _ = s.clone(); // ok
         };
         };
         let _s = s.clone(); // ok
 
     {
         use std::convert::identity as id;
         use std::convert::identity as id;
         let _u = id(id(&s));
         let _s = s.clone(); // ok, `u` borrows `s`
 
-    let _s = s;
-    let _t = t;
-    let _t = t;
+    let _s = s.clone();
+    let _t = t.clone();
     #[derive(Clone)]
     struct Foo {
         x: usize,
     }
     }
 
     {
         let f = Foo { x: 123 };
         let _x = Some(f.x);
-        let _f = f;
+        let _f = f.clone();
 
     {
         let f = Foo { x: 123 };
         let f = Foo { x: 123 };
         let _x = &f.x;
         let _f = f.clone(); // ok
 }
 
 fn not_consumed() {
 fn not_consumed() {
     let x = std::path::PathBuf::from("home");
-    let y = x.join("matthias");
+    let y = x.clone().join("matthias");
     // join() creates a new owned PathBuf, does not take a &mut to x variable, thus the .clone() is
     // redundant. (It also does not consume the PathBuf)
 
     println!("x: {:?}, y: {:?}", x, y);
     let mut s = String::new();
     let mut s = String::new();
     s.clone().push_str("foo"); // OK, removing this `clone()` will change the behavior.
     s.push_str("bar");
     assert_eq!(s, "bar");
     let t = Some(s);
     // OK
     // OK
     if let Some(x) = t.clone() {
     }
     }
     if let Some(x) = t {
     }
 }
 
 #[allow(clippy::clone_on_copy)]
 #[allow(clippy::clone_on_copy)]
 fn issue_5405() {
     let a: [String; 1] = [String::from("foo")];
     let _b: String = a[0].clone();
 
     let c: [usize; 2] = [2, 3];
     let _d: usize = c[1].clone();
 
 fn manually_drop() {
     use std::mem::ManuallyDrop;
     use std::sync::Arc;
     use std::sync::Arc;
 
     let a = ManuallyDrop::new(Arc::new("Hello!".to_owned()));
     let _ = a.clone(); // OK
 
     let p: *const String = Arc::into_raw(ManuallyDrop::into_inner(a));
         Arc::from_raw(p);
         Arc::from_raw(p);
     }
 }
 }
 
 fn clone_then_move_cloned() {
     // issue #5973
     let x = Some(String::new());
     // ok, x is moved while the clone is in use.
     assert_eq!(x.clone(), None, "not equal {}", x.unwrap());
     // issue #5595
     // issue #5595
     fn foo<F: Fn()>(_: &Alpha, _: F) {}
     let x = Alpha;
     // ok, data is moved while the clone is in use.
-    foo(&x, move || {
+    foo(&x.clone(), move || {
         let _ = x;
 
     // issue #6998
     struct S(String);
     impl S {
     impl S {
         fn m(&mut self) {}
     }
     let mut x = S(String::new());
     x.0.clone().chars().for_each(|_| x.m());
 
 
 fn hashmap_neg() {
     // issue 5707
     use std::path::PathBuf;
 
 
     let p = PathBuf::from("/");
 
     let mut h: HashMap<&str, &str> = HashMap::new();
     h.insert("orig-p", p.to_str().unwrap());
 
     let mut q = p.clone();
     q.push("foo");
 
     println!("{:?} {}", h, q.display());
 
 fn false_negative_5707() {
 fn false_negative_5707() {
     fn foo(_x: &Alpha, _y: &mut Alpha) {}
     let x = Alpha;
     let mut y = Alpha;
     let mut y = Alpha;
     foo(&x, &mut y);
     let _z = x.clone(); // pr 7346 can't lint on `x`
     drop(y);
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_clone.stage-id.fixed
To only update this specific test, also pass `--test-args redundant_clone.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/redundant_clone.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_clone.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_clone.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this lint expectation is unfulfilled","code":{"code":"unfulfilled_lint_expectations","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":823,"byte_end":846,"line_start":34,"line_end":34,"column_start":14,"column_end":37,"is_primary":true,"text":[{"text":"    #[expect(clippy::redundant_clone)]","highlight_start":14,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unfulfilled-lint-expectations` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this lint expectation is unfulfilled\n  --> tests/ui/redundant_clone.rs:34:14\n   |\nLL |     #[expect(clippy::redundant_clone)]\n   |              ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D unfulfilled-lint-expectations` implied by `-D warnings`\n\n"}

------------------------------------------

---
 
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:91:30
    |
 LL |     require_impl_deref_c_str(c_str.to_owned());
    |                              ^^^^^^^^^^^^^^^^ help: use: `c_str`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:92:31
    |
    |
 LL |     require_impl_deref_os_str(os_str.to_owned());
    |                               ^^^^^^^^^^^^^^^^^ help: use: `os_str`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:93:29
    |
 LL |     require_impl_deref_path(path.to_owned());
 LL |     require_impl_deref_path(path.to_owned());
    |                             ^^^^^^^^^^^^^^^ help: use: `path`
 
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:94:28
    |
 LL |     require_impl_deref_str(s.to_owned());
    |                            ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:95:30
    |
 LL |     require_impl_deref_slice(slice.to_owned());
 LL |     require_impl_deref_slice(slice.to_owned());
    |                              ^^^^^^^^^^^^^^^^ help: use: `slice`
 
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:97:29
    |
 LL |     require_deref_str_slice(s.to_owned(), slice.to_owned());
    |                             ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:97:43
    |
    |
 LL |     require_deref_str_slice(s.to_owned(), slice.to_owned());
    |                                           ^^^^^^^^^^^^^^^^ help: use: `slice`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:98:29
    |
    |
 LL |     require_deref_slice_str(slice.to_owned(), s.to_owned());
    |                             ^^^^^^^^^^^^^^^^ help: use: `slice`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:98:47
    |
    |
 LL |     require_deref_slice_str(slice.to_owned(), s.to_owned());
    |                                               ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:100:26
    |
    |
 LL |     require_as_ref_c_str(c_str.to_owned());
    |                          ^^^^^^^^^^^^^^^^ help: use: `c_str`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:101:27
    |
 LL |     require_as_ref_os_str(os_str.to_owned());
---
 
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:109:31
    |
 LL |     require_impl_as_ref_c_str(c_str.to_owned());
    |                               ^^^^^^^^^^^^^^^^ help: use: `c_str`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:110:32
    |
    |
 LL |     require_impl_as_ref_os_str(os_str.to_owned());
    |                                ^^^^^^^^^^^^^^^^^ help: use: `os_str`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:111:30
    |
    |
 LL |     require_impl_as_ref_path(path.to_owned());
    |                              ^^^^^^^^^^^^^^^ help: use: `path`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:112:29
    |
    |
 LL |     require_impl_as_ref_str(s.to_owned());
    |                             ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:113:29
    |
    |
 LL |     require_impl_as_ref_str(x.to_owned());
    |                             ^^^^^^^^^^^^ help: use: `&x`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:114:31
    |
 LL |     require_impl_as_ref_slice(array.to_owned());
---
 
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:116:31
    |
 LL |     require_impl_as_ref_slice(slice.to_owned());
    |                               ^^^^^^^^^^^^^^^^ help: use: `slice`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:118:30
    |
    |
 LL |     require_as_ref_str_slice(s.to_owned(), array.to_owned());
    |                              ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:118:44
    |
    |
 LL |     require_as_ref_str_slice(s.to_owned(), array.to_owned());
    |                                            ^^^^^^^^^^^^^^^^ help: use: `array`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:119:30
    |
    |
 LL |     require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());
    |                              ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:119:44
    |
    |
 LL |     require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());
    |                                            ^^^^^^^^^^^^^^^^^^^^ help: use: `array_ref`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:120:30
    |
    |
 LL |     require_as_ref_str_slice(s.to_owned(), slice.to_owned());
    |                              ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:120:44
    |
    |
 LL |     require_as_ref_str_slice(s.to_owned(), slice.to_owned());
    |                                            ^^^^^^^^^^^^^^^^ help: use: `slice`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:121:30
    |
    |
 LL |     require_as_ref_slice_str(array.to_owned(), s.to_owned());
    |                              ^^^^^^^^^^^^^^^^ help: use: `array`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:121:48
    |
    |
 LL |     require_as_ref_slice_str(array.to_owned(), s.to_owned());
    |                                                ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:122:30
    |
    |
 LL |     require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());
    |                              ^^^^^^^^^^^^^^^^^^^^ help: use: `array_ref`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:122:52
    |
    |
 LL |     require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());
    |                                                    ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:123:30
    |
    |
 LL |     require_as_ref_slice_str(slice.to_owned(), s.to_owned());
    |                              ^^^^^^^^^^^^^^^^ help: use: `slice`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:123:48
    |
    |
 LL |     require_as_ref_slice_str(slice.to_owned(), s.to_owned());
    |                                                ^^^^^^^^^^^^ help: use: `s`
 error: unnecessary use of `to_string`
   --> $DIR/unnecessary_to_owned.rs:125:20
    |
    |
 LL |     let _ = x.join(&x_ref.to_string());
    |                    ^^^^^^^^^^^^^^^^^^ help: use: `x_ref`
 error: unnecessary use of `to_vec`
   --> $DIR/unnecessary_to_owned.rs:127:13
    |
    |
 LL |     let _ = slice.to_vec().into_iter();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:128:13
    |
    |
 LL |     let _ = slice.to_owned().into_iter();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`
 error: unnecessary use of `to_vec`
   --> $DIR/unnecessary_to_owned.rs:129:13
    |
    |
 LL |     let _ = [std::path::PathBuf::new()][..].to_vec().into_iter();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:130:13
    |
    |
 LL |     let _ = [std::path::PathBuf::new()][..].to_owned().into_iter();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`
 error: unnecessary use of `to_vec`
   --> $DIR/unnecessary_to_owned.rs:132:13
    |
    |
 LL |     let _ = IntoIterator::into_iter(slice.to_vec());
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:133:13
    |
 LL |     let _ = IntoIterator::into_iter(slice.to_owned());
 LL |     let _ = IntoIterator::into_iter(slice.to_owned());
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`
 error: unnecessary use of `to_vec`
   --> $DIR/unnecessary_to_owned.rs:134:13
    |
    |
 LL |     let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_vec());
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`
 error: unnecessary use of `to_owned`
   --> $DIR/unnecessary_to_owned.rs:135:13
    |
    |
 LL |     let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_owned());
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`
 error: unnecessary use of `to_vec`
   --> $DIR/unnecessary_to_owned.rs:197:14
    |
    |
 LL |     for t in file_types.to_vec() {
    |
 help: use
    |
 LL |     for t in file_types {
 LL |     for t in file_types {
    |              ~~~~~~~~~~
 help: remove this `&`
    |
 LL -         let path = match get_file_path(&t) {
 LL +         let path = match get_file_path(t) {
 
 error: unnecessary use of `to_vec`
   --> $DIR/unnecessary_to_owned.rs:220:14
    |
    |
 LL |     let _ = &["x"][..].to_vec().into_iter();
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `["x"][..].iter().cloned()`
 error: unnecessary use of `to_vec`
   --> $DIR/unnecessary_to_owned.rs:225:14
    |
    |
 LL |     let _ = &["x"][..].to_vec().into_iter();
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `["x"][..].iter().copied()`
 error: unnecessary use of `to_string`
   --> $DIR/unnecessary_to_owned.rs:272:24
    |
    |
 LL |         Box::new(build(y.to_string()))
    |                        ^^^^^^^^^^^^^ help: use: `y`
 error: unnecessary use of `to_string`
   --> $DIR/unnecessary_to_owned.rs:380:12
    |
    |
 LL |         id("abc".to_string())
    |            ^^^^^^^^^^^^^^^^^ help: use: `"abc"`
-error: aborting due to 79 previous errors
+error: aborting due to 74 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_to_owned.stage-id.stderr
diff of fixed:

 //@run-rustfix
 
 #![allow(clippy::needless_borrow, clippy::ptr_arg)]
 #![warn(clippy::unnecessary_to_owned)]
 use std::borrow::Cow;
 use std::borrow::Cow;
 use std::ffi::{CStr, CString, OsStr, OsString};
 
 #[derive(Clone)]
 struct X(String);
 
 
 impl Deref for X {
     type Target = [u8];
     fn deref(&self) -> &[u8] {
         self.0.as_bytes()
 }
 
 
 impl AsRef<str> for X {
     fn as_ref(&self) -> &str {
         self.0.as_str()
 }
 
 impl ToString for X {
     fn to_string(&self) -> String {
     fn to_string(&self) -> String {
         self.0.to_string()
     }
 }
 
 impl X {
     fn join(&self, other: impl AsRef<str>) -> Self {
         let mut s = self.0.clone();
         s.push_str(other.as_ref());
         Self(s)
 }
 
 #[allow(dead_code)]
 #[derive(Clone)]
 #[derive(Clone)]
 enum FileType {
     Account,
     PrivateKey,
     Certificate,
 }
 
 fn main() {
     let c_str = CStr::from_bytes_with_nul(&[0]).unwrap();
     let os_str = OsStr::new("x");
     let path = std::path::Path::new("x");
     let s = "x";
     let array = ["x"];
     let array_ref = &["x"];
     let slice = &["x"][..];
     let x = X(String::from("x"));
     let x_ref = &x;
 
     require_c_str(&Cow::from(c_str));
     require_c_str(c_str);
     require_os_str(os_str);
     require_os_str(os_str);
     require_os_str(&Cow::from(os_str));
     require_os_str(os_str);
 
     require_path(path);
     require_path(&Cow::from(path));
     require_path(path);
     require_str(s);
     require_str(s);
     require_str(&Cow::from(s));
     require_str(s);
     require_str(x_ref.as_ref());
     require_slice(slice);
     require_slice(slice);
     require_slice(&Cow::from(slice));
     require_slice(array.as_ref());
     require_slice(array_ref.as_ref());
     require_slice(slice);
     require_slice(&x_ref.to_owned()); // No longer flagged because of #8759.
 
     require_x(&Cow::<X>::Owned(x.clone()));
     require_x(&x_ref.to_owned()); // No longer flagged because of #8759.
 
     require_deref_c_str(c_str);
     require_deref_os_str(os_str);
     require_deref_path(path);
     require_deref_str(s);
     require_deref_slice(slice);
 
     require_impl_deref_c_str(c_str);
     require_impl_deref_os_str(os_str);
     require_impl_deref_path(path);
     require_impl_deref_str(s);
     require_impl_deref_slice(slice);
     require_deref_str_slice(s, slice);
     require_deref_slice_str(slice, s);
 
 
     require_as_ref_c_str(c_str);
     require_as_ref_os_str(os_str);
     require_as_ref_path(path);
     require_as_ref_str(s);
     require_as_ref_str(&x);
     require_as_ref_slice(array_ref);
     require_as_ref_slice(slice);
 
 
     require_impl_as_ref_c_str(c_str);
     require_impl_as_ref_os_str(os_str);
     require_impl_as_ref_path(path);
     require_impl_as_ref_str(s);
     require_impl_as_ref_str(&x);
     require_impl_as_ref_slice(array);
     require_impl_as_ref_slice(array_ref);
     require_impl_as_ref_slice(slice);
     require_as_ref_str_slice(s, array);
     require_as_ref_str_slice(s, array_ref);
     require_as_ref_str_slice(s, slice);
     require_as_ref_slice_str(array, s);
     require_as_ref_slice_str(array, s);
     require_as_ref_slice_str(array_ref, s);
     require_as_ref_slice_str(slice, s);
 
     let _ = x.join(x_ref);
 
     let _ = slice.iter().copied();
     let _ = slice.iter().copied();
     let _ = [std::path::PathBuf::new()][..].iter().cloned();
     let _ = [std::path::PathBuf::new()][..].iter().cloned();
 
     let _ = slice.iter().copied();
     let _ = slice.iter().copied();
     let _ = [std::path::PathBuf::new()][..].iter().cloned();
     let _ = [std::path::PathBuf::new()][..].iter().cloned();
 
     let _ = check_files(&[FileType::Account]);
     // negative tests
     require_string(&s.to_string());
     require_string(&s.to_string());
     require_string(&Cow::from(s).into_owned());
     require_string(&s.to_owned());
     require_string(&x_ref.to_string());
 
     // `X` isn't copy.
     require_slice(&x.to_owned());
     require_deref_slice(x.to_owned());
 
     // The following should be flagged by `redundant_clone`, but not by this lint.
-    require_c_str(&CString::from_vec_with_nul(vec![0]).unwrap());
-    require_os_str(&OsString::from("x"));
-    require_path(&std::path::PathBuf::from("x"));
-    require_str(&String::from("x"));
-    require_slice(&[String::from("x")]);
+    require_c_str(&CString::from_vec_with_nul(vec![0]).unwrap().to_owned());
+    require_os_str(&OsString::from("x").to_os_string());
+    require_path(&std::path::PathBuf::from("x").to_path_buf());
+    require_str(&String::from("x").to_string());
+    require_slice(&[String::from("x")].to_owned());
 
 
 fn require_c_str(_: &CStr) {}
 fn require_os_str(_: &OsStr) {}
 fn require_path(_: &std::path::Path) {}
 fn require_str(_: &str) {}
 fn require_slice<T>(_: &[T]) {}
 fn require_x(_: &X) {}
 
 fn require_deref_c_str<T: Deref<Target = CStr>>(_: T) {}
 fn require_deref_os_str<T: Deref<Target = OsStr>>(_: T) {}
 fn require_deref_path<T: Deref<Target = std::path::Path>>(_: T) {}
 fn require_deref_str<T: Deref<Target = str>>(_: T) {}
 fn require_deref_slice<T, U: Deref<Target = [T]>>(_: U) {}
 
 fn require_impl_deref_c_str(_: impl Deref<Target = CStr>) {}
 fn require_impl_deref_os_str(_: impl Deref<Target = OsStr>) {}
 fn require_impl_deref_path(_: impl Deref<Target = std::path::Path>) {}
 fn require_impl_deref_str(_: impl Deref<Target = str>) {}
 fn require_impl_deref_slice<T>(_: impl Deref<Target = [T]>) {}
 
 fn require_deref_str_slice<T: Deref<Target = str>, U, V: Deref<Target = [U]>>(_: T, _: V) {}
 fn require_deref_slice_str<T, U: Deref<Target = [T]>, V: Deref<Target = str>>(_: U, _: V) {}
 
 fn require_as_ref_c_str<T: AsRef<CStr>>(_: T) {}
 fn require_as_ref_os_str<T: AsRef<OsStr>>(_: T) {}
 fn require_as_ref_path<T: AsRef<std::path::Path>>(_: T) {}
 fn require_as_ref_str<T: AsRef<str>>(_: T) {}
 fn require_as_ref_slice<T, U: AsRef<[T]>>(_: U) {}
 
 fn require_impl_as_ref_c_str(_: impl AsRef<CStr>) {}
 fn require_impl_as_ref_os_str(_: impl AsRef<OsStr>) {}
 fn require_impl_as_ref_path(_: impl AsRef<std::path::Path>) {}
 fn require_impl_as_ref_str(_: impl AsRef<str>) {}
 fn require_impl_as_ref_slice<T>(_: impl AsRef<[T]>) {}
 
 fn require_as_ref_str_slice<T: AsRef<str>, U, V: AsRef<[U]>>(_: T, _: V) {}
 fn require_as_ref_slice_str<T, U: AsRef<[T]>, V: AsRef<str>>(_: U, _: V) {}
 // `check_files` is based on:
 // `check_files` is based on:
 // https://github.com/breard-r/acmed/blob/1f0dcc32aadbc5e52de6d23b9703554c0f925113/acmed/src/storage.rs#L262
 fn check_files(file_types: &[FileType]) -> bool {
     for t in file_types {
         let path = match get_file_path(t) {
             Ok(p) => p,
                 return false;
             },
         };
         if !path.is_file() {
         if !path.is_file() {
             return false;
         }
     }
     true
 }
 
 fn get_file_path(_file_type: &FileType) -> Result<std::path::PathBuf, std::io::Error> {
     Ok(std::path::PathBuf::new())
 
 
 fn require_string(_: &String) {}
 
 #[clippy::msrv = "1.35"]
 fn _msrv_1_35() {
     // `copied` was stabilized in 1.36, so clippy should use `cloned`.
     let _ = &["x"][..].iter().cloned();
 
 
 #[clippy::msrv = "1.36"]
 fn _msrv_1_36() {
     let _ = &["x"][..].iter().copied();
 
 // https://github.com/rust-lang/rust-clippy/issues/8507
 mod issue_8507 {
     #![allow(dead_code)]
     #![allow(dead_code)]
 
     struct Opaque<P>(P);
     pub trait Abstracted {}
 
 
     impl<P> Abstracted for Opaque<P> {}
 
     fn build<P>(p: P) -> Opaque<P>
     where
         P: AsRef<str>,
         Opaque(p)
     }
 
     // Should not lint.
     // Should not lint.
     fn test_str(s: &str) -> Box<dyn Abstracted> {
         Box::new(build(s.to_string()))
 
     // Should not lint.
     // Should not lint.
     fn test_x(x: super::X) -> Box<dyn Abstracted> {
         Box::new(build(x))
 
     #[derive(Clone, Copy)]
     #[derive(Clone, Copy)]
     struct Y(&'static str);
 
     impl AsRef<str> for Y {
         fn as_ref(&self) -> &str {
         }
     }
 
     impl ToString for Y {
     impl ToString for Y {
         fn to_string(&self) -> String {
             self.0.to_string()
         }
     }
 
     // Should lint because Y is copy.
     fn test_y(y: Y) -> Box<dyn Abstracted> {
         Box::new(build(y))
 }
 
 // https://github.com/rust-lang/rust-clippy/issues/8759
 mod issue_8759 {
 mod issue_8759 {
     #![allow(dead_code)]
 
     #[derive(Default)]
     struct View {}
 
     impl std::borrow::ToOwned for View {
         type Owned = View;
         fn to_owned(&self) -> Self::Owned {
             View {}
     }
 
     #[derive(Default)]
     struct RenderWindow {
     struct RenderWindow {
         default_view: View,
 
     impl RenderWindow {
     impl RenderWindow {
         fn default_view(&self) -> &View {
             &self.default_view
         }
         fn set_view(&mut self, _view: &View) {}
 
     fn main() {
     fn main() {
         let mut rw = RenderWindow::default();
         rw.set_view(&rw.default_view().to_owned());
 }
 
 mod issue_8759_variant {
     #![allow(dead_code)]
     #![allow(dead_code)]
 
     #[derive(Clone, Default)]
     struct View {}
 
---
             "123".to_string()
         }
     }
 
     impl AsRef<[u8]> for Bytes {
         fn as_ref(&self) -> &[u8] {
             &[1, 2, 3]
     }
 
 
     fn consume<C: AsRef<[u8]>>(c: C) {
         let _ = c;
 
     pub fn main() {
     pub fn main() {
         let b = Bytes {};
         // Should not lint.
         consume(b.to_string());
 }
 
 mod issue_9351 {
     #![allow(dead_code)]
     #![allow(dead_code)]
 
     use std::ops::Deref;
     use std::path::{Path, PathBuf};
 
     fn require_deref_path<T: Deref<Target = std::path::Path>>(x: T) -> T {
     }
 
 
     fn generic_arg_used_elsewhere<T: AsRef<Path>>(_x: T, _y: T) {}
 
     fn id<T: AsRef<str>>(x: T) -> T {
     }
 
 
     fn predicates_are_satisfied(_x: impl std::fmt::Write) {}
     // Should lint
     // Should lint
     fn single_return() -> impl AsRef<str> {
         id("abc")
 
     // Should not lint
     // Should not lint
     fn multiple_returns(b: bool) -> impl AsRef<str> {
             return String::new();
         }
 
 
         id("abc".to_string())
 
     struct S1(String);
 
     // Should not lint
     // Should not lint
     fn fields1() -> S1 {
         S1(id("abc".to_string()))
 
     struct S2 {
         s: String,
     }
     }
 
     // Should not lint
     fn fields2() {
         let mut s = S2 { s: "abc".into() };
         s.s = id("abc".to_string());
 
     pub fn main() {
         let path = std::path::Path::new("x");
         let path_buf = path.to_owned();
         let path_buf = path.to_owned();
 
         // Should not lint.
         let _x: PathBuf = require_deref_path(path.to_owned());
         generic_arg_used_elsewhere(path.to_owned(), path_buf);
         predicates_are_satisfied(id("abc".to_string()));
 }
 
 mod issue_9504 {
     #![allow(dead_code)]
     #![allow(dead_code)]
 
     async fn foo<S: AsRef<str>>(_: S) {}
     async fn bar() {
         foo(std::path::PathBuf::new().to_string_lossy().to_string()).await;
 }
 
 mod issue_9771a {
     #![allow(dead_code)]
     #![allow(dead_code)]
 
     use std::marker::PhantomData;
 
     pub struct Key<K: AsRef<[u8]>, V: ?Sized>(K, PhantomData<V>);
 
     impl<K: AsRef<[u8]>, V: ?Sized> Key<K, V> {
         pub fn new(key: K) -> Key<K, V> {
             Key(key, PhantomData)
     }
 
 
     pub fn pkh(pkh: &[u8]) -> Key<Vec<u8>, String> {
         Key::new([b"pkh-", pkh].concat().to_vec())
 }
 
 mod issue_9771b {
     #![allow(dead_code)]
     #![allow(dead_code)]
 
     pub struct Key<K: AsRef<[u8]>>(K);
 
     pub fn from(c: &[u8]) -> Key<Vec<u8>> {
         let v = [c].concat();
         Key(v.to_vec())
 }
 
 
 // This is a watered down version of the code in: https://github.com/oxigraph/rio
 // The ICE is triggered by the call to `to_owned` on this line:
 // https://github.com/oxigraph/rio/blob/66635b9ff8e5423e58932353fa40d6e64e4820f7/testsuite/src/parser_evaluator.rs#L116
 mod issue_10021 {
 
 
     pub struct Iri<T>(T);
 
     impl<T: AsRef<str>> Iri<T> {
         pub fn parse(iri: T) -> Result<Self, ()> {
             unimplemented!()
     }
 
 
     pub fn parse_w3c_rdf_test_file(url: &str) -> Result<(), ()> {
         let base_iri = Iri::parse(url.to_owned())?;
     }
 }
 


The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_to_owned.stage-id.fixed
To only update this specific test, also pass `--test-args unnecessary_to_owned.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unnecessary_to_owned.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_to_owned.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_to_owned.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unnecessary use of `into_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1120,"byte_end":1133,"line_start":59,"line_end":59,"column_start":36,"column_end":49,"is_primary":true,"text":[{"text":"    require_c_str(&Cow::from(c_str).into_owned());","highlight_start":36,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::unnecessary-to-owned` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1120,"byte_end":1133,"line_start":59,"line_end":59,"column_start":36,"column_end":49,"is_primary":true,"text":[{"text":"    require_c_str(&Cow::from(c_str).into_owned());","highlight_start":36,"highlight_end":49}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `into_owned`\n  --> tests/ui/unnecessary_to_owned.rs:59:36\n   |\nLL |     require_c_str(&Cow::from(c_str).into_owned());\n   |                                    ^^^^^^^^^^^^^ help: remove this\n   |\n   = note: `-D clippy::unnecessary-to-owned` implied by `-D warnings`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1154,"byte_end":1171,"line_start":60,"line_end":60,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    require_c_str(&c_str.to_owned());","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1154,"byte_end":1171,"line_start":60,"line_end":60,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    require_c_str(&c_str.to_owned());","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":"c_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:60:19\n   |\nLL |     require_c_str(&c_str.to_owned());\n   |                   ^^^^^^^^^^^^^^^^^ help: use: `c_str`\n\n"}
{"message":"unnecessary use of `to_os_string`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1194,"byte_end":1216,"line_start":62,"line_end":62,"column_start":20,"column_end":42,"is_primary":true,"text":[{"text":"    require_os_str(&os_str.to_os_string());","highlight_start":20,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1194,"byte_end":1216,"line_start":62,"line_end":62,"column_start":20,"column_end":42,"is_primary":true,"text":[{"text":"    require_os_str(&os_str.to_os_string());","highlight_start":20,"highlight_end":42}],"label":null,"suggested_replacement":"os_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_os_string`\n  --> tests/ui/unnecessary_to_owned.rs:62:20\n   |\nLL |     require_os_str(&os_str.to_os_string());\n   |                    ^^^^^^^^^^^^^^^^^^^^^^ help: use: `os_str`\n\n"}
{"message":"unnecessary use of `into_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1256,"byte_end":1269,"line_start":63,"line_end":63,"column_start":38,"column_end":51,"is_primary":true,"text":[{"text":"    require_os_str(&Cow::from(os_str).into_owned());","highlight_start":38,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1256,"byte_end":1269,"line_start":63,"line_end":63,"column_start":38,"column_end":51,"is_primary":true,"text":[{"text":"    require_os_str(&Cow::from(os_str).into_owned());","highlight_start":38,"highlight_end":51}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `into_owned`\n  --> tests/ui/unnecessary_to_owned.rs:63:38\n   |\nLL |     require_os_str(&Cow::from(os_str).into_owned());\n   |                                      ^^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1291,"byte_end":1309,"line_start":64,"line_end":64,"column_start":20,"column_end":38,"is_primary":true,"text":[{"text":"    require_os_str(&os_str.to_owned());","highlight_start":20,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1291,"byte_end":1309,"line_start":64,"line_end":64,"column_start":20,"column_end":38,"is_primary":true,"text":[{"text":"    require_os_str(&os_str.to_owned());","highlight_start":20,"highlight_end":38}],"label":null,"suggested_replacement":"os_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:64:20\n   |\nLL |     require_os_str(&os_str.to_owned());\n   |                    ^^^^^^^^^^^^^^^^^^ help: use: `os_str`\n\n"}
{"message":"unnecessary use of `to_path_buf`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1330,"byte_end":1349,"line_start":66,"line_end":66,"column_start":18,"column_end":37,"is_primary":true,"text":[{"text":"    require_path(&path.to_path_buf());","highlight_start":18,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1330,"byte_end":1349,"line_start":66,"line_end":66,"column_start":18,"column_end":37,"is_primary":true,"text":[{"text":"    require_path(&path.to_path_buf());","highlight_start":18,"highlight_end":37}],"label":null,"suggested_replacement":"path","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_path_buf`\n  --> tests/ui/unnecessary_to_owned.rs:66:18\n   |\nLL |     require_path(&path.to_path_buf());\n   |                  ^^^^^^^^^^^^^^^^^^^ help: use: `path`\n\n"}
{"message":"unnecessary use of `into_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1385,"byte_end":1398,"line_start":67,"line_end":67,"column_start":34,"column_end":47,"is_primary":true,"text":[{"text":"    require_path(&Cow::from(path).into_owned());","highlight_start":34,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1385,"byte_end":1398,"line_start":67,"line_end":67,"column_start":34,"column_end":47,"is_primary":true,"text":[{"text":"    require_path(&Cow::from(path).into_owned());","highlight_start":34,"highlight_end":47}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `into_owned`\n  --> tests/ui/unnecessary_to_owned.rs:67:34\n   |\nLL |     require_path(&Cow::from(path).into_owned());\n   |                                  ^^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1418,"byte_end":1434,"line_start":68,"line_end":68,"column_start":18,"column_end":34,"is_primary":true,"text":[{"text":"    require_path(&path.to_owned());","highlight_start":18,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1418,"byte_end":1434,"line_start":68,"line_end":68,"column_start":18,"column_end":34,"is_primary":true,"text":[{"text":"    require_path(&path.to_owned());","highlight_start":18,"highlight_end":34}],"label":null,"suggested_replacement":"path","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:68:18\n   |\nLL |     require_path(&path.to_owned());\n   |                  ^^^^^^^^^^^^^^^^ help: use: `path`\n\n"}
{"message":"unnecessary use of `to_string`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1454,"byte_end":1468,"line_start":70,"line_end":70,"column_start":17,"column_end":31,"is_primary":true,"text":[{"text":"    require_str(&s.to_string());","highlight_start":17,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1454,"byte_end":1468,"line_start":70,"line_end":70,"column_start":17,"column_end":31,"is_primary":true,"text":[{"text":"    require_str(&s.to_string());","highlight_start":17,"highlight_end":31}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_string`\n  --> tests/ui/unnecessary_to_owned.rs:70:17\n   |\nLL |     require_str(&s.to_string());\n   |                 ^^^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `into_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1500,"byte_end":1513,"line_start":71,"line_end":71,"column_start":30,"column_end":43,"is_primary":true,"text":[{"text":"    require_str(&Cow::from(s).into_owned());","highlight_start":30,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1500,"byte_end":1513,"line_start":71,"line_end":71,"column_start":30,"column_end":43,"is_primary":true,"text":[{"text":"    require_str(&Cow::from(s).into_owned());","highlight_start":30,"highlight_end":43}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `into_owned`\n  --> tests/ui/unnecessary_to_owned.rs:71:30\n   |\nLL |     require_str(&Cow::from(s).into_owned());\n   |                              ^^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1532,"byte_end":1545,"line_start":72,"line_end":72,"column_start":17,"column_end":30,"is_primary":true,"text":[{"text":"    require_str(&s.to_owned());","highlight_start":17,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1532,"byte_end":1545,"line_start":72,"line_end":72,"column_start":17,"column_end":30,"is_primary":true,"text":[{"text":"    require_str(&s.to_owned());","highlight_start":17,"highlight_end":30}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:72:17\n   |\nLL |     require_str(&s.to_owned());\n   |                 ^^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_string`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1564,"byte_end":1582,"line_start":73,"line_end":73,"column_start":17,"column_end":35,"is_primary":true,"text":[{"text":"    require_str(&x_ref.to_string());","highlight_start":17,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1564,"byte_end":1582,"line_start":73,"line_end":73,"column_start":17,"column_end":35,"is_primary":true,"text":[{"text":"    require_str(&x_ref.to_string());","highlight_start":17,"highlight_end":35}],"label":null,"suggested_replacement":"x_ref.as_ref()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_string`\n  --> tests/ui/unnecessary_to_owned.rs:73:17\n   |\nLL |     require_str(&x_ref.to_string());\n   |                 ^^^^^^^^^^^^^^^^^^ help: use: `x_ref.as_ref()`\n\n"}
{"message":"unnecessary use of `to_vec`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1604,"byte_end":1619,"line_start":75,"line_end":75,"column_start":19,"column_end":34,"is_primary":true,"text":[{"text":"    require_slice(&slice.to_vec());","highlight_start":19,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1604,"byte_end":1619,"line_start":75,"line_end":75,"column_start":19,"column_end":34,"is_primary":true,"text":[{"text":"    require_slice(&slice.to_vec());","highlight_start":19,"highlight_end":34}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_vec`\n  --> tests/ui/unnecessary_to_owned.rs:75:19\n   |\nLL |     require_slice(&slice.to_vec());\n   |                   ^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `into_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1657,"byte_end":1670,"line_start":76,"line_end":76,"column_start":36,"column_end":49,"is_primary":true,"text":[{"text":"    require_slice(&Cow::from(slice).into_owned());","highlight_start":36,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1657,"byte_end":1670,"line_start":76,"line_end":76,"column_start":36,"column_end":49,"is_primary":true,"text":[{"text":"    require_slice(&Cow::from(slice).into_owned());","highlight_start":36,"highlight_end":49}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `into_owned`\n  --> tests/ui/unnecessary_to_owned.rs:76:36\n   |\nLL |     require_slice(&Cow::from(slice).into_owned());\n   |                                    ^^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1691,"byte_end":1708,"line_start":77,"line_end":77,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    require_slice(&array.to_owned());","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1691,"byte_end":1708,"line_start":77,"line_end":77,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    require_slice(&array.to_owned());","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":"array.as_ref()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:77:19\n   |\nLL |     require_slice(&array.to_owned());\n   |                   ^^^^^^^^^^^^^^^^^ help: use: `array.as_ref()`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1729,"byte_end":1750,"line_start":78,"line_end":78,"column_start":19,"column_end":40,"is_primary":true,"text":[{"text":"    require_slice(&array_ref.to_owned());","highlight_start":19,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1729,"byte_end":1750,"line_start":78,"line_end":78,"column_start":19,"column_end":40,"is_primary":true,"text":[{"text":"    require_slice(&array_ref.to_owned());","highlight_start":19,"highlight_end":40}],"label":null,"suggested_replacement":"array_ref.as_ref()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:78:19\n   |\nLL |     require_slice(&array_ref.to_owned());\n   |                   ^^^^^^^^^^^^^^^^^^^^^ help: use: `array_ref.as_ref()`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1771,"byte_end":1788,"line_start":79,"line_end":79,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    require_slice(&slice.to_owned());","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1771,"byte_end":1788,"line_start":79,"line_end":79,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    require_slice(&slice.to_owned());","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:79:19\n   |\nLL |     require_slice(&slice.to_owned());\n   |                   ^^^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `into_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1910,"byte_end":1923,"line_start":82,"line_end":82,"column_start":42,"column_end":55,"is_primary":true,"text":[{"text":"    require_x(&Cow::<X>::Owned(x.clone()).into_owned());","highlight_start":42,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":1910,"byte_end":1923,"line_start":82,"line_end":82,"column_start":42,"column_end":55,"is_primary":true,"text":[{"text":"    require_x(&Cow::<X>::Owned(x.clone()).into_owned());","highlight_start":42,"highlight_end":55}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `into_owned`\n  --> tests/ui/unnecessary_to_owned.rs:82:42\n   |\nLL |     require_x(&Cow::<X>::Owned(x.clone()).into_owned());\n   |                                          ^^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2024,"byte_end":2040,"line_start":85,"line_end":85,"column_start":25,"column_end":41,"is_primary":true,"text":[{"text":"    require_deref_c_str(c_str.to_owned());","highlight_start":25,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2024,"byte_end":2040,"line_start":85,"line_end":85,"column_start":25,"column_end":41,"is_primary":true,"text":[{"text":"    require_deref_c_str(c_str.to_owned());","highlight_start":25,"highlight_end":41}],"label":null,"suggested_replacement":"c_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:85:25\n   |\nLL |     require_deref_c_str(c_str.to_owned());\n   |                         ^^^^^^^^^^^^^^^^ help: use: `c_str`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2068,"byte_end":2085,"line_start":86,"line_end":86,"column_start":26,"column_end":43,"is_primary":true,"text":[{"text":"    require_deref_os_str(os_str.to_owned());","highlight_start":26,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2068,"byte_end":2085,"line_start":86,"line_end":86,"column_start":26,"column_end":43,"is_primary":true,"text":[{"text":"    require_deref_os_str(os_str.to_owned());","highlight_start":26,"highlight_end":43}],"label":null,"suggested_replacement":"os_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:86:26\n   |\nLL |     require_deref_os_str(os_str.to_owned());\n   |                          ^^^^^^^^^^^^^^^^^ help: use: `os_str`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2111,"byte_end":2126,"line_start":87,"line_end":87,"column_start":24,"column_end":39,"is_primary":true,"text":[{"text":"    require_deref_path(path.to_owned());","highlight_start":24,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2111,"byte_end":2126,"line_start":87,"line_end":87,"column_start":24,"column_end":39,"is_primary":true,"text":[{"text":"    require_deref_path(path.to_owned());","highlight_start":24,"highlight_end":39}],"label":null,"suggested_replacement":"path","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:87:24\n   |\nLL |     require_deref_path(path.to_owned());\n   |                        ^^^^^^^^^^^^^^^ help: use: `path`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2151,"byte_end":2163,"line_start":88,"line_end":88,"column_start":23,"column_end":35,"is_primary":true,"text":[{"text":"    require_deref_str(s.to_owned());","highlight_start":23,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2151,"byte_end":2163,"line_start":88,"line_end":88,"column_start":23,"column_end":35,"is_primary":true,"text":[{"text":"    require_deref_str(s.to_owned());","highlight_start":23,"highlight_end":35}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:88:23\n   |\nLL |     require_deref_str(s.to_owned());\n   |                       ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2190,"byte_end":2206,"line_start":89,"line_end":89,"column_start":25,"column_end":41,"is_primary":true,"text":[{"text":"    require_deref_slice(slice.to_owned());","highlight_start":25,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2190,"byte_end":2206,"line_start":89,"line_end":89,"column_start":25,"column_end":41,"is_primary":true,"text":[{"text":"    require_deref_slice(slice.to_owned());","highlight_start":25,"highlight_end":41}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:89:25\n   |\nLL |     require_deref_slice(slice.to_owned());\n   |                         ^^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2239,"byte_end":2255,"line_start":91,"line_end":91,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"    require_impl_deref_c_str(c_str.to_owned());","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2239,"byte_end":2255,"line_start":91,"line_end":91,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"    require_impl_deref_c_str(c_str.to_owned());","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":"c_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:91:30\n   |\nLL |     require_impl_deref_c_str(c_str.to_owned());\n   |                              ^^^^^^^^^^^^^^^^ help: use: `c_str`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2288,"byte_end":2305,"line_start":92,"line_end":92,"column_start":31,"column_end":48,"is_primary":true,"text":[{"text":"    require_impl_deref_os_str(os_str.to_owned());","highlight_start":31,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2288,"byte_end":2305,"line_start":92,"line_end":92,"column_start":31,"column_end":48,"is_primary":true,"text":[{"text":"    require_impl_deref_os_str(os_str.to_owned());","highlight_start":31,"highlight_end":48}],"label":null,"suggested_replacement":"os_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:92:31\n   |\nLL |     require_impl_deref_os_str(os_str.to_owned());\n   |                               ^^^^^^^^^^^^^^^^^ help: use: `os_str`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2336,"byte_end":2351,"line_start":93,"line_end":93,"column_start":29,"column_end":44,"is_primary":true,"text":[{"text":"    require_impl_deref_path(path.to_owned());","highlight_start":29,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2336,"byte_end":2351,"line_start":93,"line_end":93,"column_start":29,"column_end":44,"is_primary":true,"text":[{"text":"    require_impl_deref_path(path.to_owned());","highlight_start":29,"highlight_end":44}],"label":null,"suggested_replacement":"path","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:93:29\n   |\nLL |     require_impl_deref_path(path.to_owned());\n   |                             ^^^^^^^^^^^^^^^ help: use: `path`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2381,"byte_end":2393,"line_start":94,"line_end":94,"column_start":28,"column_end":40,"is_primary":true,"text":[{"text":"    require_impl_deref_str(s.to_owned());","highlight_start":28,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2381,"byte_end":2393,"line_start":94,"line_end":94,"column_start":28,"column_end":40,"is_primary":true,"text":[{"text":"    require_impl_deref_str(s.to_owned());","highlight_start":28,"highlight_end":40}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:94:28\n   |\nLL |     require_impl_deref_str(s.to_owned());\n   |                            ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2425,"byte_end":2441,"line_start":95,"line_end":95,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"    require_impl_deref_slice(slice.to_owned());","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2425,"byte_end":2441,"line_start":95,"line_end":95,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"    require_impl_deref_slice(slice.to_owned());","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:95:30\n   |\nLL |     require_impl_deref_slice(slice.to_owned());\n   |                              ^^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2473,"byte_end":2485,"line_start":97,"line_end":97,"column_start":29,"column_end":41,"is_primary":true,"text":[{"text":"    require_deref_str_slice(s.to_owned(), slice.to_owned());","highlight_start":29,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2473,"byte_end":2485,"line_start":97,"line_end":97,"column_start":29,"column_end":41,"is_primary":true,"text":[{"text":"    require_deref_str_slice(s.to_owned(), slice.to_owned());","highlight_start":29,"highlight_end":41}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:97:29\n   |\nLL |     require_deref_str_slice(s.to_owned(), slice.to_owned());\n   |                             ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2487,"byte_end":2503,"line_start":97,"line_end":97,"column_start":43,"column_end":59,"is_primary":true,"text":[{"text":"    require_deref_str_slice(s.to_owned(), slice.to_owned());","highlight_start":43,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2487,"byte_end":2503,"line_start":97,"line_end":97,"column_start":43,"column_end":59,"is_primary":true,"text":[{"text":"    require_deref_str_slice(s.to_owned(), slice.to_owned());","highlight_start":43,"highlight_end":59}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:97:43\n   |\nLL |     require_deref_str_slice(s.to_owned(), slice.to_owned());\n   |                                           ^^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2534,"byte_end":2550,"line_start":98,"line_end":98,"column_start":29,"column_end":45,"is_primary":true,"text":[{"text":"    require_deref_slice_str(slice.to_owned(), s.to_owned());","highlight_start":29,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2534,"byte_end":2550,"line_start":98,"line_end":98,"column_start":29,"column_end":45,"is_primary":true,"text":[{"text":"    require_deref_slice_str(slice.to_owned(), s.to_owned());","highlight_start":29,"highlight_end":45}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:98:29\n   |\nLL |     require_deref_slice_str(slice.to_owned(), s.to_owned());\n   |                             ^^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2552,"byte_end":2564,"line_start":98,"line_end":98,"column_start":47,"column_end":59,"is_primary":true,"text":[{"text":"    require_deref_slice_str(slice.to_owned(), s.to_owned());","highlight_start":47,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2552,"byte_end":2564,"line_start":98,"line_end":98,"column_start":47,"column_end":59,"is_primary":true,"text":[{"text":"    require_deref_slice_str(slice.to_owned(), s.to_owned());","highlight_start":47,"highlight_end":59}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:98:47\n   |\nLL |     require_deref_slice_str(slice.to_owned(), s.to_owned());\n   |                                               ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2593,"byte_end":2609,"line_start":100,"line_end":100,"column_start":26,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_c_str(c_str.to_owned());","highlight_start":26,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2593,"byte_end":2609,"line_start":100,"line_end":100,"column_start":26,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_c_str(c_str.to_owned());","highlight_start":26,"highlight_end":42}],"label":null,"suggested_replacement":"c_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:100:26\n   |\nLL |     require_as_ref_c_str(c_str.to_owned());\n   |                          ^^^^^^^^^^^^^^^^ help: use: `c_str`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2638,"byte_end":2655,"line_start":101,"line_end":101,"column_start":27,"column_end":44,"is_primary":true,"text":[{"text":"    require_as_ref_os_str(os_str.to_owned());","highlight_start":27,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2638,"byte_end":2655,"line_start":101,"line_end":101,"column_start":27,"column_end":44,"is_primary":true,"text":[{"text":"    require_as_ref_os_str(os_str.to_owned());","highlight_start":27,"highlight_end":44}],"label":null,"suggested_replacement":"os_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:101:27\n   |\nLL |     require_as_ref_os_str(os_str.to_owned());\n   |                           ^^^^^^^^^^^^^^^^^ help: use: `os_str`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2682,"byte_end":2697,"line_start":102,"line_end":102,"column_start":25,"column_end":40,"is_primary":true,"text":[{"text":"    require_as_ref_path(path.to_owned());","highlight_start":25,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2682,"byte_end":2697,"line_start":102,"line_end":102,"column_start":25,"column_end":40,"is_primary":true,"text":[{"text":"    require_as_ref_path(path.to_owned());","highlight_start":25,"highlight_end":40}],"label":null,"suggested_replacement":"path","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:102:25\n   |\nLL |     require_as_ref_path(path.to_owned());\n   |                         ^^^^^^^^^^^^^^^ help: use: `path`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2723,"byte_end":2735,"line_start":103,"line_end":103,"column_start":24,"column_end":36,"is_primary":true,"text":[{"text":"    require_as_ref_str(s.to_owned());","highlight_start":24,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2723,"byte_end":2735,"line_start":103,"line_end":103,"column_start":24,"column_end":36,"is_primary":true,"text":[{"text":"    require_as_ref_str(s.to_owned());","highlight_start":24,"highlight_end":36}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:103:24\n   |\nLL |     require_as_ref_str(s.to_owned());\n   |                        ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2761,"byte_end":2773,"line_start":104,"line_end":104,"column_start":24,"column_end":36,"is_primary":true,"text":[{"text":"    require_as_ref_str(x.to_owned());","highlight_start":24,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2761,"byte_end":2773,"line_start":104,"line_end":104,"column_start":24,"column_end":36,"is_primary":true,"text":[{"text":"    require_as_ref_str(x.to_owned());","highlight_start":24,"highlight_end":36}],"label":null,"suggested_replacement":"&x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:104:24\n   |\nLL |     require_as_ref_str(x.to_owned());\n   |                        ^^^^^^^^^^^^ help: use: `&x`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2801,"byte_end":2817,"line_start":105,"line_end":105,"column_start":26,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_slice(array.to_owned());","highlight_start":26,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2801,"byte_end":2817,"line_start":105,"line_end":105,"column_start":26,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_slice(array.to_owned());","highlight_start":26,"highlight_end":42}],"label":null,"suggested_replacement":"array","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:105:26\n   |\nLL |     require_as_ref_slice(array.to_owned());\n   |                          ^^^^^^^^^^^^^^^^ help: use: `array`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2845,"byte_end":2865,"line_start":106,"line_end":106,"column_start":26,"column_end":46,"is_primary":true,"text":[{"text":"    require_as_ref_slice(array_ref.to_owned());","highlight_start":26,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2845,"byte_end":2865,"line_start":106,"line_end":106,"column_start":26,"column_end":46,"is_primary":true,"text":[{"text":"    require_as_ref_slice(array_ref.to_owned());","highlight_start":26,"highlight_end":46}],"label":null,"suggested_replacement":"array_ref","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:106:26\n   |\nLL |     require_as_ref_slice(array_ref.to_owned());\n   |                          ^^^^^^^^^^^^^^^^^^^^ help: use: `array_ref`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2893,"byte_end":2909,"line_start":107,"line_end":107,"column_start":26,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_slice(slice.to_owned());","highlight_start":26,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2893,"byte_end":2909,"line_start":107,"line_end":107,"column_start":26,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_slice(slice.to_owned());","highlight_start":26,"highlight_end":42}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:107:26\n   |\nLL |     require_as_ref_slice(slice.to_owned());\n   |                          ^^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2943,"byte_end":2959,"line_start":109,"line_end":109,"column_start":31,"column_end":47,"is_primary":true,"text":[{"text":"    require_impl_as_ref_c_str(c_str.to_owned());","highlight_start":31,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2943,"byte_end":2959,"line_start":109,"line_end":109,"column_start":31,"column_end":47,"is_primary":true,"text":[{"text":"    require_impl_as_ref_c_str(c_str.to_owned());","highlight_start":31,"highlight_end":47}],"label":null,"suggested_replacement":"c_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:109:31\n   |\nLL |     require_impl_as_ref_c_str(c_str.to_owned());\n   |                               ^^^^^^^^^^^^^^^^ help: use: `c_str`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2993,"byte_end":3010,"line_start":110,"line_end":110,"column_start":32,"column_end":49,"is_primary":true,"text":[{"text":"    require_impl_as_ref_os_str(os_str.to_owned());","highlight_start":32,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":2993,"byte_end":3010,"line_start":110,"line_end":110,"column_start":32,"column_end":49,"is_primary":true,"text":[{"text":"    require_impl_as_ref_os_str(os_str.to_owned());","highlight_start":32,"highlight_end":49}],"label":null,"suggested_replacement":"os_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:110:32\n   |\nLL |     require_impl_as_ref_os_str(os_str.to_owned());\n   |                                ^^^^^^^^^^^^^^^^^ help: use: `os_str`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3042,"byte_end":3057,"line_start":111,"line_end":111,"column_start":30,"column_end":45,"is_primary":true,"text":[{"text":"    require_impl_as_ref_path(path.to_owned());","highlight_start":30,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3042,"byte_end":3057,"line_start":111,"line_end":111,"column_start":30,"column_end":45,"is_primary":true,"text":[{"text":"    require_impl_as_ref_path(path.to_owned());","highlight_start":30,"highlight_end":45}],"label":null,"suggested_replacement":"path","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:111:30\n   |\nLL |     require_impl_as_ref_path(path.to_owned());\n   |                              ^^^^^^^^^^^^^^^ help: use: `path`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3088,"byte_end":3100,"line_start":112,"line_end":112,"column_start":29,"column_end":41,"is_primary":true,"text":[{"text":"    require_impl_as_ref_str(s.to_owned());","highlight_start":29,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3088,"byte_end":3100,"line_start":112,"line_end":112,"column_start":29,"column_end":41,"is_primary":true,"text":[{"text":"    require_impl_as_ref_str(s.to_owned());","highlight_start":29,"highlight_end":41}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:112:29\n   |\nLL |     require_impl_as_ref_str(s.to_owned());\n   |                             ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3131,"byte_end":3143,"line_start":113,"line_end":113,"column_start":29,"column_end":41,"is_primary":true,"text":[{"text":"    require_impl_as_ref_str(x.to_owned());","highlight_start":29,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3131,"byte_end":3143,"line_start":113,"line_end":113,"column_start":29,"column_end":41,"is_primary":true,"text":[{"text":"    require_impl_as_ref_str(x.to_owned());","highlight_start":29,"highlight_end":41}],"label":null,"suggested_replacement":"&x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:113:29\n   |\nLL |     require_impl_as_ref_str(x.to_owned());\n   |                             ^^^^^^^^^^^^ help: use: `&x`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3176,"byte_end":3192,"line_start":114,"line_end":114,"column_start":31,"column_end":47,"is_primary":true,"text":[{"text":"    require_impl_as_ref_slice(array.to_owned());","highlight_start":31,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3176,"byte_end":3192,"line_start":114,"line_end":114,"column_start":31,"column_end":47,"is_primary":true,"text":[{"text":"    require_impl_as_ref_slice(array.to_owned());","highlight_start":31,"highlight_end":47}],"label":null,"suggested_replacement":"array","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:114:31\n   |\nLL |     require_impl_as_ref_slice(array.to_owned());\n   |                               ^^^^^^^^^^^^^^^^ help: use: `array`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3225,"byte_end":3245,"line_start":115,"line_end":115,"column_start":31,"column_end":51,"is_primary":true,"text":[{"text":"    require_impl_as_ref_slice(array_ref.to_owned());","highlight_start":31,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3225,"byte_end":3245,"line_start":115,"line_end":115,"column_start":31,"column_end":51,"is_primary":true,"text":[{"text":"    require_impl_as_ref_slice(array_ref.to_owned());","highlight_start":31,"highlight_end":51}],"label":null,"suggested_replacement":"array_ref","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:115:31\n   |\nLL |     require_impl_as_ref_slice(array_ref.to_owned());\n   |                               ^^^^^^^^^^^^^^^^^^^^ help: use: `array_ref`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3278,"byte_end":3294,"line_start":116,"line_end":116,"column_start":31,"column_end":47,"is_primary":true,"text":[{"text":"    require_impl_as_ref_slice(slice.to_owned());","highlight_start":31,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3278,"byte_end":3294,"line_start":116,"line_end":116,"column_start":31,"column_end":47,"is_primary":true,"text":[{"text":"    require_impl_as_ref_slice(slice.to_owned());","highlight_start":31,"highlight_end":47}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:116:31\n   |\nLL |     require_impl_as_ref_slice(slice.to_owned());\n   |                               ^^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3327,"byte_end":3339,"line_start":118,"line_end":118,"column_start":30,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), array.to_owned());","highlight_start":30,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3327,"byte_end":3339,"line_start":118,"line_end":118,"column_start":30,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), array.to_owned());","highlight_start":30,"highlight_end":42}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:118:30\n   |\nLL |     require_as_ref_str_slice(s.to_owned(), array.to_owned());\n   |                              ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3341,"byte_end":3357,"line_start":118,"line_end":118,"column_start":44,"column_end":60,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), array.to_owned());","highlight_start":44,"highlight_end":60}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3341,"byte_end":3357,"line_start":118,"line_end":118,"column_start":44,"column_end":60,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), array.to_owned());","highlight_start":44,"highlight_end":60}],"label":null,"suggested_replacement":"array","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:118:44\n   |\nLL |     require_as_ref_str_slice(s.to_owned(), array.to_owned());\n   |                                            ^^^^^^^^^^^^^^^^ help: use: `array`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3389,"byte_end":3401,"line_start":119,"line_end":119,"column_start":30,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());","highlight_start":30,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3389,"byte_end":3401,"line_start":119,"line_end":119,"column_start":30,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());","highlight_start":30,"highlight_end":42}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:119:30\n   |\nLL |     require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());\n   |                              ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3403,"byte_end":3423,"line_start":119,"line_end":119,"column_start":44,"column_end":64,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());","highlight_start":44,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3403,"byte_end":3423,"line_start":119,"line_end":119,"column_start":44,"column_end":64,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());","highlight_start":44,"highlight_end":64}],"label":null,"suggested_replacement":"array_ref","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:119:44\n   |\nLL |     require_as_ref_str_slice(s.to_owned(), array_ref.to_owned());\n   |                                            ^^^^^^^^^^^^^^^^^^^^ help: use: `array_ref`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3455,"byte_end":3467,"line_start":120,"line_end":120,"column_start":30,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), slice.to_owned());","highlight_start":30,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3455,"byte_end":3467,"line_start":120,"line_end":120,"column_start":30,"column_end":42,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), slice.to_owned());","highlight_start":30,"highlight_end":42}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:120:30\n   |\nLL |     require_as_ref_str_slice(s.to_owned(), slice.to_owned());\n   |                              ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3469,"byte_end":3485,"line_start":120,"line_end":120,"column_start":44,"column_end":60,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), slice.to_owned());","highlight_start":44,"highlight_end":60}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3469,"byte_end":3485,"line_start":120,"line_end":120,"column_start":44,"column_end":60,"is_primary":true,"text":[{"text":"    require_as_ref_str_slice(s.to_owned(), slice.to_owned());","highlight_start":44,"highlight_end":60}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:120:44\n   |\nLL |     require_as_ref_str_slice(s.to_owned(), slice.to_owned());\n   |                                            ^^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3517,"byte_end":3533,"line_start":121,"line_end":121,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(array.to_owned(), s.to_owned());","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3517,"byte_end":3533,"line_start":121,"line_end":121,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(array.to_owned(), s.to_owned());","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":"array","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:121:30\n   |\nLL |     require_as_ref_slice_str(array.to_owned(), s.to_owned());\n   |                              ^^^^^^^^^^^^^^^^ help: use: `array`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3535,"byte_end":3547,"line_start":121,"line_end":121,"column_start":48,"column_end":60,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(array.to_owned(), s.to_owned());","highlight_start":48,"highlight_end":60}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3535,"byte_end":3547,"line_start":121,"line_end":121,"column_start":48,"column_end":60,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(array.to_owned(), s.to_owned());","highlight_start":48,"highlight_end":60}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:121:48\n   |\nLL |     require_as_ref_slice_str(array.to_owned(), s.to_owned());\n   |                                                ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3579,"byte_end":3599,"line_start":122,"line_end":122,"column_start":30,"column_end":50,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());","highlight_start":30,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3579,"byte_end":3599,"line_start":122,"line_end":122,"column_start":30,"column_end":50,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());","highlight_start":30,"highlight_end":50}],"label":null,"suggested_replacement":"array_ref","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:122:30\n   |\nLL |     require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());\n   |                              ^^^^^^^^^^^^^^^^^^^^ help: use: `array_ref`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3601,"byte_end":3613,"line_start":122,"line_end":122,"column_start":52,"column_end":64,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());","highlight_start":52,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3601,"byte_end":3613,"line_start":122,"line_end":122,"column_start":52,"column_end":64,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());","highlight_start":52,"highlight_end":64}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:122:52\n   |\nLL |     require_as_ref_slice_str(array_ref.to_owned(), s.to_owned());\n   |                                                    ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3645,"byte_end":3661,"line_start":123,"line_end":123,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(slice.to_owned(), s.to_owned());","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3645,"byte_end":3661,"line_start":123,"line_end":123,"column_start":30,"column_end":46,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(slice.to_owned(), s.to_owned());","highlight_start":30,"highlight_end":46}],"label":null,"suggested_replacement":"slice","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:123:30\n   |\nLL |     require_as_ref_slice_str(slice.to_owned(), s.to_owned());\n   |                              ^^^^^^^^^^^^^^^^ help: use: `slice`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3663,"byte_end":3675,"line_start":123,"line_end":123,"column_start":48,"column_end":60,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(slice.to_owned(), s.to_owned());","highlight_start":48,"highlight_end":60}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3663,"byte_end":3675,"line_start":123,"line_end":123,"column_start":48,"column_end":60,"is_primary":true,"text":[{"text":"    require_as_ref_slice_str(slice.to_owned(), s.to_owned());","highlight_start":48,"highlight_end":60}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:123:48\n   |\nLL |     require_as_ref_slice_str(slice.to_owned(), s.to_owned());\n   |                                                ^^^^^^^^^^^^ help: use: `s`\n\n"}
{"message":"unnecessary use of `to_string`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3698,"byte_end":3716,"line_start":125,"line_end":125,"column_start":20,"column_end":38,"is_primary":true,"text":[{"text":"    let _ = x.join(&x_ref.to_string());","highlight_start":20,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3698,"byte_end":3716,"line_start":125,"line_end":125,"column_start":20,"column_end":38,"is_primary":true,"text":[{"text":"    let _ = x.join(&x_ref.to_string());","highlight_start":20,"highlight_end":38}],"label":null,"suggested_replacement":"x_ref","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_string`\n  --> tests/ui/unnecessary_to_owned.rs:125:20\n   |\nLL |     let _ = x.join(&x_ref.to_string());\n   |                    ^^^^^^^^^^^^^^^^^^ help: use: `x_ref`\n\n"}
{"message":"unnecessary use of `to_vec`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3732,"byte_end":3758,"line_start":127,"line_end":127,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = slice.to_vec().into_iter();","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3732,"byte_end":3758,"line_start":127,"line_end":127,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = slice.to_vec().into_iter();","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"slice.iter().copied()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_vec`\n  --> tests/ui/unnecessary_to_owned.rs:127:13\n   |\nLL |     let _ = slice.to_vec().into_iter();\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3772,"byte_end":3800,"line_start":128,"line_end":128,"column_start":13,"column_end":41,"is_primary":true,"text":[{"text":"    let _ = slice.to_owned().into_iter();","highlight_start":13,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3772,"byte_end":3800,"line_start":128,"line_end":128,"column_start":13,"column_end":41,"is_primary":true,"text":[{"text":"    let _ = slice.to_owned().into_iter();","highlight_start":13,"highlight_end":41}],"label":null,"suggested_replacement":"slice.iter().copied()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:128:13\n   |\nLL |     let _ = slice.to_owned().into_iter();\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`\n\n"}
{"message":"unnecessary use of `to_vec`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3814,"byte_end":3866,"line_start":129,"line_end":129,"column_start":13,"column_end":65,"is_primary":true,"text":[{"text":"    let _ = [std::path::PathBuf::new()][..].to_vec().into_iter();","highlight_start":13,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3814,"byte_end":3866,"line_start":129,"line_end":129,"column_start":13,"column_end":65,"is_primary":true,"text":[{"text":"    let _ = [std::path::PathBuf::new()][..].to_vec().into_iter();","highlight_start":13,"highlight_end":65}],"label":null,"suggested_replacement":"[std::path::PathBuf::new()][..].iter().cloned()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_vec`\n  --> tests/ui/unnecessary_to_owned.rs:129:13\n   |\nLL |     let _ = [std::path::PathBuf::new()][..].to_vec().into_iter();\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3880,"byte_end":3934,"line_start":130,"line_end":130,"column_start":13,"column_end":67,"is_primary":true,"text":[{"text":"    let _ = [std::path::PathBuf::new()][..].to_owned().into_iter();","highlight_start":13,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3880,"byte_end":3934,"line_start":130,"line_end":130,"column_start":13,"column_end":67,"is_primary":true,"text":[{"text":"    let _ = [std::path::PathBuf::new()][..].to_owned().into_iter();","highlight_start":13,"highlight_end":67}],"label":null,"suggested_replacement":"[std::path::PathBuf::new()][..].iter().cloned()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:130:13\n   |\nLL |     let _ = [std::path::PathBuf::new()][..].to_owned().into_iter();\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`\n\n"}
{"message":"unnecessary use of `to_vec`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3949,"byte_end":3988,"line_start":132,"line_end":132,"column_start":13,"column_end":52,"is_primary":true,"text":[{"text":"    let _ = IntoIterator::into_iter(slice.to_vec());","highlight_start":13,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":3949,"byte_end":3988,"line_start":132,"line_end":132,"column_start":13,"column_end":52,"is_primary":true,"text":[{"text":"    let _ = IntoIterator::into_iter(slice.to_vec());","highlight_start":13,"highlight_end":52}],"label":null,"suggested_replacement":"slice.iter().copied()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_vec`\n  --> tests/ui/unnecessary_to_owned.rs:132:13\n   |\nLL |     let _ = IntoIterator::into_iter(slice.to_vec());\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":4002,"byte_end":4043,"line_start":133,"line_end":133,"column_start":13,"column_end":54,"is_primary":true,"text":[{"text":"    let _ = IntoIterator::into_iter(slice.to_owned());","highlight_start":13,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":4002,"byte_end":4043,"line_start":133,"line_end":133,"column_start":13,"column_end":54,"is_primary":true,"text":[{"text":"    let _ = IntoIterator::into_iter(slice.to_owned());","highlight_start":13,"highlight_end":54}],"label":null,"suggested_replacement":"slice.iter().copied()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:133:13\n   |\nLL |     let _ = IntoIterator::into_iter(slice.to_owned());\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `slice.iter().copied()`\n\n"}
{"message":"unnecessary use of `to_vec`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":4057,"byte_end":4122,"line_start":134,"line_end":134,"column_start":13,"column_end":78,"is_primary":true,"text":[{"text":"    let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_vec());","highlight_start":13,"highlight_end":78}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":4057,"byte_end":4122,"line_start":134,"line_end":134,"column_start":13,"column_end":78,"is_primary":true,"text":[{"text":"    let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_vec());","highlight_start":13,"highlight_end":78}],"label":null,"suggested_replacement":"[std::path::PathBuf::new()][..].iter().cloned()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_vec`\n  --> tests/ui/unnecessary_to_owned.rs:134:13\n   |\nLL |     let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_vec());\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`\n\n"}
{"message":"unnecessary use of `to_owned`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":4136,"byte_end":4203,"line_start":135,"line_end":135,"column_start":13,"column_end":80,"is_primary":true,"text":[{"text":"    let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_owned());","highlight_start":13,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":4136,"byte_end":4203,"line_start":135,"line_end":135,"column_start":13,"column_end":80,"is_primary":true,"text":[{"text":"    let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_owned());","highlight_start":13,"highlight_end":80}],"label":null,"suggested_replacement":"[std::path::PathBuf::new()][..].iter().cloned()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_owned`\n  --> tests/ui/unnecessary_to_owned.rs:135:13\n   |\nLL |     let _ = IntoIterator::into_iter([std::path::PathBuf::new()][..].to_owned());\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[std::path::PathBuf::new()][..].iter().cloned()`\n\n"}
{"message":"unnecessary use of `to_vec`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":6793,"byte_end":6812,"line_start":197,"line_end":197,"column_start":14,"column_end":33,"is_primary":true,"text":[{"text":"    for t in file_types.to_vec() {","highlight_start":14,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":6793,"byte_end":6812,"line_start":197,"line_end":197,"column_start":14,"column_end":33,"is_primary":false,"text":[{"text":"    for t in file_types.to_vec() {","highlight_start":14,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":6793,"byte_end":6812,"line_start":197,"line_end":197,"column_start":14,"column_end":33,"is_primary":true,"text":[{"text":"    for t in file_types.to_vec() {","highlight_start":14,"highlight_end":33}],"label":null,"suggested_replacement":"file_types","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":6793,"byte_end":6812,"line_start":197,"line_end":197,"column_start":14,"column_end":33,"is_primary":false,"text":[{"text":"    for t in file_types.to_vec() {","highlight_start":14,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null},{"message":"remove this `&`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":6854,"byte_end":6855,"line_start":198,"line_end":198,"column_start":40,"column_end":41,"is_primary":true,"text":[{"text":"        let path = match get_file_path(&t) {","highlight_start":40,"highlight_end":41}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_vec`\n  --> tests/ui/unnecessary_to_owned.rs:197:14\n   |\nLL |     for t in file_types.to_vec() {\n   |              ^^^^^^^^^^^^^^^^^^^\n   |\nhelp: use\n   |\nLL |     for t in file_types {\n   |              ~~~~~~~~~~\nhelp: remove this `&`\n   |\nLL -         let path = match get_file_path(&t) {\nLL +         let path = match get_file_path(t) {\n   |\n\n"}
{"message":"unnecessary use of `to_vec`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":7333,"byte_end":7363,"line_start":220,"line_end":220,"column_start":14,"column_end":44,"is_primary":true,"text":[{"text":"    let _ = &[\"x\"][..].to_vec().into_iter();","highlight_start":14,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":7333,"byte_end":7363,"line_start":220,"line_end":220,"column_start":14,"column_end":44,"is_primary":true,"text":[{"text":"    let _ = &[\"x\"][..].to_vec().into_iter();","highlight_start":14,"highlight_end":44}],"label":null,"suggested_replacement":"[\"x\"][..].iter().cloned()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_vec`\n  --> tests/ui/unnecessary_to_owned.rs:220:14\n   |\nLL |     let _ = &[\"x\"][..].to_vec().into_iter();\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[\"x\"][..].iter().cloned()`\n\n"}
{"message":"unnecessary use of `to_vec`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":7424,"byte_end":7454,"line_start":225,"line_end":225,"column_start":14,"column_end":44,"is_primary":true,"text":[{"text":"    let _ = &[\"x\"][..].to_vec().into_iter();","highlight_start":14,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":7424,"byte_end":7454,"line_start":225,"line_end":225,"column_start":14,"column_end":44,"is_primary":true,"text":[{"text":"    let _ = &[\"x\"][..].to_vec().into_iter();","highlight_start":14,"highlight_end":44}],"label":null,"suggested_replacement":"[\"x\"][..].iter().copied()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_vec`\n  --> tests/ui/unnecessary_to_owned.rs:225:14\n   |\nLL |     let _ = &[\"x\"][..].to_vec().into_iter();\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use: `[\"x\"][..].iter().copied()`\n\n"}
{"message":"unnecessary use of `to_string`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":8358,"byte_end":8371,"line_start":272,"line_end":272,"column_start":24,"column_end":37,"is_primary":true,"text":[{"text":"        Box::new(build(y.to_string()))","highlight_start":24,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":8358,"byte_end":8371,"line_start":272,"line_end":272,"column_start":24,"column_end":37,"is_primary":true,"text":[{"text":"        Box::new(build(y.to_string()))","highlight_start":24,"highlight_end":37}],"label":null,"suggested_replacement":"y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_string`\n  --> tests/ui/unnecessary_to_owned.rs:272:24\n   |\nLL |         Box::new(build(y.to_string()))\n   |                        ^^^^^^^^^^^^^ help: use: `y`\n\n"}
{"message":"unnecessary use of `to_string`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":10439,"byte_end":10456,"line_start":380,"line_end":380,"column_start":12,"column_end":29,"is_primary":true,"text":[{"text":"        id(\"abc\".to_string())","highlight_start":12,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_to_owned.rs","byte_start":10439,"byte_end":10456,"line_start":380,"line_end":380,"column_start":12,"column_end":29,"is_primary":true,"text":[{"text":"        id(\"abc\".to_string())","highlight_start":12,"highlight_end":29}],"label":null,"suggested_replacement":"\"abc\"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_string`\n  --> tests/ui/unnecessary_to_owned.rs:380:12\n   |\nLL |         id(\"abc\".to_string())\n   |            ^^^^^^^^^^^^^^^^^ help: use: `\"abc\"`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.10.0/src/lib.rs:111:22
