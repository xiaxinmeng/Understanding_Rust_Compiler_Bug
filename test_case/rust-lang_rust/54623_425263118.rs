plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/f3/43/c345035af7ca97094721b98573ea23f74d899b86bd02c70928cd5ded1666/awscli-1.16.23-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 10.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:47:03] ....................................................................................................
[00:47:06] ....................................................................................................
[00:47:08] ....................................................................................................
[00:47:11] ....................................................................................................
[00:47:15] ...............................................................F....................................
[00:47:20] ....................................................................................................
[00:47:24] ................................................................................................i...
[00:47:26] ....................................................................................................
[00:47:30] ........................................................i.i..ii.....................................
[00:47:30] ........................................................i.i..ii.....................................
[00:47:33] .....F..............................................................................................
[00:47:39] ....................................................................................................
[00:47:42] ....................................................................................................
[00:47:44] ....................................................................................................
[00:47:47] ....................................................................................................
---
[00:52:33] ---- [ui] ui/impl-trait/where-allowed.rs stdout ----
[00:52:33] diff of stderr:
[00:52:33] 
[00:52:33] 21    |
[00:52:33] 22 LL | fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 24 
[00:52:33] 24 
[00:52:33] 25 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 26   --> $DIR/where-allowed.rs:32:42
[00:52:33] 27    |
[00:52:33] 27    |
[00:52:33] 28 LL | fn in_fn_return_in_parameters(_: fn() -> impl Debug) { panic!() }
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 30 
[00:52:33] 30 
[00:52:33] 31 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 32   --> $DIR/where-allowed.rs:36:38
    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 54 
[00:52:33] 54 
[00:52:33] 55 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 56   --> $DIR/where-allowed.rs:52:55
[00:52:33] 57    |
[00:52:33] 57    |
[00:52:33] 58 LL | fn in_dyn_Fn_parameter_in_return() -> &'static dyn Fn(impl Debug) { panic!() }
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 60 
[00:52:33] 60 
[00:52:33] 61 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 62   --> $DIR/where-allowed.rs:56:57
[00:52:33] 63    |
[00:52:33] 63    |
[00:52:33] 64 LL | fn in_dyn_Fn_return_in_return() -> &'static dyn Fn() -> impl Debug { panic!() }
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 66 
[00:52:33] 66 
[00:52:33] 67 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 68   --> $DIR/where-allowed.rs:60:51
[00:52:33] 69    |
[00:52:33] 69    |
[00:52:33] 70 LL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 72 
[00:52:33] 73 llowed.rs:78:38
[00:52:33] 73 llowed.rs:78:38
[00:52:33] 
[00:52:33] 93    |
[00:52:33] 94 LL | fn in_Fn_parameter_in_generics<F: Fn(impl Debug)> (_: F) { panic!() }
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 96 
[00:52:33] 96 
[00:52:33] 97 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 98   --> $DIR/where-allowed.rs:82:40
[00:52:33] 99    |
[00:52:33] 99    |
[00:52:33] 100 LL | fn in_Fn_return_in_generics<F: Fn() -> impl Debug> (_: F) { panic!() }
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 102 
[00:52:33] 102 
[00:52:33] 103 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 104   --> $DIR/where-allowed.rs:95:32
[00:52:33] 105    |
[00:52:33] 105    |
[00:52:33] 106 LL | struct InBraceStructField { x: impl Debug }
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 108 
[00:52:33] 108 
[00:52:33] 109 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 110   --> $DIR/where-allowed.rs:99:41
[00:52:33] 111    |
[00:52:33] 111    |
[00:52:33] 112 LL | struct InAdtInBraceStructField { x: Vec<impl Debug> }
[00:52:33] +    |
[00:52:33] urn types
[00:52:33] 158   --> $DIR/where-allowed.rs:154:31
[00:52:33] 
[00:52:33] 
[00:52:33] 159    |
[00:52:33] 160 LL |     fn in_foreign_return() -> impl Debug;
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 162 
[00:52:33] 162 
[00:52:33] 163 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 164   --> $DIR/where-allowed.rs:167:23
[00:52:33] 165    |
[00:52:33] 165    |
[00:52:33] 166 LL | type InTypeAlias<R> = impl Debug;
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 168 
[00:52:33] 168 
[00:52:33] 169 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 170   --> $DIR/where-allowed.rs:170:39
[00:52:33] 171    |
[00:52:33] 171    |
[00:52:33] 172 LL | type InReturnInTypeAlias<R> = fn() -> impl Debug;
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 174 
[00:52:33] 174 
[00:52:33] 175 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 176   --> $DIR/where-allowed.rs:174:16
[00:52:33] 177    |
[00:52:33] 177    |
[00:52:33] 178 LL | impl PartialEq<impl Debug> for () {
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 180 
[00:52:33] 180 
[00:52:33] 181 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 182   --> $DIR/where-allowed.rs:179:24
[00:52:33] 183    |
[00:52:33] 183    |
[00:52:33] 184 LL | impl PartialEq<()> for impl Debug {
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 186 
[00:52:33] 186 
[00:52:33] 187 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 188   --> $DIR/where-allowed.rs:184:6
[00:52:33] 189    |
[00:52:33] 189    |
[00:52:33] 190 LL | impl impl Debug {
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 192 
[00:52:33] 192 
[00:52:33] 193 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 194   --> $DIR/where-allowed.rs:190:24
[00:52:33] 195    |
[00:52:33] 195    |
[00:52:33] 196 LL | impl InInherentImplAdt<impl Debug> {
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 198 
[00:52:33] 198 
[00:52:33] 199 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 200   --> $DIR/where-allowed.rs:196:11
[00:52:33] 201    |
[00:52:33] 201    |
[00:52:33] 202 LL |     where impl Debug: Debug
[00:52:33] +    |
[00:52:33] +    = help: a                 ^^^^^^^^^^
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 228 
[00:52:33] 229 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 230   --> $DIR/where-allowed.rs:230:29
[00:52:33] 231    |
[00:52:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[00:52:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[00:52:33] 232 LL |     let _in_local_variable: impl Fn() = || {};
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 234 
[00:52:33] 234 
[00:52:33] 235 error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:52:33] 236   --> $DIR/where-allowed.rs:232:46
[00:52:33] 237    |
[00:52:33] 237    |
[00:52:33] 238 LL |     let _in_return_in_local_variable = || -> impl Fn() { || {} };
[00:52:33] +    |
[00:52:33] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[00:52:33] 240 
[00:52:33] 241 error: aborting due to 39 previous errors
[00:52:33] 241 error: aborting due to 39 previous errors
[00:52:33] 242 
[00:52:33] 
[00:52:33] 
[00:52:33] The actual stderr differed from the expected stderr.
[00:52:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/where-allowed.stderr
[00:52:33] To update references, rerun the tests and pass the `--bless` flag
[00:52:33] To only update this specific test, also pass `--test-args rs","byte_start":869,"byte_end":879,"line_start":28,"line_end":28,"column_start":40,"column_end":50,"is_primary":true,"text":[{"text":"fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }","highlight_start":40,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(impl_trait_in_bindings)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0562]: `impl Trait` not allowed outside of function and inherent method return types\n  --> /checkout/src/test/ui/impl-trait/where-allowed.rs:28:40\n   |\nLL | fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }\n   |                                        ^^^^^^^^^^\n   |\n   = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable\n\n"}
[00:52:33] {"message":"`impl Trait` not allowed outside of function and inherent method return types","code":{"code":"E0562","explanation":"\nAbstract return types (written `impl Trait` for some trait `Trait`) are only\nallowed as function and inherent impl return types.\n\nErroneous code example:\n\n