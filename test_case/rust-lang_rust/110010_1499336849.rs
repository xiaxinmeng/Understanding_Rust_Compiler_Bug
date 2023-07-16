plain

---- compile_test stdout ----
diff of stderr:

-error: an implementation of `From` is preferred since it gives you `Into<_>` for free where the reverse isn't true
+error[E0308]: mismatched types
+  --> $DIR/from_over_into.rs:88:22
    |
    |
-LL | impl Into<StringWrapper> for String {
-   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+LL | type Opaque = impl Sized;
+...
+...
+LL |     fn into(self) -> Opaque {}
+   |        |
+   |        |
+   |        implicitly returns `()` as its body has no tail or `return` expression
    |
-   = note: `-D clippy::from-over-into` implied by `-D warnings`
-help: replace the `Into` implentation with `From<std::string::String>`
+   = note: expected opaque type `Opaque`
+                found unit type `()`
+note: this item cannot register hidden type without a `#[defines(Opaque)]` attribute
    |
    |
-LL ~ impl From<String> for StringWrapper {
-LL ~     fn from(val: String) -> Self {
-LL ~         StringWrapper(val)
-   |
+LL |     fn into(self) -> Opaque {}
 
 
-error: an implementation of `From` is preferred since it gives you `Into<_>` for free where the reverse isn't true
-   |
-   |
-LL | impl Into<SelfType> for String {
-   |
-   |
-help: replace the `Into` implentation with `From<std::string::String>`
-   |
-LL ~ impl From<String> for SelfType {
-LL ~     fn from(val: String) -> Self {
-LL ~         SelfType(String::new())
+error: aborting due to previous error
 
 
-error: an implementation of `From` is preferred since it gives you `Into<_>` for free where the reverse isn't true
-   |
-   |
-LL | impl Into<SelfKeywords> for X {
-   |
-   |
-help: replace the `Into` implentation with `From<X>`
-   |
-LL ~ impl From<X> for SelfKeywords {
-LL ~     fn from(val: X) -> Self {
-LL ~         let _ = X::default();
-LL ~         let _ = X::FOO;
-LL ~         let _: X = val;
-
-
-error: an implementation of `From` is preferred since it gives you `Into<_>` for free where the reverse isn't true
-   |
-   |
-LL | impl core::convert::Into<bool> for crate::ExplicitPaths {
-   |
-   |
-   = help: `impl From<Local> for Foreign` is allowed by the orphan rules, for more information see
-           https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence
error: test failed, to rerun pass `--test compile-test`
-help: replace the `Into` implentation with `From<ExplicitPaths>`
-   |
-LL ~ impl core::convert::From<crate::ExplicitPaths> for bool {
-LL ~     fn from(mut val: crate::ExplicitPaths) -> Self {
-LL ~         let in_closure = || val.0;
-LL | 
-LL ~         val.0 = false;
-LL ~         val.0
-
-
-error: an implementation of `From` is preferred since it gives you `Into<_>` for free where the reverse isn't true
-   |
-   |
-LL |     impl<T> Into<FromOverInto<T>> for Vec<T> {
-   |
-   |
-help: replace the `Into` implentation with `From<std::vec::Vec<T>>`
-   |
-LL ~     impl<T> From<Vec<T>> for FromOverInto<T> {
-LL ~         fn from(val: Vec<T>) -> Self {
-LL ~             FromOverInto(val)
-
-error: aborting due to 5 previous errors
-
+For more information about this error, try `rustc --explain E0308`.
---
diff of fixed:

 // run-rustfix
 
 #![feature(type_alias_impl_trait)]
 #![warn(clippy::from_over_into)]
 
 // this should throw an error
 struct StringWrapper(String);
 
 
-impl From<String> for StringWrapper {
-    fn from(val: String) -> Self {
-        StringWrapper(val)
+impl Into<StringWrapper> for String {
+    fn into(self) -> StringWrapper {
+        StringWrapper(self)
 }
 
 struct SelfType(String);
 
 
-impl From<String> for SelfType {
-    fn from(val: String) -> Self {
-        SelfType(String::new())
+impl Into<SelfType> for String {
+    fn into(self) -> SelfType {
+        SelfType(Self::new())
 }
 
 #[derive(Default)]
 struct X;
 struct X;
 
 impl X {
     const FOO: &'static str = "a";
 
 struct SelfKeywords;
 
 
-impl From<X> for SelfKeywords {
-    fn from(val: X) -> Self {
-        let _ = X::default();
-        let _ = X::FOO;
-        let _: X = val;
+impl Into<SelfKeywords> for X {
+    fn into(self) -> SelfKeywords {
+        let _ = Self::default();
+        let _ = Self::FOO;
+        let _: Self = self;
         SelfKeywords
     }
 }
 
 
 struct ExplicitPaths(bool);
 
-impl core::convert::From<crate::ExplicitPaths> for bool {
-    fn from(mut val: crate::ExplicitPaths) -> Self {
-        let in_closure = || val.0;
+impl core::convert::Into<bool> for crate::ExplicitPaths {
+    fn into(mut self) -> bool {
+        let in_closure = || self.0;
-        val.0 = false;
-        val.0
+        self.0 = false;
+        self.0
+        self.0
     }
 }
 
 // this is fine
 struct A(String);
 
 impl From<String> for A {
     fn from(s: String) -> A {
         A(s)
 }
 
 
 #[clippy::msrv = "1.40"]
 fn msrv_1_40() {
     struct FromOverInto<T>(Vec<T>);
 
     impl<T> Into<FromOverInto<T>> for Vec<T> {
         fn into(self) -> FromOverInto<T> {
             FromOverInto(self)
     }
 }
 
 
 #[clippy::msrv = "1.41"]
 fn msrv_1_41() {
     struct FromOverInto<T>(Vec<T>);
 
-    impl<T> From<Vec<T>> for FromOverInto<T> {
-        fn from(val: Vec<T>) -> Self {
-            FromOverInto(val)
+    impl<T> Into<FromOverInto<T>> for Vec<T> {
+        fn into(self) -> FromOverInto<T> {
+            FromOverInto(self)
     }
 }
 
 type Opaque = impl Sized;
 type Opaque = impl Sized;
 struct IntoOpaque;
 impl Into<Opaque> for IntoOpaque {
     fn into(self) -> Opaque {}
 
 fn main() {}
 


The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/from_over_into.stage-id.fixed
To only update this specific test, also pass `--test-args from_over_into.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/from_over_into.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/from_over_into.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-23c15a454288152e.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-fc796eef9a515a44.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-d8aee1681c496322.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e5b19e58ae33983e.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-60efa73d6fad7e91.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-40ff94c070351d6c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-e22c3fc74241d5e4.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/from_over_into.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.\n\nErroneous code examples:\n\n