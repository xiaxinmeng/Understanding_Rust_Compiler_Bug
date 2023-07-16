plain
........................................................................................ 1672/13380
........................................................i......ii....................... 1760/13380
........................................................................................ 1848/13380
........................................................................................ 1936/13380
............................i............................................F............F. 2024/13380
................................................................................F....... 2112/13380
..........F........F...........F...F...F............F..F........FF...................... 2200/13380
.............................F.......FF................................................. 2288/13380
.F........F.F......F..F.F..F....FFF...F...F............................................. 2376/13380
..F..................................................................................... 2464/13380
........................................................................................ 2640/13380
........................................................................................ 2728/13380
........................................................................................ 2816/13380
........................................................................................ 2904/13380
---
---- [ui] src/test/ui/associated-consts/issue-88599-ref-self.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-88599-ref-self" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-88599-ref-self/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:10 ~ issue_88599_ref_self[e291]::Third::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     [u8; Self::CONST]:
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs:19:5
   |
LL |     [u8; Self::CONST]:
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:10 ~ issue_88599_ref_self[e291]::Third::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:12 ~ issue_88599_ref_self[e291]::Third::VAL::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs:21:5
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs:21:5
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:10 ~ issue_88599_ref_self[e291]::Third::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:13 ~ issue_88599_ref_self[e291]::Third::VAL::{constant#1}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs:21:40
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs:21:40
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: mir_const_qualif: MIR had errors
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_mir_transform/src/lib.rs:187:18


error: internal compiler error: PromoteTemps: MIR had errors
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22


error: internal compiler error: broken MIR in DefId(0:11 ~ issue_88599_ref_self[e291]::Third::VAL) ("return type"): bad type [type error]
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:11 ~ issue_88599_ref_self[e291]::Third::VAL) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/associated-consts/issue-88599-ref-self.rs:21:5: 21:33 (#0), scope: scope[0] } }): bad type [type error]
   |
   |
LL |     const VAL: [u8; Self::CONST] = [0; Self::CONST];
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f5b35ab38ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f5b35b1a608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f5b35aa3ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f5b35ab68ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f5b35ab6576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f5b36472b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5b35ab7062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f5b39288023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f5b392878a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f5b36431af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f5b3927d066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f5b3928110c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f5b3648e032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f5b36498458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f5b3646240c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f5b3645ebfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f5b36493e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f5b364563d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f5b36462739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f5b364d5e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f5b364d1492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f5b35ac2645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f5b3000d609 - start_thread
  23:     0x7f5b35920133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:974:13: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout
   |
   |
LL | / fn foo<T: Foo>()
LL | | where
LL | |     [(); T::ASSOC]: ,

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
stack backtrace:
   0:     0x7fd2af6458ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7fd2af6ac608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7fd2af635ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7fd2af6488ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7fd2af648576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7fd2b0004b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd2af649062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7fd2b030f373 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7fd2b030b586 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7fd2afe0e586 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7fd2b030b3c6 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7fd2b03055b3 - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner>::span_bug::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span, &alloc[4c228ee6381bccf6]::string::String>
  12:     0x7fd2b0305210 - <rustc_errors[6e3ab7d0d66c576b]::Handler>::span_bug::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span, &alloc[4c228ee6381bccf6]::string::String>
  13:     0x7fd2b021e705 - rustc_middle[7dcea2cd529e0def]::util::bug::opt_span_bug_fmt::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>::{closure#0}
  14:     0x7fd2b021f2eb - rustc_middle[7dcea2cd529e0def]::ty::context::tls::with_opt::<rustc_middle[7dcea2cd529e0def]::util::bug::opt_span_bug_fmt<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7fd2b021d1f6 - rustc_middle[7dcea2cd529e0def]::ty::context::tls::with_context_opt::<rustc_middle[7dcea2cd529e0def]::ty::context::tls::with_opt<rustc_middle[7dcea2cd529e0def]::util::bug::opt_span_bug_fmt<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7fd2b02167a9 - rustc_middle[7dcea2cd529e0def]::util::bug::opt_span_bug_fmt::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>
  17:     0x7fd2afe02537 - rustc_middle[7dcea2cd529e0def]::util::bug::span_bug_fmt::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>
  18:     0x7fd2b02545a5 - <rustc_codegen_llvm[55f96d9634b99f7c]::context::CodegenCx as rustc_middle[7dcea2cd529e0def]::ty::layout::LayoutOfHelpers>::handle_layout_err
  19:     0x7fd2b024ff1b - <rustc_codegen_llvm[55f96d9634b99f7c]::context::CodegenCx as rustc_middle[7dcea2cd529e0def]::ty::layout::LayoutOf>::spanned_layout_of::{closure#0}
  20:     0x7fd2b02538a2 - <rustc_codegen_llvm[55f96d9634b99f7c]::context::CodegenCx as rustc_middle[7dcea2cd529e0def]::ty::layout::LayoutOf>::spanned_layout_of
  21:     0x7fd2b024c15c - <core[86ccb9a07b15c2c9]::iter::adapters::map::Map<core[86ccb9a07b15c2c9]::slice::iter::Iter<rustc_middle[7dcea2cd529e0def]::mir::LocalDecl>, rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::non_ssa_locals<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>::{closure#0}> as core[86ccb9a07b15c2c9]::iter::traits::iterator::Iterator>::fold::<(), core[86ccb9a07b15c2c9]::iter::traits::iterator::Iterator::for_each::call<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind, <alloc[4c228ee6381bccf6]::vec::Vec<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind> as alloc[4c228ee6381bccf6]::vec::spec_extend::SpecExtend<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind, core[86ccb9a07b15c2c9]::iter::adapters::map::Map<core[86ccb9a07b15c2c9]::slice::iter::Iter<rustc_middle[7dcea2cd529e0def]::mir::LocalDecl>, rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::non_ssa_locals<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>::{closure#0}>>>::spec_extend::{closure#0}>::{closure#0}>
  22:     0x7fd2b023ade3 - <alloc[4c228ee6381bccf6]::vec::Vec<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind> as alloc[4c228ee6381bccf6]::vec::spec_from_iter::SpecFromIter<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind, core[86ccb9a07b15c2c9]::iter::adapters::map::Map<core[86ccb9a07b15c2c9]::slice::iter::Iter<rustc_middle[7dcea2cd529e0def]::mir::LocalDecl>, rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::non_ssa_locals<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>::{closure#0}>>>::from_iter
  23:     0x7fd2b02665f8 - rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::non_ssa_locals::<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>
  24:     0x7fd2b033e1b2 - rustc_codegen_ssa[b1b21d015cc3745e]::mir::codegen_mir::<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>
  25:     0x7fd2b0262f36 - rustc_codegen_ssa[b1b21d015cc3745e]::base::codegen_instance::<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>
  26:     0x7fd2b02ca094 - rustc_codegen_llvm[55f96d9634b99f7c]::base::compile_codegen_unit::module_codegen
  27:     0x7fd2b01f4bd3 - <rustc_query_system[21f1fbb3509cb783]::dep_graph::graph::DepGraph<rustc_middle[7dcea2cd529e0def]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[7dcea2cd529e0def]::ty::context::TyCtxt, rustc_span[ce74fb0acceea6fe]::symbol::Symbol, rustc_codegen_ssa[b1b21d015cc3745e]::ModuleCodegen<rustc_codegen_llvm[55f96d9634b99f7c]::ModuleLlvm>>
  28:     0x7fd2b02c9b37 - rustc_codegen_llvm[55f96d9634b99f7c]::base::compile_codegen_unit
  29:     0x7fd2b026212a - rustc_codegen_ssa[b1b21d015cc3745e]::base::codegen_crate::<rustc_codegen_llvm[55f96d9634b99f7c]::LlvmCodegenBackend>
  30:     0x7fd2b032186d - <rustc_codegen_llvm[55f96d9634b99f7c]::LlvmCodegenBackend as rustc_codegen_ssa[b1b21d015cc3745e]::traits::backend::CodegenBackend>::codegen_crate
  31:     0x7fd2b0175feb - <rustc_session[b989f28d2f7f5ae0]::session::Session>::time::<alloc[4c228ee6381bccf6]::boxed::Box<dyn core[86ccb9a07b15c2c9]::any::Any>, rustc_interface[cf2f5002c3170186]::passes::start_codegen::{closure#0}>
  32:     0x7fd2b013c4cc - <rustc_interface[cf2f5002c3170186]::passes::QueryContext>::enter::<<rustc_interface[cf2f5002c3170186]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<alloc[4c228ee6381bccf6]::boxed::Box<dyn core[86ccb9a07b15c2c9]::any::Any>, rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  33:     0x7fd2b012188d - <rustc_interface[cf2f5002c3170186]::queries::Queries>::ongoing_codegen
  34:     0x7fd2b000c086 - <rustc_interface[cf2f5002c3170186]::interface::Compiler>::enter::<rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}::{closure#2}, core[86ccb9a07b15c2c9]::result::Result<core[86ccb9a07b15c2c9]::option::Option<rustc_interface[cf2f5002c3170186]::queries::Linker>, rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  35:     0x7fd2afff0a21 - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  36:     0x7fd2b0025e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  37:     0x7fd2affe83d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  38:     0x7fd2afff4739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  39:     0x7fd2b0067e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7fd2b0063492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7fd2af654645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  42:     0x7fd2a9b9f609 - start_thread
  43:     0x7fd2af4b2133 - clone
  44:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:8 ~ conservative_is_privately_uninhabited_uses_correct_param_env_2[b139]::Iced::{constant#0}), const_param_did: None }, substs: [T], promoted: () }?
   |
   |
LL |     [(); T::ASSOC]: ;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:11 ~ conservative_is_privately_uninhabited_uses_correct_param_env_2[b139]::Iced::1::{constant#0}), const_param_did: None }, substs: [T], promoted: () }?
   |
   |
LL | struct Iced<T: Foo>(T, [(); T::ASSOC])
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2.rs:15:5
   |
LL |     [(); T::ASSOC]: ;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/conservative_is_privately_uninhabited_uses_correct_param_env-2.rs:13:24
   |
LL | struct Iced<T: Foo>(T, [(); T::ASSOC])
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
stack backtrace:
   0:     0x7f0618f688ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f0618fcf608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f0618f58ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f0618f6b8ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f0618f6b576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f0619927b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0618f6c062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f061c73d023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f061c73c8a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f06198e6af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f061c732066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f061c73610c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f0619943032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f061994d458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f061991740c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f0619913bfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f0619948e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f061990b3d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f0619917739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f061998ae3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f0619986492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f0618f77645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f06134c2609 - start_thread
  23:     0x7f0618dd5133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
note: required by a bound in `Foo`
   |
   |
LL | struct Foo<const N: u8>([u8; N as usize])
LL | where
LL | where
LL |     [(); N as usize]:;
   |          ^^^^^^^^^^ required by this bound in `Foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs:14:22
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs:14:26
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs:14:13
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
note: required by a bound in `Foo`
   |
   |
LL | struct Foo<const N: u8>([u8; N as usize])
LL | where
LL | where
LL |     [(); N as usize]:;
   |          ^^^^^^^^^^ required by this bound in `Foo`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-62504.rs#full stdout ----
diff of stderr:


7    = note: expected type `X`
8               found type `Self::SIZE`
9 
- error: unconstrained generic constant
-   --> $DIR/issue-62504.rs:18:25
-    |
- LL |         ArrayHolder([0; Self::SIZE])
-    |
-    |
-    = help: try adding a `where` bound using this expression: `where [(); Self::SIZE]:`
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
19 
20 For more information about this error, try `rustc --explain E0308`.
20 For more information about this error, try `rustc --explain E0308`.
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full/issue-62504.full.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-62504.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-62504.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-62504.rs:18:21
   |
   |
LL |         ArrayHolder([0; Self::SIZE])
   |                     ^^^^^^^^^^^^^^^ expected `X`, found `Self::SIZE`
   = note: expected type `X`
              found type `Self::SIZE`

error: aborting due to previous error
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-100217.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-100217.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-100217" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-100217/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:974:13: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout
   |
   |
LL |     fn do_two_stuff() {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
stack backtrace:
   0:     0x7f2f0adc48ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f2f0ae2b608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f2f0adb4ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f2f0adc78ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f2f0adc7576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f2f0b783b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2f0adc8062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f2f0ba8e373 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f2f0ba8a586 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f2f0b58d586 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f2f0ba8a3c6 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f2f0ba845b3 - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner>::span_bug::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span, &alloc[4c228ee6381bccf6]::string::String>
  12:     0x7f2f0ba84210 - <rustc_errors[6e3ab7d0d66c576b]::Handler>::span_bug::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span, &alloc[4c228ee6381bccf6]::string::String>
  13:     0x7f2f0b99d705 - rustc_middle[7dcea2cd529e0def]::util::bug::opt_span_bug_fmt::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>::{closure#0}
  14:     0x7f2f0b99e2eb - rustc_middle[7dcea2cd529e0def]::ty::context::tls::with_opt::<rustc_middle[7dcea2cd529e0def]::util::bug::opt_span_bug_fmt<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f2f0b99c1f6 - rustc_middle[7dcea2cd529e0def]::ty::context::tls::with_context_opt::<rustc_middle[7dcea2cd529e0def]::ty::context::tls::with_opt<rustc_middle[7dcea2cd529e0def]::util::bug::opt_span_bug_fmt<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f2f0b9957a9 - rustc_middle[7dcea2cd529e0def]::util::bug::opt_span_bug_fmt::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>
  17:     0x7f2f0b581537 - rustc_middle[7dcea2cd529e0def]::util::bug::span_bug_fmt::<rustc_span[ce74fb0acceea6fe]::span_encoding::Span>
  18:     0x7f2f0b9d35a5 - <rustc_codegen_llvm[55f96d9634b99f7c]::context::CodegenCx as rustc_middle[7dcea2cd529e0def]::ty::layout::LayoutOfHelpers>::handle_layout_err
  19:     0x7f2f0b9cef1b - <rustc_codegen_llvm[55f96d9634b99f7c]::context::CodegenCx as rustc_middle[7dcea2cd529e0def]::ty::layout::LayoutOf>::spanned_layout_of::{closure#0}
  20:     0x7f2f0b9d28a2 - <rustc_codegen_llvm[55f96d9634b99f7c]::context::CodegenCx as rustc_middle[7dcea2cd529e0def]::ty::layout::LayoutOf>::spanned_layout_of
  21:     0x7f2f0b9cb15c - <core[86ccb9a07b15c2c9]::iter::adapters::map::Map<core[86ccb9a07b15c2c9]::slice::iter::Iter<rustc_middle[7dcea2cd529e0def]::mir::LocalDecl>, rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::non_ssa_locals<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>::{closure#0}> as core[86ccb9a07b15c2c9]::iter::traits::iterator::Iterator>::fold::<(), core[86ccb9a07b15c2c9]::iter::traits::iterator::Iterator::for_each::call<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind, <alloc[4c228ee6381bccf6]::vec::Vec<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind> as alloc[4c228ee6381bccf6]::vec::spec_extend::SpecExtend<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind, core[86ccb9a07b15c2c9]::iter::adapters::map::Map<core[86ccb9a07b15c2c9]::slice::iter::Iter<rustc_middle[7dcea2cd529e0def]::mir::LocalDecl>, rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::non_ssa_locals<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>::{closure#0}>>>::spec_extend::{closure#0}>::{closure#0}>
  22:     0x7f2f0b9b9de3 - <alloc[4c228ee6381bccf6]::vec::Vec<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind> as alloc[4c228ee6381bccf6]::vec::spec_from_iter::SpecFromIter<rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::LocalKind, core[86ccb9a07b15c2c9]::iter::adapters::map::Map<core[86ccb9a07b15c2c9]::slice::iter::Iter<rustc_middle[7dcea2cd529e0def]::mir::LocalDecl>, rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::non_ssa_locals<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>::{closure#0}>>>::from_iter
  23:     0x7f2f0b9e55f8 - rustc_codegen_ssa[b1b21d015cc3745e]::mir::analyze::non_ssa_locals::<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>
  24:     0x7f2f0babd1b2 - rustc_codegen_ssa[b1b21d015cc3745e]::mir::codegen_mir::<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>
  25:     0x7f2f0b9e1f36 - rustc_codegen_ssa[b1b21d015cc3745e]::base::codegen_instance::<rustc_codegen_llvm[55f96d9634b99f7c]::builder::Builder>
  26:     0x7f2f0ba49094 - rustc_codegen_llvm[55f96d9634b99f7c]::base::compile_codegen_unit::module_codegen
  27:     0x7f2f0b973bd3 - <rustc_query_system[21f1fbb3509cb783]::dep_graph::graph::DepGraph<rustc_middle[7dcea2cd529e0def]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[7dcea2cd529e0def]::ty::context::TyCtxt, rustc_span[ce74fb0acceea6fe]::symbol::Symbol, rustc_codegen_ssa[b1b21d015cc3745e]::ModuleCodegen<rustc_codegen_llvm[55f96d9634b99f7c]::ModuleLlvm>>
  28:     0x7f2f0ba48b37 - rustc_codegen_llvm[55f96d9634b99f7c]::base::compile_codegen_unit
  29:     0x7f2f0b9e112a - rustc_codegen_ssa[b1b21d015cc3745e]::base::codegen_crate::<rustc_codegen_llvm[55f96d9634b99f7c]::LlvmCodegenBackend>
  30:     0x7f2f0baa086d - <rustc_codegen_llvm[55f96d9634b99f7c]::LlvmCodegenBackend as rustc_codegen_ssa[b1b21d015cc3745e]::traits::backend::CodegenBackend>::codegen_crate
  31:     0x7f2f0b8f4feb - <rustc_session[b989f28d2f7f5ae0]::session::Session>::time::<alloc[4c228ee6381bccf6]::boxed::Box<dyn core[86ccb9a07b15c2c9]::any::Any>, rustc_interface[cf2f5002c3170186]::passes::start_codegen::{closure#0}>
  32:     0x7f2f0b8bb4cc - <rustc_interface[cf2f5002c3170186]::passes::QueryContext>::enter::<<rustc_interface[cf2f5002c3170186]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<alloc[4c228ee6381bccf6]::boxed::Box<dyn core[86ccb9a07b15c2c9]::any::Any>, rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  33:     0x7f2f0b8a088d - <rustc_interface[cf2f5002c3170186]::queries::Queries>::ongoing_codegen
  34:     0x7f2f0b78b086 - <rustc_interface[cf2f5002c3170186]::interface::Compiler>::enter::<rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}::{closure#2}, core[86ccb9a07b15c2c9]::result::Result<core[86ccb9a07b15c2c9]::option::Option<rustc_interface[cf2f5002c3170186]::queries::Linker>, rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  35:     0x7f2f0b76fa21 - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  36:     0x7f2f0b7a4e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  37:     0x7f2f0b7673d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  38:     0x7f2f0b773739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  39:     0x7f2f0b7e6e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7f2f0b7e2492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f2f0add3645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  42:     0x7f2f0531e609 - start_thread
  43:     0x7f2f0ac31133 - clone
  44:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
---
-    |
- LL |         self.reference.size()
-    |                        ^^^^
-    |
-    = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
- note: required by a bound in `TensorSize::size`
-   --> $DIR/issue-83765.rs:9:31
-    |
- LL |     fn size(&self) -> [usize; Self::DIM];
-    |                               ^^^^^^^^^ required by this bound in `TensorSize::size`
23 error[E0308]: mismatched types
24   --> $DIR/issue-83765.rs:32:9
25    |

---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83765" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83765/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: method not compatible with trait
   |
   |
LL |     fn size(&self) -> [usize; DIM] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-83765.rs:32:9
   |
LL |         self.reference.size()
   |         ^^^^^^^^^^^^^^^^^^^^^ expected `DIM`, found `Self::DIM`
   = note: expected type `DIM`
              found type `Self::DIM`

error: aborting due to 2 previous errors
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:12 ~ issue_80561_incorrect_param_env[f504]::Bar::bar::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |         [(); Self::N]: ,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env.rs:18:9
   |
LL |         [(); Self::N]: ,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:12 ~ issue_80561_incorrect_param_env[f504]::Bar::bar::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |         Foo::<{ Self::N }>::foo();
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:13 ~ issue_80561_incorrect_param_env[f504]::Bar::bar::{constant#1}), const_param_did: Some(DefId(0:5 ~ issue_80561_incorrect_param_env[f504]::Foo::N)) }, substs: [Self], promoted: () }?
   |
   |
LL |         Foo::<{ Self::N }>::foo();
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env.rs:20:9
   |
LL |         Foo::<{ Self::N }>::foo();
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env.rs:20:9
   |
LL |         Foo::<{ Self::N }>::foo();
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: PromoteTemps: MIR had errors
   |
LL | /     fn bar()
LL | |     where
LL | |     where
LL | |         [(); Self::N]: ,
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22


error: internal compiler error: broken MIR in DefId(0:11 ~ issue_80561_incorrect_param_env[f504]::Bar::bar) ("return type"): bad type [type error]
   |
LL | /     fn bar()
LL | |     where
LL | |     where
LL | |         [(); Self::N]: ,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:11 ~ issue_80561_incorrect_param_env[f504]::Bar::bar) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80561-incorrect-param-env.rs:16:5: 18:25 (#0), scope: scope[0] } }): bad type [type error]
   |
LL | /     fn bar()
LL | |     where
LL | |     where
LL | |         [(); Self::N]: ,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f8cd7f508ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f8cd7fb7608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f8cd7f40ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f8cd7f538ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f8cd7f53576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f8cd890fb54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f8cd7f54062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f8cdb725023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f8cdb7248a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f8cd88ceaf6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f8cdb71a066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f8cdb71e10c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f8cd892b032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f8cd8935458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f8cd88ff40c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f8cd88fbbfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f8cd8930e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f8cd88f33d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f8cd88ff739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f8cd8972e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f8cd896e492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f8cd7f5f645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f8cd24aa609 - start_thread
  23:     0x7f8cd7dbd133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/let-bindings.rs stdout ----
diff of stderr:

2   --> $DIR/let-bindings.rs:6:68
3    |
4 LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
-    |                                                                    ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                                                    ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
6    |
6    |
7    = help: consider moving this anonymous constant into a `const` function

11   --> $DIR/let-bindings.rs:6:35
12    |
12    |
13 LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
-    |                                   ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                   ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
15    |
15    |
16    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings/let-bindings.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings/let-bindings.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/let-bindings.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/let-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
   |                                                                    ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/let-bindings.rs:6:35
   |
   |
LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
   |                                   ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     substs2::<{ L - 1 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { L - 1 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs:26:5
   |
   |
LL |     substs2::<{ L - 1 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { L - 1 }]:`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/unused_expr.rs stdout ----
diff of stderr:

2   --> $DIR/unused_expr.rs:4:34
3    |
4 LL | fn add<const N: usize>() -> [u8; { N + 1; 5 }] {
-    |                                  ^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                  ^^^^^^^^^^^^ blocks are not supported in generic
6    |
6    |
7    = help: consider moving this anonymous constant into a `const` function

11   --> $DIR/unused_expr.rs:9:34
12    |
12    |
13 LL | fn div<const N: usize>() -> [u8; { N / 1; 5 }] {
-    |                                  ^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                  ^^^^^^^^^^^^ blocks are not supported in generic
15    |
15    |
16    = help: consider moving this anonymous constant into a `const` function

20   --> $DIR/unused_expr.rs:16:38
21    |
21    |
22 LL | fn fn_call<const N: usize>() -> [u8; { foo(N); 5 }] {
-    |                                      ^^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                      ^^^^^^^^^^^^^ blocks are not supported in generic
24    |
24    |
25    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unused_expr/unused_expr.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/unused_expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/unused_expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unused_expr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unused_expr/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL | fn add<const N: usize>() -> [u8; { N + 1; 5 }] {
   |                                  ^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unused_expr.rs:9:34
   |
   |
LL | fn div<const N: usize>() -> [u8; { N / 1; 5 }] {
   |                                  ^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: overly complex generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unused_expr.rs:16:38
   |
   |
LL | fn fn_call<const N: usize>() -> [u8; { foo(N); 5 }] {
   |                                      ^^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     make_array::<{ N * 2 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N * 2 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:10:5
   |
   |
LL |     make_array::<{ N * 2 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N * 2 }]:`
error: aborting due to 2 previous errors
------------------------------------------


---
-    | |_____^ blocks are not supported in generic constant
+    | |_____^ blocks are not supported in generic
+             constant
12    |
13    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-2.full/issue-67945-2.full.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-67945-2.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67945-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-2.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-2.full/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL |       A: [(); {
   |  _____________^
LL | |         //[full]~^ ERROR overly complex generic constant
LL | |         let x: Option<Box<Self>> = None;
LL | |         //[min]~^ ERROR generic `Self` types are currently not permitted in anonymous constants
LL | |         0
LL | |     }],
   | |_____^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error
------------------------------------------

---
-    | |_____^ blocks are not supported in generic constant
+    | |_____^ blocks are not supported in generic
+             constant
11    |
12    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-3.full/issue-67945-3.full.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-67945-3.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67945-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-3.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-3.full/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL |       A: [(); { //[full]~ ERROR: overly complex generic constant
   |  _____________^
LL | |         let x: Option<S> = None;
LL | |         //[min]~^ ERROR: generic parameters may not be used in const operations
LL | |         0
LL | |     }],
   | |_____^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error
------------------------------------------

---
-    | |_____^ blocks are not supported in generic constant
+    | |_____^ blocks are not supported in generic
+             constant
11    |
12    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-4.full/issue-67945-4.full.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-67945-4.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67945-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-4.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-4.full/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL |       A: [(); { //[full]~ ERROR: overly complex generic constant
   |  _____________^
LL | |         let x: Option<Box<S>> = None;
LL | |         //[min]~^ ERROR: generic parameters may not be used in const operations
LL | |         0
LL | |     }],
   | |_____^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-77357.rs stdout ----
diff of stderr:

2   --> $DIR/issue-77357.rs:6:46
3    |
4 LL | fn bug<'a, T>() -> &'static dyn MyTrait<[(); { |x: &'a u32| { x }; 4 }]> {
-    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
+    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
6    |
6    |
7    = help: consider moving this anonymous constant into a `const` function


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357/issue-77357.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357/issue-77357.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-77357.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-77357.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-77357/auxiliary"
stdout: none
--- stderr -------------------------------
error: overly complex generic constant
   |
   |
LL | fn bug<'a, T>() -> &'static dyn MyTrait<[(); { |x: &'a u32| { x }; 4 }]> {
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-84659.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-84659.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-84659" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-84659/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:9 ~ issue_84659[e90b]::Foo::Baz::{constant#0}), const_param_did: Some(DefId(0:4 ~ issue_84659[e90b]::Bar::N)) }, substs: [Self, ReEarlyBound(1, 'a)], promoted: () }?
   |
   |
LL |     type Baz: Bar<{ Self::N }>;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-84659.rs:8:15
   |
LL |     type Baz: Bar<{ Self::N }>;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
stack backtrace:
   0:     0x7f458a8cd8ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f458a934608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f458a8bdff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f458a8d08ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f458a8d0576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f458b28cb54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f458a8d1062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f458e0a2023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f458e0a18a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f458b24baf6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f458e097066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f458e09b10c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f458b2a8032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f458b2b2458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f458b27c40c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f458b278bfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f458b2ade11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f458b2703d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f458b27c739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f458b2efe3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f458b2eb492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f458a8dc645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f4584e27609 - start_thread
  23:     0x7f458a73a133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-83249.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83249.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83249" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83249/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:9 ~ issue_83249[14ce]::foo::{constant#0}), const_param_did: None }, substs: [T], promoted: () }?
   |
   |
LL | fn foo<T: Foo>(_: [u8; T::N]) -> T {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:12:24
   |
LL | fn foo<T: Foo>(_: [u8; T::N]) -> T {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:9 ~ issue_83249[14ce]::foo::{constant#0}), const_param_did: None }, substs: [_], promoted: () }?
   |
   |
LL |     let _: u8 = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:17:17
   |
LL |     let _: u8 = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:9 ~ issue_83249[14ce]::foo::{constant#0}), const_param_did: None }, substs: [_], promoted: () }?
   |
   |
LL |     let _: u8 = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:17:21
   |
LL |     let _: u8 = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:9 ~ issue_83249[14ce]::foo::{constant#0}), const_param_did: None }, substs: [_], promoted: () }?
   |
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:13
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:9 ~ issue_83249[14ce]::foo::{constant#0}), const_param_did: None }, substs: [_], promoted: () }?
   |
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:17
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
---
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

error: internal compiler error: encountered ambiguity selecting `Binder(<[type error] as Foo>, [])` during codegen, presuming due to overflow or prior type error
   |
LL |     const N: usize;
   |     ^^^^^^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_ty_utils/src/instance.rs:213:37

error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83249.rs:19:17
   |
LL |     let _ = foo([0; 1]);
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: PromoteTemps: MIR had errors
   |
LL | pub fn bar() {
   | ^^^^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22

error: internal compiler error: broken MIR in DefId(0:10 ~ issue_83249[14ce]::bar) ("return type"): bad type [type error]
   |
LL | pub fn bar() {
   | ^^^^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:10 ~ issue_83249[14ce]::bar) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-83249.rs:16:1: 16:13 (#0), scope: scope[0] } }): bad type [type error]
   |
LL | pub fn bar() {
   | ^^^^^^^^^^^^
   |
---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7ff001ab78ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7ff001b1e608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7ff001aa7ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7ff001aba8ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7ff001aba576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7ff002476b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff001abb062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7ff00528c023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7ff00528b8a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7ff002435af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7ff005281066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7ff00528510c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7ff002492032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7ff00249c458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7ff00246640c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7ff002462bfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7ff002497e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7ff00245a3d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7ff002466739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7ff0024d9e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7ff0024d5492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7ff001ac6645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7feffc011609 - start_thread
  23:     0x7ff001924133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-83765.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`
-   --> $DIR/issue-83765.rs:5:5
+ error[E0308]: method not compatible with trait
3    |
3    |
- LL |     const DIM: usize;
-    |     ^^^^^^^^^^^^^^^^
+ LL |     fn size(&self) -> [usize; DIM] {
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
6    |
- note: ...which requires checking if `TensorDimension` fulfills its obligations...
-   --> $DIR/issue-83765.rs:4:1
+    = note: expected type `Self::DIM`
+               found type `DIM`
+ error[E0308]: method not compatible with trait
+   --> $DIR/issue-83765.rs:57:5
9    |
9    |
- LL | trait TensorDimension {
-    | ^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`, completing the cycle
- note: cycle used when checking if `TensorDimension` fulfills its obligations
-   --> $DIR/issue-83765.rs:4:1
+ LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
15    |
- LL | trait TensorDimension {
+    = note: expected type `Self::DIM`
+               found type `DIM`
18 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0308]: method not compatible with trait
+   --> $DIR/issue-83765.rs:81:5
+    |
+ LL |     fn size(&self) -> [usize; DIM] {
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
+    = note: expected type `Self::DIM`
+               found type `DIM`
20 
- For more information about this error, try `rustc --explain E0391`.
- For more information about this error, try `rustc --explain E0391`.
+ error[E0308]: method not compatible with trait
+   --> $DIR/issue-83765.rs:90:5
+    |
+ LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
+    = note: expected type `Self::DIM`
+               found type `DIM`
+ 
+ 
+ error[E0599]: the method `inbounds` exists for reference `&LazyUpdim<'a, T, { T::DIM }, DIM>`, but its trait bounds were not satisfied
+    |
+    |
+ LL | struct LazyUpdim<'a, T: Broadcastable, const OLDDIM: usize, const DIM: usize> {
+    | ----------------------------------------------------------------------------- doesn't satisfy `LazyUpdim<'_, T, { T::DIM }, DIM>: TensorSize`
+ ...
+ LL |         if !self.inbounds(index) {
+    |                  ^^^^^^^^ method cannot be called on `&LazyUpdim<'a, T, { T::DIM }, DIM>` due to unsatisfied trait bounds
+ note: the following trait bounds were not satisfied:
+   --> $DIR/issue-83765.rs:49:78
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |                                                                              |
+    |                                                                              unsatisfied trait bound introduced here
+ 
+ 
+ error[E0599]: the method `size` exists for reference `&LazyUpdim<'a, T, { T::DIM }, DIM>`, but its trait bounds were not satisfied
+    |
+    |
+ LL | struct LazyUpdim<'a, T: Broadcastable, const OLDDIM: usize, const DIM: usize> {
+    | ----------------------------------------------------------------------------- doesn't satisfy `LazyUpdim<'_, T, { T::DIM }, DIM>: TensorSize`
+ LL |         let size = self.size();
+    |                         ^^^^-- help: remove the arguments
+    |                         |
+    |                         field, not a method
+    |                         field, not a method
+    |
+ note: the following trait bounds were not satisfied:
+   --> $DIR/issue-83765.rs:49:78
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |                                                                              |
+    |                                                                              unsatisfied trait bound introduced here
+ 
+ 
+ error[E0277]: the trait bound `[usize; _]: Default` is not satisfied
+    |
+    |
+ LL |         let newindex: [usize; T::DIM] = Default::default();
+    |                                         ^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `[usize; _]`
+    |
+ help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> where [usize; _]: Default {
+ 
+ error[E0308]: mismatched types
+   --> $DIR/issue-83765.rs:82:9
+    |
+    |
+ LL |         self.reference.size()
+    |         ^^^^^^^^^^^^^^^^^^^^^ expected `DIM`, found `Self::DIM`
+    = note: expected type `DIM`
+               found type `Self::DIM`
+ 
+ error[E0308]: mismatched types
+ error[E0308]: mismatched types
+   --> $DIR/issue-83765.rs:91:29
+    |
+ LL |         self.reference.bget(index).map(&self.closure)
+    |                             ^^^^^ expected `Self::DIM`, found `DIM`
+    = note: expected type `Self::DIM`
+               found type `DIM`
+ 
+ 
+ error[E0599]: the method `bmap` exists for struct `LazyUpdim<'_, Vec<{integer}>, { Self::DIM }, 2>`, but its trait bounds were not satisfied
+    |
+    |
+ LL | struct LazyUpdim<'a, T: Broadcastable, const OLDDIM: usize, const DIM: usize> {
+    | |
+    | |
+    | method `bmap` not found for this struct
+    | doesn't satisfy `_: Broadcastable`
+ ...
+ LL |     let bbv = bv.bmap(|x| x * x);
+    |                  ^^^^ method cannot be called on `LazyUpdim<'_, Vec<{integer}>, { Self::DIM }, 2>` due to unsatisfied trait bounds
+ note: the following trait bounds were not satisfied:
+   --> $DIR/issue-83765.rs:55:81
+    |
+    |
+ LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
+    |                                                                                 |
+    |                                                                                 unsatisfied trait bound introduced here
+ 
+ error: aborting due to 10 previous errors
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: method not compatible with trait
   |
   |
LL |     fn size(&self) -> [usize; DIM] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: method not compatible with trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:57:5
   |
LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: method not compatible with trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:81:5
   |
LL |     fn size(&self) -> [usize; DIM] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: method not compatible with trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:90:5
   |
LL |     fn bget(&self, index: [usize; DIM]) -> Option<Self::Element> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`


error[E0599]: the method `inbounds` exists for reference `&LazyUpdim<'a, T, { T::DIM }, DIM>`, but its trait bounds were not satisfied
   |
   |
LL | struct LazyUpdim<'a, T: Broadcastable, const OLDDIM: usize, const DIM: usize> {
   | ----------------------------------------------------------------------------- doesn't satisfy `LazyUpdim<'_, T, { T::DIM }, DIM>: TensorSize`
...
LL |         if !self.inbounds(index) {
   |                  ^^^^^^^^ method cannot be called on `&LazyUpdim<'a, T, { T::DIM }, DIM>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:49:78
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |                                                                              |
   |                                                                              unsatisfied trait bound introduced here


error[E0599]: the method `size` exists for reference `&LazyUpdim<'a, T, { T::DIM }, DIM>`, but its trait bounds were not satisfied
   |
   |
LL | struct LazyUpdim<'a, T: Broadcastable, const OLDDIM: usize, const DIM: usize> {
   | ----------------------------------------------------------------------------- doesn't satisfy `LazyUpdim<'_, T, { T::DIM }, DIM>: TensorSize`
LL |         let size = self.size();
   |                         ^^^^-- help: remove the arguments
   |                         |
   |                         field, not a method
   |                         field, not a method
   |
note: the following trait bounds were not satisfied:
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:49:78
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> TensorSize for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |                                                                              |
   |                                                                              unsatisfied trait bound introduced here


error[E0277]: the trait bound `[usize; _]: Default` is not satisfied
   |
   |
LL |         let newindex: [usize; T::DIM] = Default::default();
   |                                         ^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `[usize; _]`
   |
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> where [usize; _]: Default {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:82:9
   |
   |
LL |         self.reference.size()
   |         ^^^^^^^^^^^^^^^^^^^^^ expected `DIM`, found `Self::DIM`
   = note: expected type `DIM`
              found type `Self::DIM`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:91:29
   |
LL |         self.reference.bget(index).map(&self.closure)
   |                             ^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`


error[E0599]: the method `bmap` exists for struct `LazyUpdim<'_, Vec<{integer}>, { Self::DIM }, 2>`, but its trait bounds were not satisfied
   |
   |
LL | struct LazyUpdim<'a, T: Broadcastable, const OLDDIM: usize, const DIM: usize> {
   | |
   | |
   | method `bmap` not found for this struct
   | doesn't satisfy `_: Broadcastable`
...
LL |     let bbv = bv.bmap(|x| x * x);
   |                  ^^^^ method cannot be called on `LazyUpdim<'_, Vec<{integer}>, { Self::DIM }, 2>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:55:81
   |
   |
LL | impl<'a, T: Broadcastable, const DIM: usize> Broadcastable for LazyUpdim<'a, T, { T::DIM }, DIM> {
   |                                                                                 |
   |                                                                                 unsatisfied trait bound introduced here

error: aborting due to 10 previous errors
---
---- [ui] src/test/ui/const-generics/issues/issue-83288.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83288.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83288" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83288/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:43 ~ issue_83288[082f]::Tensor::{constant#0}), const_param_did: None }, substs: [I, Const { ty: usize, kind: Param(N/#1) }], promoted: () }?
   |
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:45 ~ issue_83288[082f]::Tensor::data::{constant#0}), const_param_did: None }, substs: [I, Const { ty: usize, kind: Param(N/#1) }], promoted: () }?
   |
   |
LL |     pub data: [u8; I::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:45:10
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:47:15
   |
LL |     pub data: [u8; I::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:51 ~ issue_83288[082f]::{impl#4}::{constant#0}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:52 ~ issue_83288[082f]::{impl#4}::{constant#1}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:53 ~ issue_83288[082f]::{impl#4}::{constant#2}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:43 ~ issue_83288[082f]::Tensor::{constant#0}), const_param_did: None }, substs: [I, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL | impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:43 ~ issue_83288[082f]::Tensor::{constant#0}), const_param_did: None }, substs: [J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL | impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:55:10
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:55:10
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:55:10
   |
LL |     [u8; I::NUM_ELEMS]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:51:74
   |
LL | impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:51:52
   |
LL | impl<I: Indices<N>, J: Indices<N>, const N: usize> Mul<Tensor<J, N>> for Tensor<I, N>
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:43 ~ issue_83288[082f]::Tensor::{constant#0}), const_param_did: None }, substs: [<I as Concat<J>>::Output, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     type Output = Tensor<<I as Concat<J>>::Output, N>;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:59:19
   |
LL |     type Output = Tensor<<I as Concat<J>>::Output, N>;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:51 ~ issue_83288[082f]::{impl#4}::{constant#0}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:52 ~ issue_83288[082f]::{impl#4}::{constant#1}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:53 ~ issue_83288[082f]::{impl#4}::{constant#2}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:43 ~ issue_83288[082f]::Tensor::{constant#0}), const_param_did: None }, substs: [I, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:43 ~ issue_83288[082f]::Tensor::{constant#0}), const_param_did: None }, substs: [J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:43 ~ issue_83288[082f]::Tensor::{constant#0}), const_param_did: None }, substs: [<I as Concat<J>>::Output, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:8
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:8
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:8
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:12
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:24
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:41
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:51 ~ issue_83288[082f]::{impl#4}::{constant#0}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:52 ~ issue_83288[082f]::{impl#4}::{constant#1}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:53 ~ issue_83288[082f]::{impl#4}::{constant#2}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:5
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:5
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:5
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:51 ~ issue_83288[082f]::{impl#4}::{constant#0}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |       fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
LL | |         Tensor {
LL | |         Tensor {
LL | |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
LL | |             _phantom: PhantomData,
LL | |         }
LL | |     }
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:52 ~ issue_83288[082f]::{impl#4}::{constant#1}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |       fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
LL | |         Tensor {
LL | |         Tensor {
LL | |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
LL | |             _phantom: PhantomData,
LL | |         }
LL | |     }
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:53 ~ issue_83288[082f]::{impl#4}::{constant#2}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |       fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
LL | |         Tensor {
LL | |         Tensor {
LL | |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
LL | |             _phantom: PhantomData,
LL | |         }
LL | |     }
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:43 ~ issue_83288[082f]::Tensor::{constant#0}), const_param_did: None }, substs: [_, Const { ty: usize, kind: Infer(Var(_#1c)) }], promoted: () }?
   |
LL |         Tensor {
   |         ^^^^^^
   |
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:54
   |
LL |       fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
LL | |         Tensor {
LL | |         Tensor {
LL | |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
LL | |             _phantom: PhantomData,
LL | |         }
LL | |     }
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:54
   |
LL |       fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
LL | |         Tensor {
LL | |         Tensor {
LL | |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
LL | |             _phantom: PhantomData,
LL | |         }
LL | |     }
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:61:54
   |
LL |       fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
LL | |         Tensor {
LL | |         Tensor {
LL | |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
LL | |             _phantom: PhantomData,
LL | |         }
LL | |     }
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:62:9
   |
LL |         Tensor {
   |         ^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:51 ~ issue_83288[082f]::{impl#4}::{constant#0}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:52 ~ issue_83288[082f]::{impl#4}::{constant#1}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:53 ~ issue_83288[082f]::{impl#4}::{constant#2}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:56 ~ issue_83288[082f]::{impl#4}::mul::{constant#0}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   |
   |
LL |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:63:25
   |
LL |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:63:25
   |
LL |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:63:25
   |
LL |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-83288.rs:63:25
   |
LL |             data: [0u8; <I as Concat<J>>::Output::NUM_ELEMS],
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: PromoteTemps: MIR had errors
   |
   |
LL |     fn mul(self, _rhs: Tensor<J, N>) -> Self::Output {
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:51 ~ issue_83288[082f]::{impl#4}::{constant#0}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:52 ~ issue_83288[082f]::{impl#4}::{constant#1}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:53 ~ issue_83288[082f]::{impl#4}::{constant#2}), const_param_did: None }, substs: [I, J, Const { ty: usize, kind: Param(N/#2) }], promoted: () }?
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: failed to normalize <Tensor<I, N> as std::ops::Mul<Tensor<J, N>>>::Output
   = note: delayed at compiler/rustc_borrowck/src/type_check/free_region_relations.rs:256:30

---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f2b24af48ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f2b24b5b608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f2b24ae4ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f2b24af78ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f2b24af7576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f2b254b3b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2b24af8062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f2b282c9023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f2b282c88a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f2b25472af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f2b282be066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f2b282c210c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f2b254cf032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f2b254d9458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f2b254a340c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f2b2549fbfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f2b254d4e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f2b254973d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f2b254a3739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f2b25516e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f2b25512492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f2b24b03645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f2b1f04e609 - start_thread
  23:     0x7f2b24961133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-86535.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-86535.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86535/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86535/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:18 ~ issue_86535[df77]::X::d::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     fn d(r: &[u8; Self::W]) -> Self;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-86535.rs:17:19
   |
LL |     fn d(r: &[u8; Self::W]) -> Self;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
stack backtrace:
   0:     0x7fbc553a18ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7fbc55408608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7fbc55391ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7fbc553a48ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7fbc553a4576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7fbc55d60b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fbc553a5062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7fbc58b76023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7fbc58b758a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7fbc55d1faf6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7fbc58b6b066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7fbc58b6f10c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7fbc55d7c032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7fbc55d86458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7fbc55d5040c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7fbc55d4cbfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fbc55d81e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7fbc55d443d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7fbc55d50739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7fbc55dc3e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7fbc55dbf492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7fbc553b0645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7fbc4f8fb609 - start_thread
  23:     0x7fbc5520e133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-86535-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-86535-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86535-2/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86535-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ issue_86535_2[7dcf]::Foo::foo::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     fn foo() where [(); Self::ASSOC_C]:;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-86535-2.rs:7:20
   |
LL |     fn foo() where [(); Self::ASSOC_C]:;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
stack backtrace:
   0:     0x7ff1191b48ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7ff11921b608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7ff1191a4ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7ff1191b78ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7ff1191b7576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7ff119b73b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff1191b8062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7ff11c989023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7ff11c9888a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7ff119b32af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7ff11c97e066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7ff11c98210c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7ff119b8f032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7ff119b99458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7ff119b6340c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7ff119b5fbfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7ff119b94e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7ff119b573d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7ff119b63739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7ff119bd6e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7ff119bd2492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7ff1191c3645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7ff11370e609 - start_thread
  23:     0x7ff119021133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-87964.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-87964.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-87964" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-87964/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:7 ~ issue_87964[c336]::Container::{constant#0}), const_param_did: None }, substs: [T], promoted: () }?
   |
   |
LL |     [(); T::LENGTH]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:13:10
   |
LL |     [(); T::LENGTH]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:11 ~ issue_87964[c336]::{impl#0}::{constant#0}), const_param_did: None }, substs: [T], promoted: () }?
   |
   |
LL |     [(); T::LENGTH]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:7 ~ issue_87964[c336]::Container::{constant#0}), const_param_did: None }, substs: [T], promoted: () }?
   |
   |
LL | impl<T: Target> Container<T>
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:20:10
   |
LL |     [(); T::LENGTH]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:18:17
   |
LL | impl<T: Target> Container<T>
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:11 ~ issue_87964[c336]::{impl#0}::{constant#0}), const_param_did: None }, substs: [T], promoted: () }?
   |
   |
LL |     [(); T::LENGTH]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:7 ~ issue_87964[c336]::Container::{constant#0}), const_param_did: None }, substs: [T], promoted: () }?
   |
LL |     ) -> Container<T> {
   |          ^^^^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:20:10
   |
LL |     [(); T::LENGTH]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:24:10
   |
LL |     ) -> Container<T> {
   |          ^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:7 ~ issue_87964[c336]::Container::{constant#0}), const_param_did: None }, substs: [_], promoted: () }?
   |
   |
LL |         Container { _target }
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-87964.rs:25:9
   |
LL |         Container { _target }
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: PromoteTemps: MIR had errors
   |
LL | /     pub fn start(
LL | |         _target: T,
LL | |     ) -> Container<T> {
LL | |     ) -> Container<T> {
   | |_____________________^
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22

error: internal compiler error: broken MIR in DefId(0:12 ~ issue_87964[c336]::{impl#0}::start) ("return type"): bad type [type error]
   |
LL | /     pub fn start(
LL | |         _target: T,
LL | |     ) -> Container<T> {
---
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:12 ~ issue_87964[c336]::{impl#0}::start) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-87964.rs:22:5: 24:22 (#0), scope: scope[0] } }): bad type [type error]
   |
LL | /     pub fn start(
LL | |         _target: T,
LL | |     ) -> Container<T> {
---
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:12 ~ issue_87964[c336]::{impl#0}::start) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-87964.rs:22:5: 24:22 (#0), scope: scope[0] } }): bad type [type error]
   |
LL | /     pub fn start(
LL | |         _target: T,
LL | |     ) -> Container<T> {
---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f9c177218ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f9c17788608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f9c17711ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f9c177248ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f9c17724576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f9c180e0b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f9c17725062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f9c1aef6023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f9c1aef58a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f9c1809faf6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f9c1aeeb066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f9c1aeef10c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f9c180fc032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f9c18106458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f9c180d040c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f9c180ccbfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f9c18101e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f9c180c43d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f9c180d0739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f9c18143e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f9c1813f492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f9c17730645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f9c11c7b609 - start_thread
  23:     0x7f9c1758e133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-89146.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-89146.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89146" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89146/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:9 ~ issue_89146[a52a]::bar::{constant#0}), const_param_did: None }, substs: [G], promoted: () }?
   |
   |
LL |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:14:10
   |
LL |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:12 ~ issue_89146[a52a]::deeper_bar::{constant#0}), const_param_did: None }, substs: [G], promoted: () }?
   |
   |
LL |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:21:10
   |
LL |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ issue_89146[a52a]::Foo::to_bytes::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     fn to_bytes(&self) -> [u8; Self::SIZE];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:9:32
   |
LL |     fn to_bytes(&self) -> [u8; Self::SIZE];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:12 ~ issue_89146[a52a]::deeper_bar::{constant#0}), const_param_did: None }, substs: [_], promoted: () }?
   |
LL |     deeper_bar(a)
   |     ^^^^^^^^^^
   |
---
   |     ^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ issue_89146[a52a]::Foo::to_bytes::{constant#0}), const_param_did: None }, substs: [G], promoted: () }?
   |
   |
LL |     a.to_bytes()[0]
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:23:7
   |
LL |     a.to_bytes()[0]
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ issue_89146[a52a]::Foo::to_bytes::{constant#0}), const_param_did: None }, substs: [G], promoted: () }?
   |
   |
LL |     a.to_bytes()[0]
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89146.rs:23:5
   |
LL |     a.to_bytes()[0]
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: PromoteTemps: MIR had errors
   |
   |
LL | / pub fn bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22


error: internal compiler error: broken MIR in DefId(0:7 ~ issue_89146[a52a]::bar) ("return type"): bad type [type error]
   |
   |
LL | / pub fn bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:7 ~ issue_89146[a52a]::bar) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-89146.rs:12:1: 14:26 (#0), scope: scope[0] } }): bad type [type error]
   |
   |
LL | / pub fn bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:7 ~ issue_89146[a52a]::bar) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-89146.rs:12:1: 14:26 (#0), scope: scope[0] } }): bad type [type error]
   |
   |
LL | / pub fn bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
---
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: PromoteTemps: MIR had errors
   |
   |
LL | / fn deeper_bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22


error: internal compiler error: broken MIR in DefId(0:10 ~ issue_89146[a52a]::deeper_bar) ("return type"): bad type [type error]
   |
   |
LL | / fn deeper_bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:10 ~ issue_89146[a52a]::deeper_bar) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-89146.rs:19:1: 21:26 (#0), scope: scope[0] } }): bad type [type error]
   |
   |
LL | / fn deeper_bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:10 ~ issue_89146[a52a]::deeper_bar) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-89146.rs:19:1: 21:26 (#0), scope: scope[0] } }): bad type [type error]
   |
   |
LL | / fn deeper_bar<G: Foo>(a: &G) -> u8
LL | | where
LL | |     [(); G::SIZE]: Sized,
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13

error: internal compiler error: TyKind::Error constructed but no error reported
---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f69a945b8ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f69a94c2608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f69a944bff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f69a945e8ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f69a945e576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f69a9e1ab54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f69a945f062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f69acc30023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f69acc2f8a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f69a9dd9af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f69acc25066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f69acc2910c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f69a9e36032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f69a9e40458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f69a9e0a40c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f69a9e06bfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f69a9e3be11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f69a9dfe3d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f69a9e0a739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f69a9e7de3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f69a9e79492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f69a946a645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f69a39b5609 - start_thread
  23:     0x7f69a92c8133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-87470.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-87470.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-87470" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-87470/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:7 ~ issue_87470[e45b]::OtherTrait::some_fn::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     fn some_fn(self) -> [u8 ; <Self as TraitWithConst>::SOME_CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-87470.rs:11:31
   |
LL |     fn some_fn(self) -> [u8 ; <Self as TraitWithConst>::SOME_CONST];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
stack backtrace:
   0:     0x7f89be8998ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f89be900608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f89be889ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f89be89c8ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f89be89c576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f89bf258b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f89be89d062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f89c206e023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f89c206d8a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f89bf217af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f89c2063066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f89c206710c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f89bf274032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f89bf27e458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f89bf24840c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f89bf244bfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f89bf279e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f89bf23c3d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f89bf248739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f89bf2bbe3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f89bf2b7492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f89be8a8645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f89b8df3609 - start_thread
  23:     0x7f89be706133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-89334.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-89334.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89334" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89334/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:7 ~ issue_89334[f160]::Shard::{constant#0}), const_param_did: None }, substs: [Self, T], promoted: () }?
   |
   |
LL |     [(); T::ARRAY_SIZE]: Sized
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:8 ~ issue_89334[f160]::Shard::{constant#1}), const_param_did: None }, substs: [Self, T], promoted: () }?
   |
   |
LL |     AsMut<[[u8; T::ARRAY_SIZE]]>
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89334.rs:12:10
   |
LL |     [(); T::ARRAY_SIZE]: Sized
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89334.rs:10:5
   |
LL |     AsMut<[[u8; T::ARRAY_SIZE]]>
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
stack backtrace:
   0:     0x7f2ff29e68ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f2ff2a4d608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f2ff29d6ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f2ff29e98ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f2ff29e9576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f2ff33a5b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2ff29ea062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f2ff61bb023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f2ff61ba8a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f2ff3364af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f2ff61b0066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f2ff61b410c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f2ff33c1032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f2ff33cb458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f2ff339540c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f2ff3391bfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f2ff33c6e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f2ff33893d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f2ff3395739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f2ff3408e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f2ff3404492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f2ff29f5645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f2fecf40609 - start_thread
  23:     0x7f2ff2853133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-89320.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-89320.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89320" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-89320/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:8 ~ issue_89320[7596]::SymmetricGroup::{constant#0}), const_param_did: None }, substs: [S], promoted: () }?
   |
   |
LL |     [(); S::N]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89320.rs:14:10
   |
LL |     [(); S::N]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:12 ~ issue_89320[7596]::{impl#0}::{constant#0}), const_param_did: None }, substs: [S], promoted: () }?
   |
   |
LL |     [(); S::N]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:8 ~ issue_89320[7596]::SymmetricGroup::{constant#0}), const_param_did: None }, substs: [S], promoted: () }?
   |
LL | #[derive(Clone)]
   |          ^^^^^
   |
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63
   = note: this error: internal compiler error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89320.rs:14:10
   |
LL |     [(); S::N]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
---
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31
   = note: this error: internal compiler error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:12 ~ issue_89320[7596]::{impl#0}::{constant#0}), const_param_did: None }, substs: [S], promoted: () }?
   |
   |
LL |     [(); S::N]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:8 ~ issue_89320[7596]::SymmetricGroup::{constant#0}), const_param_did: None }, substs: [S], promoted: () }?
   |
LL | #[derive(Clone)]
   |          ^^^^^
   |
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63
   = note: this error: internal compiler error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/issues/issue-89320.rs:14:10
   |
LL |     [(); S::N]: Sized,
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
---
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31
   = note: this error: internal compiler error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:8 ~ issue_89320[7596]::SymmetricGroup::{constant#0}), const_param_did: None }, substs: [_], promoted: () }?
   |
LL | #[derive(Clone)]
   |          ^
   |
---
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31
   = note: this error: internal compiler error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:749:18

error: internal compiler error: PromoteTemps: MIR had errors
   |
LL | #[derive(Clone)]
   |          ^^^^^
   |
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22
   = note: this error: internal compiler error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: broken MIR in DefId(0:13 ~ issue_89320[7596]::{impl#0}::clone) ("return type"): bad type [type error]
   |
LL | #[derive(Clone)]
   |          ^^^^^
   |
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13
   = note: this error: internal compiler error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:13 ~ issue_89320[7596]::{impl#0}::clone) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-89320.rs:10:10: 10:15 (#4), scope: scope[0] } }): bad type [type error]
   |
LL | #[derive(Clone)]
   |          ^^^^^
   |
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:543:13
   = note: this error: internal compiler error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:798:20

error: internal compiler error: broken MIR in DefId(0:13 ~ issue_89320[7596]::{impl#0}::clone) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/const-generics/issues/issue-89320.rs:10:10: 10:15 (#4), scope: scope[0] } }): bad type [type error]
   |
LL | #[derive(Clone)]
   |          ^^^^^
   |
---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7faa9a7778ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7faa9a7de608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7faa9a767ff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7faa9a77a8ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7faa9a77a576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7faa9b136b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7faa9a77b062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7faa9df4c023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7faa9df4b8a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7faa9b0f5af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7faa9df41066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7faa9df4510c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7faa9b152032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7faa9b15c458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7faa9b12640c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7faa9b122bfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7faa9b157e11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7faa9b11a3d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7faa9b126739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7faa9b199e3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7faa9b195492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7faa9a786645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7faa94cd1609 - start_thread
  23:     0x7faa9a5e4133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/try_unify_ignore_lifetimes.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/try_unify_ignore_lifetimes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/try_unify_ignore_lifetimes/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:15 ~ try_unify_ignore_lifetimes[948c]::Foo::{constant#0}), const_param_did: None }, substs: [ReEarlyBound(0, 'a), N], promoted: () }?
   |
   |
LL | struct Foo<'a, N: NumT>(&'a [u32; N::VALUE]) where [(); N::VALUE]:;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:17 ~ try_unify_ignore_lifetimes[948c]::Foo::0::{constant#0}), const_param_did: None }, substs: [ReEarlyBound(0, 'a), N], promoted: () }?
   |
   |
LL | struct Foo<'a, N: NumT>(&'a [u32; N::VALUE]) where [(); N::VALUE]:;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:15:52
   |
LL | struct Foo<'a, N: NumT>(&'a [u32; N::VALUE]) where [(); N::VALUE]:;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:15:25
   |
LL | struct Foo<'a, N: NumT>(&'a [u32; N::VALUE]) where [(); N::VALUE]:;
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:22 ~ try_unify_ignore_lifetimes[948c]::Bar::bar::{constant#0}), const_param_did: None }, substs: [Self], promoted: () }?
   |
   |
LL |     fn bar<'a>(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:15 ~ try_unify_ignore_lifetimes[948c]::Foo::{constant#0}), const_param_did: None }, substs: [ReFree(DefId(0:20 ~ try_unify_ignore_lifetimes[948c]::Bar::bar), BrNamed(DefId(0:21 ~ try_unify_ignore_lifetimes[948c]::Bar::bar::'a), 'a)), <Self as Bar>::Size], promoted: () }?
   |
   |
LL |     fn bar<'a>(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:20:49
   |
LL |     fn bar<'a>(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:20:21
   |
LL |     fn bar<'a>(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:27 ~ try_unify_ignore_lifetimes[948c]::Baz::baz::{constant#0}), const_param_did: None }, substs: [Self, ReEarlyBound(1, 'a)], promoted: () }?
   |
   |
LL |     fn baz(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63


error: internal compiler error: Missing value for constant, but no error reported Unevaluated { def: WithOptConstParam { did: DefId(0:15 ~ try_unify_ignore_lifetimes[948c]::Foo::{constant#0}), const_param_did: None }, substs: [ReEarlyBound(1, 'a), <Self as Baz<'a>>::Size], promoted: () }?
   |
   |
LL |     fn baz(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:182:63

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:28:45
   |
LL |     fn baz(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31

error: internal compiler error: `ErrorGuaranteed` without an error
error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs:28:17
   |
LL |     fn baz(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1036:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:209:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
stack backtrace:
   0:     0x7f6e4d82a8ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbefe4e9e4f4a849d
   1:     0x7f6e4d891608 - core::fmt::write::hc3536cbb28a0e56b
   2:     0x7f6e4d81aff1 - std::io::Write::write_fmt::h2fde585f32389108
   3:     0x7f6e4d82d8ae - std::panicking::default_hook::{{closure}}::h4e8654da845723d0
   4:     0x7f6e4d82d576 - std::panicking::default_hook::h913a18ec97ee87bf
   5:     0x7f6e4e1e9b54 - rustc_driver[a59b54d20b4b012e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6e4d82e062 - std::panicking::rust_panic_with_hook::h7312af11ba1d6141
   7:     0x7f6e50fff023 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}
   8:     0x7f6e50ffe8a6 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_end_short_backtrace::<std[fe43f7d36de38762]::panicking::begin_panic<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f6e4e1a8af6 - std[fe43f7d36de38762]::panicking::begin_panic::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  10:     0x7f6e50ff4066 - std[fe43f7d36de38762]::panic::panic_any::<rustc_errors[6e3ab7d0d66c576b]::ExplicitBug>
  11:     0x7f6e50ff810c - <rustc_errors[6e3ab7d0d66c576b]::HandlerInner as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  12:     0x7f6e4e205032 - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_session[b989f28d2f7f5ae0]::parse::ParseSess>
  13:     0x7f6e4e20f458 - <alloc[4c228ee6381bccf6]::rc::Rc<rustc_session[b989f28d2f7f5ae0]::session::Session> as core[86ccb9a07b15c2c9]::ops::drop::Drop>::drop
  14:     0x7f6e4e1d940c - core[86ccb9a07b15c2c9]::ptr::drop_in_place::<rustc_interface[cf2f5002c3170186]::interface::Compiler>
  15:     0x7f6e4e1d5bfd - rustc_span[ce74fb0acceea6fe]::with_source_map::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f6e4e20ae11 - rustc_interface[cf2f5002c3170186]::interface::create_compiler_and_run::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>
  17:     0x7f6e4e1cd3d2 - <scoped_tls[8c93cd97abf9c805]::ScopedKey<rustc_span[ce74fb0acceea6fe]::SessionGlobals>>::set::<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  18:     0x7f6e4e1d9739 - std[fe43f7d36de38762]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>
  19:     0x7f6e4e24ce3e - std[fe43f7d36de38762]::panicking::try::<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, core[86ccb9a07b15c2c9]::panic::unwind_safe::AssertUnwindSafe<<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f6e4e248492 - <<std[fe43f7d36de38762]::thread::Builder>::spawn_unchecked_<rustc_interface[cf2f5002c3170186]::util::run_in_thread_pool_with_globals<rustc_interface[cf2f5002c3170186]::interface::run_compiler<core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>, rustc_driver[a59b54d20b4b012e]::run_compiler::{closure#1}>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#0}, core[86ccb9a07b15c2c9]::result::Result<(), rustc_errors[6e3ab7d0d66c576b]::ErrorGuaranteed>>::{closure#1} as core[86ccb9a07b15c2c9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f6e4d839645 - std::sys::unix::thread::Thread::new::thread_start::h4d6e7087e0827938
  22:     0x7f6e47d84609 - start_thread
  23:     0x7f6e4d697133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e1802f398 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/privacy/where-priv-type.rs stdout ----
diff of stderr:

52    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
53    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
54 
- error[E0446]: private type `fn(u8) -> u8 {my_const_fn}` in public interface
-   --> $DIR/where-priv-type.rs:80:5
-    |
- LL |     type AssocTy = Const<{ my_const_fn(U) }>;
-    |     ^^^^^^^^^^^^ can't leak private type
- ...
- LL | const fn my_const_fn(val: u8) -> u8 {
-    | ----------------------------------- `fn(u8) -> u8 {my_const_fn}` declared as private
- error: aborting due to 2 previous errors; 4 warnings emitted
+ error: aborting due to previous error; 4 warnings emitted
65 
66 For more information about this error, try `rustc --explain E0446`.
---
To only update this specific test, also pass `--test-args privacy/where-priv-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/where-priv-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/where-priv-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/where-priv-type/auxiliary"
stdout: none
--- stderr -------------------------------
warning: private type `PrivTy` in public interface (error E0446)
   |
LL | pub struct S
   | ^^^^^^^^^^^^
   |
   |
   = note: `#[warn(private_in_public)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

warning: private type `PrivTy` in public interface (error E0446)
   |
LL | pub enum E
   | ^^^^^^^^^^
   |
   |
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
LL | struct PrivTy;
LL | struct PrivTy;
   | ------------- `PrivTy` declared as private
LL | impl S
   | ^^^^^^ can't leak private type


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

---

---- [ui] src/test/ui/simd/array-trait.rs stdout ----
diff of stderr:

- error: unconstrained generic constant
-   --> $DIR/array-trait.rs:23:23
+ error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
3    |
3    |
4 LL | pub struct T<S: Simd>([S::Lane; S::SIZE]);
-    |
-    |
-    = help: try adding a `where` bound using this expression: `where [(); S::SIZE]:`
8 
9 error: aborting due to previous error
10 

---
To only update this specific test, also pass `--test-args simd/array-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/array-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/array-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/array-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
   |
   |
LL | pub struct T<S: Simd>([S::Lane; S::SIZE]);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0077`.
