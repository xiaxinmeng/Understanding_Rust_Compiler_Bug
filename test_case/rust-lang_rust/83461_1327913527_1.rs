
stack backtrace:
   0:        0x10d5c6496 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h388d18e35423230a
   1:        0x10d624e0a - core::fmt::write::hae057d6b754f72d8
   2:        0x10d5b883c - std::io::Write::write_fmt::h442179bd60267d8e
   3:        0x10d5c627a - std::sys_common::backtrace::print::habab5a5c263740eb
   4:        0x10d5c95c3 - std::panicking::default_hook::{{closure}}::h831e1cc678bd888c
   5:        0x10d5c9318 - std::panicking::default_hook::h8f0af5f1b913f212
   6:        0x115c6ff8d - rustc_driver[c28c713964f6db5f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x10d5c9da7 - std::panicking::rust_panic_with_hook::h20ce9404b4438d87
   8:        0x10d5c9b54 - std::panicking::begin_panic_handler::{{closure}}::h153e2afe83634090
   9:        0x10d5c6938 - std::sys_common::backtrace::__rust_end_short_backtrace::hb594464367a2d0dc
  10:        0x10d5c981d - _rust_begin_unwind
  11:        0x10d6513e3 - core::panicking::panic_fmt::h07c9fba3a925cee1
  12:        0x10d62198b - core::panicking::panic_display::hcfed2b33fafb0908
  13:        0x10d62193c - core::panicking::panic_str::hf710a6722843e2d7
  14:        0x10d6513a9 - core::option::expect_failed::h6c202761b9020bfc
  15:        0x119377b3a - <rustc_mir_dataflow[294d552b03b9f85c]::rustc_peek::SanityCheck as rustc_middle[b06de2f7571e6fcc]::mir::MirPass>::run_pass
  16:        0x1187e1c39 - rustc_mir_transform[1ad4961f1785d654]::pass_manager::run_passes_inner
  17:        0x11891a55a - rustc_mir_transform[1ad4961f1785d654]::mir_const
  18:        0x11942e39d - rustc_query_system[727f66452900dcdf]::query::plumbing::try_execute_query::<rustc_query_impl[e6bb220d622b6c4e]::plumbing::QueryCtxt, rustc_query_system[727f66452900dcdf]::query::caches::DefaultCache<rustc_middle[b06de2f7571e6fcc]::ty::WithOptConstParam<rustc_span[a499e2c93bb8a38c]::def_id::LocalDefId>, &rustc_data_structures[efdbdd24b9867b6]::steal::Steal<rustc_middle[b06de2f7571e6fcc]::mir::Body>>>
  19:        0x11956b76d - rustc_query_system[727f66452900dcdf]::query::plumbing::get_query::<rustc_query_impl[e6bb220d622b6c4e]::queries::mir_const, rustc_query_impl[e6bb220d622b6c4e]::plumbing::QueryCtxt>
  20:        0x119642477 - <rustc_query_impl[e6bb220d622b6c4e]::Queries as rustc_middle[b06de2f7571e6fcc]::ty::query::QueryEngine>::mir_const
  21:        0x11891b453 - rustc_mir_transform[1ad4961f1785d654]::mir_promoted
  22:        0x11942f5a1 - rustc_query_system[727f66452900dcdf]::query::plumbing::try_execute_query::<rustc_query_impl[e6bb220d622b6c4e]::plumbing::QueryCtxt, rustc_query_system[727f66452900dcdf]::query::caches::DefaultCache<rustc_middle[b06de2f7571e6fcc]::ty::WithOptConstParam<rustc_span[a499e2c93bb8a38c]::def_id::LocalDefId>, (&rustc_data_structures[efdbdd24b9867b6]::steal::Steal<rustc_middle[b06de2f7571e6fcc]::mir::Body>, &rustc_data_structures[efdbdd24b9867b6]::steal::Steal<rustc_index[c336d4038c12cec8]::vec::IndexVec<rustc_middle[b06de2f7571e6fcc]::mir::Promoted, rustc_middle[b06de2f7571e6fcc]::mir::Body>>)>>
  23:        0x119555940 - rustc_query_system[727f66452900dcdf]::query::plumbing::get_query::<rustc_query_impl[e6bb220d622b6c4e]::queries::mir_promoted, rustc_query_impl[e6bb220d622b6c4e]::plumbing::QueryCtxt>
  24:        0x119642637 - <rustc_query_impl[e6bb220d622b6c4e]::Queries as rustc_middle[b06de2f7571e6fcc]::ty::query::QueryEngine>::mir_promoted
  25:        0x118fdd61f - rustc_borrowck[e6dd8db3b09abc46]::mir_borrowck
  26:        0x118fb81d5 - <rustc_borrowck[e6dd8db3b09abc46]::provide::{closure#0} as core[af36ead597221bd6]::ops::function::FnOnce<(rustc_middle[b06de2f7571e6fcc]::ty::context::TyCtxt, rustc_span[a499e2c93bb8a38c]::def_id::LocalDefId)>>::call_once
  27:        0x1194e0614 - rustc_query_system[727f66452900dcdf]::query::plumbing::try_execute_query::<rustc_query_impl[e6bb220d622b6c4e]::plumbing::QueryCtxt, rustc_query_system[727f66452900dcdf]::query::caches::VecCache<rustc_span[a499e2c93bb8a38c]::def_id::LocalDefId, &rustc_middle[b06de2f7571e6fcc]::mir::query::BorrowCheckResult>>
  28:        0x1195556a7 - rustc_query_system[727f66452900dcdf]::query::plumbing::get_query::<rustc_query_impl[e6bb220d622b6c4e]::queries::mir_borrowck, rustc_query_impl[e6bb220d622b6c4e]::plumbing::QueryCtxt>
  29:        0x115d846bb - rustc_data_structures[efdbdd24b9867b6]::sync::par_for_each_in::<&[rustc_span[a499e2c93bb8a38c]::def_id::LocalDefId], <rustc_middle[b06de2f7571e6fcc]::hir::map::Map>::par_body_owners<rustc_interface[e8482e30f906f63e]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  30:        0x115cf59e0 - <rustc_session[7e50a26a31d731b]::session::Session>::time::<(), rustc_interface[e8482e30f906f63e]::passes::analysis::{closure#2}>
  31:        0x115d388a3 - rustc_interface[e8482e30f906f63e]::passes::analysis
  32:        0x1194af54b - rustc_query_system[727f66452900dcdf]::query::plumbing::try_execute_query::<rustc_query_impl[e6bb220d622b6c4e]::plumbing::QueryCtxt, rustc_query_system[727f66452900dcdf]::query::caches::DefaultCache<(), core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>>>
  33:        0x11956a66d - rustc_query_system[727f66452900dcdf]::query::plumbing::get_query::<rustc_query_impl[e6bb220d622b6c4e]::queries::analysis, rustc_query_impl[e6bb220d622b6c4e]::plumbing::QueryCtxt>
  34:        0x115c02077 - <rustc_interface[e8482e30f906f63e]::passes::QueryContext>::enter::<rustc_driver[c28c713964f6db5f]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>>
  35:        0x115c48525 - <rustc_interface[e8482e30f906f63e]::interface::Compiler>::enter::<rustc_driver[c28c713964f6db5f]::run_compiler::{closure#1}::{closure#2}, core[af36ead597221bd6]::result::Result<core[af36ead597221bd6]::option::Option<rustc_interface[e8482e30f906f63e]::queries::Linker>, rustc_errors[31109329a537279e]::ErrorGuaranteed>>
  36:        0x115be479d - rustc_span[a499e2c93bb8a38c]::with_source_map::<core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>, rustc_interface[e8482e30f906f63e]::interface::run_compiler<core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>, rustc_driver[c28c713964f6db5f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  37:        0x115c3b43d - <scoped_tls[826d51480f7487f1]::ScopedKey<rustc_span[a499e2c93bb8a38c]::SessionGlobals>>::set::<rustc_interface[e8482e30f906f63e]::interface::run_compiler<core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>, rustc_driver[c28c713964f6db5f]::run_compiler::{closure#1}>::{closure#0}, core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>>
  38:        0x115c0a99a - std[c99b25d27e8b6be0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e8482e30f906f63e]::util::run_in_thread_pool_with_globals<rustc_interface[e8482e30f906f63e]::interface::run_compiler<core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>, rustc_driver[c28c713964f6db5f]::run_compiler::{closure#1}>::{closure#0}, core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>>
  39:        0x115beac95 - <<std[c99b25d27e8b6be0]::thread::Builder>::spawn_unchecked_<rustc_interface[e8482e30f906f63e]::util::run_in_thread_pool_with_globals<rustc_interface[e8482e30f906f63e]::interface::run_compiler<core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>, rustc_driver[c28c713964f6db5f]::run_compiler::{closure#1}>::{closure#0}, core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af36ead597221bd6]::result::Result<(), rustc_errors[31109329a537279e]::ErrorGuaranteed>>::{closure#1} as core[af36ead597221bd6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:        0x10d5d2f97 - std::sys::unix::thread::Thread::new::thread_start::ha16bf32d64a7247b
  41:     0x7ff807a764e1 - __pthread_start
