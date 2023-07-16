plain
........................................................................................ 1672/13115
......................i.....ii.......................................................... 1760/13115
........................................................................................ 1848/13115
............................................................................i........... 1936/13115
.........................F...F..........F............................................... 2024/13115
..................................................FF.F...........F...........F...F...... 2112/13115
....FF.....F....................F.....................................................F. 2200/13115
.................................F...FFF.F.F....FF..F.F.F....F.......................... 2288/13115
........................................................................................ 2464/13115
........................................................................................ 2552/13115
........................................................................................ 2640/13115
........................................................................................ 2728/13115
---
---- [ui] src/test/ui/associated-consts/issue-88599-ref-self.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-88599-ref-self" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-88599-ref-self/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     [u8; Self::CONST]:
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::CONST]:`
note: required by a bound in `Third`
   |
   |
LL | trait Third: First
   |       ----- required by a bound in this
LL | where
LL |     [u8; Self::CONST]:
   |          ^^^^^^^^^^^ required by this bound in `Third`
error: unconstrained generic constant
  --> /checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs:21:5
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::CONST]:`
note: required by a bound in `Third`
   |
   |
LL | trait Third: First
   |       ----- required by a bound in this
LL | where
LL |     [u8; Self::CONST]:
   |          ^^^^^^^^^^^ required by this bound in `Third`
error: unconstrained generic constant
  --> /checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs:21:5
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::CONST]:`
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     [(); T::ASSOC]: ;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::ASSOC]:`
note: required by a bound in `Iced`
   |
   |
LL | struct Iced<T: Foo>(T, [(); T::ASSOC])
   |        ---- required by a bound in this
LL | where
LL |     [(); T::ASSOC]: ;
   |          ^^^^^^^^ required by this bound in `Iced`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1.rs:12:24
   |
   |
LL | struct Iced<T: Foo>(T, [(); T::ASSOC])
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::ASSOC]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1.rs:20:5
   |
   |
LL |     [(); T::ASSOC]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::ASSOC]:`
note: required by a bound in `foo`
   |
LL | fn foo<T: Foo>()
   |    --- required by a bound in this
LL | where
LL | where
LL |     [(); T::ASSOC]: ,
   |          ^^^^^^^^ required by this bound in `foo`
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     [(); T::ASSOC]: ;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::ASSOC]:`
note: required by a bound in `Iced`
   |
   |
LL | struct Iced<T: Foo>(T, [(); T::ASSOC])
   |        ---- required by a bound in this
LL | where
LL |     [(); T::ASSOC]: ;
   |          ^^^^^^^^ required by this bound in `Iced`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2.rs:12:24
   |
   |
LL | struct Iced<T: Foo>(T, [(); T::ASSOC])
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::ASSOC]:`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/const-argument-if-length.rs#full stdout ----
diff of stderr:

1 error[E0277]: the size for values of type `T` cannot be known at compilation time
+   --> $DIR/const-argument-if-length.rs:7:28
+    |
+ LL | pub const fn is_zst<T: ?Sized>() -> usize {
+    |                     - this type parameter needs to be `std::marker::Sized`
+ LL |     if std::mem::size_of::<T>() == 0 {
+    |
+ note: required by a bound in `std::mem::size_of`
+   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
+    |
+    |
+ LL | pub const fn size_of<T>() -> usize {
+    |                      ^ required by this bound in `std::mem::size_of`
+ help: consider removing the `?Sized` bound to make the type parameter `Sized`
+    |
+ LL - pub const fn is_zst<T: ?Sized>() -> usize {
+ LL + pub const fn is_zst<T>() -> usize {
+ 
+ error[E0277]: the size for values of type `T` cannot be known at compilation time
2   --> $DIR/const-argument-if-length.rs:15:12
3    |
3    |
4 LL | pub struct AtLeastByte<T: ?Sized> {
21    |
22 LL |     value: Box<T>,
23    |            ++++ +
- 
- 
- error: unconstrained generic constant
-   --> $DIR/const-argument-if-length.rs:17:10
-    |
- LL |     pad: [u8; is_zst::<T>()],
-    |
-    |
-    = help: try adding a `where` bound using this expression: `where [(); is_zst::<T>()]:`
33 error: aborting due to 2 previous errors
34 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.full/const-argument-if-length.full.stderr
To only update this specific test, also pass `--test-args const-generics/const-argument-if-length.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-argument-if-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.full/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
   |
   |
LL | pub const fn is_zst<T: ?Sized>() -> usize {
   |                     - this type parameter needs to be `std::marker::Sized`
LL |     if std::mem::size_of::<T>() == 0 {
   |
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:310:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL - pub const fn is_zst<T: ?Sized>() -> usize {
LL + pub const fn is_zst<T>() -> usize {

error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/const-generics/const-argument-if-length.rs:15:12
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL | pub struct AtLeastByte<T: ?Sized> {
   |                        - this type parameter needs to be `std::marker::Sized`
LL |     value: T,
   |
   |
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL - pub struct AtLeastByte<T: ?Sized> {
LL + pub struct AtLeastByte<T> {
   |
help: borrowed types always have a statically known size
LL |     value: &T,
   |            +
   |            +
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     value: Box<T>,
   |            ++++ +

error: aborting due to 2 previous errors
---

---- [ui] src/test/ui/const-generics/generic_const_exprs/different-fn.rs stdout ----
diff of stderr:

7    = note: expected type `size_of::<T>()`
8               found type `size_of::<Foo<T>>()`
- error: unconstrained generic constant
-   --> $DIR/different-fn.rs:10:9
-    |
-    |
- LL |     [0; size_of::<Foo<T>>()]
-    |
-    |
-    = help: try adding a `where` bound using this expression: `where [(); size_of::<Foo<T>>()]:`
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
19 
20 For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/different-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/different-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/different-fn/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs:10:5
   |
   |
LL |     [0; size_of::<Foo<T>>()]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ expected `size_of::<T>()`, found `size_of::<Foo<T>>()`
   = note: expected type `size_of::<T>()`
   = note: expected type `size_of::<T>()`
              found type `size_of::<Foo<T>>()`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/dependence_lint.rs#gce stdout ----
diff of stderr:

14    |
15    = help: consider moving this anonymous constant into a `const` function
- error: unconstrained generic constant
-   --> $DIR/dependence_lint.rs:13:12
-    |
-    |
- LL |     let _: [u8; size_of::<*mut T>()]; // error on stable, error with gce
-    |
-    |
-    = help: try adding a `where` bound using this expression: `where [(); size_of::<*mut T>()]:`
- error: unconstrained generic constant
-   --> $DIR/dependence_lint.rs:9:9
-    |
-    |
- LL |     [0; size_of::<*mut T>()]; // lint on stable, error with `generic_const_exprs`
-    |
-    |
-    = help: try adding a `where` bound using this expression: `where [(); size_of::<*mut T>()]:`
- error: aborting due to 4 previous errors
+ error: aborting due to 2 previous errors
34 
35 
35 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/dependence_lint.gce/dependence_lint.gce.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/dependence_lint.rs`


error in revision `gce`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/dependence_lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "gce" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/dependence_lint.gce" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/dependence_lint.gce/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL |     [0; if false { size_of::<T>() } else { 3 }]; // lint on stable, error with gce
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ control flow is not supported in generic constants
   |
   = help: consider moving this anonymous constant into a `const` function
error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/dependence_lint.rs:20:17
   |
   |
LL |     let _: [u8; if true { size_of::<T>() } else { 3 }]; // error on stable, error with gce
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ control flow is not supported in generic constants
   |
   = help: consider moving this anonymous constant into a `const` function
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/generic_const_exprs/dont-eagerly-error-in-is-const-evaluatable.rs stdout ----

error: test compilation failed although it shouldn't!
error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/dont-eagerly-error-in-is-const-evaluatable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/dont-eagerly-error-in-is-const-evaluatable/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/dont-eagerly-error-in-is-const-evaluatable/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: unexpected `TooGeneric` for Unevaluated { def: WithOptConstParam { did: DefId(0:8 ~ dont_eagerly_error_in_is_const_evaluatable[6da3]::{impl#0}::{constant#0}), const_param_did: None }, substs: [T], promoted: () }
   |
   |
LL | fn foo<T: Foo>() {}
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:102:22

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7fa78ee55a6c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4d1d5b5d7ad6ca32
   1:     0x7fa78eebc928 - core::fmt::write::h5d0178c8399d33ee
   2:     0x7fa78ee457d1 - std::io::Write::write_fmt::h7d73abf5d94af071
   3:     0x7fa78ee58a5e - std::panicking::default_hook::{{closure}}::h870525f2537f6e6a
   4:     0x7fa78ee58749 - std::panicking::default_hook::h5733045cfed6fd61
   5:     0x7fa78f976fb4 - rustc_driver[d0b4eace83cf17af]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa78ee591c2 - std::panicking::rust_panic_with_hook::hceb3b3934b6eff21
   7:     0x7fa792581443 - std[72292dcfdc22d8d]::panicking::begin_panic::<rustc_errors[3fc7d5a664f3d9a0]::ExplicitBug>::{closure#0}
   8:     0x7fa79257df56 - std[72292dcfdc22d8d]::sys_common::backtrace::__rust_end_short_backtrace::<std[72292dcfdc22d8d]::panicking::begin_panic<rustc_errors[3fc7d5a664f3d9a0]::ExplicitBug>::{closure#0}, !>
   9:     0x7fa78f934d46 - std[72292dcfdc22d8d]::panicking::begin_panic::<rustc_errors[3fc7d5a664f3d9a0]::ExplicitBug>
  10:     0x7fa792595756 - std[72292dcfdc22d8d]::panic::panic_any::<rustc_errors[3fc7d5a664f3d9a0]::ExplicitBug>
  11:     0x7fa79259a5cd - <rustc_errors[3fc7d5a664f3d9a0]::HandlerInner as core[d44a7f951e4a6a35]::ops::drop::Drop>::drop
  12:     0x7fa78f98f4a2 - core[d44a7f951e4a6a35]::ptr::drop_in_place::<rustc_session[b280f6ea5c2255ea]::parse::ParseSess>
  13:     0x7fa78f99305d - <alloc[975d494ed09dbcfb]::rc::Rc<rustc_session[b280f6ea5c2255ea]::session::Session> as core[d44a7f951e4a6a35]::ops::drop::Drop>::drop
  14:     0x7fa78f9edbfc - core[d44a7f951e4a6a35]::ptr::drop_in_place::<rustc_interface[12a4464204ea5fc8]::interface::Compiler>
  15:     0x7fa78f9ec24b - rustc_span[c3173e39c285156b]::with_source_map::<core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>, rustc_interface[12a4464204ea5fc8]::interface::create_compiler_and_run<core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>, rustc_driver[d0b4eace83cf17af]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fa78f995ba1 - <scoped_tls[e726221942e6035f]::ScopedKey<rustc_span[c3173e39c285156b]::SessionGlobals>>::set::<rustc_interface[12a4464204ea5fc8]::interface::run_compiler<core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>, rustc_driver[d0b4eace83cf17af]::run_compiler::{closure#1}>::{closure#0}, core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>>
  17:     0x7fa78f9e1639 - std[72292dcfdc22d8d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[12a4464204ea5fc8]::util::run_in_thread_pool_with_globals<rustc_interface[12a4464204ea5fc8]::interface::run_compiler<core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>, rustc_driver[d0b4eace83cf17af]::run_compiler::{closure#1}>::{closure#0}, core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>>::{closure#0}, core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>>
  18:     0x7fa78f9e22d9 - <<std[72292dcfdc22d8d]::thread::Builder>::spawn_unchecked_<rustc_interface[12a4464204ea5fc8]::util::run_in_thread_pool_with_globals<rustc_interface[12a4464204ea5fc8]::interface::run_compiler<core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>, rustc_driver[d0b4eace83cf17af]::run_compiler::{closure#1}>::{closure#0}, core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>>::{closure#0}, core[d44a7f951e4a6a35]::result::Result<(), rustc_errors[3fc7d5a664f3d9a0]::ErrorGuaranteed>>::{closure#1} as core[d44a7f951e4a6a35]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7fa78ee645a3 - std::sys::unix::thread::Thread::new::thread_start::h9c7f768f0982f0bf
  20:     0x7fa7893b7609 - start_thread
  21:     0x7fa78ecca133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (cc04042fc 2022-06-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |         [(); Self::N]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::N]:`
note: required by a bound in `Bar::bar`
   |
LL |     fn bar()
   |        --- required by a bound in this
LL |     where
LL |     where
LL |         [(); Self::N]: ,
   |              ^^^^^^^ required by this bound in `Bar::bar`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-83765.rs stdout ----
diff of stderr:

- error[E0308]: method not compatible with trait
-   --> $DIR/issue-83765.rs:30:5
-    |
- LL |     fn size(&self) -> [usize; DIM] {
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
-    = note: expected type `Self::DIM`
-               found type `DIM`
- 
10 error: unconstrained generic constant
10 error: unconstrained generic constant
-   --> $DIR/issue-83765.rs:32:24
+   --> $DIR/issue-83765.rs:9:31
12    |
- LL |         self.reference.size()
-    |                        ^^^^
+ LL |     fn size(&self) -> [usize; Self::DIM];
15    |
15    |
16    = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
17 note: required by a bound in `TensorSize::size`

20 LL |     fn size(&self) -> [usize; Self::DIM];
21    |                               ^^^^^^^^^ required by this bound in `TensorSize::size`
- error[E0308]: mismatched types
-   --> $DIR/issue-83765.rs:32:9
-    |
- LL |         self.reference.size()
- LL |         self.reference.size()
-    |         ^^^^^^^^^^^^^^^^^^^^^ expected `DIM`, found `Self::DIM`
-    = note: expected type `DIM`
-               found type `Self::DIM`
+ error: aborting due to previous error
31 
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83765" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83765/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn size(&self) -> [usize; Self::DIM];
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
note: required by a bound in `TensorSize::size`
   |
   |
LL |     fn size(&self) -> [usize; Self::DIM];
   |                               ^^^^^^^^^ required by this bound in `TensorSize::size`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/object-safety-err-ret.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-err-ret.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-err-ret" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-err-ret/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-err-where-bounds/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/generic_const_exprs/unify-op-with-fn-call.rs stdout ----
diff of stderr:


- error: unconstrained generic constant
-   --> $DIR/unify-op-with-fn-call.rs:28:12
+ error: `<usize as Add>::add` is not yet stable as a const fn
3    |
3    |
4 LL |     bar2::<{ std::ops::Add::add(N, N) }>();
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
-    = help: try adding a `where` bound using this expression: `where [(); { std::ops::Add::add(N, N) }]:`
+    = help: add `#![feature(const_ops)]` to the crate attributes to enable
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unify-op-with-fn-call/unify-op-with-fn-call.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/unify-op-with-fn-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/unify-op-with-fn-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unify-op-with-fn-call" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unify-op-with-fn-call/auxiliary"
stdout: none
--- stderr -------------------------------
error: `<usize as Add>::add` is not yet stable as a const fn
   |
   |
LL |     bar2::<{ std::ops::Add::add(N, N) }>();
   |
   = help: add `#![feature(const_ops)]` to the crate attributes to enable

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/const-generics/invariant.rs stdout ----
diff of stderr:

12    = note: for more information, see issue #56105 <https://github.com/rust-lang/rust/issues/56105>
13    = note: this behavior recently changed as a result of a bug fix; see rust-lang/rust#56105 for details
- error[E0308]: mismatched types
-   --> $DIR/invariant.rs:27:5
+ error: unconstrained generic constant
+   --> $DIR/invariant.rs:22:5
+   --> $DIR/invariant.rs:22:5
17    |
- LL |     v
-    |     ^ one type is more general than the other
+ LL |     [(); <T as SadBee>::ASSOC]: ;
20    |
20    |
-    = note: expected reference `&Foo<fn(&())>`
-               found reference `&Foo<for<'a> fn(&'a ())>`
+    = help: try adding a `where` bound using this expression: `where [(); <T as SadBee>::ASSOC]:`
+ note: required by a bound in `Foo`
+    |
+    |
+ LL | struct Foo<T: SadBee>([u8; <T as SadBee>::ASSOC], PhantomData<T>)
+    |        --- required by a bound in this
+ LL | where
+ LL |     [(); <T as SadBee>::ASSOC]: ;
+    |          ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Foo`
- error: aborting due to previous error; 1 warning emitted
+ error: unconstrained generic constant
+   --> $DIR/invariant.rs:20:23
+    |
+    |
+ LL | struct Foo<T: SadBee>([u8; <T as SadBee>::ASSOC], PhantomData<T>)
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); <T as SadBee>::ASSOC]:`
- For more information about this error, try `rustc --explain E0308`.
+ error: aborting due to 2 previous errors; 1 warning emitted
+ 
27 
---
To only update this specific test, also pass `--test-args const-generics/invariant.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invariant" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invariant/auxiliary"
stdout: none
--- stderr -------------------------------
warning: conflicting implementations of trait `SadBee` for type `for<'a> fn(&'a ())`
   |
   |
LL | impl SadBee for for<'a> fn(&'a ()) {
   | ---------------------------------- first implementation here
...
LL | impl SadBee for fn(&'static ()) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `for<'a> fn(&'a ())`
   |
   = note: `#[warn(coherence_leak_check)]` on by default
   = note: for more information, see issue #56105 <https://github.com/rust-lang/rust/issues/56105>
   = note: for more information, see issue #56105 <https://github.com/rust-lang/rust/issues/56105>
   = note: this behavior recently changed as a result of a bug fix; see rust-lang/rust#56105 for details
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/invariant.rs:22:5
   |
   |
LL |     [(); <T as SadBee>::ASSOC]: ;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); <T as SadBee>::ASSOC]:`
note: required by a bound in `Foo`
   |
   |
LL | struct Foo<T: SadBee>([u8; <T as SadBee>::ASSOC], PhantomData<T>)
   |        --- required by a bound in this
LL | where
LL |     [(); <T as SadBee>::ASSOC]: ;
   |          ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/invariant.rs:20:23
   |
   |
LL | struct Foo<T: SadBee>([u8; <T as SadBee>::ASSOC], PhantomData<T>)
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); <T as SadBee>::ASSOC]:`
error: aborting due to 2 previous errors; 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-71202.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-71202.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-71202" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-71202/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/issues/issue-83249.rs stdout ----
diff of stderr:


- error[E0282]: type annotations needed
-   --> $DIR/issue-83249.rs:19:9
+ error: unconstrained generic constant
+   --> $DIR/issue-83249.rs:12:24
3    |
- LL |     let _ = foo([0; 1]);
-    |         ^
+ LL | fn foo<T: Foo>(_: [u8; T::N]) -> T {
6    |
- help: consider giving this pattern a type
- help: consider giving this pattern a type
+    = help: try adding a `where` bound using this expression: `where [(); T::N]:`
+ note: required by a bound in `foo`
8    |
8    |
- LL |     let _: _ = foo([0; 1]);
-    |          +++
+ LL | fn foo<T: Foo>(_: [u8; T::N]) -> T {
+    |                        ^^^^ required by this bound in `foo`
12 error: aborting due to previous error
13 

- For more information about this error, try `rustc --explain E0282`.
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-83249.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83249.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83249" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83249/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL | fn foo<T: Foo>(_: [u8; T::N]) -> T {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::N]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<T: Foo>(_: [u8; T::N]) -> T {
   |                        ^^^^ required by this bound in `foo`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-86530.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `&str: X` is not satisfied
-   --> $DIR/issue-86530.rs:16:7
-    |
- LL |     z(" ");
-    |     - ^^^ the trait `X` is not implemented for `&str`
-    |     required by a bound introduced by this call
-    |
- note: required by a bound in `z`
-   --> $DIR/issue-86530.rs:10:8
-   --> $DIR/issue-86530.rs:10:8
-    |
- LL | fn z<T>(t: T)
-    |    - required by a bound in this
- LL | where
- LL |     T: X,
-    |        ^ required by this bound in `z`
18 error: unconstrained generic constant
-   --> $DIR/issue-86530.rs:16:5
+   --> $DIR/issue-86530.rs:11:5
20    |
20    |
- LL |     z(" ");
-    |     ^
+ LL |     [(); T::Y]: ,
23    |
23    |
24    = help: try adding a `where` bound using this expression: `where [(); T::Y]:`
25 note: required by a bound in `z`

31 LL |     [(); T::Y]: ,
32    |          ^^^^ required by this bound in `z`
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
35 
- For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-86530.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-86530.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86530" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86530/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     [(); T::Y]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::Y]:`
note: required by a bound in `z`
   |
   |
LL | fn z<T>(t: T)
   |    - required by a bound in this
...
LL |     [(); T::Y]: ,
   |          ^^^^ required by this bound in `z`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-86535-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-86535-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86535-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86535-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn foo() where [(); Self::ASSOC_C]:;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::ASSOC_C]:`
note: required by a bound in `Foo::foo`
   |
   |
LL |     fn foo() where [(); Self::ASSOC_C]:;
   |                         ^^^^^^^^^^^^^ required by this bound in `Foo::foo`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-83288.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83288.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83288" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83288/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
note: required by a bound in `Tensor`
   |
   |
LL | pub struct Tensor<I: Indices<N>, const N: usize>
   |            ------ required by a bound in this
LL | where
LL |     [u8; I::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `Tensor`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:47:15
   |
   |
LL |     pub data: [u8; I::NUM_ELEMS],
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:55:10
   |
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
note: required by a bound in `<Tensor<I, N> as Mul<Tensor<J, N>>>`
   |
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `<Tensor<I, N> as Mul<Tensor<J, N>>>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:55:10
   |
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); J::NUM_ELEMS]:`
note: required by a bound in `<Tensor<I, N> as Mul<Tensor<J, N>>>`
   |
   |
LL |     [u8; J::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `<Tensor<I, N> as Mul<Tensor<J, N>>>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:55:10
   |
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); <I as Concat<J>>::Output::NUM_ELEMS]:`
note: required by a bound in `<Tensor<I, N> as Mul<Tensor<J, N>>>`
   |
   |
LL |     [u8; <I as Concat<J>>::Output::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `<Tensor<I, N> as Mul<Tensor<J, N>>>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:51:74
   |
   |
LL | impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
note: required by a bound in `Tensor`
   |
   |
LL | pub struct Tensor<I: Indices<N>, const N: usize>
   |            ------ required by a bound in this
LL | where
LL |     [u8; I::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `Tensor`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:51:52
   |
   |
LL | impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
note: required by a bound in `Tensor`
   |
   |
LL | pub struct Tensor<I: Indices<N>, const N: usize>
   |            ------ required by a bound in this
LL | where
LL |     [u8; I::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `Tensor`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:59:19
   |
   |
LL |     type Output = Tensor<<I as Concat<J>>::Output, N>;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
note: required by a bound in `Tensor`
   |
   |
LL | pub struct Tensor<I: Indices<N>, const N: usize>
   |            ------ required by a bound in this
LL | where
LL |     [u8; I::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `Tensor`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:8
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
note: required because of the requirements on the impl of `Mul<Tensor<J, N>>` for `Tensor<I, N>`
   |
   |
LL | impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:8
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); J::NUM_ELEMS]:`
note: required because of the requirements on the impl of `Mul<Tensor<J, N>>` for `Tensor<I, N>`
   |
   |
LL | impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:8
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); <I as Concat<J>>::Output::NUM_ELEMS]:`
note: required because of the requirements on the impl of `Mul<Tensor<J, N>>` for `Tensor<I, N>`
   |
   |
LL | impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:12
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
note: required by a bound in `Tensor`
   |
   |
LL | pub struct Tensor<I: Indices<N>, const N: usize>
   |            ------ required by a bound in this
LL | where
LL |     [u8; I::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `Tensor`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:24
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
note: required by a bound in `Tensor`
   |
   |
LL | pub struct Tensor<I: Indices<N>, const N: usize>
   |            ------ required by a bound in this
LL | where
LL |     [u8; I::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `Tensor`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:41
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); I::NUM_ELEMS]:`
note: required by a bound in `Tensor`
   |
   |
LL | pub struct Tensor<I: Indices<N>, const N: usize>
   |            ------ required by a bound in this
LL | where
LL |     [u8; I::NUM_ELEMS]: Sized,
   |          ^^^^^^^^^^^^ required by this bound in `Tensor`
error: aborting due to 14 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-86535.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-86535.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86535/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86535/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn d(r: &[u8; Self::W]) -> Self;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::W]:`
note: required by a bound in `X::d`
   |
   |
LL |     fn d(r: &[u8; Self::W]) -> Self;
   |                   ^^^^^^^ required by this bound in `X::d`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-83765.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`
-   --> $DIR/issue-83765.rs:5:5
+ error: unconstrained generic constant
3    |
3    |
- LL |     const DIM: usize;
-    |     ^^^^^^^^^^^^^^^^^
+ LL |     fn size(&self) -> [usize; Self::DIM];
6    |
6    |
- note: ...which requires checking if `TensorDimension` fulfills its obligations...
-   --> $DIR/issue-83765.rs:4:1
+    = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
+ note: required by a bound in `TensorSize::size`
9    |
9    |
- LL | trait TensorDimension {
-    | ^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`, completing the cycle
- note: cycle used when checking if `TensorDimension` fulfills its obligations
-   --> $DIR/issue-83765.rs:4:1
+ LL |     fn size(&self) -> [usize; Self::DIM];
+    |                               ^^^^^^^^^ required by this bound in `TensorSize::size`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:17:39
15    |
15    |
- LL | trait TensorDimension {
-    | ^^^^^^^^^^^^^^^^^^^^^
+ LL |     fn inbounds(&self, index: [usize; Self::DIM]) -> bool {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
+ note: required by a bound in `TensorSize::inbounds`
+    |
+    |
+ LL |     fn inbounds(&self, index: [usize; Self::DIM]) -> bool {
+    |                                       ^^^^^^^^^ required by this bound in `TensorSize::inbounds`
- error: aborting due to previous error
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:24:35
+    |
+    |
+ LL |     fn bget(&self, index: [usize; Self::DIM]) -> Option<Self::Element>;
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
+ note: required by a bound in `Broadcastable::bget`
+    |
+    |
+ LL |     fn bget(&self, index: [usize; Self::DIM]) -> Option<Self::Element>;
+    |                                   ^^^^^^^^^ required by this bound in `Broadcastable::bget`
- For more information about this error, try `rustc --explain E0391`.
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:28:26
+    |
+    |
+ LL |     ) -> LazyUpdim<Self, { Self::DIM }, NEWDIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { Self::DIM }]:`
+ note: required by a bound in `Broadcastable::lazy_updim`
+    |
+    |
+ LL |     fn lazy_updim<const NEWDIM: usize>(
+    |        ---------- required by a bound in this
+ ...
+ LL |     ) -> LazyUpdim<Self, { Self::DIM }, NEWDIM> {
+    |                          ^^^^^^^^^^^^^ required by this bound in `Broadcastable::lazy_updim`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:28:10
+    |
+    |
+ LL |     ) -> LazyUpdim<Self, { Self::DIM }, NEWDIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { Self::DIM }]:`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:35:78
+    |
+    |
+ LL |     fn bmap<T, F: Fn(Self::Element) -> T>(&self, foo: F) -> BMap<T, Self, F, { Self::DIM }> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { Self::DIM }]:`
+ note: required by a bound in `Broadcastable::bmap`
+    |
+    |
+ LL |     fn bmap<T, F: Fn(Self::Element) -> T>(&self, foo: F) -> BMap<T, Self, F, { Self::DIM }> {
+    |                                                                              ^^^^^^^^^^^^^ required by this bound in `Broadcastable::bmap`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:35:61
+    |
+    |
+ LL |     fn bmap<T, F: Fn(Self::Element) -> T>(&self, foo: F) -> BMap<T, Self, F, { Self::DIM }> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { Self::DIM }]:`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:45:83
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
+ note: required by a bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorDimension>`
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |                                                                                   ^^^^^^^^^^ required by this bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorDimension>`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:45:46
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
+ note: required because of the requirements on the impl of `TensorDimension` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
+ 
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:45:66
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:49:78
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
+ note: required by a bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorSize>`
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |                                                                              ^^^^^^^^^^ required by this bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorSize>`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:49:46
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
+ note: required because of the requirements on the impl of `TensorSize` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+ 
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:49:46
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
+ note: required because of the requirements on the impl of `TensorDimension` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |                                              ^^^^^^^^^^^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: required by a bound in `TensorSize`
+    |
+    |
+ LL | trait TensorSize: TensorDimension {
+    |                   ^^^^^^^^^^^^^^^ required by this bound in `TensorSize`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:49:61
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:49:78
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
+ note: required by a bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorSize>`
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |                                                                              ^^^^^^^^^^ required by this bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorSize>`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:50:13
+    |
+    |
+ LL |     fn size(&self) -> [usize; DIM] {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
+ error: unconstrained generic constant
+   --> $DIR/issue-83765.rs:55:81
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn size(&self) -> [usize; Self::DIM];
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
note: required by a bound in `TensorSize::size`
   |
   |
LL |     fn size(&self) -> [usize; Self::DIM];
   |                               ^^^^^^^^^ required by this bound in `TensorSize::size`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:17:39
   |
   |
LL |     fn inbounds(&self, index: [usize; Self::DIM]) -> bool {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
note: required by a bound in `TensorSize::inbounds`
   |
   |
LL |     fn inbounds(&self, index: [usize; Self::DIM]) -> bool {
   |                                       ^^^^^^^^^ required by this bound in `TensorSize::inbounds`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:24:35
   |
   |
LL |     fn bget(&self, index: [usize; Self::DIM]) -> Option<Self::Element>;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
note: required by a bound in `Broadcastable::bget`
   |
   |
LL |     fn bget(&self, index: [usize; Self::DIM]) -> Option<Self::Element>;
   |                                   ^^^^^^^^^ required by this bound in `Broadcastable::bget`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:28:26
   |
   |
LL |     ) -> LazyUpdim<Self, { Self::DIM }, NEWDIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { Self::DIM }]:`
note: required by a bound in `Broadcastable::lazy_updim`
   |
   |
LL |     fn lazy_updim<const NEWDIM: usize>(
   |        ---------- required by a bound in this
...
LL |     ) -> LazyUpdim<Self, { Self::DIM }, NEWDIM> {
   |                          ^^^^^^^^^^^^^ required by this bound in `Broadcastable::lazy_updim`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:28:10
   |
   |
LL |     ) -> LazyUpdim<Self, { Self::DIM }, NEWDIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { Self::DIM }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:35:78
   |
   |
LL |     fn bmap<T, F: Fn(Self::Element) -> T>(&self, foo: F) -> BMap<T, Self, F, { Self::DIM }> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { Self::DIM }]:`
note: required by a bound in `Broadcastable::bmap`
   |
   |
LL |     fn bmap<T, F: Fn(Self::Element) -> T>(&self, foo: F) -> BMap<T, Self, F, { Self::DIM }> {
   |                                                                              ^^^^^^^^^^^^^ required by this bound in `Broadcastable::bmap`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:35:61
   |
   |
LL |     fn bmap<T, F: Fn(Self::Element) -> T>(&self, foo: F) -> BMap<T, Self, F, { Self::DIM }> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { Self::DIM }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:45:83
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required by a bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorDimension>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |                                                                                   ^^^^^^^^^^ required by this bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorDimension>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:45:46
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required because of the requirements on the impl of `TensorDimension` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:45:66
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:49:78
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required by a bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorSize>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |                                                                              ^^^^^^^^^^ required by this bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorSize>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:49:46
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required because of the requirements on the impl of `TensorSize` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:49:46
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required because of the requirements on the impl of `TensorDimension` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
note: required by a bound in `TensorSize`
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:15:19
   |
   |
LL | trait TensorSize: TensorDimension {
   |                   ^^^^^^^^^^^^^^^ required by this bound in `TensorSize`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:49:61
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:49:78
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required by a bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorSize>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |                                                                              ^^^^^^^^^^ required by this bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as TensorSize>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:50:13
   |
   |
LL |     fn size(&self) -> [usize; DIM] {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:55:81
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required by a bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as Broadcastable>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |                                                                                 ^^^^^^^^^^ required by this bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as Broadcastable>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:55:46
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required because of the requirements on the impl of `Broadcastable` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:55:46
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required because of the requirements on the impl of `TensorSize` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
note: required by a bound in `Broadcastable`
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:22:22
   |
   |
LL | trait Broadcastable: TensorSize + Sized {
   |                      ^^^^^^^^^^ required by this bound in `Broadcastable`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:55:46
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required because of the requirements on the impl of `TensorDimension` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorDimension for LazyUpdim<'a, T, { T::DIM }, DIM> {
note: required by a bound in `Broadcastable`
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:22:22
   |
   |
LL | trait Broadcastable: TensorSize + Sized {
   |                      ^^^^^^^^^^ required by this bound in `Broadcastable`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:55:64
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:57:8
   |
   |
LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required because of the requirements on the impl of `Broadcastable` for `LazyUpdim<'a, T, { T::DIM }, DIM>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:55:81
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
note: required by a bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as Broadcastable>`
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |                                                                                 ^^^^^^^^^^ required by this bound in `<LazyUpdim<'a, T, { T::DIM }, DIM> as Broadcastable>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:57:13
   |
   |
LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { T::DIM }]:`
error: aborting due to 24 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-87470.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-87470.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-87470" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-87470/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn some_fn(self) -> [u8 ; <Self as TraitWithConst>::SOME_CONST];
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); <Self as TraitWithConst>::SOME_CONST]:`
note: required by a bound in `OtherTrait::some_fn`
   |
   |
LL |     fn some_fn(self) -> [u8 ; <Self as TraitWithConst>::SOME_CONST];
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `OtherTrait::some_fn`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-87964.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-87964.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-87964" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-87964/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     [(); T::LENGTH]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::LENGTH]:`
note: required by a bound in `Container`
   |
   |
LL | pub struct Container<T: Target>
   |            --------- required by a bound in this
LL | where
LL |     [(); T::LENGTH]: Sized,
   |          ^^^^^^^^^ required by this bound in `Container`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:20:10
   |
   |
LL |     [(); T::LENGTH]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::LENGTH]:`
note: required by a bound in `Container<T>`
   |
   |
LL |     [(); T::LENGTH]: Sized,
   |          ^^^^^^^^^ required by this bound in `Container<T>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:18:17
   |
   |
LL | impl<T: Target> Container<T>
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::LENGTH]:`
note: required by a bound in `Container`
   |
   |
LL | pub struct Container<T: Target>
   |            --------- required by a bound in this
LL | where
LL |     [(); T::LENGTH]: Sized,
   |          ^^^^^^^^^ required by this bound in `Container`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:20:10
   |
   |
LL |     [(); T::LENGTH]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::LENGTH]:`
note: required by a bound in `Container<T>`
   |
   |
LL |     [(); T::LENGTH]: Sized,
   |          ^^^^^^^^^ required by this bound in `Container<T>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:24:10
   |
LL |     ) -> Container<T> {
LL |     ) -> Container<T> {
   |          ^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); T::LENGTH]:`
note: required by a bound in `Container`
   |
   |
LL | pub struct Container<T: Target>
   |            --------- required by a bound in this
LL | where
LL |     [(); T::LENGTH]: Sized,
   |          ^^^^^^^^^ required by this bound in `Container`
error: aborting due to 5 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-89146.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-89146.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89146" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89146/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn to_bytes(&self) -> [u8; Self::SIZE];
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::SIZE]:`
note: required by a bound in `Foo::to_bytes`
   |
   |
LL |     fn to_bytes(&self) -> [u8; Self::SIZE];
   |                                ^^^^^^^^^^ required by this bound in `Foo::to_bytes`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:14:10
   |
   |
LL |     [(); G::SIZE]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); G::SIZE]:`
note: required by a bound in `bar`
   |
   |
LL | pub fn bar<G: Foo>(a: &G) -> u8
   |        --- required by a bound in this
LL | where
LL |     [(); G::SIZE]: Sized,
   |          ^^^^^^^ required by this bound in `bar`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:21:10
   |
   |
LL |     [(); G::SIZE]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); G::SIZE]:`
note: required by a bound in `deeper_bar`
   |
   |
LL | fn deeper_bar<G: Foo>(a: &G) -> u8
   |    ---------- required by a bound in this
LL | where
LL |     [(); G::SIZE]: Sized,
   |          ^^^^^^^ required by this bound in `deeper_bar`
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-89334.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-89334.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89334" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89334/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     [(); T::ARRAY_SIZE]: Sized
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::ARRAY_SIZE]:`
note: required by a bound in `Shard`
   |
   |
LL | pub trait Shard<T: AnotherTrait>:
   |           ----- required by a bound in this
...
LL |     [(); T::ARRAY_SIZE]: Sized
   |          ^^^^^^^^^^^^^ required by this bound in `Shard`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-89334.rs:10:5
   |
   |
LL |     AsMut<[[u8; T::ARRAY_SIZE]]>
   |
   |
---
To only update this specific test, also pass `--test-args privacy/where-priv-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/where-priv-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/where-priv-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/where-priv-type/auxiliary"
stdout: none
--- stderr -------------------------------
warning: private type `PrivTy` in public interface (error E0446)
   |
LL | / pub struct S
LL | / pub struct S
LL | | //~^ WARNING private type `PrivTy` in public interface
LL | | //~| WARNING hard error
LL | | where
LL | |     PrivTy:
LL | | {}
   | |__^
   = note: `#[warn(private_in_public)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


warning: private type `PrivTy` in public interface (error E0446)
   |
LL | / pub enum E
LL | / pub enum E
LL | | //~^ WARNING private type `PrivTy` in public interface
LL | | //~| WARNING hard error
LL | | where
LL | |     PrivTy:
LL | | {}
   | |__^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


warning: private type `PrivTy` in public interface (error E0446)
   |
LL | / pub fn f()
LL | / pub fn f()
LL | | //~^ WARNING private type `PrivTy` in public interface
LL | | //~| WARNING hard error
LL | | where
LL | |     PrivTy:
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error[E0446]: private type `PrivTy` in public interface
   |
LL |   struct PrivTy;
LL |   struct PrivTy;
   |   -------------- `PrivTy` declared as private
LL | / impl S
LL | / impl S
LL | | //~^ ERROR private type `PrivTy` in public interface
LL | | where
LL | |     PrivTy:
LL | |     {}
LL | | }
LL | | }
   | |_^ can't leak private type

warning: private type `PrivTy` in public interface (error E0446)
   |
LL | /     pub fn f()
LL | /     pub fn f()
LL | |     //~^ WARNING private type `PrivTy` in public interface
LL | |     //~| WARNING hard error
LL | |     where
LL | |         PrivTy:
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

