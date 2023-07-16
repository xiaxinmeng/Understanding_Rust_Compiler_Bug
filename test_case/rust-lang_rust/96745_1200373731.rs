plain

---- compile_test stdout ----
diff of stderr:

 error: deref which would be done by auto-deref
    |
    |
 LL |     let _: &str = &*s;
    |                    ^^ help: try this: `s`
    |
    = note: `-D clippy::explicit-auto-deref` implied by `-D warnings`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |     f_str(&*s);
    |            ^^ help: try this: `s`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |     f_str_t(&*s, &*s); // Don't lint second param.
    |              ^^ help: try this: `s`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |     let _: &Box<i32> = &**b;
    |                         ^^^ help: try this: `b`
 
 error: deref which would be done by auto-deref
    |
 LL |     c(&*s);
    |        ^^ help: try this: `s`
 
 
 error: deref which would be done by auto-deref
    |
 LL |         &**x
 LL |         &**x
    |         ^^^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         { &**x }
    |           ^^^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         &**{ x }
    |         ^^^^^^^^ help: try this: `{ x }`
 
 error: deref which would be done by auto-deref
    |
 LL |         &***x
 LL |         &***x
    |         ^^^^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f1(&*x);
    |             ^^ help: try this: `x`
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
 error: deref which would be done by auto-deref
    |
    |
 LL |         f2(&*x);
    |             ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f3(&*x);
    |             ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f4.callable_str()(&*x);
    |                            ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f5(&*x);
    |             ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f6(&*x);
    |             ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f7.callable_str()(&*x);
    |                            ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f8.callable_t()(&*x);
    |                          ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f9(&*x);
    |             ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f10(&*x);
    |              ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |         f11.callable_t()(&*x);
    |                           ^^ help: try this: `x`
 
 error: deref which would be done by auto-deref
    |
 LL |     let _ = S1(&*s);
    |                 ^^ help: try this: `s`
 
 
 error: deref which would be done by auto-deref
-   |
-   |
-LL |     let _ = S2 { s: &*s };
-   |                      ^^ help: try this: `s`
-
-error: deref which would be done by auto-deref
    |
    |
 LL |             let _ = Self::S1(&**s);
    |                              ^^^^ help: try this: `s`
 
 error: deref which would be done by auto-deref
-   |
-   |
-LL |             let _ = Self::S2 { s: &**s };
-   |                                   ^^^^ help: try this: `s`
-
-error: deref which would be done by auto-deref
    |
    |
 LL |     let _ = E1::S1(&*s);
    |                     ^^ help: try this: `s`
 
 error: deref which would be done by auto-deref
-   |
-   |
-LL |     let _ = E1::S2 { s: &*s };
-   |                          ^^ help: try this: `s`
-
-error: deref which would be done by auto-deref
    |
    |
 LL |     let _ = (*b).foo;
    |             ^^^^ help: try this: `b`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |     let _ = (**b).foo;
    |             ^^^^^ help: try this: `b`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |     let _ = f_str(*ref_str);
    |                   ^^^^^^^^ help: try this: `ref_str`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |     let _ = f_str(**ref_ref_str);
    |                   ^^^^^^^^^^^^^ help: try this: `ref_ref_str`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |     f_str(&&*ref_str); // `needless_borrow` will suggest removing both references
    |             ^^^^^^^^ help: try this: `ref_str`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |     f_str(&&**ref_str); // `needless_borrow` will suggest removing only one reference
    |            ^^^^^^^^^^ help: try this: `ref_str`
 
 error: deref which would be done by auto-deref
    |
    |
 LL |     let _ = || -> &'static str { return *s };
    |                                         ^^ help: try this: `s`
-error: aborting due to 33 previous errors
+error: aborting due to 30 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_auto_deref.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![warn(clippy::explicit_auto_deref)]
     dead_code,
     unused_braces,
     clippy::borrowed_box,
     clippy::needless_borrow,
     clippy::needless_borrow,
     clippy::needless_return,
     clippy::ptr_arg,
     clippy::redundant_field_names,
     clippy::too_many_arguments,
     clippy::borrow_deref_ref,
     clippy::let_unit_value
 
 trait CallableStr {
 trait CallableStr {
     type T: Fn(&str);
     fn callable_str(&self) -> Self::T;
 }
 impl CallableStr for () {
     type T = fn(&str);
     fn callable_str(&self) -> Self::T {
         fn f(_: &str) {}
     }
 }
 impl CallableStr for i32 {
 impl CallableStr for i32 {
     type T = <() as CallableStr>::T;
     fn callable_str(&self) -> Self::T {
         ().callable_str()
 }
 
 
 trait CallableT<U: ?Sized> {
     type T: Fn(&U);
     fn callable_t(&self) -> Self::T;
 }
 impl<U: ?Sized> CallableT<U> for () {
     type T = fn(&U);
     fn callable_t(&self) -> Self::T {
         fn f<U: ?Sized>(_: &U) {}
         f::<U>
 }
 }
 impl<U: ?Sized> CallableT<U> for i32 {
     type T = <() as CallableT<U>>::T;
     fn callable_t(&self) -> Self::T {
         ().callable_t()
 }
 
 
 fn f_str(_: &str) {}
 fn f_string(_: &String) {}
 fn f_t<T>(_: T) {}
 fn f_ref_t<T: ?Sized>(_: &T) {}
 
 fn f_str_t<T>(_: &str, _: T) {}
 
 fn f_box_t<T>(_: &Box<T>) {}
 extern "C" {
 extern "C" {
     fn var(_: u32, ...);
 
 fn main() {
     let s = String::new();
 
 
     let _: &str = &s;
     let _ = &*s; // Don't lint. Inferred type would change.
     let _: &_ = &*s; // Don't lint. Inferred type would change.
 
     f_str(&s);
     f_t(&*s); // Don't lint. Inferred type would change.
     f_ref_t(&*s); // Don't lint. Inferred type would change.
 
     f_str_t(&s, &*s); // Don't lint second param.
 
     let b = Box::new(Box::new(Box::new(5)));
     let _: &Box<i32> = &b;
     let _: &Box<_> = &**b; // Don't lint. Inferred type would change.
 
     f_box_t(&**b); // Don't lint. Inferred type would change.
 
     let c = |_x: &str| ();
     c(&s);
 
     let c = |_x| ();
     c(&*s); // Don't lint. Inferred type would change.
 
     fn _f(x: &String) -> &str {
     }
 
 
     fn _f1(x: &String) -> &str {
         { x }
 
 
     fn _f2(x: &String) -> &str {
         { x }
 
 
     fn _f3(x: &Box<Box<Box<i32>>>) -> &Box<i32> {
     }
 
     fn _f4(
         x: String,
         x: String,
         f1: impl Fn(&str),
         f2: &dyn Fn(&str),
         f3: fn(&str),
         f4: impl CallableStr,
         f5: <() as CallableStr>::T,
         f6: <i32 as CallableStr>::T,
         f7: &dyn CallableStr<T = fn(&str)>,
         f8: impl CallableT<str>,
         f9: <() as CallableT<str>>::T,
         f10: <i32 as CallableT<str>>::T,
         f11: &dyn CallableT<str, T = fn(&str)>,
     ) {
         f1(&x);
         f2(&x);
         f3(&x);
         f4.callable_str()(&x);
         f5(&x);
         f6(&x);
         f7.callable_str()(&x);
         f8.callable_t()(&x);
         f9(&x);
         f10(&x);
         f11.callable_t()(&x);
 
 
     struct S1<'a>(&'a str);
     let _ = S1(&s);
     struct S2<'a> {
         s: &'a str,
     }
     }
-    let _ = S2 { s: &s };
+    let _ = S2 { s: &*s };
 
     struct S3<'a, T: ?Sized>(&'a T);
     let _ = S3(&*s); // Don't lint. Inferred type would change.
 
     struct S4<'a, T: ?Sized> {
         s: &'a T,
     }
     let _ = S4 { s: &*s }; // Don't lint. Inferred type would change.
     enum E1<'a> {
     enum E1<'a> {
         S1(&'a str),
         S2 { s: &'a str },
     }
     impl<'a> E1<'a> {
         fn m1(s: &'a String) {
             let _ = Self::S1(s);
-            let _ = Self::S2 { s: s };
+            let _ = Self::S2 { s: &**s };
     }
     }
     let _ = E1::S1(&s);
-    let _ = E1::S2 { s: &s };
+    let _ = E1::S2 { s: &*s };
 
     enum E2<'a, T: ?Sized> {
         S1(&'a T),
         S2 { s: &'a T },
     }
     let _ = E2::S1(&*s); // Don't lint. Inferred type would change.
     let _ = E2::S2 { s: &*s }; // Don't lint. Inferred type would change.
     let ref_s = &s;
     let ref_s = &s;
     let _: &String = &*ref_s; // Don't lint reborrow.
     f_string(&*ref_s); // Don't lint reborrow.
     struct S5 {
         foo: u32,
     }
     }
     let b = Box::new(Box::new(S5 { foo: 5 }));
     let _ = b.foo;
     let _ = b.foo;
     let _ = b.foo;
     struct S6 {
         foo: S5,
     }
     impl core::ops::Deref for S6 {
     impl core::ops::Deref for S6 {
         type Target = S5;
         fn deref(&self) -> &Self::Target {
             &self.foo
     }
     }
     let s6 = S6 { foo: S5 { foo: 5 } };
     let _ = (*s6).foo; // Don't lint. `S6` also has a field named `foo`
 
     let ref_str = &"foo";
     let _ = f_str(ref_str);
     let ref_ref_str = &ref_str;
     let _ = f_str(ref_ref_str);
 
     fn _f5(x: &u32) -> u32 {
             *x
         } else {
             return *x;
         }
         }
     }
 
     f_str(&&ref_str); // `needless_borrow` will suggest removing both references
     f_str(&ref_str); // `needless_borrow` will suggest removing only one reference
     let x = &&40;
     unsafe {
     unsafe {
         var(0, &**x);
 
 
     let s = &"str";
     let _ = || return *s;
     let _ = || -> &'static str { return s };
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_auto_deref.stage-id.fixed
To only update this specific test, also pass `--test-args explicit_auto_deref.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/explicit_auto_deref.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_auto_deref.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0e96f2e9d30bd37.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-b92911696ae4394a.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-36709515b9cb16b6.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-ead25036d552bfe8.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-aceff80e643e9fe7.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-507b29393c1a728f.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_auto_deref.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1338,"byte_end":1340,"line_start":69,"line_end":69,"column_start":20,"column_end":22,"is_primary":true,"text":[{"text":"    let _: &str = &*s;","highlight_start":20,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::explicit-auto-deref` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1338,"byte_end":1340,"line_start":69,"line_end":69,"column_start":20,"column_end":22,"is_primary":true,"text":[{"text":"    let _: &str = &*s;","highlight_start":20,"highlight_end":22}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:69:20\n   |\nLL |     let _: &str = &*s;\n   |                    ^^ help: try this: `s`\n   |\n   = note: `-D clippy::explicit-auto-deref` implied by `-D warnings`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1478,"byte_end":1480,"line_start":73,"line_end":73,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    f_str(&*s);","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1478,"byte_end":1480,"line_start":73,"line_end":73,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    f_str(&*s);","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:73:12\n   |\nLL |     f_str(&*s);\n   |            ^^ help: try this: `s`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1615,"byte_end":1617,"line_start":77,"line_end":77,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"    f_str_t(&*s, &*s); // Don't lint second param.","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1615,"byte_end":1617,"line_start":77,"line_end":77,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"    f_str_t(&*s, &*s); // Don't lint second param.","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:77:14\n   |\nLL |     f_str_t(&*s, &*s); // Don't lint second param.\n   |              ^^ help: try this: `s`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1723,"byte_end":1726,"line_start":80,"line_end":80,"column_start":25,"column_end":28,"is_primary":true,"text":[{"text":"    let _: &Box<i32> = &**b;","highlight_start":25,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1723,"byte_end":1726,"line_start":80,"line_end":80,"column_start":25,"column_end":28,"is_primary":true,"text":[{"text":"    let _: &Box<i32> = &**b;","highlight_start":25,"highlight_end":28}],"label":null,"suggested_replacement":"b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:80:25\n   |\nLL |     let _: &Box<i32> = &**b;\n   |                         ^^^ help: try this: `b`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1896,"byte_end":1898,"line_start":86,"line_end":86,"column_start":8,"column_end":10,"is_primary":true,"text":[{"text":"    c(&*s);","highlight_start":8,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":1896,"byte_end":1898,"line_start":86,"line_end":86,"column_start":8,"column_end":10,"is_primary":true,"text":[{"text":"    c(&*s);","highlight_start":8,"highlight_end":10}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:86:8\n   |\nLL |     c(&*s);\n   |        ^^ help: try this: `s`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2019,"byte_end":2023,"line_start":92,"line_end":92,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        &**x","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2019,"byte_end":2023,"line_start":92,"line_end":92,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        &**x","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:92:9\n   |\nLL |         &**x\n   |         ^^^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2074,"byte_end":2078,"line_start":96,"line_end":96,"column_start":11,"column_end":15,"is_primary":true,"text":[{"text":"        { &**x }","highlight_start":11,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2074,"byte_end":2078,"line_start":96,"line_end":96,"column_start":11,"column_end":15,"is_primary":true,"text":[{"text":"        { &**x }","highlight_start":11,"highlight_end":15}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:96:11\n   |\nLL |         { &**x }\n   |           ^^^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2129,"byte_end":2137,"line_start":100,"line_end":100,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        &**{ x }","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2129,"byte_end":2137,"line_start":100,"line_end":100,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        &**{ x }","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":"{ x }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:100:9\n   |\nLL |         &**{ x }\n   |         ^^^^^^^^ help: try this: `{ x }`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2203,"byte_end":2208,"line_start":104,"line_end":104,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        &***x","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2203,"byte_end":2208,"line_start":104,"line_end":104,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        &***x","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:104:9\n   |\nLL |         &***x\n   |         ^^^^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2651,"byte_end":2653,"line_start":121,"line_end":121,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f1(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2651,"byte_end":2653,"line_start":121,"line_end":121,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f1(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:121:13\n   |\nLL |         f1(&*x);\n   |             ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2668,"byte_end":2670,"line_start":122,"line_end":122,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f2(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2668,"byte_end":2670,"line_start":122,"line_end":122,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f2(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:122:13\n   |\nLL |         f2(&*x);\n   |             ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2685,"byte_end":2687,"line_start":123,"line_end":123,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f3(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2685,"byte_end":2687,"line_start":123,"line_end":123,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f3(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:123:13\n   |\nLL |         f3(&*x);\n   |             ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2717,"byte_end":2719,"line_start":124,"line_end":124,"column_start":28,"column_end":30,"is_primary":true,"text":[{"text":"        f4.callable_str()(&*x);","highlight_start":28,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2717,"byte_end":2719,"line_start":124,"line_end":124,"column_start":28,"column_end":30,"is_primary":true,"text":[{"text":"        f4.callable_str()(&*x);","highlight_start":28,"highlight_end":30}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:124:28\n   |\nLL |         f4.callable_str()(&*x);\n   |                            ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2734,"byte_end":2736,"line_start":125,"line_end":125,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f5(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2734,"byte_end":2736,"line_start":125,"line_end":125,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f5(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:125:13\n   |\nLL |         f5(&*x);\n   |             ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2751,"byte_end":2753,"line_start":126,"line_end":126,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f6(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2751,"byte_end":2753,"line_start":126,"line_end":126,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f6(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:126:13\n   |\nLL |         f6(&*x);\n   |             ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2783,"byte_end":2785,"line_start":127,"line_end":127,"column_start":28,"column_end":30,"is_primary":true,"text":[{"text":"        f7.callable_str()(&*x);","highlight_start":28,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2783,"byte_end":2785,"line_start":127,"line_end":127,"column_start":28,"column_end":30,"is_primary":true,"text":[{"text":"        f7.callable_str()(&*x);","highlight_start":28,"highlight_end":30}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:127:28\n   |\nLL |         f7.callable_str()(&*x);\n   |                            ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2813,"byte_end":2815,"line_start":128,"line_end":128,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"        f8.callable_t()(&*x);","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2813,"byte_end":2815,"line_start":128,"line_end":128,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"        f8.callable_t()(&*x);","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:128:26\n   |\nLL |         f8.callable_t()(&*x);\n   |                          ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2830,"byte_end":2832,"line_start":129,"line_end":129,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f9(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2830,"byte_end":2832,"line_start":129,"line_end":129,"column_start":13,"column_end":15,"is_primary":true,"text":[{"text":"        f9(&*x);","highlight_start":13,"highlight_end":15}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:129:13\n   |\nLL |         f9(&*x);\n   |             ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2848,"byte_end":2850,"line_start":130,"line_end":130,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        f10(&*x);","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2848,"byte_end":2850,"line_start":130,"line_end":130,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        f10(&*x);","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:130:14\n   |\nLL |         f10(&*x);\n   |              ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2879,"byte_end":2881,"line_start":131,"line_end":131,"column_start":27,"column_end":29,"is_primary":true,"text":[{"text":"        f11.callable_t()(&*x);","highlight_start":27,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2879,"byte_end":2881,"line_start":131,"line_end":131,"column_start":27,"column_end":29,"is_primary":true,"text":[{"text":"        f11.callable_t()(&*x);","highlight_start":27,"highlight_end":29}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:131:27\n   |\nLL |         f11.callable_t()(&*x);\n   |                           ^^ help: try this: `x`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2935,"byte_end":2937,"line_start":135,"line_end":135,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = S1(&*s);","highlight_start":17,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":2935,"byte_end":2937,"line_start":135,"line_end":135,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = S1(&*s);","highlight_start":17,"highlight_end":19}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:135:17\n   |\nLL |     let _ = S1(&*s);\n   |                 ^^ help: try this: `s`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":3397,"byte_end":3401,"line_start":156,"line_end":156,"column_start":30,"column_end":34,"is_primary":true,"text":[{"text":"            let _ = Self::S1(&**s);","highlight_start":30,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":3397,"byte_end":3401,"line_start":156,"line_end":156,"column_start":30,"column_end":34,"is_primary":true,"text":[{"text":"            let _ = Self::S1(&**s);","highlight_start":30,"highlight_end":34}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:156:30\n   |\nLL |             let _ = Self::S1(&**s);\n   |                              ^^^^ help: try this: `s`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":3482,"byte_end":3484,"line_start":160,"line_end":160,"column_start":21,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = E1::S1(&*s);","highlight_start":21,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":3482,"byte_end":3484,"line_start":160,"line_end":160,"column_start":21,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = E1::S1(&*s);","highlight_start":21,"highlight_end":23}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:160:21\n   |\nLL |     let _ = E1::S1(&*s);\n   |                     ^^ help: try this: `s`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":3981,"byte_end":3985,"line_start":179,"line_end":179,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    let _ = (*b).foo;","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":3981,"byte_end":3985,"line_start":179,"line_end":179,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    let _ = (*b).foo;","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":"b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:179:13\n   |\nLL |     let _ = (*b).foo;\n   |             ^^^^ help: try this: `b`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4003,"byte_end":4008,"line_start":180,"line_end":180,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"    let _ = (**b).foo;","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4003,"byte_end":4008,"line_start":180,"line_end":180,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"    let _ = (**b).foo;","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":"b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:180:13\n   |\nLL |     let _ = (**b).foo;\n   |             ^^^^^ help: try this: `b`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4353,"byte_end":4361,"line_start":195,"line_end":195,"column_start":19,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = f_str(*ref_str);","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4353,"byte_end":4361,"line_start":195,"line_end":195,"column_start":19,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = f_str(*ref_str);","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":"ref_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:195:19\n   |\nLL |     let _ = f_str(*ref_str);\n   |                   ^^^^^^^^ help: try this: `ref_str`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4414,"byte_end":4427,"line_start":197,"line_end":197,"column_start":19,"column_end":32,"is_primary":true,"text":[{"text":"    let _ = f_str(**ref_ref_str);","highlight_start":19,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4414,"byte_end":4427,"line_start":197,"line_end":197,"column_start":19,"column_end":32,"is_primary":true,"text":[{"text":"    let _ = f_str(**ref_ref_str);","highlight_start":19,"highlight_end":32}],"label":null,"suggested_replacement":"ref_ref_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:197:19\n   |\nLL |     let _ = f_str(**ref_ref_str);\n   |                   ^^^^^^^^^^^^^ help: try this: `ref_ref_str`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4562,"byte_end":4570,"line_start":207,"line_end":207,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"    f_str(&&*ref_str); // `needless_borrow` will suggest removing both references","highlight_start":13,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4562,"byte_end":4570,"line_start":207,"line_end":207,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"    f_str(&&*ref_str); // `needless_borrow` will suggest removing both references","highlight_start":13,"highlight_end":21}],"label":null,"suggested_replacement":"ref_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:207:13\n   |\nLL |     f_str(&&*ref_str); // `needless_borrow` will suggest removing both references\n   |             ^^^^^^^^ help: try this: `ref_str`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4643,"byte_end":4653,"line_start":208,"line_end":208,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"    f_str(&&**ref_str); // `needless_borrow` will suggest removing only one reference","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4643,"byte_end":4653,"line_start":208,"line_end":208,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"    f_str(&&**ref_str); // `needless_borrow` will suggest removing only one reference","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":"ref_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:208:12\n   |\nLL |     f_str(&&**ref_str); // `needless_borrow` will suggest removing only one reference\n   |            ^^^^^^^^^^ help: try this: `ref_str`\n\n"}
{"message":"deref which would be done by auto-deref","code":{"code":"clippy::explicit_auto_deref","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4865,"byte_end":4867,"line_start":217,"line_end":217,"column_start":41,"column_end":43,"is_primary":true,"text":[{"text":"    let _ = || -> &'static str { return *s };","highlight_start":41,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_auto_deref.rs","byte_start":4865,"byte_end":4867,"line_start":217,"line_end":217,"column_start":41,"column_end":43,"is_primary":true,"text":[{"text":"    let _ = || -> &'static str { return *s };","highlight_start":41,"highlight_end":43}],"label":null,"suggested_replacement":"s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: deref which would be done by auto-deref\n  --> tests/ui/explicit_auto_deref.rs:217:41\n   |\nLL |     let _ = || -> &'static str { return *s };\n   |                                         ^^ help: try this: `s`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
