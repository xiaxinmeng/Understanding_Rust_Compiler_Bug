plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `bb1`,
 right: `bb0`', compiler/rustc_mir_transform/src/const_prop.rs:82:9
   0:     0x7f1dbe495441 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99d41b7989943530
   1:     0x7f1dbe4fceb8 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f1dbe489041 - std::io::Write::write_fmt::h0d88c7b0bf910f28
   3:     0x7f1dbe495255 - std::sys_common::backtrace::print::h1940fb990ef00a99
   3:     0x7f1dbe495255 - std::sys_common::backtrace::print::h1940fb990ef00a99
   4:     0x7f1dbe4982cf - std::panicking::default_hook::{{closure}}::h83dd335ad7b1b7e8
   5:     0x7f1dbe497e8b - std::panicking::default_hook::h04915ebfc8268c91
   6:     0x7f1dbeef38e5 - rustc_driver_impl[da23db044bca8fe]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1dbe498ac9 - std::panicking::rust_panic_with_hook::h75e7ea4c0112022c
   8:     0x7f1dbe4987fc - std::panicking::begin_panic_handler::{{closure}}::he8a20f54f0048e2d
   9:     0x7f1dbe4958f6 - std::sys_common::backtrace::__rust_end_short_backtrace::h15ea8666fd1dfbb3
  10:     0x7f1dbe498482 - rust_begin_unwind
  11:     0x7f1dbe44ff43 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f1dbe4502cf - core::panicking::assert_failed_inner::he3c123340f62e981
  13:     0x7f1dbecf617b - core[54ab13d2a06817e1]::panicking::assert_failed::<rustc_middle[de34187bf87e63dc]::mir::BasicBlock, rustc_middle[de34187bf87e63dc]::mir::BasicBlock>
  14:     0x7f1dbf9bd68d - <rustc_mir_transform[622c87f6326cb152]::const_prop::ConstProp as rustc_middle[de34187bf87e63dc]::mir::MirPass>::run_pass
  15:     0x7f1dbfa7ae28 - rustc_mir_transform[622c87f6326cb152]::pass_manager::run_passes_inner
  16:     0x7f1dbf8e3436 - rustc_mir_transform[622c87f6326cb152]::mir_for_ctfe
  17:     0x7f1dc0999395 - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[de34187bf87e63dc]::ty::context::tls::enter_context<rustc_query_system[db02d12f7f698b79]::query::plumbing::execute_job_non_incr<rustc_query_impl[85ec180858eafba0]::queries::mir_for_ctfe, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 8usize]>>
  18:     0x7f1dc0bb415c - rustc_query_system[db02d12f7f698b79]::query::plumbing::try_execute_query::<rustc_query_impl[85ec180858eafba0]::queries::mir_for_ctfe, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>
  19:     0x7f1dc0b199bb - <rustc_query_impl[85ec180858eafba0]::Queries as rustc_middle[de34187bf87e63dc]::ty::query::QueryEngine>::mir_for_ctfe
  20:     0x7f1dbfc32b2a - <rustc_const_eval[a87996b446aefae1]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[a87996b446aefae1]::interpret::machine::Machine>::load_mir
  21:     0x7f1dbfb18d1e - <rustc_const_eval[a87996b446aefae1]::interpret::eval_context::InterpCx<rustc_const_eval[a87996b446aefae1]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  22:     0x7f1dbfc69f94 - rustc_const_eval[a87996b446aefae1]::const_eval::eval_queries::eval_to_allocation_raw_provider
  23:     0x7f1dc09a0b98 - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[de34187bf87e63dc]::ty::context::tls::enter_context<rustc_query_system[db02d12f7f698b79]::query::plumbing::execute_job_non_incr<rustc_query_impl[85ec180858eafba0]::queries::eval_to_allocation_raw, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 16usize]>>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 16usize]>>
  24:     0x7f1dc0c12b44 - rustc_query_system[db02d12f7f698b79]::query::plumbing::try_execute_query::<rustc_query_impl[85ec180858eafba0]::queries::eval_to_allocation_raw, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>
  25:     0x7f1dc0b36379 - <rustc_query_impl[85ec180858eafba0]::Queries as rustc_middle[de34187bf87e63dc]::ty::query::QueryEngine>::eval_to_allocation_raw
  26:     0x7f1dbfc6879f - rustc_const_eval[a87996b446aefae1]::const_eval::eval_queries::eval_to_const_value_raw_provider
  27:     0x7f1dc09a18ec - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[de34187bf87e63dc]::ty::context::tls::enter_context<rustc_query_system[db02d12f7f698b79]::query::plumbing::execute_job_non_incr<rustc_query_impl[85ec180858eafba0]::queries::eval_to_const_value_raw, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 32usize]>>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 32usize]>>
  28:     0x7f1dc0c1c1cc - rustc_query_system[db02d12f7f698b79]::query::plumbing::try_execute_query::<rustc_query_impl[85ec180858eafba0]::queries::eval_to_const_value_raw, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>
  29:     0x7f1dc0b369f9 - <rustc_query_impl[85ec180858eafba0]::Queries as rustc_middle[de34187bf87e63dc]::ty::query::QueryEngine>::eval_to_const_value_raw
  30:     0x7f1dbfc68558 - rustc_const_eval[a87996b446aefae1]::const_eval::eval_queries::eval_to_const_value_raw_provider
  31:     0x7f1dc09a18ec - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[de34187bf87e63dc]::ty::context::tls::enter_context<rustc_query_system[db02d12f7f698b79]::query::plumbing::execute_job_non_incr<rustc_query_impl[85ec180858eafba0]::queries::eval_to_const_value_raw, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 32usize]>>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 32usize]>>
  32:     0x7f1dc0c1c1cc - rustc_query_system[db02d12f7f698b79]::query::plumbing::try_execute_query::<rustc_query_impl[85ec180858eafba0]::queries::eval_to_const_value_raw, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>
  33:     0x7f1dc0b369f9 - <rustc_query_impl[85ec180858eafba0]::Queries as rustc_middle[de34187bf87e63dc]::ty::query::QueryEngine>::eval_to_const_value_raw
  34:     0x7f1dc1baeb05 - <rustc_middle[de34187bf87e63dc]::ty::context::TyCtxt>::const_eval_global_id
  35:     0x7f1dc1bad617 - <rustc_middle[de34187bf87e63dc]::ty::context::TyCtxt>::const_eval_poly
  36:     0x7f1dbf799125 - <rustc_middle[de34187bf87e63dc]::ty::adt::AdtDef>::eval_explicit_discr
  37:     0x7f1dbf7a77e3 - <rustc_hir_analysis[eace7e3177a37ff1]::collect::CollectItemTypesVisitor as rustc_hir[6520ce46fc5d3b0d]::intravisit::Visitor>::visit_item
  38:     0x7f1dbf6d23d0 - <rustc_middle[de34187bf87e63dc]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[eace7e3177a37ff1]::collect::CollectItemTypesVisitor>
  39:     0x7f1dbf7a55fd - rustc_hir_analysis[eace7e3177a37ff1]::collect::collect_mod_item_types
  40:     0x7f1dc09a08b2 - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[de34187bf87e63dc]::ty::context::tls::enter_context<rustc_query_system[db02d12f7f698b79]::query::plumbing::execute_job_non_incr<rustc_query_impl[85ec180858eafba0]::queries::collect_mod_item_types, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 0usize]>>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 0usize]>>
  41:     0x7f1dc0c10105 - rustc_query_system[db02d12f7f698b79]::query::plumbing::try_execute_query::<rustc_query_impl[85ec180858eafba0]::queries::collect_mod_item_types, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>
  42:     0x7f1dc0b30dc4 - <rustc_query_impl[85ec180858eafba0]::Queries as rustc_middle[de34187bf87e63dc]::ty::query::QueryEngine>::collect_mod_item_types
  43:     0x7f1dbf6d1bc7 - <rustc_middle[de34187bf87e63dc]::hir::map::Map>::for_each_module::<rustc_hir_analysis[eace7e3177a37ff1]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  44:     0x7f1dbf705d7e - <rustc_session[bceea7ceb1c1ff35]::session::Session>::track_errors::<rustc_hir_analysis[eace7e3177a37ff1]::check_crate::{closure#0}, ()>
  45:     0x7f1dbf75787b - rustc_hir_analysis[eace7e3177a37ff1]::check_crate
  46:     0x7f1dbefabdc5 - rustc_interface[47abca61d93bb21c]::passes::analysis
  47:     0x7f1dc09a50bc - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[de34187bf87e63dc]::ty::context::tls::enter_context<rustc_query_system[db02d12f7f698b79]::query::plumbing::execute_job_non_incr<rustc_query_impl[85ec180858eafba0]::queries::analysis, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[de34187bf87e63dc]::query::erase::Erased<[u8; 1usize]>>
  48:     0x7f1dc0c4c526 - rustc_query_system[db02d12f7f698b79]::query::plumbing::try_execute_query::<rustc_query_impl[85ec180858eafba0]::queries::analysis, rustc_query_impl[85ec180858eafba0]::plumbing::QueryCtxt>
  49:     0x7f1dc0b1055a - <rustc_query_impl[85ec180858eafba0]::Queries as rustc_middle[de34187bf87e63dc]::ty::query::QueryEngine>::analysis
  50:     0x7f1dbef4513f - <std[9c6ad123400df44d]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[de34187bf87e63dc]::ty::context::tls::enter_context<<rustc_middle[de34187bf87e63dc]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[da23db044bca8fe]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  51:     0x7f1dbef2b121 - <rustc_interface[47abca61d93bb21c]::queries::QueryResult<&rustc_middle[de34187bf87e63dc]::ty::context::GlobalCtxt>>::enter::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[da23db044bca8fe]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  52:     0x7f1dbef46062 - <rustc_interface[47abca61d93bb21c]::interface::Compiler>::enter::<rustc_driver_impl[da23db044bca8fe]::run_compiler::{closure#1}::{closure#2}, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::option::Option<rustc_interface[47abca61d93bb21c]::queries::Linker>, rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  53:     0x7f1dbef63660 - rustc_span[de5c6ef7fb663679]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_interface[47abca61d93bb21c]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[da23db044bca8fe]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  54:     0x7f1dbef05e4a - std[9c6ad123400df44d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[47abca61d93bb21c]::util::run_in_thread_pool_with_globals<rustc_interface[47abca61d93bb21c]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[da23db044bca8fe]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>
  55:     0x7f1dbef68fc8 - std[9c6ad123400df44d]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[47abca61d93bb21c]::util::run_in_thread_pool_with_globals<rustc_interface[47abca61d93bb21c]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[da23db044bca8fe]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  56:     0x7f1dbef07006 - <<std[9c6ad123400df44d]::thread::Builder>::spawn_unchecked_<rustc_interface[47abca61d93bb21c]::util::run_in_thread_pool_with_globals<rustc_interface[47abca61d93bb21c]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>, rustc_driver_impl[da23db044bca8fe]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[de5c6ef7fb663679]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  57:     0x7f1dbe4a4d0e - std::sys::unix::thread::Thread::new::thread_start::h0ee55f1c660c8984
  58:     0x7f1dbe241b43 - <unknown>
  59:     0x7f1dbe2d3a00 - <unknown>
  60:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (08a37fd65 2023-04-23) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_for_ctfe] caching mir of `ptr::alignment::AlignmentEnum16::_Align1Shl0::{constant#0}` for CTFE
#1 [eval_to_allocation_raw] const-evaluating + checking `ptr::alignment::AlignmentEnum16::_Align1Shl0::{constant#0}`
#2 [eval_to_const_value_raw] simplifying constant for the type system `ptr::alignment::AlignmentEnum16::_Align1Shl0::{constant#0}`
#3 [eval_to_const_value_raw] simplifying constant for the type system `ptr::alignment::AlignmentEnum16::_Align1Shl0::{constant#0}`
#4 [collect_mod_item_types] collecting item types in module `ptr::alignment`
#5 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:03:04
