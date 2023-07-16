plain
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:27:5
    |
 LL |     return true;
    |     ^^^^^^^^^^^^ help: remove `return`: `true`
    |
    = note: `-D clippy::needless-return` implied by `-D warnings`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:31:5
    |
 LL |     return true;
 LL |     return true;
    |     ^^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:36:9
    |
 LL |         return true;
 LL |         return true;
    |         ^^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
-  --> $DIR/needless_return.rs:38:9
-   |
-LL |         return false;
-LL |         return false;
-   |         ^^^^^^^^^^^^^ help: remove `return`: `false`
-error: unneeded `return` statement
   --> $DIR/needless_return.rs:44:17
    |
 LL |         true => return false,
 LL |         true => return false,
    |                 ^^^^^^^^^^^^ help: remove `return`: `false`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:46:13
    |
 LL |             return true;
 LL |             return true;
    |             ^^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:53:9
    |
 LL |         return true;
 LL |         return true;
    |         ^^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:55:16
    |
 LL |     let _ = || return true;
 LL |     let _ = || return true;
    |                ^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:59:5
    |
    |
 LL |     return the_answer!();
    |     ^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `the_answer!()`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:63:5
    |
 LL |     return;
---
-
-error: unneeded `return` statement
   --> $DIR/needless_return.rs:77:14
    |
 LL |         _ => return,
    |              ^^^^^^ help: replace `return` with a unit value: `()`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:86:13
    |
 LL |             return;
 LL |             return;
error: test failed, to rerun pass '--test compile-test'
    |             ^^^^^^^ help: remove `return`
 
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:88:14
    |
 LL |         _ => return,
    |              ^^^^^^ help: replace `return` with a unit value: `()`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:101:9
    |
    |
 LL |         return String::from("test");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `String::from("test")`
 error: unneeded `return` statement
-  --> $DIR/needless_return.rs:103:9
-   |
-LL |         return String::new();
-LL |         return String::new();
-   |         ^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `String::new()`
-error: unneeded `return` statement
   --> $DIR/needless_return.rs:125:32
    |
    |
 LL |         bar.unwrap_or_else(|_| return)
    |                                ^^^^^^ help: replace `return` with an empty block: `{}`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:130:13
    |
 LL |             return;
 LL |             return;
    |             ^^^^^^^ help: remove `return`
 
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:132:20
    |
 LL |         let _ = || return;
    |                    ^^^^^^ help: replace `return` with an empty block: `{}`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:138:32
    |
    |
 LL |         res.unwrap_or_else(|_| return Foo)
    |                                ^^^^^^^^^^ help: remove `return`: `Foo`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:147:5
    |
 LL |     return true;
 LL |     return true;
    |     ^^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:151:5
    |
 LL |     return true;
 LL |     return true;
    |     ^^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:156:9
    |
 LL |         return true;
 LL |         return true;
    |         ^^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
-  --> $DIR/needless_return.rs:158:9
-   |
-LL |         return false;
-LL |         return false;
-   |         ^^^^^^^^^^^^^ help: remove `return`: `false`
-error: unneeded `return` statement
   --> $DIR/needless_return.rs:164:17
    |
 LL |         true => return false,
 LL |         true => return false,
    |                 ^^^^^^^^^^^^ help: remove `return`: `false`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:166:13
    |
 LL |             return true;
 LL |             return true;
    |             ^^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:173:9
    |
 LL |         return true;
 LL |         return true;
    |         ^^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:175:16
    |
 LL |     let _ = || return true;
 LL |     let _ = || return true;
    |                ^^^^^^^^^^^ help: remove `return`: `true`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:179:5
    |
    |
 LL |     return the_answer!();
    |     ^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `the_answer!()`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:183:5
    |
 LL |     return;
---
-
-error: unneeded `return` statement
   --> $DIR/needless_return.rs:197:14
    |
 LL |         _ => return,
    |              ^^^^^^ help: replace `return` with a unit value: `()`
 error: unneeded `return` statement
   --> $DIR/needless_return.rs:210:9
    |
    |
 LL |         return String::from("test");
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `String::from("test")`
 error: unneeded `return` statement
-  --> $DIR/needless_return.rs:212:9
-   |
-LL |         return String::new();
-LL |         return String::new();
-   |         ^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `String::new()`
-error: unneeded `return` statement
   --> $DIR/needless_return.rs:228:5
    |
 LL |     return format!("Hello {}", "world!");
 LL |     return format!("Hello {}", "world!");
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `format!("Hello {}", "world!")`
-error: aborting due to 37 previous errors
+error: aborting due to 31 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_return.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![feature(lint_reasons)]
 #![feature(let_else)]
 #![allow(
 #![allow(
     clippy::if_same_then_else,
     clippy::single_match,
     clippy::needless_bool,
     clippy::equatable_if_let
 )]
 #![warn(clippy::needless_return)]
 use std::cell::RefCell;
 
 
 macro_rules! the_answer {
         42
     };
 }
 
---
+        return false;
     }
 }
 
 fn test_match(x: bool) -> bool {
     match x {
         true => false,
         false => {
         },
     }
 }
 
 
 fn test_closure() {
     let _ = || {
         true
     };
     let _ = || true;
 }
 
 fn test_macro_call() -> i32 {
     the_answer!()
 
 
 fn test_void_fun() {
 }
 
 
 fn test_void_if_fun(b: bool) {
         
     } else {
-        
+        return;
+        return;
     }
 }
 
 fn test_void_match(x: u32) {
     match x {
         0 => (),
         _ => (),
 }
 
 
 fn test_nested_match(x: u32) {
     match x {
         0 => (),
             let _ = 42;
             
         },
         _ => (),
         _ => (),
     }
 }
 
 fn temporary_outlives_local() -> String {
     let x = RefCell::<String>::default();
     return x.borrow().clone();
 
 
 fn borrows_but_not_last(value: bool) -> String {
     if value {
         let x = RefCell::<String>::default();
         let _a = x.borrow().clone();
         String::from("test")
-        String::new()
+        return String::new();
     }
 }
 }
 
 macro_rules! needed_return {
     ($e:expr) => {
         if $e > 3 {
         }
     };
 }
 
 
 fn test_return_in_macro() {
     // This will return and the macro below won't be executed. Removing the `return` from the macro
     // will change semantics.
     needed_return!(10);
     needed_return!(0);
 
 mod issue6501 {
 mod issue6501 {
     #[allow(clippy::unnecessary_lazy_evaluations)]
     fn foo(bar: Result<(), ()>) {
         bar.unwrap_or_else(|_| {})
 
     fn test_closure() {
         let _ = || {
             
             
         };
         let _ = || {};
 
     struct Foo;
     struct Foo;
     #[allow(clippy::unnecessary_lazy_evaluations)]
     fn bar(res: Result<Foo, u8>) -> Foo {
         res.unwrap_or_else(|_| Foo)
 }
 
 
 async fn async_test_end_of_fn() -> bool {
         // no error!
         return true;
     }
     true
---
+        return false;
     }
 }
 
 async fn async_test_match(x: bool) -> bool {
     match x {
         true => false,
         false => {
         },
     }
 }
 
 
 async fn async_test_closure() {
     let _ = || {
         true
     };
     let _ = || true;
 }
 
 async fn async_test_macro_call() -> i32 {
     the_answer!()
 
 
 async fn async_test_void_fun() {
 }
 
 
 async fn async_test_void_if_fun(b: bool) {
         
     } else {
-        
+        return;
+        return;
     }
 }
 
 async fn async_test_void_match(x: u32) {
     match x {
         0 => (),
         _ => (),
 }
 
 
 async fn async_temporary_outlives_local() -> String {
     let x = RefCell::<String>::default();
     return x.borrow().clone();
 
 
 async fn async_borrows_but_not_last(value: bool) -> String {
     if value {
         let x = RefCell::<String>::default();
         let _a = x.borrow().clone();
         String::from("test")
-        String::new()
+        return String::new();
     }
 }
 }
 
 async fn async_test_return_in_macro() {
     needed_return!(10);
     needed_return!(0);
 
 fn let_else() {
 fn let_else() {
     let Some(1) = Some(1) else { return };
 
 
 fn needless_return_macro() -> String {
     let _ = "foo";
     let _ = "bar";
     format!("Hello {}", "world!")
 
 fn check_expect() -> bool {
     if true {
         // no error!
         // no error!
         return true;
     }
     #[expect(clippy::needless_return)]
 }
 
 fn main() {}
 
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_return.stage-id.fixed
To only update this specific test, also pass `--test-args needless_return.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/needless_return.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_return.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-e10463a0415f4331.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-c9628d561c54b58c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-528e210b0f79da7e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_return.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":423,"byte_end":435,"line_start":27,"line_end":27,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    return true;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-return` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":423,"byte_end":435,"line_start":27,"line_end":27,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    return true;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:27:5\n   |\nLL |     return true;\n   |     ^^^^^^^^^^^^ help: remove `return`: `true`\n   |\n   = note: `-D clippy::needless-return` implied by `-D warnings`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":476,"byte_end":488,"line_start":31,"line_end":31,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    return true;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":476,"byte_end":488,"line_start":31,"line_end":31,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    return true;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:31:5\n   |\nLL |     return true;\n   |     ^^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":543,"byte_end":555,"line_start":36,"line_end":36,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        return true;","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":543,"byte_end":555,"line_start":36,"line_end":36,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        return true;","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:36:9\n   |\nLL |         return true;\n   |         ^^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":663,"byte_end":675,"line_start":44,"line_end":44,"column_start":17,"column_end":29,"is_primary":true,"text":[{"text":"        true => return false,","highlight_start":17,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":663,"byte_end":675,"line_start":44,"line_end":44,"column_start":17,"column_end":29,"is_primary":true,"text":[{"text":"        true => return false,","highlight_start":17,"highlight_end":29}],"label":null,"suggested_replacement":"false","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:44:17\n   |\nLL |         true => return false,\n   |                 ^^^^^^^^^^^^ help: remove `return`: `false`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":708,"byte_end":720,"line_start":46,"line_end":46,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"            return true;","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":708,"byte_end":720,"line_start":46,"line_end":46,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"            return true;","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:46:13\n   |\nLL |             return true;\n   |             ^^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":786,"byte_end":798,"line_start":53,"line_end":53,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        return true;","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":786,"byte_end":798,"line_start":53,"line_end":53,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        return true;","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:53:9\n   |\nLL |         return true;\n   |         ^^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":821,"byte_end":832,"line_start":55,"line_end":55,"column_start":16,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = || return true;","highlight_start":16,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":821,"byte_end":832,"line_start":55,"line_end":55,"column_start":16,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = || return true;","highlight_start":16,"highlight_end":27}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:55:16\n   |\nLL |     let _ = || return true;\n   |                ^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":871,"byte_end":892,"line_start":59,"line_end":59,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    return the_answer!();","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":871,"byte_end":892,"line_start":59,"line_end":59,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    return the_answer!();","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":"the_answer!()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:59:5\n   |\nLL |     return the_answer!();\n   |     ^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `the_answer!()`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":921,"byte_end":928,"line_start":63,"line_end":63,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    return;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":921,"byte_end":928,"line_start":63,"line_end":63,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    return;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:63:5\n   |\nLL |     return;\n   |     ^^^^^^^ help: remove `return`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":982,"byte_end":989,"line_start":68,"line_end":68,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        return;","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":982,"byte_end":989,"line_start":68,"line_end":68,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        return;","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:68:9\n   |\nLL |         return;\n   |         ^^^^^^^ help: remove `return`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":1101,"byte_end":1107,"line_start":77,"line_end":77,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"        _ => return,","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"replace `return` with a unit value","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":1101,"byte_end":1107,"line_start":77,"line_end":77,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"        _ => return,","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":"()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:77:14\n   |\nLL |         _ => return,\n   |              ^^^^^^ help: replace `return` with a unit value: `()`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":1231,"byte_end":1238,"line_start":86,"line_end":86,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            return;","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":1231,"byte_end":1238,"line_start":86,"line_end":86,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            return;","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:86:13\n   |\nLL |             return;\n   |             ^^^^^^^ help: remove `return`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":1263,"byte_end":1269,"line_start":88,"line_end":88,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"        _ => return,","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"replace `return` with a unit value","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":1263,"byte_end":1269,"line_start":88,"line_end":88,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"        _ => return,","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":"()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:88:14\n   |\nLL |         _ => return,\n   |              ^^^^^^ help: replace `return` with a unit value: `()`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":1553,"byte_end":1581,"line_start":101,"line_end":101,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"        return String::from(\"test\");","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":1553,"byte_end":1581,"line_start":101,"line_end":101,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"        return String::from(\"test\");","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"String::from(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:101:9\n   |\nLL |         return String::from(\"test\");\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `String::from(\"test\")`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2082,"byte_end":2088,"line_start":125,"line_end":125,"column_start":32,"column_end":38,"is_primary":true,"text":[{"text":"        bar.unwrap_or_else(|_| return)","highlight_start":32,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"replace `return` with an empty block","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2082,"byte_end":2088,"line_start":125,"line_end":125,"column_start":32,"column_end":38,"is_primary":true,"text":[{"text":"        bar.unwrap_or_else(|_| return)","highlight_start":32,"highlight_end":38}],"label":null,"suggested_replacement":"{}","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:125:32\n   |\nLL |         bar.unwrap_or_else(|_| return)\n   |                                ^^^^^^ help: replace `return` with an empty block: `{}`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2154,"byte_end":2161,"line_start":130,"line_end":130,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            return;","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2154,"byte_end":2161,"line_start":130,"line_end":130,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            return;","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:130:13\n   |\nLL |             return;\n   |             ^^^^^^^ help: remove `return`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2192,"byte_end":2198,"line_start":132,"line_end":132,"column_start":20,"column_end":26,"is_primary":true,"text":[{"text":"        let _ = || return;","highlight_start":20,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"replace `return` with an empty block","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2192,"byte_end":2198,"line_start":132,"line_end":132,"column_start":20,"column_end":26,"is_primary":true,"text":[{"text":"        let _ = || return;","highlight_start":20,"highlight_end":26}],"label":null,"suggested_replacement":"{}","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:132:20\n   |\nLL |         let _ = || return;\n   |                    ^^^^^^ help: replace `return` with an empty block: `{}`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2347,"byte_end":2357,"line_start":138,"line_end":138,"column_start":32,"column_end":42,"is_primary":true,"text":[{"text":"        res.unwrap_or_else(|_| return Foo)","highlight_start":32,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2347,"byte_end":2357,"line_start":138,"line_end":138,"column_start":32,"column_end":42,"is_primary":true,"text":[{"text":"        res.unwrap_or_else(|_| return Foo)","highlight_start":32,"highlight_end":42}],"label":null,"suggested_replacement":"Foo","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:138:32\n   |\nLL |         res.unwrap_or_else(|_| return Foo)\n   |                                ^^^^^^^^^^ help: remove `return`: `Foo`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2476,"byte_end":2488,"line_start":147,"line_end":147,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    return true;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2476,"byte_end":2488,"line_start":147,"line_end":147,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    return true;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:147:5\n   |\nLL |     return true;\n   |     ^^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2541,"byte_end":2553,"line_start":151,"line_end":151,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    return true;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2541,"byte_end":2553,"line_start":151,"line_end":151,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    return true;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:151:5\n   |\nLL |     return true;\n   |     ^^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2620,"byte_end":2632,"line_start":156,"line_end":156,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        return true;","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2620,"byte_end":2632,"line_start":156,"line_end":156,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        return true;","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:156:9\n   |\nLL |         return true;\n   |         ^^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2752,"byte_end":2764,"line_start":164,"line_end":164,"column_start":17,"column_end":29,"is_primary":true,"text":[{"text":"        true => return false,","highlight_start":17,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2752,"byte_end":2764,"line_start":164,"line_end":164,"column_start":17,"column_end":29,"is_primary":true,"text":[{"text":"        true => return false,","highlight_start":17,"highlight_end":29}],"label":null,"suggested_replacement":"false","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:164:17\n   |\nLL |         true => return false,\n   |                 ^^^^^^^^^^^^ help: remove `return`: `false`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2797,"byte_end":2809,"line_start":166,"line_end":166,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"            return true;","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2797,"byte_end":2809,"line_start":166,"line_end":166,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"            return true;","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:166:13\n   |\nLL |             return true;\n   |             ^^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2887,"byte_end":2899,"line_start":173,"line_end":173,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        return true;","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2887,"byte_end":2899,"line_start":173,"line_end":173,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        return true;","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:173:9\n   |\nLL |         return true;\n   |         ^^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2922,"byte_end":2933,"line_start":175,"line_end":175,"column_start":16,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = || return true;","highlight_start":16,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2922,"byte_end":2933,"line_start":175,"line_end":175,"column_start":16,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = || return true;","highlight_start":16,"highlight_end":27}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:175:16\n   |\nLL |     let _ = || return true;\n   |                ^^^^^^^^^^^ help: remove `return`: `true`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2984,"byte_end":3005,"line_start":179,"line_end":179,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    return the_answer!();","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":2984,"byte_end":3005,"line_start":179,"line_end":179,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    return the_answer!();","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":"the_answer!()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:179:5\n   |\nLL |     return the_answer!();\n   |     ^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `the_answer!()`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3046,"byte_end":3053,"line_start":183,"line_end":183,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    return;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3046,"byte_end":3053,"line_start":183,"line_end":183,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    return;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:183:5\n   |\nLL |     return;\n   |     ^^^^^^^ help: remove `return`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3119,"byte_end":3126,"line_start":188,"line_end":188,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        return;","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3119,"byte_end":3126,"line_start":188,"line_end":188,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        return;","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:188:9\n   |\nLL |         return;\n   |         ^^^^^^^ help: remove `return`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3250,"byte_end":3256,"line_start":197,"line_end":197,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"        _ => return,","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"replace `return` with a unit value","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3250,"byte_end":3256,"line_start":197,"line_end":197,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"        _ => return,","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":"()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:197:14\n   |\nLL |         _ => return,\n   |              ^^^^^^ help: replace `return` with a unit value: `()`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3564,"byte_end":3592,"line_start":210,"line_end":210,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"        return String::from(\"test\");","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3564,"byte_end":3592,"line_start":210,"line_end":210,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"        return String::from(\"test\");","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"String::from(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:210:9\n   |\nLL |         return String::from(\"test\");\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `String::from(\"test\")`\n\n"}
{"message":"unneeded `return` statement","code":{"code":"clippy::needless_return","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3878,"byte_end":3915,"line_start":228,"line_end":228,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    return format!(\"Hello {}\", \"world!\");","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove `return`","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_return.rs","byte_start":3878,"byte_end":3915,"line_start":228,"line_end":228,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    return format!(\"Hello {}\", \"world!\");","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":"format!(\"Hello {}\", \"world!\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `return` statement\n  --> tests/ui/needless_return.rs:228:5\n   |\nLL |     return format!(\"Hello {}\", \"world!\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove `return`: `format!(\"Hello {}\", \"world!\")`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
