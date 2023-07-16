plain
.................................................................................................... 3300/12142
.................................................................................................... 3400/12142
.................................................................................................... 3500/12142
............................i........i.........i.................................................... 3600/12142
................F...............................................................F..............ii... 3700/12142
.............i............................................i......................................... 3900/12142
.................................................................................................... 4000/12142
.................................................................................................... 4100/12142
.................................................................................................... 4200/12142
.................................................................................................... 4200/12142
.................................................................................................... 4300/12142
.....................................................F................................F............. 4400/12142
.............F...................................................................................... 4500/12142
......................................................i............................................. 4700/12142
.................................................................................................... 4800/12142
.................................................................................................... 4900/12142
.................................................................................................... 5000/12142
---
---- [ui] ui/feature-gates/feature-gate-associated_type_bounds.rs stdout ----
diff of stderr:

115    = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
116    = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
117 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
120    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
121 LL | const _cdef: impl Tr1<As1: Copy> = S1;
122    |              ^^^^^^^^^^^^^^^^^^^
123 
123 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
126    |
126    |
127 LL | static _sdef: impl Tr1<As1: Copy> = S1;
128    |               ^^^^^^^^^^^^^^^^^^^
129 
129 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
132    |
132    |
133 LL |     let _: impl Tr1<As1: Copy> = S1;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/feature-gate-associated_type_bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-associated_type_bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:15:22
   |
LL |     type A: Iterator<Item: Copy>;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:19:22
   |
   |
LL |     type B: Iterator<Item: 'static>;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:23:20
   |
   |
LL | struct _St1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:30:18
   |
   |
LL | enum _En1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:37:19
   |
   |
LL | union _Un1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:44:37
   |
   |
LL | type _TaWhere1<T> where T: Iterator<Item: Copy> = T;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:47:22
   |
   |
LL | fn _apit(_: impl Tr1<As1: Copy>) {}
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:49:26
   |
   |
LL | fn _apit_dyn(_: &dyn Tr1<As1: Copy>) {}
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:52:24
   |
   |
LL | fn _rpit() -> impl Tr1<As1: Copy> { S1 }
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:55:31
   |
   |
LL | fn _rpit_dyn() -> Box<dyn Tr1<As1: Copy>> { Box::new(S1) }
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:58:23
   |
   |
LL | const _cdef: impl Tr1<As1: Copy> = S1;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:64:24
   |
   |
LL | static _sdef: impl Tr1<As1: Copy> = S1;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:71:21
   |
   |
LL |     let _: impl Tr1<As1: Copy> = S1;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable

error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | const _cdef: impl Tr1<As1: Copy> = S1;


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | static _sdef: impl Tr1<As1: Copy> = S1;


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL |     let _: impl Tr1<As1: Copy> = S1;


error[E0277]: the trait bound `<<Self as _Tr3>::A as Iterator>::Item: Copy` is not satisfied
   |
   |
LL |     type A: Iterator<Item: Copy>;
   |                            ^^^^ the trait `Copy` is not implemented for `<<Self as _Tr3>::A as Iterator>::Item`
  ::: /checkout/library/core/src/marker.rs:385:1
   |
LL | pub trait Copy: Clone {
   | --------------------- required by this bound in `Copy`
   | --------------------- required by this bound in `Copy`
   |
help: consider further restricting the associated type
   |
LL | trait _Tr3 where <<Self as _Tr3>::A as Iterator>::Item: Copy {

error: aborting due to 17 previous errors

Some errors have detailed explanations: E0277, E0562, E0658.
---
---- [ui] ui/feature-gates/feature-gate-min_type_alias_impl_trait.rs stdout ----
diff of stderr:

106    = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
107    = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable
108 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
111    |
111    |
112 LL |     type Assoc = impl Debug;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-min_type_alias_impl_trait/feature-gate-min_type_alias_impl_trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-min_type_alias_impl_trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-min_type_alias_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-min_type_alias_impl_trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-min_type_alias_impl_trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL | type Foo = impl Debug; //~ ERROR `impl Trait` in type aliases is unstable
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL |     type Baa = impl Debug; //~ ERROR `impl Trait` in type aliases is unstable
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable
error[E0658]: associated type defaults are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-min_type_alias_impl_trait.rs:23:5
   |
   |
LL |     type Assoc = impl Debug;
   |
   = note: see issue #29661 <https://github.com/rust-lang/rust/issues/29661> for more information
   = note: see issue #29661 <https://github.com/rust-lang/rust/issues/29661> for more information
   = help: add `#![feature(associated_type_defaults)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL |     type Assoc = impl Debug;
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL | type NestedFree = (Vec<impl Debug>, impl Debug, impl Iterator<Item = impl Debug>);
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL | type NestedFree = (Vec<impl Debug>, impl Debug, impl Iterator<Item = impl Debug>);
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL | type NestedFree = (Vec<impl Debug>, impl Debug, impl Iterator<Item = impl Debug>);
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL | type NestedFree = (Vec<impl Debug>, impl Debug, impl Iterator<Item = impl Debug>);
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL |     type Baa = (Vec<impl Debug>, impl Debug, impl Iterator<Item = impl Debug> + Debug);
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL |     type Baa = (Vec<impl Debug>, impl Debug, impl Iterator<Item = impl Debug> + Debug);
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL |     type Baa = (Vec<impl Debug>, impl Debug, impl Iterator<Item = impl Debug> + Debug);
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL |     type Baa = (Vec<impl Debug>, impl Debug, impl Iterator<Item = impl Debug> + Debug);
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL |     type Assoc = impl Debug;

error: aborting due to 13 previous errors

Some errors have detailed explanations: E0562, E0658.
---

---- [ui] ui/impl-trait/issues/issue-83929-impl-trait-in-generic-default.rs stdout ----
diff of stderr:

- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
3    |
3    |
4 LL | struct Foo<T = impl Copy>(T);
5    |                ^^^^^^^^^
6 
6 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
9    |
9    |
10 LL | type Result<T, E = impl std::error::Error> = std::result::Result<T, E>;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-83929-impl-trait-in-generic-default/issue-83929-impl-trait-in-generic-default.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/issues/issue-83929-impl-trait-in-generic-default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-83929-impl-trait-in-generic-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-83929-impl-trait-in-generic-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-83929-impl-trait-in-generic-default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | struct Foo<T = impl Copy>(T);


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | type Result<T, E = impl std::error::Error> = std::result::Result<T, E>;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0562`.
For more information about this error, try `rustc --explain E0562`.

------------------------------------------


---- [ui] ui/impl-trait/nested_impl_trait.rs stdout ----
diff of stderr:

34    |                                  |         nested `impl Trait` here
35    |                                  outer `impl Trait`
36 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
39    |
39    |
40 LL | fn bad_in_fn_syntax(x: fn() -> impl Into<impl Debug>) {}
41    |                                ^^^^^^^^^^^^^^^^^^^^^
42 
42 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
45    |
45    |
46 LL | fn allowed_in_ret_type() -> impl Fn() -> impl Into<u32> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested_impl_trait/nested_impl_trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/nested_impl_trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/nested_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested_impl_trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested_impl_trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0666]: nested `impl Trait` is not allowed
   |
   |
LL | fn bad_in_ret_position(x: impl Into<u32>) -> impl Into<impl Debug> { x }
   |                                              |         |
   |                                              |         |
   |                                              |         nested `impl Trait` here
   |                                              outer `impl Trait`

error[E0666]: nested `impl Trait` is not allowed
   |
   |
LL | fn bad_in_fn_syntax(x: fn() -> impl Into<impl Debug>) {}
   |                                |         |
   |                                |         |
   |                                |         nested `impl Trait` here
   |                                outer `impl Trait`

error[E0666]: nested `impl Trait` is not allowed
   |
   |
LL | fn bad_in_arg_position(_: impl Into<impl Debug>) { }
   |                           |         |
   |                           |         |
   |                           |         nested `impl Trait` here
   |                           outer `impl Trait`

error[E0666]: nested `impl Trait` is not allowed
   |
   |
LL |     fn bad(x: impl Into<u32>) -> impl Into<impl Debug> { x }
   |                                  |         |
   |                                  |         |
   |                                  |         nested `impl Trait` here
   |                                  outer `impl Trait`

error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn bad_in_fn_syntax(x: fn() -> impl Into<impl Debug>) {}


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn allowed_in_ret_type() -> impl Fn() -> impl Into<u32> {

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0562, E0666.
---
---- [ui] ui/impl-trait/where-allowed.rs stdout ----
diff of stderr:

43    = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
44    = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable
45 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
48    |
48    |
49 LL | fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }
50    |                                        ^^^^^^^^^^
51 
51 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
54    |
54    |
55 LL | fn in_fn_return_in_parameters(_: fn() -> impl Debug) { panic!() }
56    |                                          ^^^^^^^^^^
57 
57 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
60    |
60    |
61 LL | fn in_fn_parameter_in_return() -> fn(impl Debug) { panic!() }
62    |                                      ^^^^^^^^^^
63 
63 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
66    |
66    |
67 LL | fn in_fn_return_in_return() -> fn() -> impl Debug { panic!() }
68    |                                        ^^^^^^^^^^
69 
69 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
72    |
72    |
73 LL | fn in_dyn_Fn_parameter_in_parameters(_: &dyn Fn(impl Debug)) { panic!() }
74    |                                                 ^^^^^^^^^^
75 
75 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
78    |
78    |
79 LL | fn in_dyn_Fn_return_in_parameters(_: &dyn Fn() -> impl Debug) { panic!() }
80    |                                                   ^^^^^^^^^^
81 
81 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
84    |
84    |
85 LL | fn in_dyn_Fn_parameter_in_return() -> &'static dyn Fn(impl Debug) { panic!() }
86    |                                                       ^^^^^^^^^^
87 
87 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
90    |
90    |
91 LL | fn in_dyn_Fn_return_in_return() -> &'static dyn Fn() -> impl Debug { panic!() }
92    |                                                         ^^^^^^^^^^
93 
93 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
96    |
96    |
97 LL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }
98    |                                                   ^^^^^^^^^^
99 
99 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
102    |
102    |
103 LL | fn in_impl_Fn_return_in_parameters(_: &impl Fn() -> impl Debug) { panic!() }
104    |                                                     ^^^^^^^^^^
105 
105 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
108    |
108    |
109 LL | fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }
110    |                                                         ^^^^^^^^^^
111 
111 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
114    |
114    |
115 LL | fn in_impl_Fn_return_in_return() -> &'static impl Fn() -> impl Debug { panic!() }
116    |                                                           ^^^^^^^^^^
117 
117 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
120    |
120    |
121 LL | fn in_Fn_parameter_in_generics<F: Fn(impl Debug)> (_: F) { panic!() }
122    |                                      ^^^^^^^^^^
123 
123 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
126    |
126    |
127 LL | fn in_Fn_return_in_generics<F: Fn() -> impl Debug> (_: F) { panic!() }
128    |                                        ^^^^^^^^^^
129 
129 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
132    |
132    |
133 LL | struct InBraceStructField { x: impl Debug }
134    |                                ^^^^^^^^^^
135 
135 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
138    |
138    |
139 LL | struct InAdtInBraceStructField { x: Vec<impl Debug> }
140    |                                         ^^^^^^^^^^
141 
141 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
144    |
144    |
145 LL | struct InTupleStructField(impl Debug);
146    |                           ^^^^^^^^^^
147 
147 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
150    |
150    |
151 LL |     InBraceVariant { x: impl Debug },
152    |                         ^^^^^^^^^^
153 
153 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
156    |
156    |
157 LL |     InTupleVariant(impl Debug),
158    |                    ^^^^^^^^^^
159 
159 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
162    |
162    |
163 LL |     fn in_return() -> impl Debug;
164    |                       ^^^^^^^^^^
165 
165 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
168    |
168    |
169 LL |     fn in_trait_impl_return() -> impl Debug { () }
170    |                                  ^^^^^^^^^^
171 
171 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
174    |
174    |
175 LL |     fn in_foreign_parameters(_: impl Debug);
176    |                                 ^^^^^^^^^^
177 
177 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
180    |
180    |
181 LL |     fn in_foreign_return() -> impl Debug;
182    |                               ^^^^^^^^^^
183 
183 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
186    |
186    |
187 LL | type InReturnInTypeAlias<R> = fn() -> impl Debug;
188    |                                       ^^^^^^^^^^
189 
189 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
192    |
192    |
193 LL | impl PartialEq<impl Debug> for () {
194    |                ^^^^^^^^^^
195 
195 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
198    |
198    |
199 LL | impl PartialEq<()> for impl Debug {
200    |                        ^^^^^^^^^^
201 
201 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
204    |
205 LL | impl impl Debug {

206    |      ^^^^^^^^^^
206    |      ^^^^^^^^^^
207 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
210    |
210    |
211 LL | impl InInherentImplAdt<impl Debug> {
212    |                        ^^^^^^^^^^
213 
213 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
216    |
216    |
217 LL |     where impl Debug: Debug
218    |           ^^^^^^^^^^
219 
219 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
222    |
222    |
223 LL |     where Vec<impl Debug>: Debug
224    |               ^^^^^^^^^^
225 
225 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
228    |
228    |
229 LL |     where T: PartialEq<impl Debug>
230    |                        ^^^^^^^^^^
231 
231 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
234    |
234    |
235 LL |     where T: Fn(impl Debug)
236    |                 ^^^^^^^^^^
237 
237 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
240    |
240    |
241 LL |     where T: Fn() -> impl Debug
242    |                      ^^^^^^^^^^
243 
243 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
246    |
246    |
247 LL | struct InStructGenericParamDefault<T = impl Debug>(T);
248    |                                        ^^^^^^^^^^
249 
249 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
252    |
252    |
253 LL | enum InEnumGenericParamDefault<T = impl Debug> { Variant(T) }
254    |                                    ^^^^^^^^^^
255 
255 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
258    |
258    |
259 LL | trait InTraitGenericParamDefault<T = impl Debug> {}
260    |                                      ^^^^^^^^^^
261 
261 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
264    |
264    |
265 LL | type InTypeAliasGenericParamDefault<T = impl Debug> = T;
266    |                                         ^^^^^^^^^^
267 
267 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
270    |
270    |
271 LL | impl <T = impl Debug> T {}
272    |           ^^^^^^^^^^
273 
273 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
276    |
276    |
277 LL | fn in_method_generic_param_default<T = impl Debug>(_: T) {}
278    |                                        ^^^^^^^^^^
279 
279 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
282    |
282    |
283 LL |     let _in_local_variable: impl Fn() = || {};
284    |                             ^^^^^^^^^
285 
285 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
288    |
288    |
289 LL |     let _in_return_in_local_variable = || -> impl Fn() { || {} };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/where-allowed.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/where-allowed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/where-allowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0666]: nested `impl Trait` is not allowed
   |
   |
LL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }
   |                                           |       |
   |                                           |       |
   |                                           |       nested `impl Trait` here
   |                                           outer `impl Trait`

error[E0666]: nested `impl Trait` is not allowed
   |
   |
LL | fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }
   |                                                 |       |
   |                                                 |       |
   |                                                 |       nested `impl Trait` here
   |                                                 outer `impl Trait`

error[E0658]: `impl Trait` in type aliases is unstable
   |
LL |     type Out = impl Debug;
   |                ^^^^^^^^^^
   |
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL | type InTypeAlias<R> = impl Debug;
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0658]: `impl Trait` in type aliases is unstable
   |
   |
LL | type InReturnInTypeAlias<R> = fn() -> impl Debug;
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_fn_return_in_parameters(_: fn() -> impl Debug) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_fn_parameter_in_return() -> fn(impl Debug) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_fn_return_in_return() -> fn() -> impl Debug { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_dyn_Fn_parameter_in_parameters(_: &dyn Fn(impl Debug)) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_dyn_Fn_return_in_parameters(_: &dyn Fn() -> impl Debug) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_dyn_Fn_parameter_in_return() -> &'static dyn Fn(impl Debug) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_dyn_Fn_return_in_return() -> &'static dyn Fn() -> impl Debug { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_impl_Fn_return_in_parameters(_: &impl Fn() -> impl Debug) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_impl_Fn_return_in_return() -> &'static impl Fn() -> impl Debug { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_Fn_parameter_in_generics<F: Fn(impl Debug)> (_: F) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | fn in_Fn_return_in_generics<F: Fn() -> impl Debug> (_: F) { panic!() }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | struct InBraceStructField { x: impl Debug }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | struct InAdtInBraceStructField { x: Vec<impl Debug> }


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | struct InTupleStructField(impl Debug);


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL |     InBraceVariant { x: impl Debug },


---

---- [ui] ui/issues/issue-47715.rs stdout ----
diff of stderr:

- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
3    |
3    |
4 LL | struct Container<T: Iterable<Item = impl Foo>> {
5    |                                     ^^^^^^^^
6 
6 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
9    |
9    |
10 LL | enum Enum<T: Iterable<Item = impl Foo>> {
11    |                              ^^^^^^^^
12 
12 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
15    |
15    |
16 LL | union Union<T: Iterable<Item = impl Foo> + Copy> {
17    |                                ^^^^^^^^
18 
18 
- error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+ error[E0562]: `impl Trait` not allowed outside of function and method return types
21    |
21    |
22 LL | type Type<T: Iterable<Item = impl Foo>> = T;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47715/issue-47715.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-47715.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47715.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47715" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47715/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | struct Container<T: Iterable<Item = impl Foo>> {


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | enum Enum<T: Iterable<Item = impl Foo>> {


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | union Union<T: Iterable<Item = impl Foo> + Copy> {


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | type Type<T: Iterable<Item = impl Foo>> = T;

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0562`.
---
test result: FAILED. 12034 passed; 6 failed; 102 ignored; 0 measured; 0 filtered out; finished in 107.29s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:48
