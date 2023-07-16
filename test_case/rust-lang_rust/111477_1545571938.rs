plain

---- [ui] tests/ui/impl-trait/where-allowed.rs stdout ----
diff of stderr:

+ error: unexpected `impl` keyword
+    |
+    |
+ LL | impl impl Debug {
+    |      ^^^^ help: remove the extra `impl`
1 error[E0666]: nested `impl Trait` is not allowed
2   --> $DIR/where-allowed.rs:49:51
3    |


194    |                        ^^^^^^^^^^
195 
196 error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in impl headers
-   --> $DIR/where-allowed.rs:173:6
-    |
- LL | impl impl Debug {
- 
- 
- error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in impl headers
204    |
204    |
205 LL | impl InInherentImplAdt<impl Debug> {

283 LL |     let _in_return_in_local_variable = || -> impl Fn() { || {} };
285 
+ warning: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/where-allowed.rs:173:11
+    |
+    |
+ LL | impl impl Debug {
+    |
+    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+    = note: `#[warn(bare_trait_objects)]` on by default
+    = note: `#[warn(bare_trait_objects)]` on by default
+ help: use `dyn`
+    |
+ LL | impl impl dyn Debug {
+ 
+ 
286 error: defaults for type parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions
288    |

302    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
303    = note: for more information, see issue #36887 <https://github.com/rust-lang/rust/issues/36887>
303    = note: for more information, see issue #36887 <https://github.com/rust-lang/rust/issues/36887>
304 
+ error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
+   --> $DIR/where-allowed.rs:173:1
+    |
+ LL | impl impl Debug {
+    | ^^^^^^^^^^^^^^^ impl for type defined outside of crate.
+    = note: define and implement a trait or new type instead
+ 
305 error[E0118]: no nominal type found for inherent implementation
306   --> $DIR/where-allowed.rs:235:1
---
To only update this specific test, also pass `--test-args impl-trait/where-allowed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/where-allowed.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/auxiliary"
stdout: none
error: unexpected `impl` keyword
  --> fake-test-src-base/impl-trait/where-allowed.rs:173:6
   |
LL | impl impl Debug {
LL | impl impl Debug {
   |      ^^^^ help: remove the extra `impl`

error[E0666]: nested `impl Trait` is not allowed
  --> fake-test-src-base/impl-trait/where-allowed.rs:49:51
   |
LL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }
Build completed unsuccessfully in 0:13:04
   |                                           |       |
   |                                           |       nested `impl Trait` here
   |                                           outer `impl Trait`
   |                                           outer `impl Trait`

error[E0666]: nested `impl Trait` is not allowed
  --> fake-test-src-base/impl-trait/where-allowed.rs:58:57
   |
LL | fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }
   |                                                 |       |
   |                                                 |       nested `impl Trait` here
   |                                                 outer `impl Trait`

---

error[E0658]: `impl Trait` in type aliases is unstable
  --> fake-test-src-base/impl-trait/where-allowed.rs:155:23
   |
LL | type InTypeAlias<R> = impl Debug;
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(type_alias_impl_trait)]` to the crate attributes to enable


error[E0658]: `impl Trait` in type aliases is unstable
  --> fake-test-src-base/impl-trait/where-allowed.rs:158:39
   |
LL | type InReturnInTypeAlias<R> = fn() -> impl Debug;
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(type_alias_impl_trait)]` to the crate attributes to enable


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `fn` pointer params
  --> fake-test-src-base/impl-trait/where-allowed.rs:18:40
   |
LL | fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `fn` pointer return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:22:42
   |
LL | fn in_fn_return_in_parameters(_: fn() -> impl Debug) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `fn` pointer params
  --> fake-test-src-base/impl-trait/where-allowed.rs:26:38
   |
LL | fn in_fn_parameter_in_return() -> fn(impl Debug) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `fn` pointer return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:30:40
   |
LL | fn in_fn_return_in_return() -> fn() -> impl Debug { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait params
  --> fake-test-src-base/impl-trait/where-allowed.rs:34:49
   |
LL | fn in_dyn_Fn_parameter_in_parameters(_: &dyn Fn(impl Debug)) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:38:51
   |
LL | fn in_dyn_Fn_return_in_parameters(_: &dyn Fn() -> impl Debug) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait params
  --> fake-test-src-base/impl-trait/where-allowed.rs:42:55
   |
LL | fn in_dyn_Fn_parameter_in_return() -> &'static dyn Fn(impl Debug) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait params
  --> fake-test-src-base/impl-trait/where-allowed.rs:49:51
   |
LL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:54:53
   |
LL | fn in_impl_Fn_return_in_parameters(_: &impl Fn() -> impl Debug) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait params
  --> fake-test-src-base/impl-trait/where-allowed.rs:58:57
   |
LL | fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait params
  --> fake-test-src-base/impl-trait/where-allowed.rs:66:38
   |
LL | fn in_Fn_parameter_in_generics<F: Fn(impl Debug)> (_: F) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:70:40
   |
LL | fn in_Fn_return_in_generics<F: Fn() -> impl Debug> (_: F) { panic!() }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in field types
  --> fake-test-src-base/impl-trait/where-allowed.rs:83:32
   |
LL | struct InBraceStructField { x: impl Debug }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in field types
  --> fake-test-src-base/impl-trait/where-allowed.rs:87:41
   |
LL | struct InAdtInBraceStructField { x: Vec<impl Debug> }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in field types
  --> fake-test-src-base/impl-trait/where-allowed.rs:91:27
   |
LL | struct InTupleStructField(impl Debug);


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in field types
  --> fake-test-src-base/impl-trait/where-allowed.rs:96:25
   |
LL |     InBraceVariant { x: impl Debug },


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in field types
  --> fake-test-src-base/impl-trait/where-allowed.rs:98:20
   |
LL |     InTupleVariant(impl Debug),


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in trait method return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:109:23
   |
LL |     fn in_return() -> impl Debug;
   |
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(return_position_impl_trait_in_trait)]` to the crate attributes to enable


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `impl` method return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:126:34
   |
LL |     fn in_trait_impl_return() -> impl Debug { () }
   |
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(return_position_impl_trait_in_trait)]` to the crate attributes to enable


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `extern fn` params
  --> fake-test-src-base/impl-trait/where-allowed.rs:139:33
   |
LL |     fn in_foreign_parameters(_: impl Debug);


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `extern fn` return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:142:31
   |
LL |     fn in_foreign_return() -> impl Debug;


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `fn` pointer return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:158:39
   |
LL | type InReturnInTypeAlias<R> = fn() -> impl Debug;


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in traits
  --> fake-test-src-base/impl-trait/where-allowed.rs:163:16
   |
LL | impl PartialEq<impl Debug> for () {


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in impl headers
  --> fake-test-src-base/impl-trait/where-allowed.rs:168:24
   |
LL | impl PartialEq<()> for impl Debug {


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in impl headers
  --> fake-test-src-base/impl-trait/where-allowed.rs:179:24
   |
LL | impl InInherentImplAdt<impl Debug> {


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in bounds
  --> fake-test-src-base/impl-trait/where-allowed.rs:185:11
LL |     where impl Debug: Debug
   |           ^^^^^^^^^^


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in bounds
  --> fake-test-src-base/impl-trait/where-allowed.rs:192:15
   |
LL |     where Vec<impl Debug>: Debug


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in bounds
  --> fake-test-src-base/impl-trait/where-allowed.rs:199:24
   |
LL |     where T: PartialEq<impl Debug>


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait params
  --> fake-test-src-base/impl-trait/where-allowed.rs:206:17
   |
LL |     where T: Fn(impl Debug)


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:213:22
   |
LL |     where T: Fn() -> impl Debug


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in generic parameter defaults
  --> fake-test-src-base/impl-trait/where-allowed.rs:219:40
   |
LL | struct InStructGenericParamDefault<T = impl Debug>(T);


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in generic parameter defaults
  --> fake-test-src-base/impl-trait/where-allowed.rs:223:36
   |
LL | enum InEnumGenericParamDefault<T = impl Debug> { Variant(T) }


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in generic parameter defaults
  --> fake-test-src-base/impl-trait/where-allowed.rs:227:38
   |
LL | trait InTraitGenericParamDefault<T = impl Debug> {}


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in generic parameter defaults
  --> fake-test-src-base/impl-trait/where-allowed.rs:231:41
   |
LL | type InTypeAliasGenericParamDefault<T = impl Debug> = T;


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in generic parameter defaults
  --> fake-test-src-base/impl-trait/where-allowed.rs:235:11
   |
LL | impl <T = impl Debug> T {}


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in generic parameter defaults
  --> fake-test-src-base/impl-trait/where-allowed.rs:242:40
   |
LL | fn in_method_generic_param_default<T = impl Debug>(_: T) {}


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in variable bindings
  --> fake-test-src-base/impl-trait/where-allowed.rs:248:29
   |
LL |     let _in_local_variable: impl Fn() = || {};


error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in closure return types
  --> fake-test-src-base/impl-trait/where-allowed.rs:250:46
   |
LL |     let _in_return_in_local_variable = || -> impl Fn() { || {} };

warning: trait objects without an explicit `dyn` are deprecated
  --> fake-test-src-base/impl-trait/where-allowed.rs:173:11
   |
---
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: `#[warn(bare_trait_objects)]` on by default
help: use `dyn`
   |
LL | impl impl dyn Debug {


error: defaults for type parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions
  --> fake-test-src-base/impl-trait/where-allowed.rs:235:7
   |
LL | impl <T = impl Debug> T {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #36887 <https://github.com/rust-lang/rust/issues/36887>
   = note: `#[deny(invalid_type_param_default)]` on by default
   = note: `#[deny(invalid_type_param_default)]` on by default

error: defaults for type parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions
  --> fake-test-src-base/impl-trait/where-allowed.rs:242:36
   |
LL | fn in_method_generic_param_default<T = impl Debug>(_: T) {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #36887 <https://github.com/rust-lang/rust/issues/36887>

---

error[E0118]: no nominal type found for inherent implementation
  --> fake-test-src-base/impl-trait/where-allowed.rs:235:1
   |
LL | impl <T = impl Debug> T {}
   | ^^^^^^^^^^^^^^^^^^^^^^^ impl requires a nominal type
   = note: either implement a trait on it or create a newtype to wrap it instead

error: aborting due to 48 previous errors; 1 warning emitted

