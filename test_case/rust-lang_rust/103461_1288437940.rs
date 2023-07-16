plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7feb003882ecf7699e6705b537673e20985accff and 206a1ae326c47da8cbbb534369a2baa30bb70abf
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

 error: redundant clone
   --> $DIR/redundant_clone.rs:10:42
    |
 LL |     let _s = ["lorem", "ipsum"].join(" ").to_string();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:10:14
    |
    |
 LL |     let _s = ["lorem", "ipsum"].join(" ").to_string();
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `-D clippy::redundant-clone` implied by `-D warnings`
 error: redundant clone
   --> $DIR/redundant_clone.rs:13:15
    |
    |
 LL |     let _s = s.clone();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:13:14
    |
    |
 LL |     let _s = s.clone();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:16:15
    |
    |
 LL |     let _s = s.to_string();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:16:14
    |
    |
 LL |     let _s = s.to_string();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:19:15
    |
    |
 LL |     let _s = s.to_owned();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:19:14
    |
    |
 LL |     let _s = s.to_owned();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:21:42
    |
    |
 LL |     let _s = Path::new("/a/b/").join("c").to_owned();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:21:14
    |
    |
 LL |     let _s = Path::new("/a/b/").join("c").to_owned();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:23:42
    |
    |
 LL |     let _s = Path::new("/a/b/").join("c").to_path_buf();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:23:14
    |
    |
 LL |     let _s = Path::new("/a/b/").join("c").to_path_buf();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:25:29
    |
    |
 LL |     let _s = OsString::new().to_owned();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:25:14
    |
    |
 LL |     let _s = OsString::new().to_owned();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:27:29
    |
    |
 LL |     let _s = OsString::new().to_os_string();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:27:14
    |
    |
 LL |     let _s = OsString::new().to_os_string();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:38:19
    |
    |
 LL |     let _t = tup.0.clone();
error: test failed, to rerun pass `--test compile-test`
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:38:14
   --> $DIR/redundant_clone.rs:38:14
    |
 LL |     let _t = tup.0.clone();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:70:25
    |
    |
 LL |     if b { (a.clone(), a.clone()) } else { (Alpha, a) }
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:70:24
    |
    |
 LL |     if b { (a.clone(), a.clone()) } else { (Alpha, a) }
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:127:15
    |
    |
 LL |     let _s = s.clone();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:127:14
    |
    |
 LL |     let _s = s.clone();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:128:15
    |
    |
 LL |     let _t = t.clone();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:128:14
    |
    |
 LL |     let _t = t.clone();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:138:19
    |
    |
 LL |         let _f = f.clone();
    |
 note: this value is dropped without further use
   --> $DIR/redundant_clone.rs:138:18
    |
    |
 LL |         let _f = f.clone();
 
 error: redundant clone
   --> $DIR/redundant_clone.rs:150:14
    |
    |
 LL |     let y = x.clone().join("matthias");
    |
 note: cloned value is neither consumed nor mutated
   --> $DIR/redundant_clone.rs:150:13
    |
    |
 LL |     let y = x.clone().join("matthias");
 
-error: redundant clone
-  --> $DIR/redundant_clone.rs:204:11
-   |
-   |
-LL |     foo(&x.clone(), move || {
-   |
-note: this value is dropped without further use
-  --> $DIR/redundant_clone.rs:204:10
-   |
-   |
-LL |     foo(&x.clone(), move || {
-
-error: aborting due to 15 previous errors
+error: aborting due to 14 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_clone.stage-id.stderr
diff of fixed:

 // run-rustfix
 // rustfix-only-machine-applicable
 #![feature(lint_reasons)]
 #![allow(clippy::drop_non_drop, clippy::implicit_clone, clippy::uninlined_format_args)]
 use std::ffi::OsString;
 use std::path::Path;
 
 fn main() {
 fn main() {
     let _s = ["lorem", "ipsum"].join(" ");
     let s = String::from("foo");
     let _s = s;
 
     let s = String::from("foo");
     let s = String::from("foo");
     let _s = s;
 
     let s = String::from("foo");
     let _s = s;
 
     let _s = Path::new("/a/b/").join("c");
 
     let _s = Path::new("/a/b/").join("c");
     let _s = OsString::new();
 
     let _s = OsString::new();
 
 
     // Check that lint level works
     #[allow(clippy::redundant_clone)]
     let _s = String::new().to_string();
     // Check that lint level works
     // Check that lint level works
     #[expect(clippy::redundant_clone)]
     let _s = String::new().to_string();
 
     let tup = (String::from("foo"),);
     let _t = tup.0;
 
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
     if b { (a.clone(), a) } else { (Alpha, a) }
 
 
 fn cannot_double_move(a: Alpha) -> (Alpha, Alpha) {
     (a.clone(), a)
 
 
 struct TypeWithDrop {
     x: String,
 
 
 impl Drop for TypeWithDrop {
     fn drop(&mut self) {}
 
 
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
 
     let _s = s;
     let _t = t;
 
---
         let _f = f;
     }
 
     {
         let f = Foo { x: 123 };
         let _x = &f.x;
         let _f = f.clone(); // ok
 }
 
 fn not_consumed() {
 fn not_consumed() {
     let x = std::path::PathBuf::from("home");
     let y = x.join("matthias");
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/redundant_clone.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_clone.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_clone.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":264,"byte_end":276,"line_start":10,"line_end":10,"column_start":42,"column_end":54,"is_primary":true,"text":[{"text":"    let _s = [\"lorem\", \"ipsum\"].join(\" \").to_string();","highlight_start":42,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":236,"byte_end":264,"line_start":10,"line_end":10,"column_start":14,"column_end":42,"is_primary":true,"text":[{"text":"    let _s = [\"lorem\", \"ipsum\"].join(\" \").to_string();","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`-D clippy::redundant-clone` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":264,"byte_end":276,"line_start":10,"line_end":10,"column_start":42,"column_end":54,"is_primary":true,"text":[{"text":"    let _s = [\"lorem\", \"ipsum\"].join(\" \").to_string();","highlight_start":42,"highlight_end":54}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:10:42\n   |\nLL |     let _s = [\"lorem\", \"ipsum\"].join(\" \").to_string();\n   |                                          ^^^^^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:10:14\n   |\nLL |     let _s = [\"lorem\", \"ipsum\"].join(\" \").to_string();\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = note: `-D clippy::redundant-clone` implied by `-D warnings`\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":326,"byte_end":334,"line_start":13,"line_end":13,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    let _s = s.clone();","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":325,"byte_end":326,"line_start":13,"line_end":13,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"    let _s = s.clone();","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":326,"byte_end":334,"line_start":13,"line_end":13,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    let _s = s.clone();","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:13:15\n   |\nLL |     let _s = s.clone();\n   |               ^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:13:14\n   |\nLL |     let _s = s.clone();\n   |              ^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":384,"byte_end":396,"line_start":16,"line_end":16,"column_start":15,"column_end":27,"is_primary":true,"text":[{"text":"    let _s = s.to_string();","highlight_start":15,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":383,"byte_end":384,"line_start":16,"line_end":16,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"    let _s = s.to_string();","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":384,"byte_end":396,"line_start":16,"line_end":16,"column_start":15,"column_end":27,"is_primary":true,"text":[{"text":"    let _s = s.to_string();","highlight_start":15,"highlight_end":27}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:16:15\n   |\nLL |     let _s = s.to_string();\n   |               ^^^^^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:16:14\n   |\nLL |     let _s = s.to_string();\n   |              ^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":446,"byte_end":457,"line_start":19,"line_end":19,"column_start":15,"column_end":26,"is_primary":true,"text":[{"text":"    let _s = s.to_owned();","highlight_start":15,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":445,"byte_end":446,"line_start":19,"line_end":19,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"    let _s = s.to_owned();","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":446,"byte_end":457,"line_start":19,"line_end":19,"column_start":15,"column_end":26,"is_primary":true,"text":[{"text":"    let _s = s.to_owned();","highlight_start":15,"highlight_end":26}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:19:15\n   |\nLL |     let _s = s.to_owned();\n   |               ^^^^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:19:14\n   |\nLL |     let _s = s.to_owned();\n   |              ^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":501,"byte_end":512,"line_start":21,"line_end":21,"column_start":42,"column_end":53,"is_primary":true,"text":[{"text":"    let _s = Path::new(\"/a/b/\").join(\"c\").to_owned();","highlight_start":42,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":473,"byte_end":501,"line_start":21,"line_end":21,"column_start":14,"column_end":42,"is_primary":true,"text":[{"text":"    let _s = Path::new(\"/a/b/\").join(\"c\").to_owned();","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":501,"byte_end":512,"line_start":21,"line_end":21,"column_start":42,"column_end":53,"is_primary":true,"text":[{"text":"    let _s = Path::new(\"/a/b/\").join(\"c\").to_owned();","highlight_start":42,"highlight_end":53}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:21:42\n   |\nLL |     let _s = Path::new(\"/a/b/\").join(\"c\").to_owned();\n   |                                          ^^^^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:21:14\n   |\nLL |     let _s = Path::new(\"/a/b/\").join(\"c\").to_owned();\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":556,"byte_end":570,"line_start":23,"line_end":23,"column_start":42,"column_end":56,"is_primary":true,"text":[{"text":"    let _s = Path::new(\"/a/b/\").join(\"c\").to_path_buf();","highlight_start":42,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":528,"byte_end":556,"line_start":23,"line_end":23,"column_start":14,"column_end":42,"is_primary":true,"text":[{"text":"    let _s = Path::new(\"/a/b/\").join(\"c\").to_path_buf();","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":556,"byte_end":570,"line_start":23,"line_end":23,"column_start":42,"column_end":56,"is_primary":true,"text":[{"text":"    let _s = Path::new(\"/a/b/\").join(\"c\").to_path_buf();","highlight_start":42,"highlight_end":56}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:23:42\n   |\nLL |     let _s = Path::new(\"/a/b/\").join(\"c\").to_path_buf();\n   |                                          ^^^^^^^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:23:14\n   |\nLL |     let _s = Path::new(\"/a/b/\").join(\"c\").to_path_buf();\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":601,"byte_end":612,"line_start":25,"line_end":25,"column_start":29,"column_end":40,"is_primary":true,"text":[{"text":"    let _s = OsString::new().to_owned();","highlight_start":29,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":586,"byte_end":601,"line_start":25,"line_end":25,"column_start":14,"column_end":29,"is_primary":true,"text":[{"text":"    let _s = OsString::new().to_owned();","highlight_start":14,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":601,"byte_end":612,"line_start":25,"line_end":25,"column_start":29,"column_end":40,"is_primary":true,"text":[{"text":"    let _s = OsString::new().to_owned();","highlight_start":29,"highlight_end":40}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:25:29\n   |\nLL |     let _s = OsString::new().to_owned();\n   |                             ^^^^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:25:14\n   |\nLL |     let _s = OsString::new().to_owned();\n   |              ^^^^^^^^^^^^^^^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":643,"byte_end":658,"line_start":27,"line_end":27,"column_start":29,"column_end":44,"is_primary":true,"text":[{"text":"    let _s = OsString::new().to_os_string();","highlight_start":29,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":628,"byte_end":643,"line_start":27,"line_end":27,"column_start":14,"column_end":29,"is_primary":true,"text":[{"text":"    let _s = OsString::new().to_os_string();","highlight_start":14,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":643,"byte_end":658,"line_start":27,"line_end":27,"column_start":29,"column_end":44,"is_primary":true,"text":[{"text":"    let _s = OsString::new().to_os_string();","highlight_start":29,"highlight_end":44}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:27:29\n   |\nLL |     let _s = OsString::new().to_os_string();\n   |                             ^^^^^^^^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:27:14\n   |\nLL |     let _s = OsString::new().to_os_string();\n   |              ^^^^^^^^^^^^^^^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":946,"byte_end":954,"line_start":38,"line_end":38,"column_start":19,"column_end":27,"is_primary":true,"text":[{"text":"    let _t = tup.0.clone();","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":941,"byte_end":946,"line_start":38,"line_end":38,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    let _t = tup.0.clone();","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":946,"byte_end":954,"line_start":38,"line_end":38,"column_start":19,"column_end":27,"is_primary":true,"text":[{"text":"    let _t = tup.0.clone();","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:38:19\n   |\nLL |     let _t = tup.0.clone();\n   |                   ^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:38:14\n   |\nLL |     let _t = tup.0.clone();\n   |              ^^^^^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":1716,"byte_end":1724,"line_start":70,"line_end":70,"column_start":25,"column_end":33,"is_primary":true,"text":[{"text":"    if b { (a.clone(), a.clone()) } else { (Alpha, a) }","highlight_start":25,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":1715,"byte_end":1716,"line_start":70,"line_end":70,"column_start":24,"column_end":25,"is_primary":true,"text":[{"text":"    if b { (a.clone(), a.clone()) } else { (Alpha, a) }","highlight_start":24,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":1716,"byte_end":1724,"line_start":70,"line_end":70,"column_start":25,"column_end":33,"is_primary":true,"text":[{"text":"    if b { (a.clone(), a.clone()) } else { (Alpha, a) }","highlight_start":25,"highlight_end":33}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:70:25\n   |\nLL |     if b { (a.clone(), a.clone()) } else { (Alpha, a) }\n   |                         ^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:70:24\n   |\nLL |     if b { (a.clone(), a.clone()) } else { (Alpha, a) }\n   |                        ^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":2833,"byte_end":2841,"line_start":127,"line_end":127,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    let _s = s.clone();","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":2832,"byte_end":2833,"line_start":127,"line_end":127,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"    let _s = s.clone();","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":2833,"byte_end":2841,"line_start":127,"line_end":127,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    let _s = s.clone();","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:127:15\n   |\nLL |     let _s = s.clone();\n   |               ^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:127:14\n   |\nLL |     let _s = s.clone();\n   |              ^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":2857,"byte_end":2865,"line_start":128,"line_end":128,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    let _t = t.clone();","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":2856,"byte_end":2857,"line_start":128,"line_end":128,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"    let _t = t.clone();","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":2857,"byte_end":2865,"line_start":128,"line_end":128,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    let _t = t.clone();","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:128:15\n   |\nLL |     let _t = t.clone();\n   |               ^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:128:14\n   |\nLL |     let _t = t.clone();\n   |              ^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":3015,"byte_end":3023,"line_start":138,"line_end":138,"column_start":19,"column_end":27,"is_primary":true,"text":[{"text":"        let _f = f.clone();","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this value is dropped without further use","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":3014,"byte_end":3015,"line_start":138,"line_end":138,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"        let _f = f.clone();","highlight_start":18,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":3015,"byte_end":3023,"line_start":138,"line_end":138,"column_start":19,"column_end":27,"is_primary":true,"text":[{"text":"        let _f = f.clone();","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:138:19\n   |\nLL |         let _f = f.clone();\n   |                   ^^^^^^^^ help: remove this\n   |\nnote: this value is dropped without further use\n  --> tests/ui/redundant_clone.rs:138:18\n   |\nLL |         let _f = f.clone();\n   |                  ^\n\n"}
{"message":"redundant clone","code":{"code":"clippy::redundant_clone","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":3215,"byte_end":3223,"line_start":150,"line_end":150,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    let y = x.clone().join(\"matthias\");","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"cloned value is neither consumed nor mutated","code":null,"level":"note","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":3214,"byte_end":3223,"line_start":150,"line_end":150,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"    let y = x.clone().join(\"matthias\");","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/redundant_clone.rs","byte_start":3215,"byte_end":3223,"line_start":150,"line_end":150,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    let y = x.clone().join(\"matthias\");","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: redundant clone\n  --> tests/ui/redundant_clone.rs:150:14\n   |\nLL |     let y = x.clone().join(\"matthias\");\n   |              ^^^^^^^^ help: remove this\n   |\nnote: cloned value is neither consumed nor mutated\n  --> tests/ui/redundant_clone.rs:150:13\n   |\nLL |     let y = x.clone().join(\"matthias\");\n   |             ^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.9.0/src/lib.rs:111:22
