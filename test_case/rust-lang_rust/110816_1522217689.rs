plain
diff of stderr:

16 help: to apply to the crate, use an inner attribute
17    |
18 LL | #![doc(cfg_hide(doc))]
-    | ~~~~~~~~~~~~~~~~~~~~~~
+    |  +
20 
21 error: `#[doc(cfg_hide(...)]` takes a list of attributes


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc_cfg_hide/doc_cfg_hide.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc_cfg_hide/doc_cfg_hide.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doc_cfg_hide.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doc_cfg_hide.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc_cfg_hide" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc_cfg_hide/auxiliary"
stdout: none
error: this attribute can only be applied at the crate level
  --> /checkout/tests/rustdoc-ui/doc_cfg_hide.rs:9:7
   |
   |
LL | #[doc(cfg_hide(doc))] //~ ERROR
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
  --> /checkout/tests/rustdoc-ui/doc_cfg_hide.rs:2:9
   |
LL | #![deny(warnings)]
   |         ^^^^^^^^
   |         ^^^^^^^^
   = note: `#[deny(invalid_doc_attributes)]` implied by `#[deny(warnings)]`
help: to apply to the crate, use an inner attribute
   |
LL | #![doc(cfg_hide(doc))] //~ ERROR
   |  +

error: `#[doc(cfg_hide(...)]` takes a list of attributes
   |
   |
LL | #![doc(cfg_hide = "test")] //~ ERROR
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>


error: `#[doc(cfg_hide(...)]` takes a list of attributes
   |
   |
LL | #![doc(cfg_hide)] //~ ERROR
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>

---
diff of stderr:

16 help: to apply to the crate, use an inner attribute
17    |
18 LL | #![doc(test(no_crate_inject))]
+    |  +
20 
21 error: this attribute can only be applied to a `use` item
22   --> $DIR/invalid-doc-attr.rs:9:7
---
To only update this specific test, also pass `--test-args invalid-doc-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/invalid-doc-attr.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-doc-attr" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-doc-attr/auxiliary"
stdout: none
error: this attribute can only be applied at the crate level
  --> /checkout/tests/rustdoc-ui/invalid-doc-attr.rs:4:7
   |
   |
LL | #[doc(test(no_crate_inject))]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
  --> /checkout/tests/rustdoc-ui/invalid-doc-attr.rs:2:9
   |
LL | #![deny(warnings)]
   |         ^^^^^^^^
   |         ^^^^^^^^
   = note: `#[deny(invalid_doc_attributes)]` implied by `#[deny(warnings)]`
help: to apply to the crate, use an inner attribute
   |
LL | #![doc(test(no_crate_inject))]
   |  +
error: this attribute can only be applied to a `use` item
  --> /checkout/tests/rustdoc-ui/invalid-doc-attr.rs:9:7
   |
LL | #[doc(inline)]
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
  --> /checkout/tests/rustdoc-ui/invalid-doc-attr.rs:15:12
   |
   |
LL |     #![doc(test(no_crate_inject))]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
error: conflicting doc inlining attributes
  --> /checkout/tests/rustdoc-ui/invalid-doc-attr.rs:28:7
   |
LL | #[doc(inline)]
LL | #[doc(inline)]
   |       ^^^^^^ this attribute...
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ ...conflicts with this attribute
   = help: remove one of the conflicting attributes

error: this attribute can only be applied at the crate level
  --> /checkout/tests/rustdoc-ui/invalid-doc-attr.rs:19:11
  --> /checkout/tests/rustdoc-ui/invalid-doc-attr.rs:19:11
   |
LL |     #[doc(test(no_crate_inject))]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read <https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level> for more information
error: this attribute can only be applied to a `use` item
  --> /checkout/tests/rustdoc-ui/invalid-doc-attr.rs:22:11
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


