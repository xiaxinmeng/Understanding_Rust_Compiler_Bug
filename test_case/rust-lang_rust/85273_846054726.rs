plain
................................................................................F................... 3900/11893
...................................F................................................................ 4000/11893
.................................................................................................... 4100/11893
.................................................................................................... 4200/11893
.............................F..........F........................F.................................. 4300/11893
...................................................F................................................ 4400/11893
..................................................................................................ii 4500/11893
.................................................................................................... 4700/11893
.................................................................................................... 4800/11893
.................................................................................................... 4900/11893
.................................................................................................... 5000/11893
---
.................................................................................................... 10100/11893
.................................................................................................... 10200/11893
.................................................................................................... 10300/11893
.................................................................................................... 10400/11893
..........................................................................F......................F.. 10500/11893
.................i......................................i........................................... 10700/11893
.................................................................................................... 10800/11893
.................................................................................................... 10900/11893
.................................................................................................... 11000/11893
---
failures:

---- [ui] ui/async-await/issues/issue-65159.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-65159.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65159" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65159/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: this enum takes 2 generic arguments but 1 generic argument was supplied
   |
   |
LL | async fn copy() -> Result<()>
   |                    ^^^^^^ -- supplied 1 generic argument
   |                    expected 2 generic arguments
   |
   |
note: enum defined here, with 2 generic parameters: `T`, `E`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL | pub enum Result<T, E> {
   |          ^^^^^^ -  -
help: add missing generic argument
   |
LL | async fn copy() -> Result<(), E>

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/async-await/issues/issue-65159.rs:8:5
   |
   |
LL |     Ok(())
   |     ^^ cannot infer type for type parameter `E` declared on the enum `Result`

error: internal compiler error: compiler/rustc_mir_build/src/thir/cx/expr.rs:419:25: closure expr w/o closure type: [type error]
   |
LL | / {
LL | |     Ok(())
LL | |     Ok(())
LL | |     //~^ ERROR type annotations needed
LL | | }

thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (a66a23c1f 2021-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [thir_body] building THIR for `copy`
#1 [mir_built] building MIR for `copy`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0107, E0282.
For more information about an error, try `rustc --explain E0107`.
For more information about an error, try `rustc --explain E0107`.

------------------------------------------


---- [ui] ui/const-generics/const_evaluatable_checked/simple.rs#min stdout ----

error in revision `min`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const_evaluatable_checked/simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/simple.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/simple.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL | fn test<const N: usize>() -> [u8; N - 1] where [u8; N - 1]: Default {
   |                                                     ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL | fn test<const N: usize>() -> [u8; N - 1] where [u8; N - 1]: Default {
   |                                   ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions

error: internal compiler error: compiler/rustc_mir_build/src/thir/cx/expr.rs:942:18: res `Err` not yet implemented
   |
   |
LL | fn test<const N: usize>() -> [u8; N - 1] where [u8; N - 1]: Default {

thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (a66a23c1f 2021-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [thir_body] building THIR for `test::{constant#1}`
#1 [mir_built] building MIR for `test::{constant#1}`
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/min_const_generics/macro-fail.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/macro-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/macro-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/macro-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected type, found `{`
  --> /checkout/src/test/ui/const-generics/min_const_generics/macro-fail.rs:29:27
   |
LL | fn make_marker() -> impl Marker<gimme_a_const!(marker)> {
   |                                 |
   |                                 this macro call doesn't expand to a type
   |                                 in this macro invocation
...
...
LL |       ($rusty: ident) => {{ let $rusty = 3; *&$rusty }}
   |
   |
   = note: this error originates in the macro `gimme_a_const` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected type, found `{`
  --> /checkout/src/test/ui/const-generics/min_const_generics/macro-fail.rs:29:27
   |
   |
LL |   Example::<gimme_a_const!(marker)>
   |             |
   |             this macro call doesn't expand to a type
   |             in this macro invocation
...
...
LL |       ($rusty: ident) => {{ let $rusty = 3; *&$rusty }}
   |
   |
   = note: this error originates in the macro `gimme_a_const` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected type, found `{`
  --> /checkout/src/test/ui/const-generics/min_const_generics/macro-fail.rs:4:10
   |
LL |     () => {{
LL |     () => {{
   |  __________^
LL | |     //~^ ERROR expected type
LL | |     const X: usize = 1337;
LL | |     X
LL | |   }}
   | |___^ expected type
...
LL |     let _fail = Example::<external_macro!()>;
   |                           |
   |                           this macro call doesn't expand to a type
   |                           in this macro invocation
   |
   |
   = note: this error originates in the macro `external_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unexpected end of macro invocation
  --> /checkout/src/test/ui/const-generics/min_const_generics/macro-fail.rs:39:25
   |
LL |     macro_rules! gimme_a_const {
   |     -------------------------- when calling this macro
...
LL |   let _fail = Example::<gimme_a_const!()>;
   |                         ^^^^^^^^^^^^^^^^ missing tokens in macro arguments
error[E0747]: type provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/min_const_generics/macro-fail.rs:14:33
   |
   |
LL | fn make_marker() -> impl Marker<gimme_a_const!(marker)> {

error[E0747]: type provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/min_const_generics/macro-fail.rs:16:13
   |
   |
LL |   Example::<gimme_a_const!(marker)>


error: internal compiler error: compiler/rustc_mir_build/src/thir/cx/expr.rs:917:26: unexpected ty: [type error]
thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (a66a23c1f 2021-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [thir_body] building THIR for `make_marker`
#1 [mir_built] building MIR for `make_marker`
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0747`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-type_alias_impl_trait.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-type_alias_impl_trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-type_alias_impl_trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:10:9
   |
LL | type Foo = impl Debug;
   |            ---------- the expected opaque type
...
LL |     Bar(42) //~ ERROR mismatched types
   |         ^^ expected opaque type, found integer
   = note: expected opaque type `impl Debug`
   = note: expected opaque type `impl Debug`
                     found type `{integer}`
error[E0658]: type alias impl trait is not permitted here
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:16:19
   |
   |
LL |     let x = || -> Foo2 { 42 }; //~ ERROR not permitted here
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(type_alias_impl_trait)]` to the crate attributes to enable
error[E0308]: mismatched types
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:23:18
   |
LL | type Foo3 = impl Debug;
LL | type Foo3 = impl Debug;
   |             ---------- the found opaque type
...
LL |     let y: i32 = x; //~ ERROR mismatched types
   |            ---   ^ expected `i32`, found opaque type
   |            expected due to this
   |
   = note:     expected type `i32`
           found opaque type `impl Debug`
           found opaque type `impl Debug`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:26:13
   |
LL | type Foo3 = impl Debug;
   |             ---------- the expected opaque type
...
LL |     define3(42) //~ ERROR mismatched types
   |             ^^ expected opaque type, found integer
   = note: expected opaque type `impl Debug`
   = note: expected opaque type `impl Debug`
                     found type `{integer}`
error[E0658]: type alias impl trait is not permitted here
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:33:12
   |
LL |     let y: Foo4 = 42;
---
   |
LL | type Foo = impl Debug;
   |            ^^^^^^^^^^

error: internal compiler error: compiler/rustc_mir_build/src/thir/cx/mod.rs:77:48: const_eval_literal: had type error
thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (a66a23c1f 2021-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [thir_body] building THIR for `define2::{closure#0}`
#1 [mir_built] building MIR for `define2::{closure#0}`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/generator/layout-error.rs#full_tait stdout ----

error in revision `full_tait`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/layout-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/layout-error.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/layout-error.full_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `Foo` in this scope
  --> /checkout/src/test/ui/generator/layout-error.rs:25:17
   |
LL |         let a = Foo; //~ ERROR cannot find value `Foo` in this scope


warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full_tait, feature(impl_trait_in_bindings, type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information


warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full_tait, feature(impl_trait_in_bindings, type_alias_impl_trait))]
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information


error: internal compiler error: compiler/rustc_mir_build/src/thir/cx/expr.rs:942:18: res `Err` not yet implemented
   |
   |
LL |         let a = Foo; //~ ERROR cannot find value `Foo` in this scope

thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (a66a23c1f 2021-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [thir_body] building THIR for `main::cb::{closure#0}`
#1 [mir_built] building MIR for `main::cb::{closure#0}`
error: aborting due to 2 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0425`.


------------------------------------------


---- [ui] ui/generator/type-mismatch-signature-deduction.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/type-mismatch-signature-deduction.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/type-mismatch-signature-deduction" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/type-mismatch-signature-deduction/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/generator/type-mismatch-signature-deduction.rs:13:9
   |
LL |         5 //~ ERROR mismatched types [E0308]
   |         ^ expected enum `Result`, found integer
   |
   = note: expected type `Result<{integer}, _>`
              found type `{integer}`
note: return type inferred to be `Result<{integer}, _>` here
   |
LL |             return Ok(6);
   |                    ^^^^^


error[E0271]: type mismatch resolving `<[generator@/checkout/src/test/ui/generator/type-mismatch-signature-deduction.rs:6:5: 14:6] as Generator>::Return == i32`
   |
   |
LL | fn foo() -> impl Generator<Return = i32> { //~ ERROR type mismatch
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found enum `Result`
   = note: expected type `i32`
   = note: expected type `i32`
              found enum `Result<{integer}, _>`

error: internal compiler error: compiler/rustc_mir_build/src/thir/cx/expr.rs:419:25: closure expr w/o closure type: [type error]
   |
LL | /     || {
LL | |         if false {
LL | |             return Ok(6);
LL | |             return Ok(6);
LL | |         }
...  |
LL | |         5 //~ ERROR mismatched types [E0308]
LL | |     }

thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (a66a23c1f 2021-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [thir_body] building THIR for `foo`
#1 [mir_built] building MIR for `foo`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0271, E0308.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/impl-trait/bindings.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/impl-trait/bindings.rs:5:29
   |
LL |     const foo: impl Clone = x;
   |     ---------               ^ non-constant value
   |     |
   |     help: consider using `let` instead of `const`: `let foo`
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/impl-trait/bindings.rs:11:33
   |
   |
LL |         const foo: impl Clone = x;
   |         ---------               ^ non-constant value
   |         |
   |         help: consider using `let` instead of `const`: `let foo`
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/impl-trait/bindings.rs:18:33
   |
   |
LL |         const foo: impl Clone = x;
   |         ---------               ^ non-constant value
   |         |
   |         help: consider using `let` instead of `const`: `let foo`
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/impl-trait/bindings.rs:25:33
   |
   |
LL |         const foo: impl Clone = x;
   |         ---------               ^ non-constant value
   |         |
   |         help: consider using `let` instead of `const`: `let foo`

warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(impl_trait_in_bindings)]
   |            ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information

error: internal compiler error: compiler/rustc_mir_build/src/thir/cx/expr.rs:942:18: res `Err` not yet implemented
   |
   |
LL |     const foo: impl Clone = x;

thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (a66a23c1f 2021-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [thir_body] building THIR for `a::foo`
#1 [mir_built] building MIR for `a::foo`
error: aborting due to 5 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0435`.


------------------------------------------


---- [ui] ui/impl-trait/auto-trait-leak.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
   |
   |
LL | fn cycle1() -> impl Clone {
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
note: ...which requires computing type of `cycle2::{opaque#0}`...
   |
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
   = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | | fn send<T: Send>(_: T) {}
...  |
LL | |     Rc::new(String::from("foo"))
LL | | }


error[E0277]: `Rc<String>` cannot be sent between threads safely
   |
   |
LL | fn send<T: Send>(_: T) {}
   |            ---- required by this bound in `send`
...
LL |     send(cycle2().clone());
   |     ^^^^ `Rc<String>` cannot be sent between threads safely
...
LL | fn cycle2() -> impl Clone {
   |                ---------- within this `impl Clone`
   |
   = help: within `impl Clone`, the trait `Send` is not implemented for `Rc<String>`
   = note: required because it appears within the type `impl Clone`

error: internal compiler error: compiler/rustc_mir_build/src/thir/cx/mod.rs:77:48: const_eval_literal: had type error
thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (a66a23c1f 2021-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [thir_body] building THIR for `cycle1`
#1 [mir_built] building MIR for `cycle1`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0391.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/impl-trait/equality.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/equality.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/equality" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/equality/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(specialization)] //~ WARN the feature `specialization` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/equality.rs:15:5
   |
   |
LL | fn two(x: bool) -> impl Foo {
   |                    -------- expected because this return type...
LL |     if x {
LL |         return 1_i32;
   |                ----- ...is found to be `i32` here
LL |     0_u32
LL |     0_u32
   |     ^^^^^ expected `i32`, found `u32`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = help: if the trait `Foo` were object safe, you could return a boxed trait object
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type

error[E0277]: cannot add `impl Foo` to `u32`
   |
   |
LL |         n + sum_to(n - 1)
   |           ^ no implementation for `u32 + impl Foo`
   |
   = help: the trait `Add<impl Foo>` is not implemented for `u32`

error: internal compiler error: compiler/rustc_mir_build/src/thir/cx/mod.rs:77:48: const_eval_literal: had type error
thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (a66a23c1f 2021-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [thir_body] building THIR for `sum_to`
#1 [mir_built] building MIR for `sum_to`
error: aborting due to 3 previous errors; 1 warning emitted

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs:5:5
   |
LL | fn foo() -> impl std::fmt::Display {
   |             ---------------------- expected because this return type...
LL |     if false {
LL |         return 0i32;
   |                ---- ...is found to be `i32` here
LL |     }
LL |     1u32 //~ ERROR mismatched types
   |     ^^^^ expected `i32`, found `u32`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = help: you could instead create a new `enum` with a variant for each returned type
help: you could change the return type to be a boxed trait object
LL | fn foo() -> Box<dyn std::fmt::Display> {
   |             ^^^^^^^                  ^
   |             ^^^^^^^                  ^
help: if you change the return type to expect trait objects, box the returned expressions
   |
LL |         return Box::new(0i32);
LL |     }
LL |     Box::new(1u32) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/point-to-type-err-cause-on-impl-trait-return.rs:12:16
   |
   |
LL | fn bar() -> impl std::fmt::Display {
   |             ---------------------- expected because this return type...
LL |     if false {
LL |         return 0i32;
   |                ---- ...is found to be `i32` here
LL |     } else {
LL |         return 1u32; //~ ERROR mismatched types
   |                ^^^^ expected `i32`, found `u32`
   |
   = note: to return `impl Trait`, all returned values must be of the same type
---
test result: FAILED. 11783 passed; 13 failed; 97 ignored; 0 measured; 0 filtered out; finished in 122.18s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:41
