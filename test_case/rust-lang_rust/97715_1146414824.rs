plain

---- [ui] src/test/ui/attributes/attrs-on-params.rs stdout ----
diff of stderr:

- error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
+ error: allow, cfg, cfg_attr, deny, expect, forbid, and warn are the only allowed built-in attributes in function parameters
3    |
3    |
4 LL | fn function(#[inline] param: u32) {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attrs-on-params/attrs-on-params.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args attributes/attrs-on-params.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/attrs-on-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attrs-on-params" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attrs-on-params/auxiliary"
stdout: none
--- stderr -------------------------------
error: allow, cfg, cfg_attr, deny, expect, forbid, and warn are the only allowed built-in attributes in function parameters
   |
   |
LL | fn function(#[inline] param: u32) {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error[E0518]: attribute should be applied to function or closure
  --> /checkout/src/test/ui/attributes/attrs-on-params.rs:3:13
  --> /checkout/src/test/ui/attributes/attrs-on-params.rs:3:13
   |
LL | fn function(#[inline] param: u32) {
   |             |
   |             not a function or closure

error: aborting due to 2 previous errors
---

---- [ui] src/test/ui/rustdoc/check-doc-alias-attr-location.rs stdout ----
diff of stderr:

- error: allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in attributes in function parameters
+ error: allow, cfg, cfg_attr, deny, expect, forbid, and warn are the only allowed built-in attributes in function parameters
3    |
3    |
4 LL |     fn foo(#[doc(alias = "qux")] _x: u32) -> Self::X {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustdoc/check-doc-alias-attr-location/check-doc-alias-attr-location.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rustdoc/check-doc-alias-attr-location.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rustdoc/check-doc-alias-attr-location.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustdoc/check-doc-alias-attr-location" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustdoc/check-doc-alias-attr-location/auxiliary"
stdout: none
--- stderr -------------------------------
error: allow, cfg, cfg_attr, deny, expect, forbid, and warn are the only allowed built-in attributes in function parameters
   |
   |
LL |     fn foo(#[doc(alias = "qux")] _x: u32) -> Self::X {


error: `#[doc(alias = "...")]` isn't allowed on extern block
   |
   |
LL | #[doc(alias = "foo")] //~ ERROR


error: `#[doc(alias = "...")]` isn't allowed on implementation block
   |
   |
LL | #[doc(alias = "bar")] //~ ERROR


error: `#[doc(alias = "...")]` isn't allowed on implementation block
   |
   |
LL | #[doc(alias = "foobar")] //~ ERROR


error: `#[doc(alias = "...")]` isn't allowed on type alias in implementation block
   |
   |
LL |     #[doc(alias = "assoc")] //~ ERROR

error: aborting due to 5 previous errors
------------------------------------------

