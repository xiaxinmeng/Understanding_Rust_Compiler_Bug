plain

12    = note: `#[deny(invalid_doc_attributes)]` implied by `#[deny(warnings)]`
13    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
14    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
+    = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
16 help: to apply to the crate, use an inner attribute
17    |
18 LL | #![doc(test(no_crate_inject))]
29    |
30    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
31    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#inline-and-no_inline for more information
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#inline-and-no_inline for more information
+    = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#inline-and-no_inline> for more information
34 error: this attribute can only be applied at the crate level
35   --> $DIR/invalid-doc-attr.rs:15:12

39    |
39    |
40    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
41    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
+    = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
44 error: conflicting doc inlining attributes
45   --> $DIR/invalid-doc-attr.rs:28:7

59    |
59    |
60    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
61    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
+    = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
64 error: this attribute can only be applied to a `use` item
65   --> $DIR/invalid-doc-attr.rs:22:11

72    |
72    |
73    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
74    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#inline-and-no_inline for more information
+    = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#inline-and-no_inline> for more information
77 error: aborting due to 6 previous errors
78 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-doc-attr/invalid-doc-attr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args invalid-doc-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/invalid-doc-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-doc-attr" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-doc-attr/auxiliary"
stdout: none
--- stderr -------------------------------
error: this attribute can only be applied at the crate level
   |
   |
LL | #[doc(test(no_crate_inject))]
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/invalid-doc-attr.rs:2:9
   |
   |
LL | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(invalid_doc_attributes)]` implied by `#[deny(warnings)]`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
help: to apply to the crate, use an inner attribute
   |
LL | #![doc(test(no_crate_inject))]

error: this attribute can only be applied to a `use` item
  --> /checkout/src/test/rustdoc-ui/invalid-doc-attr.rs:9:7
   |
   |
LL | #[doc(inline)]
   |       ^^^^^^ only applicable on `use` items
LL | pub fn foo() {}
   | ------------ not a `use` item
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#inline-and-no_inline> for more information
error: this attribute can only be applied at the crate level
  --> /checkout/src/test/rustdoc-ui/invalid-doc-attr.rs:15:12
   |
   |
LL |     #![doc(test(no_crate_inject))]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
error: conflicting doc inlining attributes
  --> /checkout/src/test/rustdoc-ui/invalid-doc-attr.rs:28:7
   |
   |
LL | #[doc(inline)]
   |       ^^^^^^ this attribute...
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ ...conflicts with this attribute
   = help: remove one of the conflicting attributes

error: this attribute can only be applied at the crate level
  --> /checkout/src/test/rustdoc-ui/invalid-doc-attr.rs:19:11
  --> /checkout/src/test/rustdoc-ui/invalid-doc-attr.rs:19:11
   |
LL |     #[doc(test(no_crate_inject))]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
error: this attribute can only be applied to a `use` item
  --> /checkout/src/test/rustdoc-ui/invalid-doc-attr.rs:22:11
   |
LL |     #[doc(inline)]
LL |     #[doc(inline)]
   |           ^^^^^^ only applicable on `use` items
...
LL |     pub fn baz() {}
   |     ------------ not a `use` item
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#inline-and-no_inline> for more information
error: aborting due to 6 previous errors
------------------------------------------


