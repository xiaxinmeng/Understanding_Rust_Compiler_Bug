plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
Executing the job since clippy subtree was updated
with:
  github_token: ***
  check_every_seconds: 60
env:
---
normalized stderr:
error: redundant closure found
  --> $DIR/eta.rs:20:27
   |
LL |     let a = Some(1u8).map(|a| foo(a));
   |                           ^^^^^^^^^^ help: remove closure as shown: `foo`
   |
   = note: `-D clippy::redundant-closure` implied by `-D warnings`
error: redundant closure found
  --> $DIR/eta.rs:21:10
   |
   |
LL |     meta(|a| foo(a));
   |          ^^^^^^^^^^ help: remove closure as shown: `foo`

error: this expression borrows a reference (`&u8`) that is immediately dereferenced by the compiler
  --> $DIR/eta.rs:24:21
   |
LL |     all(&[1, 2, 3], &&2, |x, y| below(x, y)); //is adjusted
   |                     ^^^ help: change this to: `&2`
   |
   = note: `-D clippy::needless-borrow` implied by `-D warnings`
error: redundant closure found
  --> $DIR/eta.rs:31:27
   |
   |
LL |     let e = Some(1u8).map(|a| generic(a));
   |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `generic`
error: test failed, to rerun pass '--test compile-test'
error: redundant closure found
  --> $DIR/eta.rs:74:51
   |
   |
LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.foo());
   |                                                   ^^^^^^^^^^^ help: remove closure as shown: `TestStruct::foo`
   |
   = note: `-D clippy::redundant-closure-for-method-calls` implied by `-D warnings`
error: redundant closure found
  --> $DIR/eta.rs:76:51
   |
   |
LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.trait_foo());
   |                                                   ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `TestTrait::trait_foo`
error: redundant closure found
  --> $DIR/eta.rs:79:42
   |
   |
LL |     let e = Some(&mut vec![1, 2, 3]).map(|v| v.clear());
   |                                          ^^^^^^^^^^^^^ help: remove closure as shown: `std::vec::Vec::clear`
error: redundant closure found
  --> $DIR/eta.rs:84:29
   |
   |
LL |     let e = Some("str").map(|s| s.to_string());
   |                             ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `std::string::ToString::to_string`
error: redundant closure found
  --> $DIR/eta.rs:86:27
   |
   |
LL |     let e = Some('a').map(|s| s.to_uppercase());
   |                           ^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_uppercase`
error: redundant closure found
  --> $DIR/eta.rs:89:65
   |
   |
LL |     let e: std::vec::Vec<char> = vec!['a', 'b', 'c'].iter().map(|c| c.to_ascii_uppercase()).collect();
   |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_ascii_uppercase`

thread 'rustc' panicked at 'escaping bound vars for TraitPredicate(<&F as std::ops::FnOnce<(&X, &X)>>)', compiler/rustc_middle/src/ty/mod.rs:1361:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.51 (a8920a9 2021-01-04)
query stack during panic:
query stack during panic:
#0 [type_implements_trait] evaluating `type_implements_trait` `(DefId(2:2304 ~ core[8e42]::ops::function::FnOnce), &F, [(&X, &X)], ParamEnv { caller_bounds: [Binder(ProjectionPredicate(ProjectionTy { substs: [F, (&X, &X)], item_def_id: DefId(2:2306 ~ core[8e42]::ops::function::FnOnce::Output) }, bool)), Binder(TraitPredicate(<F as std::ops::Fn<(&X, &X)>>)), Binder(TraitPredicate(<F as std::ops::FnMut<(&X, &X)>>)), Binder(TraitPredicate(<F as std::ops::FnOnce<(&X, &X)>>)), Binder(TraitPredicate(<F as std::marker::Sized>)), Binder(TraitPredicate(<X as std::marker::Sized>))], reveal: UserFacing })`
#1 [analysis] running analysis passes on this crate
end of query stack



expected stderr:
expected stderr:
error: redundant closure found
  --> $DIR/eta.rs:20:27
   |
LL |     let a = Some(1u8).map(|a| foo(a));
   |                           ^^^^^^^^^^ help: remove closure as shown: `foo`
   |
   = note: `-D clippy::redundant-closure` implied by `-D warnings`
error: redundant closure found
  --> $DIR/eta.rs:21:10
   |
   |
LL |     meta(|a| foo(a));
   |          ^^^^^^^^^^ help: remove closure as shown: `foo`

error: this expression borrows a reference (`&u8`) that is immediately dereferenced by the compiler
  --> $DIR/eta.rs:24:21
   |
LL |     all(&[1, 2, 3], &&2, |x, y| below(x, y)); //is adjusted
   |                     ^^^ help: change this to: `&2`
   |
   = note: `-D clippy::needless-borrow` implied by `-D warnings`
error: redundant closure found
  --> $DIR/eta.rs:31:27
   |
   |
LL |     let e = Some(1u8).map(|a| generic(a));
   |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `generic`
error: redundant closure found
  --> $DIR/eta.rs:74:51
   |
   |
LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.foo());
   |                                                   ^^^^^^^^^^^ help: remove closure as shown: `TestStruct::foo`
   |
   = note: `-D clippy::redundant-closure-for-method-calls` implied by `-D warnings`
error: redundant closure found
  --> $DIR/eta.rs:76:51
   |
   |
LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.trait_foo());
   |                                                   ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `TestTrait::trait_foo`
error: redundant closure found
  --> $DIR/eta.rs:79:42
   |
   |
LL |     let e = Some(&mut vec![1, 2, 3]).map(|v| v.clear());
   |                                          ^^^^^^^^^^^^^ help: remove closure as shown: `std::vec::Vec::clear`
error: redundant closure found
  --> $DIR/eta.rs:84:29
   |
   |
LL |     let e = Some("str").map(|s| s.to_string());
   |                             ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `std::string::ToString::to_string`
error: redundant closure found
  --> $DIR/eta.rs:86:27
   |
   |
LL |     let e = Some('a').map(|s| s.to_uppercase());
   |                           ^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_uppercase`
error: redundant closure found
  --> $DIR/eta.rs:89:65
   |
   |
LL |     let e: std::vec::Vec<char> = vec!['a', 'b', 'c'].iter().map(|c| c.to_ascii_uppercase()).collect();
   |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_ascii_uppercase`
error: redundant closure found
  --> $DIR/eta.rs:172:27
   |
   |
LL |     let a = Some(1u8).map(|a| foo_ptr(a));
   |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `foo_ptr`
error: redundant closure found
  --> $DIR/eta.rs:177:27
   |
   |
LL |     let a = Some(1u8).map(|a| closure(a));
   |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `closure`
error: aborting due to 12 previous errors




diff of stderr:

 error: redundant closure found
   --> $DIR/eta.rs:20:27
    |
 LL |     let a = Some(1u8).map(|a| foo(a));
    |                           ^^^^^^^^^^ help: remove closure as shown: `foo`
    |
    = note: `-D clippy::redundant-closure` implied by `-D warnings`
 error: redundant closure found
   --> $DIR/eta.rs:21:10
    |
    |
 LL |     meta(|a| foo(a));
    |          ^^^^^^^^^^ help: remove closure as shown: `foo`
 
 error: this expression borrows a reference (`&u8`) that is immediately dereferenced by the compiler
   --> $DIR/eta.rs:24:21
    |
 LL |     all(&[1, 2, 3], &&2, |x, y| below(x, y)); //is adjusted
    |                     ^^^ help: change this to: `&2`
    |
    = note: `-D clippy::needless-borrow` implied by `-D warnings`
 error: redundant closure found
   --> $DIR/eta.rs:31:27
    |
    |
 LL |     let e = Some(1u8).map(|a| generic(a));
    |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `generic`
 error: redundant closure found
   --> $DIR/eta.rs:74:51
    |
    |
 LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.foo());
    |                                                   ^^^^^^^^^^^ help: remove closure as shown: `TestStruct::foo`
    |
    = note: `-D clippy::redundant-closure-for-method-calls` implied by `-D warnings`
 error: redundant closure found
   --> $DIR/eta.rs:76:51
    |
    |
 LL |     let e = Some(TestStruct { some_ref: &i }).map(|a| a.trait_foo());
    |                                                   ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `TestTrait::trait_foo`
 error: redundant closure found
   --> $DIR/eta.rs:79:42
    |
    |
 LL |     let e = Some(&mut vec![1, 2, 3]).map(|v| v.clear());
    |                                          ^^^^^^^^^^^^^ help: remove closure as shown: `std::vec::Vec::clear`
 error: redundant closure found
   --> $DIR/eta.rs:84:29
    |
    |
 LL |     let e = Some("str").map(|s| s.to_string());
    |                             ^^^^^^^^^^^^^^^^^ help: remove closure as shown: `std::string::ToString::to_string`
 error: redundant closure found
   --> $DIR/eta.rs:86:27
    |
    |
 LL |     let e = Some('a').map(|s| s.to_uppercase());
    |                           ^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_uppercase`
 error: redundant closure found
   --> $DIR/eta.rs:89:65
    |
    |
 LL |     let e: std::vec::Vec<char> = vec!['a', 'b', 'c'].iter().map(|c| c.to_ascii_uppercase()).collect();
    |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `char::to_ascii_uppercase`
-error: redundant closure found
-  --> $DIR/eta.rs:172:27
-   |
-   |
-LL |     let a = Some(1u8).map(|a| foo_ptr(a));
-   |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `foo_ptr`
+thread 'rustc' panicked at 'escaping bound vars for TraitPredicate(<&F as std::ops::FnOnce<(&X, &X)>>)', compiler/rustc_middle/src/ty/mod.rs:1361:9
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
-error: redundant closure found
-  --> $DIR/eta.rs:177:27
-   |
-   |
-LL |     let a = Some(1u8).map(|a| closure(a));
-   |                           ^^^^^^^^^^^^^^ help: remove closure as shown: `closure`
 
-error: aborting due to 12 previous errors
+note: the compiler unexpectedly panicked. this is a bug.
+
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
+
+note: Clippy version: clippy 0.1.51 (a8920a9 2021-01-04)
+query stack during panic:
+query stack during panic:
+#0 [type_implements_trait] evaluating `type_implements_trait` `(DefId(2:2304 ~ core[8e42]::ops::function::FnOnce), &F, [(&X, &X)], ParamEnv { caller_bounds: [Binder(ProjectionPredicate(ProjectionTy { substs: [F, (&X, &X)], item_def_id: DefId(2:2306 ~ core[8e42]::ops::function::FnOnce::Output) }, bool)), Binder(TraitPredicate(<F as std::ops::Fn<(&X, &X)>>)), Binder(TraitPredicate(<F as std::ops::FnMut<(&X, &X)>>)), Binder(TraitPredicate(<F as std::ops::FnOnce<(&X, &X)>>)), Binder(TraitPredicate(<F as std::marker::Sized>)), Binder(TraitPredicate(<X as std::marker::Sized>))], reveal: UserFacing })`
+#1 [analysis] running analysis passes on this crate
+end of query stack
 
 

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/eta.stderr
thread '[ui] ui/eta.rs' panicked at 'Could not retrieve suggestions from JSON: Error("expected ident", line: 11, column: 2)', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/runtest.rs:2311:15
normalized stderr:
normalized stderr:
error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:4:1
   |
LL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}
   |
   |
   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`

error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:6:1
   |
LL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:16:1
   |
LL | fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:45:1
   |
LL | fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:50:1
   |
LL | fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:62:1
   |
LL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:86:1
   |
LL | fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>


thread 'rustc' panicked at 'escaping bound vars for TraitPredicate(<&F as std::ops::FnOnce<(Lt<'x, I>,)>>)', compiler/rustc_middle/src/ty/mod.rs:1361:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.51 (a8920a9 2021-01-04)
query stack during panic:
query stack during panic:
#0 [type_implements_trait] evaluating `type_implements_trait` `(DefId(2:2304 ~ core[8e42]::ops::function::FnOnce), &F, [(Lt<'x, I>,)], ParamEnv { caller_bounds: [Binder(ProjectionPredicate(ProjectionTy { substs: [F, (Lt<'x, I>,)], item_def_id: DefId(2:2306 ~ core[8e42]::ops::function::FnOnce::Output) }, Lt<'x, I>)), Binder(TraitPredicate(<F as std::ops::Fn<(Lt<'x, I>,)>>)), Binder(TraitPredicate(<F as std::ops::FnMut<(Lt<'x, I>,)>>)), Binder(TraitPredicate(<F as std::ops::FnOnce<(Lt<'x, I>,)>>)), Binder(TraitPredicate(<I as std::marker::Sized>)), Binder(TraitPredicate(<F as std::marker::Sized>))], reveal: UserFacing })`
#1 [analysis] running analysis passes on this crate
end of query stack



expected stderr:
expected stderr:
error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:4:1
   |
LL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}
   |
   |
   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`

error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:6:1
   |
LL | fn distinct_and_static<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: &'static u8) {}


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:16:1
   |
LL | fn in_and_out<'a>(x: &'a u8, _y: u8) -> &'a u8 {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:45:1
   |
LL | fn deep_reference_3<'a>(x: &'a u8, _y: u8) -> Result<&'a u8, ()> {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:50:1
   |
LL | fn where_clause_without_lt<'a, T>(x: &'a u8, _y: u8) -> Result<&'a u8, ()>


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:62:1
   |
LL | fn lifetime_param_2<'a, 'b>(_x: Ref<'a>, _y: &'b u8) {}


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:86:1
   |
LL | fn fn_bound_2<'a, F, I>(_m: Lt<'a, I>, _f: F) -> Lt<'a, I>


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:120:5
   |
LL |     fn self_and_out<'s>(&'s self) -> &'s u8 {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:129:5
   |
LL |     fn distinct_self_and_in<'s, 't>(&'s self, _x: &'t u8) {}


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:148:1
   |
LL | fn struct_with_lt<'a>(_foo: Foo<'a>) -> &'a str {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:178:1
   |
LL | fn trait_obj_elided2<'a>(_arg: &'a dyn Drop) -> &'a str {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:184:1
   |
LL | fn alias_with_lt<'a>(_foo: FooAlias<'a>) -> &'a str {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:203:1
   |
LL | fn named_input_elided_output<'a>(_arg: &'a str) -> &str {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:211:1
   |
LL | fn trait_bound_ok<'a, T: WithLifetime<'static>>(_: &'a u8, _: T) {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:247:1
   |
LL | fn out_return_type_lts<'a>(e: &'a str) -> Cow<'a> {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:254:9
   |
LL |         fn needless_lt<'a>(x: &'a u8) {}


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:258:9
   |
LL |         fn needless_lt<'a>(_x: &'a u8) {}


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:271:9
   |
LL |         fn baz<'a>(&'a self) -> impl Foo + 'a {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:300:5
   |
LL |     fn impl_trait_elidable_nested_named_lifetimes<'a>(i: &'a i32, f: impl for<'b> Fn(&'b i32) -> &'b i32) -> &'a i32 {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:303:5
   |
LL |     fn impl_trait_elidable_nested_anonymous_lifetimes<'a>(i: &'a i32, f: impl Fn(&i32) -> &i32) -> &'a i32 {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:312:5
   |
LL |     fn generics_elidable<'a, T: Fn(&i32) -> &i32>(i: &'a i32, f: T) -> &'a i32 {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:324:5
   |
LL |     fn where_clause_elidadable<'a, T>(i: &'a i32, f: T) -> &'a i32


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:339:5
   |
LL |     fn pointer_fn_elidable<'a>(i: &'a i32, f: fn(&i32) -> &i32) -> &'a i32 {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:352:5
   |
LL |     fn nested_fn_pointer_3<'a>(_: &'a i32) -> fn(fn(&i32) -> &i32) -> i32 {


error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
  --> $DIR/needless_lifetimes.rs:355:5
   |
LL |     fn nested_fn_pointer_4<'a>(_: &'a i32) -> impl Fn(fn(&i32)) {

error: aborting due to 25 previous errors




diff of stderr:

 error: explicit lifetimes given in parameter types where they could be elided (or replaced with `'_` if needed by type declaration)
   --> $DIR/needless_lifetimes.rs:4:1
    |
 LL | fn distinct_lifetimes<'a, 'b>(_x: &'a u8, _y: &'b u8, _z: u8) {}
    |
