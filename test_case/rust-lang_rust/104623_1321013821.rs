plain
   Compiling object v0.29.0
   Compiling miniz_oxide v0.5.3
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling addr2line v0.17.0
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
             3: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             4: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
             5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             7: <rustc_middle::ty::context::TyCtxt>::def_path_str
             7: <rustc_middle::ty::context::TyCtxt>::def_path_str
             8: <rustc_mir_build::check_unsafety::UnsafeOpKind>::description_and_note
             9: <rustc_mir_build::check_unsafety::UnsafetyVisitor>::requires_unsafe
            10: <rustc_mir_build::check_unsafety::UnsafetyVisitor as rustc_middle::thir::visit::Visitor>::visit_expr
            11: <rustc_mir_build::check_unsafety::UnsafetyVisitor as rustc_middle::thir::visit::Visitor>::visit_expr
            12: rustc_middle::thir::visit::walk_expr::<rustc_mir_build::check_unsafety::UnsafetyVisitor>
            13: <rustc_mir_build::check_unsafety::UnsafetyVisitor as rustc_middle::thir::visit::Visitor>::visit_expr
            14: <rustc_mir_build::check_unsafety::UnsafetyVisitor as rustc_middle::thir::visit::Visitor>::visit_expr
            15: <rustc_mir_build::check_unsafety::UnsafetyVisitor as rustc_middle::thir::visit::Visitor>::visit_expr
            16: <rustc_mir_build::check_unsafety::UnsafetyVisitor as rustc_middle::thir::visit::Visitor>::visit_expr
            17: <rustc_mir_build::check_unsafety::UnsafetyVisitor as rustc_middle::thir::visit::Visitor>::visit_expr
            18: <rustc_mir_build::check_unsafety::UnsafetyVisitor as rustc_middle::thir::visit::Visitor>::visit_expr
            19: <rustc_mir_build::check_unsafety::UnsafetyVisitor as rustc_middle::thir::visit::Visitor>::visit_expr
            21: rustc_mir_build::check_unsafety::thir_check_unsafety
            22: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
            23: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::thir_check_unsafety, rustc_query_impl::plumbing::QueryCtxt>
            24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::thir_check_unsafety
            24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::thir_check_unsafety
            25: rustc_mir_build::build::mir_built
            26: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
            27: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_built, rustc_query_impl::plumbing::QueryCtxt>
            29: rustc_mir_transform::ffi_unwind_calls::has_ffi_unwind_calls
            30: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, bool>>
            31: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::has_ffi_unwind_calls, rustc_query_impl::plumbing::QueryCtxt>
            32: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::has_ffi_unwind_calls
            32: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::has_ffi_unwind_calls
            33: rustc_mir_transform::mir_const
            34: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
            36: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_const
            37: rustc_mir_transform::mir_promoted
            38: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_promoted, rustc_query_impl::plumbing::QueryCtxt>
            39: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_promoted
            39: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_promoted
            40: rustc_borrowck::mir_borrowck
            41: <rustc_borrowck::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
            42: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::BorrowCheckResult>>
            43: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_borrowck, rustc_query_impl::plumbing::QueryCtxt>
            44: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
            45: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_interface::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
            46: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_interface::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
            47: rustc_data_structures::sync::par_for_each_in::<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_interface::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
            48: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
            50: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
            51: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            52: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
            53: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            53: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            54: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
            55: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
            56: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            57: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            58: std::panicking::try::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
            59: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            61: <unknown>
            62: <unknown>
          


thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1591:13
stack backtrace:
   0:     0x7f15d7d5b4a2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6f63aca0d818ff3d
   1:     0x7f15d7dc9608 - core::fmt::write::h0cd9f5419c66d611
   2:     0x7f15d7d4c0c1 - std::io::Write::write_fmt::hdbe174a0534139e1
   3:     0x7f15d7d5b265 - std::sys_common::backtrace::print::h8337b838189f3756
   4:     0x7f15d7d5e577 - std::panicking::default_hook::{{closure}}::h40a27f353e96fa92
   5:     0x7f15d7d5e2d5 - std::panicking::default_hook::h7a5f0283402423a0
   6:     0x7f15d874ee54 - rustc_driver[f3565ded8b659160]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f15d7d5ee83 - std::panicking::rust_panic_with_hook::hec8edde2808e982b
   8:     0x7f15db68df53 - std[f70911762a7dc09b]::panicking::begin_panic::<rustc_errors[e0840290f4597a2f]::ExplicitBug>::{closure#0}
   9:     0x7f15db68a1c6 - std[f70911762a7dc09b]::sys_common::backtrace::__rust_end_short_backtrace::<std[f70911762a7dc09b]::panicking::begin_panic<rustc_errors[e0840290f4597a2f]::ExplicitBug>::{closure#0}, !>
  10:     0x7f15d8715da6 - std[f70911762a7dc09b]::panicking::begin_panic::<rustc_errors[e0840290f4597a2f]::ExplicitBug>
  11:     0x7f15db6f19a6 - std[f70911762a7dc09b]::panic::panic_any::<rustc_errors[e0840290f4597a2f]::ExplicitBug>
  12:     0x7f15db6f6090 - <rustc_errors[e0840290f4597a2f]::HandlerInner as core[867cfca19013d5a]::ops::drop::Drop>::drop
  13:     0x7f15d8769ee2 - core[867cfca19013d5a]::ptr::drop_in_place::<rustc_session[159b8dccd23cb90]::parse::ParseSess>
  14:     0x7f15d876b288 - core[867cfca19013d5a]::ptr::drop_in_place::<rustc_session[159b8dccd23cb90]::session::Session>
  15:     0x7f15d8772043 - <alloc[a6df30274c58c997]::rc::Rc<rustc_session[159b8dccd23cb90]::session::Session> as core[867cfca19013d5a]::ops::drop::Drop>::drop
  16:     0x7f15d8754b1c - core[867cfca19013d5a]::ptr::drop_in_place::<rustc_interface[74b2568bc152e7ff]::interface::Compiler>
  17:     0x7f15d8750629 - rustc_span[8615aa9fcc15d8c5]::with_source_map::<core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>, rustc_interface[74b2568bc152e7ff]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>, rustc_driver[f3565ded8b659160]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  18:     0x7f15d87ac62c - <scoped_tls[b64ea83672690cf8]::ScopedKey<rustc_span[8615aa9fcc15d8c5]::SessionGlobals>>::set::<rustc_interface[74b2568bc152e7ff]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>, rustc_driver[f3565ded8b659160]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>>
  19:     0x7f15d876c54a - std[f70911762a7dc09b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[74b2568bc152e7ff]::util::run_in_thread_pool_with_globals<rustc_interface[74b2568bc152e7ff]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>, rustc_driver[f3565ded8b659160]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>>
  20:     0x7f15d87b18b6 - std[f70911762a7dc09b]::panicking::try::<core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>, core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<<std[f70911762a7dc09b]::thread::Builder>::spawn_unchecked_<rustc_interface[74b2568bc152e7ff]::util::run_in_thread_pool_with_globals<rustc_interface[74b2568bc152e7ff]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>, rustc_driver[f3565ded8b659160]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  21:     0x7f15d875e9f9 - <<std[f70911762a7dc09b]::thread::Builder>::spawn_unchecked_<rustc_interface[74b2568bc152e7ff]::util::run_in_thread_pool_with_globals<rustc_interface[74b2568bc152e7ff]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>, rustc_driver[f3565ded8b659160]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[e0840290f4597a2f]::ErrorGuaranteed>>::{closure#1} as core[867cfca19013d5a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:     0x7f15d7d6bd9e - std::sys::unix::thread::Thread::new::thread_start::h8e0f0b30fe6295ff
  23:     0x7f15d7b00b43 - <unknown>
  24:     0x7f15d7b92a00 - <unknown>
  25:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (46f168a34 2022-11-20) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
