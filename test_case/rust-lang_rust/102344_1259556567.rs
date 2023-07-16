plain
---- [ui] src/test/ui/traits/issue-97695-double-trivial-bound.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-97695-double-trivial-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-97695-double-trivial-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zinline-mir" "--emit=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-97695-double-trivial-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:198:90: Failed to normalize <Wrap<'_> as Associate>::Associated, maybe try to call `try_normalize_erasing_regions` instead
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1495:9
stack backtrace:
stack backtrace:
   0:     0x7f7f26c3653e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h91653f5cf5cd7d8e
   1:     0x7f7f26c9f3c8 - core::fmt::write::hf1619cfcb65ce90d
   2:     0x7f7f26c272d1 - std::io::Write::write_fmt::ha78b4ee158509b2a
   3:     0x7f7f26c395ee - std::panicking::default_hook::{{closure}}::h59aaaa9246f12210
   4:     0x7f7f26c392ae - std::panicking::default_hook::h11942854dcb29079
   5:     0x7f7f275ecc24 - rustc_driver[5e25011d52c03d73]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f7f26c39dad - std::panicking::rust_panic_with_hook::he31483420d9db25d
   7:     0x7f7f29fdb8f3 - std[f3b1b0f07ba26383]::panicking::begin_panic::<rustc_errors[f9e52a26da800f21]::ExplicitBug>::{closure#0}
   8:     0x7f7f29fd84a6 - std[f3b1b0f07ba26383]::sys_common::backtrace::__rust_end_short_backtrace::<std[f3b1b0f07ba26383]::panicking::begin_panic<rustc_errors[f9e52a26da800f21]::ExplicitBug>::{closure#0}, !>
   9:     0x7f7f275900c6 - std[f3b1b0f07ba26383]::panicking::begin_panic::<rustc_errors[f9e52a26da800f21]::ExplicitBug>
  10:     0x7f7f29fd8496 - std[f3b1b0f07ba26383]::panic::panic_any::<rustc_errors[f9e52a26da800f21]::ExplicitBug>
  11:     0x7f7f29fc59f6 - <rustc_errors[f9e52a26da800f21]::HandlerInner>::bug::<&alloc[404c16f0a8c2541e]::string::String>
  12:     0x7f7f29fc5310 - <rustc_errors[f9e52a26da800f21]::Handler>::bug::<&alloc[404c16f0a8c2541e]::string::String>
  13:     0x7f7f2a1632be - rustc_middle[3d1caf743043645a]::ty::context::tls::with_context_opt::<rustc_middle[3d1caf743043645a]::ty::context::tls::with_opt<rustc_middle[3d1caf743043645a]::util::bug::opt_span_bug_fmt<rustc_span[532da1a7d22fa5e]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f7f2a16caa9 - rustc_middle[3d1caf743043645a]::util::bug::opt_span_bug_fmt::<rustc_span[532da1a7d22fa5e]::span_encoding::Span>
  15:     0x7f7f2759d8d8 - rustc_middle[3d1caf743043645a]::util::bug::bug_fmt
  16:     0x7f7f2a1bb25e - <rustc_middle[3d1caf743043645a]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::normalize_generic_arg_after_erasing_regions
  17:     0x7f7f2a1bb2b3 - <rustc_middle[3d1caf743043645a]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[3d1caf743043645a]::ty::fold::TypeFolder>::fold_ty
  18:     0x7f7f29cc04a9 - <rustc_middle[3d1caf743043645a]::ty::subst::GenericArg as rustc_middle[3d1caf743043645a]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[3d1caf743043645a]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  19:     0x7f7f29c40e1d - <&rustc_middle[3d1caf743043645a]::ty::list::List<rustc_middle[3d1caf743043645a]::ty::subst::GenericArg> as rustc_middle[3d1caf743043645a]::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle[3d1caf743043645a]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  20:     0x7f7f29cae182 - <rustc_middle[3d1caf743043645a]::ty::sty::Binder<rustc_middle[3d1caf743043645a]::ty::sty::TraitRef> as rustc_middle[3d1caf743043645a]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle[3d1caf743043645a]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  21:     0x7f7f29d6e33b - <rustc_middle[3d1caf743043645a]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[3d1caf743043645a]::ty::fold::FallibleTypeFolder>::try_fold_binder::<rustc_middle[3d1caf743043645a]::ty::sty::TraitRef>
  22:     0x7f7f29cae04b - <rustc_middle[3d1caf743043645a]::ty::sty::Binder<rustc_middle[3d1caf743043645a]::ty::sty::TraitRef> as rustc_middle[3d1caf743043645a]::ty::fold::TypeFoldable>::fold_with::<rustc_middle[3d1caf743043645a]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  23:     0x7f7f29d591e0 - <rustc_middle[3d1caf743043645a]::ty::context::TyCtxt>::normalize_erasing_regions::<rustc_middle[3d1caf743043645a]::ty::sty::Binder<rustc_middle[3d1caf743043645a]::ty::sty::TraitRef>>
  24:     0x7f7f29cc943d - rustc_trait_selection[5e8a521c4750f2e3]::traits::codegen::codegen_select_candidate
  25:     0x7f7f292e02c2 - rustc_query_system[639359bfb7681a37]::query::plumbing::get_query::<rustc_query_impl[97515ba788d7469]::queries::codegen_select_candidate, rustc_query_impl[97515ba788d7469]::plumbing::QueryCtxt>
  26:     0x7f7f28dfe7ca - <rustc_query_impl[97515ba788d7469]::Queries as rustc_middle[3d1caf743043645a]::ty::query::QueryEngine>::codegen_select_candidate
  27:     0x7f7f282c90e4 - rustc_ty_utils[378ebfcc88d45ba7]::instance::inner_resolve_instance
  28:     0x7f7f282c82e7 - rustc_ty_utils[378ebfcc88d45ba7]::instance::resolve_instance
  29:     0x7f7f292ba5f5 - rustc_query_system[639359bfb7681a37]::query::plumbing::get_query::<rustc_query_impl[97515ba788d7469]::queries::resolve_instance, rustc_query_impl[97515ba788d7469]::plumbing::QueryCtxt>
  30:     0x7f7f28e36523 - <rustc_query_impl[97515ba788d7469]::Queries as rustc_middle[3d1caf743043645a]::ty::query::QueryEngine>::resolve_instance
  31:     0x7f7f2a07b7b5 - <rustc_middle[3d1caf743043645a]::ty::instance::Instance>::resolve_opt_const_arg
  32:     0x7f7f2a07b434 - <rustc_middle[3d1caf743043645a]::ty::instance::Instance>::resolve
  33:     0x7f7f27c1187b - <rustc_mir_transform[3b6f62df2f60ef2a]::inline::Inliner>::process_blocks
  34:     0x7f7f27c11238 - <rustc_mir_transform[3b6f62df2f60ef2a]::inline::Inline as rustc_middle[3d1caf743043645a]::mir::MirPass>::run_pass
  35:     0x7f7f27c0c70b - rustc_mir_transform[3b6f62df2f60ef2a]::pass_manager::run_passes_inner
  36:     0x7f7f27bee6c0 - rustc_mir_transform[3b6f62df2f60ef2a]::optimized_mir
  37:     0x7f7f292001fb - rustc_query_system[639359bfb7681a37]::query::plumbing::try_execute_query::<rustc_query_impl[97515ba788d7469]::plumbing::QueryCtxt, rustc_query_system[639359bfb7681a37]::query::caches::DefaultCache<rustc_span[532da1a7d22fa5e]::def_id::DefId, &rustc_middle[3d1caf743043645a]::mir::Body>>
  38:     0x7f7f292ae658 - rustc_query_system[639359bfb7681a37]::query::plumbing::get_query::<rustc_query_impl[97515ba788d7469]::queries::optimized_mir, rustc_query_impl[97515ba788d7469]::plumbing::QueryCtxt>
  39:     0x7f7f28dcb115 - <rustc_query_impl[97515ba788d7469]::Queries as rustc_middle[3d1caf743043645a]::ty::query::QueryEngine>::optimized_mir
  40:     0x7f7f2a11126c - <rustc_middle[3d1caf743043645a]::ty::context::TyCtxt>::instance_mir
  41:     0x7f7f2a041f7c - rustc_middle[3d1caf743043645a]::mir::pretty::write_mir_pretty
  42:     0x7f7f27c0e2ba - rustc_mir_transform[3b6f62df2f60ef2a]::dump_mir::emit_mir
  43:     0x7f7f27748043 - <rustc_interface[fd1c22d6b6ae60e]::passes::QueryContext>::enter::<<rustc_interface[fd1c22d6b6ae60e]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[a49f45c180e44e77]::result::Result<alloc[404c16f0a8c2541e]::boxed::Box<dyn core[a49f45c180e44e77]::any::Any>, rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>>
  44:     0x7f7f2771ef2d - <rustc_interface[fd1c22d6b6ae60e]::queries::Queries>::ongoing_codegen
  45:     0x7f7f275f0b39 - rustc_interface[fd1c22d6b6ae60e]::interface::create_compiler_and_run::<core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>, rustc_driver[5e25011d52c03d73]::run_compiler::{closure#1}>
  46:     0x7f7f2765bc82 - <scoped_tls[24ef5aa2a6efaae]::ScopedKey<rustc_span[532da1a7d22fa5e]::SessionGlobals>>::set::<rustc_interface[fd1c22d6b6ae60e]::interface::run_compiler<core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>, rustc_driver[5e25011d52c03d73]::run_compiler::{closure#1}>::{closure#0}, core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>>
  47:     0x7f7f276511ef - std[f3b1b0f07ba26383]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fd1c22d6b6ae60e]::util::run_in_thread_pool_with_globals<rustc_interface[fd1c22d6b6ae60e]::interface::run_compiler<core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>, rustc_driver[5e25011d52c03d73]::run_compiler::{closure#1}>::{closure#0}, core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>>::{closure#0}, core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>>
  48:     0x7f7f27604a1e - std[f3b1b0f07ba26383]::panic::catch_unwind::<core[a49f45c180e44e77]::panic::unwind_safe::AssertUnwindSafe<<std[f3b1b0f07ba26383]::thread::Builder>::spawn_unchecked_<rustc_interface[fd1c22d6b6ae60e]::util::run_in_thread_pool_with_globals<rustc_interface[fd1c22d6b6ae60e]::interface::run_compiler<core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>, rustc_driver[5e25011d52c03d73]::run_compiler::{closure#1}>::{closure#0}, core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>>::{closure#0}, core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>>
  49:     0x7f7f27654de2 - <<std[f3b1b0f07ba26383]::thread::Builder>::spawn_unchecked_<rustc_interface[fd1c22d6b6ae60e]::util::run_in_thread_pool_with_globals<rustc_interface[fd1c22d6b6ae60e]::interface::run_compiler<core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>, rustc_driver[5e25011d52c03d73]::run_compiler::{closure#1}>::{closure#0}, core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>>::{closure#0}, core[a49f45c180e44e77]::result::Result<(), rustc_errors[f9e52a26da800f21]::ErrorGuaranteed>>::{closure#1} as core[a49f45c180e44e77]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f7f26c46ad5 - std::sys::unix::thread::Thread::new::thread_start::h0fe3672a1db63bf8
  51:     0x7f7f269e1b43 - <unknown>
  52:     0x7f7f26a73a00 - <unknown>
  53:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (89ae252b4 2022-09-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z inline-mir
query stack during panic:
query stack during panic:
#0 [codegen_select_candidate] computing candidate for `<<Wrap<'_> as Associate>::Associated as Create<T>>`
#1 [resolve_instance] resolving instance `<<Wrap<'_> as Associate>::Associated as Create<T>>::create`
#2 [optimized_mir] optimizing MIR for `oh_no`
error: aborting due to previous error
------------------------------------------


