plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `false`,
 right: `true`', compiler/rustc_middle/src/ty/visit.rs:140:9
stack backtrace:
   0:     0x7f9034f621a1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h27d2bef9ce4499e0
   1:     0x7f9034fcc4c8 - core::fmt::write::h01a374123323c4c4
   2:     0x7f9034f563e1 - std::io::Write::write_fmt::h351f38272d79d8ca
   3:     0x7f9034f61fb5 - std::sys_common::backtrace::print::hf420ab5a388f598d
   4:     0x7f9034f65244 - std::panicking::default_hook::{{closure}}::h4d075ef78b814ad5
   5:     0x7f9034f64ff8 - std::panicking::default_hook::haef818e4c7239170
   6:     0x7f90359d7945 - rustc_driver_impl[4d10a7b995a25e78]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f9034f65a2d - std::panicking::rust_panic_with_hook::hb756c02d0c9e355b
   8:     0x7f9034f65769 - std::panicking::begin_panic_handler::{{closure}}::h1bb433c26ea18572
   9:     0x7f9034f62666 - std::sys_common::backtrace::__rust_end_short_backtrace::h7cf0c11852be866b
  10:     0x7f9034f65432 - rust_begin_unwind
  11:     0x7f9034f1ce73 - core::panicking::panic_fmt::hba5dfd24d0a29bf3
  12:     0x7f9034f1d2ef - core::panicking::assert_failed_inner::hb05fafcb347088b6
  13:     0x7f9035970c2b - core[b17f3ca5674245f1]::panicking::assert_failed::<bool, bool>
  14:     0x7f90386d1654 - <rustc_middle[1a31e7e3c4a98ba8]::ty::Ty as rustc_middle[1a31e7e3c4a98ba8]::ty::visit::TypeVisitableExt>::has_param
  15:     0x7f90386d0843 - <rustc_middle[1a31e7e3c4a98ba8]::ty::subst::SubstFolder as rustc_type_ir[41dd989255f7f46b]::fold::TypeFolder<rustc_middle[1a31e7e3c4a98ba8]::ty::context::TyCtxt>>::fold_ty
  16:     0x7f9036334eac - <rustc_middle[1a31e7e3c4a98ba8]::ty::subst::EarlyBinder<rustc_middle[1a31e7e3c4a98ba8]::ty::Ty>>::subst
  17:     0x7f90361cc261 - <dyn rustc_hir_analysis[60cfc734cdfcb38d]::astconv::AstConv>::ast_path_to_ty
  18:     0x7f90361dae0a - <dyn rustc_hir_analysis[60cfc734cdfcb38d]::astconv::AstConv>::res_to_ty
  19:     0x7f90361ef901 - <dyn rustc_hir_analysis[60cfc734cdfcb38d]::astconv::AstConv>::ast_ty_to_ty_inner
  20:     0x7f90361ee742 - <dyn rustc_hir_analysis[60cfc734cdfcb38d]::astconv::AstConv>::ast_ty_to_ty_inner
  21:     0x7f90361ef1ce - <dyn rustc_hir_analysis[60cfc734cdfcb38d]::astconv::AstConv>::ast_ty_to_ty_inner
  22:     0x7f90362ff4c1 - rustc_hir_analysis[60cfc734cdfcb38d]::collect::type_of::type_of
  23:     0x7f90374f92dd - <std[885720ba79e04cb1]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[1a31e7e3c4a98ba8]::ty::context::tls::enter_context<rustc_query_system[bb2556f58b17bab0]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[83198dce7ef5f465]::queries::type_of, rustc_query_impl[83198dce7ef5f465]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[1a31e7e3c4a98ba8]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[1a31e7e3c4a98ba8]::query::erase::Erased<[u8; 8usize]>>
  24:     0x7f90377bb4c8 - rustc_query_system[bb2556f58b17bab0]::query::plumbing::try_execute_query::<rustc_query_impl[83198dce7ef5f465]::queries::type_of, rustc_query_impl[83198dce7ef5f465]::plumbing::QueryCtxt>
  25:     0x7f90375fcee3 - rustc_query_impl[83198dce7ef5f465]::get_query::type_of
  26:     0x7f903611c4c2 - rustc_middle[1a31e7e3c4a98ba8]::ty::query::query_get_at::<rustc_query_system[bb2556f58b17bab0]::query::caches::DefaultCache<rustc_span[ec87ba21d16cc5]::def_id::DefId, rustc_middle[1a31e7e3c4a98ba8]::query::erase::Erased<[u8; 8usize]>>>
  27:     0x7f9036123004 - <rustc_privacy[e241401a4ae21295]::ReachEverythingInTheInterfaceVisitor>::ty
  28:     0x7f9036121646 - <rustc_privacy[e241401a4ae21295]::EmbargoVisitor>::update_macro_reachable_def
  29:     0x7f903612126e - <rustc_privacy[e241401a4ae21295]::EmbargoVisitor>::update_macro_reachable
  30:     0x7f903612166b - <rustc_privacy[e241401a4ae21295]::EmbargoVisitor>::update_macro_reachable_def
  31:     0x7f903612126e - <rustc_privacy[e241401a4ae21295]::EmbargoVisitor>::update_macro_reachable
  32:     0x7f903612166b - <rustc_privacy[e241401a4ae21295]::EmbargoVisitor>::update_macro_reachable_def
  33:     0x7f903612126e - <rustc_privacy[e241401a4ae21295]::EmbargoVisitor>::update_macro_reachable
  34:     0x7f90361228b2 - <rustc_privacy[e241401a4ae21295]::EmbargoVisitor as rustc_hir[7e053eab961f8785]::intravisit::Visitor>::visit_item
  35:     0x7f903613305d - rustc_hir[7e053eab961f8785]::intravisit::walk_item::<rustc_privacy[e241401a4ae21295]::EmbargoVisitor>
  36:     0x7f90361225fc - <rustc_privacy[e241401a4ae21295]::EmbargoVisitor as rustc_hir[7e053eab961f8785]::intravisit::Visitor>::visit_item
  37:     0x7f903613305d - rustc_hir[7e053eab961f8785]::intravisit::walk_item::<rustc_privacy[e241401a4ae21295]::EmbargoVisitor>
  38:     0x7f90361225fc - <rustc_privacy[e241401a4ae21295]::EmbargoVisitor as rustc_hir[7e053eab961f8785]::intravisit::Visitor>::visit_item
  39:     0x7f903612ffaa - rustc_hir[7e053eab961f8785]::intravisit::walk_mod::<rustc_privacy[e241401a4ae21295]::EmbargoVisitor>
  40:     0x7f903612788b - rustc_privacy[e241401a4ae21295]::effective_visibilities
  41:     0x7f90374f4836 - <std[885720ba79e04cb1]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[1a31e7e3c4a98ba8]::ty::context::tls::enter_context<rustc_query_system[bb2556f58b17bab0]::query::plumbing::execute_job_non_incr<rustc_query_impl[83198dce7ef5f465]::queries::effective_visibilities, rustc_query_impl[83198dce7ef5f465]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[1a31e7e3c4a98ba8]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[1a31e7e3c4a98ba8]::query::erase::Erased<[u8; 8usize]>>
  42:     0x7f903777caf6 - rustc_query_system[bb2556f58b17bab0]::query::plumbing::try_execute_query::<rustc_query_impl[83198dce7ef5f465]::queries::effective_visibilities, rustc_query_impl[83198dce7ef5f465]::plumbing::QueryCtxt>
  43:     0x7f903762ac11 - rustc_query_impl[83198dce7ef5f465]::get_query::effective_visibilities
  44:     0x7f9036b3305a - rustc_middle[1a31e7e3c4a98ba8]::ty::query::query_get_at::<rustc_query_system[bb2556f58b17bab0]::query::caches::SingleCache<rustc_middle[1a31e7e3c4a98ba8]::query::erase::Erased<[u8; 8usize]>>>
  45:     0x7f9036b4406a - rustc_passes[bed5f3adfb725dd1]::stability::check_unused_or_stable_features
  46:     0x7f9035b32e37 - <rustc_session[d98954d82c0fa496]::session::Session>::time::<(), rustc_interface[5a935bf8077dd2cf]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  47:     0x7f9035b6e935 - std[885720ba79e04cb1]::panicking::try::<(), core[b17f3ca5674245f1]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5a935bf8077dd2cf]::passes::analysis::{closure#0}::{closure#2}>>
  48:     0x7f9035b33483 - <rustc_session[d98954d82c0fa496]::session::Session>::time::<(), rustc_interface[5a935bf8077dd2cf]::passes::analysis::{closure#0}>
  49:     0x7f9035ab9c4d - rustc_interface[5a935bf8077dd2cf]::passes::analysis
  50:     0x7f90374f9374 - <std[885720ba79e04cb1]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[1a31e7e3c4a98ba8]::ty::context::tls::enter_context<rustc_query_system[bb2556f58b17bab0]::query::plumbing::execute_job_non_incr<rustc_query_impl[83198dce7ef5f465]::queries::analysis, rustc_query_impl[83198dce7ef5f465]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[1a31e7e3c4a98ba8]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[1a31e7e3c4a98ba8]::query::erase::Erased<[u8; 1usize]>>
  51:     0x7f90377bbe4d - rustc_query_system[bb2556f58b17bab0]::query::plumbing::try_execute_query::<rustc_query_impl[83198dce7ef5f465]::queries::analysis, rustc_query_impl[83198dce7ef5f465]::plumbing::QueryCtxt>
  52:     0x7f90375fe8e4 - rustc_query_impl[83198dce7ef5f465]::get_query::analysis
  53:     0x7f9035a3fb54 - rustc_middle[1a31e7e3c4a98ba8]::ty::query::query_get_at::<rustc_query_system[bb2556f58b17bab0]::query::caches::SingleCache<rustc_middle[1a31e7e3c4a98ba8]::query::erase::Erased<[u8; 1usize]>>>
  54:     0x7f9035a2c1b5 - <std[885720ba79e04cb1]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[1a31e7e3c4a98ba8]::ty::context::tls::enter_context<<rustc_middle[1a31e7e3c4a98ba8]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[4d10a7b995a25e78]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>
  55:     0x7f90359f0481 - <rustc_interface[5a935bf8077dd2cf]::queries::QueryResult<&rustc_middle[1a31e7e3c4a98ba8]::ty::context::GlobalCtxt>>::enter::<core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>, rustc_driver_impl[4d10a7b995a25e78]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  56:     0x7f9035a2cf37 - <rustc_interface[5a935bf8077dd2cf]::interface::Compiler>::enter::<rustc_driver_impl[4d10a7b995a25e78]::run_compiler::{closure#1}::{closure#2}, core[b17f3ca5674245f1]::result::Result<core[b17f3ca5674245f1]::option::Option<rustc_interface[5a935bf8077dd2cf]::queries::Linker>, rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>
  57:     0x7f90359f8477 - std[885720ba79e04cb1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5a935bf8077dd2cf]::util::run_in_thread_pool_with_globals<rustc_interface[5a935bf8077dd2cf]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>, rustc_driver_impl[4d10a7b995a25e78]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>
  58:     0x7f9035a2e058 - std[885720ba79e04cb1]::panicking::try::<core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>, core[b17f3ca5674245f1]::panic::unwind_safe::AssertUnwindSafe<<std[885720ba79e04cb1]::thread::Builder>::spawn_unchecked_<rustc_interface[5a935bf8077dd2cf]::util::run_in_thread_pool_with_globals<rustc_interface[5a935bf8077dd2cf]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>, rustc_driver_impl[4d10a7b995a25e78]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  59:     0x7f90359fa286 - <<std[885720ba79e04cb1]::thread::Builder>::spawn_unchecked_<rustc_interface[5a935bf8077dd2cf]::util::run_in_thread_pool_with_globals<rustc_interface[5a935bf8077dd2cf]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>, rustc_driver_impl[4d10a7b995a25e78]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[ec87ba21d16cc5]::ErrorGuaranteed>>::{closure#1} as core[b17f3ca5674245f1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  60:     0x7f9034f72afe - std::sys::unix::thread::Thread::new::thread_start::h0edef914c2c59c43
  61:     0x7f9034d0eb43 - <unknown>
  62:     0x7f9034da0a00 - <unknown>
  63:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (255fb2a49 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [type_of] computing type of `num::fmt::Formatted::parts`
#1 [effective_visibilities] checking effective visibilities
#2 [analysis] running analysis passes on this crate
error: could not compile `core` (lib)
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:33
