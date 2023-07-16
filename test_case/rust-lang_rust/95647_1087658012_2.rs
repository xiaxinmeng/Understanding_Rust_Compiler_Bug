text
thread 'rustc' panicked at 'assertion failed: self.scc_universes[scc] == ty::UniverseIndex::ROOT', compiler/rustc_borrowck/src/region_infer/mod.rs:708:9
stack backtrace:
   0:     0x7f859f9cb22d - std::backtrace_rs::backtrace::libunwind::trace::h35daaff92d098ed0
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f859f9cb22d - std::backtrace_rs::backtrace::trace_unsynchronized::h2a49b328879f3c62
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f859f9cb22d - std::sys_common::backtrace::_print_fmt::hbf865a25210831d9
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f859f9cb22d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h320151e111e06ced
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f859fa2508c - core::fmt::write::h7413aa28bee1cfbc
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/core/src/fmt/mod.rs:1194:17
   5:     0x7f859f9bc781 - std::io::Write::write_fmt::h0e72b6ed17789afa
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/io/mod.rs:1655:15
   6:     0x7f859f9ce315 - std::sys_common::backtrace::_print::h983440bc61c2f2c3
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f859f9ce315 - std::sys_common::backtrace::print::ha1f03110bb78cc22
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f859f9ce315 - std::panicking::default_hook::{{closure}}::heffa35a5a8b52a05
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/panicking.rs:295:22
   9:     0x7f859f9cdfc9 - std::panicking::default_hook::h7c0cbeac47c399c7
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/panicking.rs:314:9
  10:     0x7f85a014ba41 - rustc_driver[76d125f389222871]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f859f9cea60 - std::panicking::rust_panic_with_hook::hffbd90fcd9b72f27
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/panicking.rs:702:17
  12:     0x7f859f9ce859 - std::panicking::begin_panic_handler::{{closure}}::h0f77f2c8e7085cd5
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/panicking.rs:586:13
  13:     0x7f859f9cb6e4 - std::sys_common::backtrace::__rust_end_short_backtrace::h28c89a19b94c2378
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f859f9ce5c9 - rust_begin_unwind
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/panicking.rs:584:5
  15:     0x7f859f992713 - core::panicking::panic_fmt::h3125391edcfdcf8d
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/core/src/panicking.rs:143:14
  16:     0x7f859f9925dd - core::panicking::panic::hde315a6250347fbb
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/core/src/panicking.rs:48:5
  17:     0x7f85a192c570 - <rustc_borrowck[c1d826d90d372c4a]::region_infer::RegionInferenceContext>::solve
  18:     0x7f85a1838014 - rustc_borrowck[c1d826d90d372c4a]::nll::compute_regions
  19:     0x7f85a18f6bf6 - rustc_borrowck[c1d826d90d372c4a]::do_mir_borrowck
  20:     0x7f85a18eb579 - rustc_borrowck[c1d826d90d372c4a]::mir_borrowck
  21:     0x7f85a18e8210 - <rustc_borrowck[c1d826d90d372c4a]::provide::{closure#0} as core[48b87fce6ec407e9]::ops::function::FnOnce<(rustc_middle[fbba0f33d4f9946e]::ty::context::TyCtxt, rustc_span[ee8a23fea69ba3d2]::def_id::LocalDefId)>>::call_once
  22:     0x7f85a1a7db61 - rustc_query_system[951f4c75552d1d3a]::query::plumbing::try_execute_query::<rustc_query_impl[fe5829773555cd6b]::plumbing::QueryCtxt, rustc_query_system[951f4c75552d1d3a]::query::caches::DefaultCache<rustc_span[ee8a23fea69ba3d2]::def_id::LocalDefId, &rustc_middle[fbba0f33d4f9946e]::mir::query::BorrowCheckResult>>
  23:     0x7f85a1b61dfa - <rustc_query_impl[fe5829773555cd6b]::Queries as rustc_middle[fbba0f33d4f9946e]::ty::query::QueryEngine>::mir_borrowck
  24:     0x7f85a164200e - rustc_typeck[4d7f983240b49f12]::collect::type_of::type_of
  25:     0x7f85a1ac030e - rustc_query_system[951f4c75552d1d3a]::query::plumbing::get_query::<rustc_query_impl[fe5829773555cd6b]::queries::type_of, rustc_query_impl[fe5829773555cd6b]::plumbing::QueryCtxt>
  26:     0x7f85a1652ceb - rustc_typeck[4d7f983240b49f12]::check::check::check_item_type
  27:     0x7f85a16328eb - <rustc_middle[fbba0f33d4f9946e]::hir::map::Map>::visit_item_likes_in_module::<rustc_typeck[4d7f983240b49f12]::check::CheckItemTypesVisitor>
  28:     0x7f85a233cfec - rustc_typeck[4d7f983240b49f12]::check::check::check_mod_item_types
  29:     0x7f85a1a7fb7e - rustc_query_system[951f4c75552d1d3a]::query::plumbing::try_execute_query::<rustc_query_impl[fe5829773555cd6b]::plumbing::QueryCtxt, rustc_query_system[951f4c75552d1d3a]::query::caches::DefaultCache<rustc_span[ee8a23fea69ba3d2]::def_id::LocalDefId, ()>>
  30:     0x7f85a2554763 - rustc_query_system[951f4c75552d1d3a]::query::plumbing::get_query::<rustc_query_impl[fe5829773555cd6b]::queries::check_mod_item_types, rustc_query_impl[fe5829773555cd6b]::plumbing::QueryCtxt>
  31:     0x7f85a231d10c - <rustc_middle[fbba0f33d4f9946e]::hir::map::Map>::for_each_module::<rustc_typeck[4d7f983240b49f12]::check_crate::{closure#6}::{closure#0}>
  32:     0x7f85a22eba18 - <rustc_session[4c620bd5685dfc65]::session::Session>::time::<(), rustc_typeck[4d7f983240b49f12]::check_crate::{closure#6}>
  33:     0x7f85a22de478 - rustc_typeck[4d7f983240b49f12]::check_crate
  34:     0x7f85a205cd77 - rustc_interface[3efeb2cf171df459]::passes::analysis
  35:     0x7f85a252025e - rustc_query_system[951f4c75552d1d3a]::query::plumbing::try_execute_query::<rustc_query_impl[fe5829773555cd6b]::plumbing::QueryCtxt, rustc_query_system[951f4c75552d1d3a]::query::caches::DefaultCache<(), core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>>>
  36:     0x7f85a256340e - rustc_query_system[951f4c75552d1d3a]::query::plumbing::get_query::<rustc_query_impl[fe5829773555cd6b]::queries::analysis, rustc_query_impl[fe5829773555cd6b]::plumbing::QueryCtxt>
  37:     0x7f85a2022af1 - <rustc_interface[3efeb2cf171df459]::passes::QueryContext>::enter::<rustc_driver[76d125f389222871]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>>
  38:     0x7f85a203ccb8 - <rustc_interface[3efeb2cf171df459]::interface::Compiler>::enter::<rustc_driver[76d125f389222871]::run_compiler::{closure#1}::{closure#2}, core[48b87fce6ec407e9]::result::Result<core[48b87fce6ec407e9]::option::Option<rustc_interface[3efeb2cf171df459]::queries::Linker>, rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>>
  39:     0x7f85a20222af - rustc_span[ee8a23fea69ba3d2]::with_source_map::<core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>, rustc_interface[3efeb2cf171df459]::interface::create_compiler_and_run<core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>, rustc_driver[76d125f389222871]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f85a203d8f4 - rustc_interface[3efeb2cf171df459]::interface::create_compiler_and_run::<core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>, rustc_driver[76d125f389222871]::run_compiler::{closure#1}>
  41:     0x7f85a2020a42 - <scoped_tls[57a9157413d11ba7]::ScopedKey<rustc_span[ee8a23fea69ba3d2]::SessionGlobals>>::set::<rustc_interface[3efeb2cf171df459]::interface::run_compiler<core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>, rustc_driver[76d125f389222871]::run_compiler::{closure#1}>::{closure#0}, core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>>
  42:     0x7f85a20365ef - std[bd2101aea7a2ace9]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3efeb2cf171df459]::util::run_in_thread_pool_with_globals<rustc_interface[3efeb2cf171df459]::interface::run_compiler<core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>, rustc_driver[76d125f389222871]::run_compiler::{closure#1}>::{closure#0}, core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>>::{closure#0}, core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>>
  43:     0x7f85a2021d69 - <<std[bd2101aea7a2ace9]::thread::Builder>::spawn_unchecked_<rustc_interface[3efeb2cf171df459]::util::run_in_thread_pool_with_globals<rustc_interface[3efeb2cf171df459]::interface::run_compiler<core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>, rustc_driver[76d125f389222871]::run_compiler::{closure#1}>::{closure#0}, core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>>::{closure#0}, core[48b87fce6ec407e9]::result::Result<(), rustc_errors[bd5cb8d9692a92d1]::ErrorGuaranteed>>::{closure#1} as core[48b87fce6ec407e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f859f9d8c53 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h899a6206a2507bd0
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/alloc/src/boxed.rs:1861:9
  45:     0x7f859f9d8c53 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd62269684d9bd73e
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/alloc/src/boxed.rs:1861:9
  46:     0x7f859f9d8c53 - std::sys::unix::thread::Thread::new::thread_start::h7ef91fa8e493bb6e
                               at /rustc/6af09d2505f38e4f1df291df56d497fb2ad935ed/library/std/src/sys/unix/thread.rs:108:17
  47:     0x7f859f908609 - start_thread
  48:     0x7f859f821163 - clone
  49:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (6af09d250 2022-04-03) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `g`
#1 [type_of] computing type of `g::{opaque#0}`
#2 [check_mod_item_types] checking item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `playground`
