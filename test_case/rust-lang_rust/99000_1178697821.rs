plain
........................................................................................ 1848/13157
................................................................................i....... 1936/13157
........................................................................................ 2024/13157
........................................................................................ 2112/13157
...F......F............................................................................. 2200/13157
........................................F............................................... 2376/13157
........................................................................................ 2464/13157
........................................................................................ 2552/13157
........................................................................................ 2640/13157
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #95174 <https://github.com/rust-lang/rust/issues/95174> for more information


warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

warning: 2 warnings emitted
warning: 2 warnings emitted
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:11 ~ issue_97047_ice_1[da0f]::{impl#0}::new) (NoSolution): could not prove Binder(WellFormed([(); _]), [])
   |
LL | /         Self {
LL | /         Self {
LL | |             changes: [0; CHANGES.len()],
LL | |         }
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:151:13


error: internal compiler error: broken MIR in DefId(0:11 ~ issue_97047_ice_1[da0f]::{impl#0}::new) (NoSolution): could not prove Binder(ConstEvaluatable(WithOptConstParam { did: DefId(0:5 ~ issue_97047_ice_1[da0f]::Changes::{constant#0}), const_param_did: None }, [Const { ty: &[&str], kind: Param(CHANGES/#0) }]), [])
   |
LL | /         Self {
LL | /         Self {
LL | |             changes: [0; CHANGES.len()],
LL | |         }
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:151:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f6ece395bbc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hac7782ee209caf43
   1:     0x7f6ece3fa778 - core::fmt::write::h097f7801632fc7f7
   2:     0x7f6ece386201 - std::io::Write::write_fmt::hd0d88ece9a869546
   3:     0x7f6ece398a6e - std::panicking::default_hook::{{closure}}::h5fba928fdb0aec8e
   4:     0x7f6ece398756 - std::panicking::default_hook::h0bfdc2c342e004e2
   5:     0x7f6eced4cba4 - rustc_driver[8b2f5466158b6d59]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6ece399202 - std::panicking::rust_panic_with_hook::h0e98323326afc6a0
   7:     0x7f6ed1a5a1f3 - std[e90da48a4539237a]::panicking::begin_panic::<rustc_errors[92fe1ecf9fbf53d3]::ExplicitBug>::{closure#0}
   8:     0x7f6ed1a59866 - std[e90da48a4539237a]::sys_common::backtrace::__rust_end_short_backtrace::<std[e90da48a4539237a]::panicking::begin_panic<rustc_errors[92fe1ecf9fbf53d3]::ExplicitBug>::{closure#0}, !>
   9:     0x7f6eced08aa6 - std[e90da48a4539237a]::panicking::begin_panic::<rustc_errors[92fe1ecf9fbf53d3]::ExplicitBug>
  10:     0x7f6ed1a4f096 - std[e90da48a4539237a]::panic::panic_any::<rustc_errors[92fe1ecf9fbf53d3]::ExplicitBug>
  11:     0x7f6ed1a5338d - <rustc_errors[92fe1ecf9fbf53d3]::HandlerInner as core[4fb87be2cc853b61]::ops::drop::Drop>::drop
  12:     0x7f6eced66ac2 - core[4fb87be2cc853b61]::ptr::drop_in_place::<rustc_session[67327e817c935b9d]::parse::ParseSess>
  13:     0x7f6eced728fd - <alloc[fb46bb70e0a31dc0]::rc::Rc<rustc_session[67327e817c935b9d]::session::Session> as core[4fb87be2cc853b61]::ops::drop::Drop>::drop
  14:     0x7f6eced3f61c - core[4fb87be2cc853b61]::ptr::drop_in_place::<rustc_interface[8d40e1ab20a73e16]::interface::Compiler>
  15:     0x7f6eced3be59 - rustc_span[4cee6969d27afa43]::with_source_map::<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_interface[8d40e1ab20a73e16]::interface::create_compiler_and_run<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f6eced6e549 - rustc_interface[8d40e1ab20a73e16]::interface::create_compiler_and_run::<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>
  17:     0x7f6eced35d0f - <scoped_tls[943b0cc329ca22d2]::ScopedKey<rustc_span[4cee6969d27afa43]::SessionGlobals>>::set::<rustc_interface[8d40e1ab20a73e16]::interface::run_compiler<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>
  18:     0x7f6eced3fdb9 - std[e90da48a4539237a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8d40e1ab20a73e16]::util::run_in_thread_pool_with_globals<rustc_interface[8d40e1ab20a73e16]::interface::run_compiler<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>
  19:     0x7f6eceda9b49 - <<std[e90da48a4539237a]::thread::Builder>::spawn_unchecked_<rustc_interface[8d40e1ab20a73e16]::util::run_in_thread_pool_with_globals<rustc_interface[8d40e1ab20a73e16]::interface::run_compiler<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>::{closure#1} as core[4fb87be2cc853b61]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f6ece3a40f3 - std::sys::unix::thread::Thread::new::thread_start::h8200630194b81f2b
  21:     0x7f6ec88f2609 - start_thread
  22:     0x7f6ece205133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (d99a98385 2022-07-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #95174 <https://github.com/rust-lang/rust/issues/95174> for more information


warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

warning: 2 warnings emitted
warning: 2 warnings emitted

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:11 ~ issue_97047_ice_2[19cb]::{impl#0}::combine) (NoSolution): could not prove Binder(WellFormed(&[usize; _]), [])
   |
   |
LL |         for _change in &self.changes {}
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:151:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f798fe3dbbc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hac7782ee209caf43
   1:     0x7f798fea2778 - core::fmt::write::h097f7801632fc7f7
   2:     0x7f798fe2e201 - std::io::Write::write_fmt::hd0d88ece9a869546
   3:     0x7f798fe40a6e - std::panicking::default_hook::{{closure}}::h5fba928fdb0aec8e
   4:     0x7f798fe40756 - std::panicking::default_hook::h0bfdc2c342e004e2
   5:     0x7f79907f4ba4 - rustc_driver[8b2f5466158b6d59]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f798fe41202 - std::panicking::rust_panic_with_hook::h0e98323326afc6a0
   7:     0x7f79935021f3 - std[e90da48a4539237a]::panicking::begin_panic::<rustc_errors[92fe1ecf9fbf53d3]::ExplicitBug>::{closure#0}
   8:     0x7f7993501866 - std[e90da48a4539237a]::sys_common::backtrace::__rust_end_short_backtrace::<std[e90da48a4539237a]::panicking::begin_panic<rustc_errors[92fe1ecf9fbf53d3]::ExplicitBug>::{closure#0}, !>
   9:     0x7f79907b0aa6 - std[e90da48a4539237a]::panicking::begin_panic::<rustc_errors[92fe1ecf9fbf53d3]::ExplicitBug>
  10:     0x7f79934f7096 - std[e90da48a4539237a]::panic::panic_any::<rustc_errors[92fe1ecf9fbf53d3]::ExplicitBug>
  11:     0x7f79934fb38d - <rustc_errors[92fe1ecf9fbf53d3]::HandlerInner as core[4fb87be2cc853b61]::ops::drop::Drop>::drop
  12:     0x7f799080eac2 - core[4fb87be2cc853b61]::ptr::drop_in_place::<rustc_session[67327e817c935b9d]::parse::ParseSess>
  13:     0x7f799081a8fd - <alloc[fb46bb70e0a31dc0]::rc::Rc<rustc_session[67327e817c935b9d]::session::Session> as core[4fb87be2cc853b61]::ops::drop::Drop>::drop
  14:     0x7f79907e761c - core[4fb87be2cc853b61]::ptr::drop_in_place::<rustc_interface[8d40e1ab20a73e16]::interface::Compiler>
  15:     0x7f79907e3e59 - rustc_span[4cee6969d27afa43]::with_source_map::<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_interface[8d40e1ab20a73e16]::interface::create_compiler_and_run<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f7990816549 - rustc_interface[8d40e1ab20a73e16]::interface::create_compiler_and_run::<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>
  17:     0x7f79907ddd0f - <scoped_tls[943b0cc329ca22d2]::ScopedKey<rustc_span[4cee6969d27afa43]::SessionGlobals>>::set::<rustc_interface[8d40e1ab20a73e16]::interface::run_compiler<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>
  18:     0x7f79907e7db9 - std[e90da48a4539237a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8d40e1ab20a73e16]::util::run_in_thread_pool_with_globals<rustc_interface[8d40e1ab20a73e16]::interface::run_compiler<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>
  19:     0x7f7990851b49 - <<std[e90da48a4539237a]::thread::Builder>::spawn_unchecked_<rustc_interface[8d40e1ab20a73e16]::util::run_in_thread_pool_with_globals<rustc_interface[8d40e1ab20a73e16]::interface::run_compiler<core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>, rustc_driver[8b2f5466158b6d59]::run_compiler::{closure#1}>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>::{closure#0}, core[4fb87be2cc853b61]::result::Result<(), rustc_errors[92fe1ecf9fbf53d3]::ErrorGuaranteed>>::{closure#1} as core[4fb87be2cc853b61]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f798fe4c0f3 - std::sys::unix::thread::Thread::new::thread_start::h8200630194b81f2b
  21:     0x7f798a39a609 - start_thread
  22:     0x7f798fcad133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (d99a98385 2022-07-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/try_unify_ignore_lifetimes.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/try_unify_ignore_lifetimes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/try_unify_ignore_lifetimes/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn baz(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N::VALUE]:`
note: required by a bound in `Foo`
   |
   |
LL | struct Foo<'a, N: NumT>(&'a [u32; N::VALUE]) where [(); N::VALUE]:;
   |                                                         ^^^^^^^^ required by this bound in `Foo`
error: aborting due to previous error
------------------------------------------


