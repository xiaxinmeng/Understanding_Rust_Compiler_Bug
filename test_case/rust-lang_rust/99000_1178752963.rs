plain
........................................................................................ 1848/13157
................................................................................i....... 1936/13157
........................................................................................ 2024/13157
........................................................................................ 2112/13157
.......F.F.............................................................................. 2200/13157
..........................................F............................................. 2376/13157
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

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:11 ~ issue_97047_ice_1[da0f]::{impl#0}::new) (NoSolution): could not prove Binder(WellFormed([(); _]), [])
   |
LL | /         Self {
LL | /         Self {
LL | |             changes: [0; CHANGES.len()],
LL | |         }
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:151:13

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
   0:     0x7f7bc71eabbc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h825df9686e495be6
   1:     0x7f7bc724f778 - core::fmt::write::h3bf79bc51d8a2b72
   2:     0x7f7bc71db201 - std::io::Write::write_fmt::hc271f847770a6cee
   3:     0x7f7bc71eda6e - std::panicking::default_hook::{{closure}}::h55d17edb80cff4ef
   4:     0x7f7bc71ed756 - std::panicking::default_hook::h6c0a40f26cbdf27a
   5:     0x7f7bc7ba1af4 - rustc_driver[b97556f09f515b8e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f7bc71ee202 - std::panicking::rust_panic_with_hook::hd81edae2e0e7f406
   7:     0x7f7bca8abce3 - std[9b3ecdffad5ef05]::panicking::begin_panic::<rustc_errors[6470943a3a43fbc0]::ExplicitBug>::{closure#0}
   8:     0x7f7bca8ab356 - std[9b3ecdffad5ef05]::sys_common::backtrace::__rust_end_short_backtrace::<std[9b3ecdffad5ef05]::panicking::begin_panic<rustc_errors[6470943a3a43fbc0]::ExplicitBug>::{closure#0}, !>
   9:     0x7f7bc7b5d9f6 - std[9b3ecdffad5ef05]::panicking::begin_panic::<rustc_errors[6470943a3a43fbc0]::ExplicitBug>
  10:     0x7f7bca89fcc6 - std[9b3ecdffad5ef05]::panic::panic_any::<rustc_errors[6470943a3a43fbc0]::ExplicitBug>
  11:     0x7f7bca8a4e7d - <rustc_errors[6470943a3a43fbc0]::HandlerInner as core[a65b3f236fb58039]::ops::drop::Drop>::drop
  12:     0x7f7bc7bbe5d2 - core[a65b3f236fb58039]::ptr::drop_in_place::<rustc_session[7218a87adf00bd80]::parse::ParseSess>
  13:     0x7f7bc7bc79dd - <alloc[4d9cac8b5342a401]::rc::Rc<rustc_session[7218a87adf00bd80]::session::Session> as core[a65b3f236fb58039]::ops::drop::Drop>::drop
  14:     0x7f7bc7b945bc - core[a65b3f236fb58039]::ptr::drop_in_place::<rustc_interface[73f0229dbe8d7e76]::interface::Compiler>
  15:     0x7f7bc7b90da9 - rustc_span[f2283066026c9888]::with_source_map::<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_interface[73f0229dbe8d7e76]::interface::create_compiler_and_run<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f7bc7baafa9 - rustc_interface[73f0229dbe8d7e76]::interface::create_compiler_and_run::<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>
  17:     0x7f7bc7b8ac5f - <scoped_tls[a91756807ab07d23]::ScopedKey<rustc_span[f2283066026c9888]::SessionGlobals>>::set::<rustc_interface[73f0229dbe8d7e76]::interface::run_compiler<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>
  18:     0x7f7bc7b94d09 - std[9b3ecdffad5ef05]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73f0229dbe8d7e76]::util::run_in_thread_pool_with_globals<rustc_interface[73f0229dbe8d7e76]::interface::run_compiler<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>
  19:     0x7f7bc7bfe869 - <<std[9b3ecdffad5ef05]::thread::Builder>::spawn_unchecked_<rustc_interface[73f0229dbe8d7e76]::util::run_in_thread_pool_with_globals<rustc_interface[73f0229dbe8d7e76]::interface::run_compiler<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>::{closure#1} as core[a65b3f236fb58039]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f7bc71f90f3 - std::sys::unix::thread::Thread::new::thread_start::hb22c02bef3b80e8c
  21:     0x7f7bc1747609 - start_thread
  22:     0x7f7bc705a133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (beb2474da 2022-07-08) running on x86_64-unknown-linux-gnu

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
   0:     0x7f0c1bf63bbc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h825df9686e495be6
   1:     0x7f0c1bfc8778 - core::fmt::write::h3bf79bc51d8a2b72
   2:     0x7f0c1bf54201 - std::io::Write::write_fmt::hc271f847770a6cee
   3:     0x7f0c1bf66a6e - std::panicking::default_hook::{{closure}}::h55d17edb80cff4ef
   4:     0x7f0c1bf66756 - std::panicking::default_hook::h6c0a40f26cbdf27a
   5:     0x7f0c1c91aaf4 - rustc_driver[b97556f09f515b8e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0c1bf67202 - std::panicking::rust_panic_with_hook::hd81edae2e0e7f406
   7:     0x7f0c1f624ce3 - std[9b3ecdffad5ef05]::panicking::begin_panic::<rustc_errors[6470943a3a43fbc0]::ExplicitBug>::{closure#0}
   8:     0x7f0c1f624356 - std[9b3ecdffad5ef05]::sys_common::backtrace::__rust_end_short_backtrace::<std[9b3ecdffad5ef05]::panicking::begin_panic<rustc_errors[6470943a3a43fbc0]::ExplicitBug>::{closure#0}, !>
   9:     0x7f0c1c8d69f6 - std[9b3ecdffad5ef05]::panicking::begin_panic::<rustc_errors[6470943a3a43fbc0]::ExplicitBug>
  10:     0x7f0c1f618cc6 - std[9b3ecdffad5ef05]::panic::panic_any::<rustc_errors[6470943a3a43fbc0]::ExplicitBug>
  11:     0x7f0c1f61de7d - <rustc_errors[6470943a3a43fbc0]::HandlerInner as core[a65b3f236fb58039]::ops::drop::Drop>::drop
  12:     0x7f0c1c9375d2 - core[a65b3f236fb58039]::ptr::drop_in_place::<rustc_session[7218a87adf00bd80]::parse::ParseSess>
  13:     0x7f0c1c9409dd - <alloc[4d9cac8b5342a401]::rc::Rc<rustc_session[7218a87adf00bd80]::session::Session> as core[a65b3f236fb58039]::ops::drop::Drop>::drop
  14:     0x7f0c1c90d5bc - core[a65b3f236fb58039]::ptr::drop_in_place::<rustc_interface[73f0229dbe8d7e76]::interface::Compiler>
  15:     0x7f0c1c909da9 - rustc_span[f2283066026c9888]::with_source_map::<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_interface[73f0229dbe8d7e76]::interface::create_compiler_and_run<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f0c1c923fa9 - rustc_interface[73f0229dbe8d7e76]::interface::create_compiler_and_run::<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>
  17:     0x7f0c1c903c5f - <scoped_tls[a91756807ab07d23]::ScopedKey<rustc_span[f2283066026c9888]::SessionGlobals>>::set::<rustc_interface[73f0229dbe8d7e76]::interface::run_compiler<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>
  18:     0x7f0c1c90dd09 - std[9b3ecdffad5ef05]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[73f0229dbe8d7e76]::util::run_in_thread_pool_with_globals<rustc_interface[73f0229dbe8d7e76]::interface::run_compiler<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>
  19:     0x7f0c1c977869 - <<std[9b3ecdffad5ef05]::thread::Builder>::spawn_unchecked_<rustc_interface[73f0229dbe8d7e76]::util::run_in_thread_pool_with_globals<rustc_interface[73f0229dbe8d7e76]::interface::run_compiler<core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>, rustc_driver[b97556f09f515b8e]::run_compiler::{closure#1}>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>::{closure#0}, core[a65b3f236fb58039]::result::Result<(), rustc_errors[6470943a3a43fbc0]::ErrorGuaranteed>>::{closure#1} as core[a65b3f236fb58039]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f0c1bf720f3 - std::sys::unix::thread::Thread::new::thread_start::hb22c02bef3b80e8c
  21:     0x7f0c164c0609 - start_thread
  22:     0x7f0c1bdd3133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (beb2474da 2022-07-08) running on x86_64-unknown-linux-gnu

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


