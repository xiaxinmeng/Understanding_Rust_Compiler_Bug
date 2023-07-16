plain
........................................................................................ 1848/13161
................................................................................i....... 1936/13161
........................................................................................ 2024/13161
........................................................................................ 2112/13161
...F....F............................................................................... 2200/13161
........................................F............................................... 2376/13161
........................................................................................ 2464/13161
........................................................................................ 2552/13161
........................................................................................ 2640/13161
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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
   0:     0x7f5a6459abac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h46985249b4fde61e
   1:     0x7f5a645ff768 - core::fmt::write::h1f38fd3f14fbfcd9
   2:     0x7f5a6458b161 - std::io::Write::write_fmt::hefd23e66c09d0ddc
   3:     0x7f5a6459da5e - std::panicking::default_hook::{{closure}}::h5bd0dc3e5a6545aa
   4:     0x7f5a6459d746 - std::panicking::default_hook::h3c4a5be9dfc96c32
   5:     0x7f5a64f51d84 - rustc_driver[ce2b57e498f4ff26]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5a6459e1f2 - std::panicking::rust_panic_with_hook::h44508e28bdf5601e
   7:     0x7f5a67c0dd43 - std[9c5307111ddb5a4b]::panicking::begin_panic::<rustc_errors[3dcacfd90c04174d]::ExplicitBug>::{closure#0}
   8:     0x7f5a67c0d856 - std[9c5307111ddb5a4b]::sys_common::backtrace::__rust_end_short_backtrace::<std[9c5307111ddb5a4b]::panicking::begin_panic<rustc_errors[3dcacfd90c04174d]::ExplicitBug>::{closure#0}, !>
   9:     0x7f5a64f0dd16 - std[9c5307111ddb5a4b]::panicking::begin_panic::<rustc_errors[3dcacfd90c04174d]::ExplicitBug>
  10:     0x7f5a67c52276 - std[9c5307111ddb5a4b]::panic::panic_any::<rustc_errors[3dcacfd90c04174d]::ExplicitBug>
  11:     0x7f5a67c5651d - <rustc_errors[3dcacfd90c04174d]::HandlerInner as core[39ae02758373f9ab]::ops::drop::Drop>::drop
  12:     0x7f5a64f6da72 - core[39ae02758373f9ab]::ptr::drop_in_place::<rustc_session[70dfb57471a1e1e]::parse::ParseSess>
  13:     0x7f5a64f77c4d - <alloc[9f114b93d93a3894]::rc::Rc<rustc_session[70dfb57471a1e1e]::session::Session> as core[39ae02758373f9ab]::ops::drop::Drop>::drop
  14:     0x7f5a64f44abc - core[39ae02758373f9ab]::ptr::drop_in_place::<rustc_interface[ac2b9354bc125af]::interface::Compiler>
  15:     0x7f5a64f41299 - rustc_span[d4763e6fefcb39ed]::with_source_map::<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_interface[ac2b9354bc125af]::interface::create_compiler_and_run<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f5a64f73659 - rustc_interface[ac2b9354bc125af]::interface::create_compiler_and_run::<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>
  17:     0x7f5a64f3b0f2 - <scoped_tls[f4a0b7205ad90b9e]::ScopedKey<rustc_span[d4763e6fefcb39ed]::SessionGlobals>>::set::<rustc_interface[ac2b9354bc125af]::interface::run_compiler<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>
  18:     0x7f5a64f44ed9 - std[9c5307111ddb5a4b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ac2b9354bc125af]::util::run_in_thread_pool_with_globals<rustc_interface[ac2b9354bc125af]::interface::run_compiler<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>
  19:     0x7f5a64faeb69 - <<std[9c5307111ddb5a4b]::thread::Builder>::spawn_unchecked_<rustc_interface[ac2b9354bc125af]::util::run_in_thread_pool_with_globals<rustc_interface[ac2b9354bc125af]::interface::run_compiler<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>::{closure#1} as core[39ae02758373f9ab]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f5a645a90e3 - std::sys::unix::thread::Thread::new::thread_start::h3ca8127e8f64d2f3
  21:     0x7f5a5eaf7609 - start_thread
  22:     0x7f5a6440a133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (13ee83015 2022-07-09) running on x86_64-unknown-linux-gnu

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
   0:     0x7fd993d8cbac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h46985249b4fde61e
   1:     0x7fd993df1768 - core::fmt::write::h1f38fd3f14fbfcd9
   2:     0x7fd993d7d161 - std::io::Write::write_fmt::hefd23e66c09d0ddc
   3:     0x7fd993d8fa5e - std::panicking::default_hook::{{closure}}::h5bd0dc3e5a6545aa
   4:     0x7fd993d8f746 - std::panicking::default_hook::h3c4a5be9dfc96c32
   5:     0x7fd994743d84 - rustc_driver[ce2b57e498f4ff26]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd993d901f2 - std::panicking::rust_panic_with_hook::h44508e28bdf5601e
   7:     0x7fd9973ffd43 - std[9c5307111ddb5a4b]::panicking::begin_panic::<rustc_errors[3dcacfd90c04174d]::ExplicitBug>::{closure#0}
   8:     0x7fd9973ff856 - std[9c5307111ddb5a4b]::sys_common::backtrace::__rust_end_short_backtrace::<std[9c5307111ddb5a4b]::panicking::begin_panic<rustc_errors[3dcacfd90c04174d]::ExplicitBug>::{closure#0}, !>
   9:     0x7fd9946ffd16 - std[9c5307111ddb5a4b]::panicking::begin_panic::<rustc_errors[3dcacfd90c04174d]::ExplicitBug>
  10:     0x7fd997444276 - std[9c5307111ddb5a4b]::panic::panic_any::<rustc_errors[3dcacfd90c04174d]::ExplicitBug>
  11:     0x7fd99744851d - <rustc_errors[3dcacfd90c04174d]::HandlerInner as core[39ae02758373f9ab]::ops::drop::Drop>::drop
  12:     0x7fd99475fa72 - core[39ae02758373f9ab]::ptr::drop_in_place::<rustc_session[70dfb57471a1e1e]::parse::ParseSess>
  13:     0x7fd994769c4d - <alloc[9f114b93d93a3894]::rc::Rc<rustc_session[70dfb57471a1e1e]::session::Session> as core[39ae02758373f9ab]::ops::drop::Drop>::drop
  14:     0x7fd994736abc - core[39ae02758373f9ab]::ptr::drop_in_place::<rustc_interface[ac2b9354bc125af]::interface::Compiler>
  15:     0x7fd994733299 - rustc_span[d4763e6fefcb39ed]::with_source_map::<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_interface[ac2b9354bc125af]::interface::create_compiler_and_run<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fd994765659 - rustc_interface[ac2b9354bc125af]::interface::create_compiler_and_run::<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>
  17:     0x7fd99472d0f2 - <scoped_tls[f4a0b7205ad90b9e]::ScopedKey<rustc_span[d4763e6fefcb39ed]::SessionGlobals>>::set::<rustc_interface[ac2b9354bc125af]::interface::run_compiler<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>
  18:     0x7fd994736ed9 - std[9c5307111ddb5a4b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ac2b9354bc125af]::util::run_in_thread_pool_with_globals<rustc_interface[ac2b9354bc125af]::interface::run_compiler<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>
  19:     0x7fd9947a0b69 - <<std[9c5307111ddb5a4b]::thread::Builder>::spawn_unchecked_<rustc_interface[ac2b9354bc125af]::util::run_in_thread_pool_with_globals<rustc_interface[ac2b9354bc125af]::interface::run_compiler<core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>, rustc_driver[ce2b57e498f4ff26]::run_compiler::{closure#1}>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>::{closure#0}, core[39ae02758373f9ab]::result::Result<(), rustc_errors[3dcacfd90c04174d]::ErrorGuaranteed>>::{closure#1} as core[39ae02758373f9ab]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7fd993d9b0e3 - std::sys::unix::thread::Thread::new::thread_start::h3ca8127e8f64d2f3
  21:     0x7fd98e2e9609 - start_thread
  22:     0x7fd993bfc133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (13ee83015 2022-07-09) running on x86_64-unknown-linux-gnu

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


