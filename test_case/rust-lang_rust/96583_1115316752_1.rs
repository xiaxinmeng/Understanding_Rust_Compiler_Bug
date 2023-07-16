
  Compiling playground v0.0.1 (/playground)
thread 'rustc' panicked at 'DefId::expect_local: `DefId(2:2100 ~ core[9082]::mem::drop)` isn't local', /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/compiler/rustc_span/src/def_id.rs:283:43
stack backtrace:
   0:     0x7fd908b17ebd - std::backtrace_rs::backtrace::libunwind::trace::h2110ce138777722b
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fd908b17ebd - std::backtrace_rs::backtrace::trace_unsynchronized::h8581d1b0ad43c04f
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd908b17ebd - std::sys_common::backtrace::_print_fmt::h61a0a5b1306ac3f8
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fd908b17ebd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h926b8315f8994dd9
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fd908b73bfc - core::fmt::write::h3f4cb6a6cbebab0c
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/core/src/fmt/mod.rs:1194:17
   5:     0x7fd908b095b1 - std::io::Write::write_fmt::h473895de5b528a4c
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/io/mod.rs:1655:15
   6:     0x7fd908b1abd5 - std::sys_common::backtrace::_print::h1e966216ddc81c55
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fd908b1abd5 - std::sys_common::backtrace::print::h506321a44638b03e
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fd908b1abd5 - std::panicking::default_hook::{{closure}}::hdcf8a0182858e663
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/panicking.rs:295:22
   9:     0x7fd908b1a849 - std::panicking::default_hook::hd68b372c6d095666
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/panicking.rs:314:9
  10:     0x7fd9092bfe41 - rustc_driver[8bde46e801238545]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fd908b1b3a6 - std::panicking::rust_panic_with_hook::haab191f924575952
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/panicking.rs:702:17
  12:     0x7fd908b1b1a7 - std::panicking::begin_panic_handler::{{closure}}::hc0c42a4137d83225
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/panicking.rs:588:13
  13:     0x7fd908b18374 - std::sys_common::backtrace::__rust_end_short_backtrace::h3452b9a7e02930a8
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7fd908b1aed9 - rust_begin_unwind
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/panicking.rs:584:5
  15:     0x7fd908ae01a3 - core::panicking::panic_fmt::h7a33cdd94709772f
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/core/src/panicking.rs:142:14
  16:     0x7fd90a74b69e - <rustc_typeck[73fe2133e9515c12]::check::fn_ctxt::FnCtxt>::check_field
  17:     0x7fd90a78e13f - <rustc_typeck[73fe2133e9515c12]::check::fn_ctxt::FnCtxt>::check_expr_kind
  18:     0x7fd90a78c7b0 - <rustc_typeck[73fe2133e9515c12]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  19:     0x7fd90a77c02b - <rustc_typeck[73fe2133e9515c12]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  20:     0x7fd90a78da9e - <rustc_typeck[73fe2133e9515c12]::check::fn_ctxt::FnCtxt>::check_expr_kind
  21:     0x7fd90a78c7b0 - <rustc_typeck[73fe2133e9515c12]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7fd90a7b8d12 - rustc_typeck[73fe2133e9515c12]::check::check::check_fn
  23:     0x7fd90a81778c - <rustc_infer[354e02e237cfa2bd]::infer::InferCtxtBuilder>::enter::<&rustc_middle[ddd14b4469de6474]::ty::context::TypeckResults, <rustc_typeck[73fe2133e9515c12]::check::inherited::InheritedBuilder>::enter<rustc_typeck[73fe2133e9515c12]::check::typeck_with_fallback<rustc_typeck[73fe2133e9515c12]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[ddd14b4469de6474]::ty::context::TypeckResults>::{closure#0}>
  24:     0x7fd90a7cc350 - rustc_typeck[73fe2133e9515c12]::check::typeck
  25:     0x7fd90acf8900 - rustc_query_system[edad6c0bb6767a6f]::query::plumbing::try_execute_query::<rustc_query_impl[6a79f5e9aada5532]::plumbing::QueryCtxt, rustc_query_system[edad6c0bb6767a6f]::query::caches::DefaultCache<rustc_span[b93d47c7c5255e26]::def_id::LocalDefId, &rustc_middle[ddd14b4469de6474]::ty::context::TypeckResults>>
  26:     0x7fd90ae01768 - <rustc_query_impl[6a79f5e9aada5532]::Queries as rustc_middle[ddd14b4469de6474]::ty::query::QueryEngine>::typeck
  27:     0x7fd90a8a0498 - <rustc_middle[ddd14b4469de6474]::hir::map::Map>::par_body_owners::<rustc_typeck[73fe2133e9515c12]::check::typeck_item_bodies::{closure#0}>
  28:     0x7fd90b5bf11c - rustc_typeck[73fe2133e9515c12]::check::typeck_item_bodies
  29:     0x7fd90b827772 - rustc_query_system[edad6c0bb6767a6f]::query::plumbing::try_execute_query::<rustc_query_impl[6a79f5e9aada5532]::plumbing::QueryCtxt, rustc_query_system[edad6c0bb6767a6f]::query::caches::DefaultCache<(), ()>>
  30:     0x7fd90b84e8c1 - rustc_query_system[edad6c0bb6767a6f]::query::plumbing::get_query::<rustc_query_impl[6a79f5e9aada5532]::queries::typeck_item_bodies, rustc_query_impl[6a79f5e9aada5532]::plumbing::QueryCtxt>
  31:     0x7fd90b5f3783 - <rustc_session[c7e02bcec93d4126]::session::Session>::time::<(), rustc_typeck[73fe2133e9515c12]::check_crate::{closure#7}>
  32:     0x7fd90b5c8743 - rustc_typeck[73fe2133e9515c12]::check_crate
  33:     0x7fd90b353737 - rustc_interface[c5c78a653a9ca231]::passes::analysis
  34:     0x7fd90b81d8a4 - rustc_query_system[edad6c0bb6767a6f]::query::plumbing::try_execute_query::<rustc_query_impl[6a79f5e9aada5532]::plumbing::QueryCtxt, rustc_query_system[edad6c0bb6767a6f]::query::caches::DefaultCache<(), core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>>
  35:     0x7fd90b85e72e - rustc_query_system[edad6c0bb6767a6f]::query::plumbing::get_query::<rustc_query_impl[6a79f5e9aada5532]::queries::analysis, rustc_query_impl[6a79f5e9aada5532]::plumbing::QueryCtxt>
  36:     0x7fd90b32f417 - <rustc_interface[c5c78a653a9ca231]::passes::QueryContext>::enter::<rustc_driver[8bde46e801238545]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>
  37:     0x7fd90b319f08 - <rustc_interface[c5c78a653a9ca231]::interface::Compiler>::enter::<rustc_driver[8bde46e801238545]::run_compiler::{closure#1}::{closure#2}, core[90820e35db584f87]::result::Result<core[90820e35db584f87]::option::Option<rustc_interface[c5c78a653a9ca231]::queries::Linker>, rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>
  38:     0x7fd90b3437cf - rustc_span[b93d47c7c5255e26]::with_source_map::<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_interface[c5c78a653a9ca231]::interface::create_compiler_and_run<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>::{closure#1}>
  39:     0x7fd90b32e4f4 - rustc_interface[c5c78a653a9ca231]::interface::create_compiler_and_run::<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>
  40:     0x7fd90b3189a2 - <scoped_tls[bc6bf2a250b07188]::ScopedKey<rustc_span[b93d47c7c5255e26]::SessionGlobals>>::set::<rustc_interface[c5c78a653a9ca231]::interface::run_compiler<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>
  41:     0x7fd90b33e13f - std[ed3f406e496fe960]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c5c78a653a9ca231]::util::run_in_thread_pool_with_globals<rustc_interface[c5c78a653a9ca231]::interface::run_compiler<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>
  42:     0x7fd90b330959 - <<std[ed3f406e496fe960]::thread::Builder>::spawn_unchecked_<rustc_interface[c5c78a653a9ca231]::util::run_in_thread_pool_with_globals<rustc_interface[c5c78a653a9ca231]::interface::run_compiler<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>::{closure#1} as core[90820e35db584f87]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7fd908b252f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfeaf6f240b8eb3a3
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/alloc/src/boxed.rs:1866:9
  44:     0x7fd908b252f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h18f279dec3207e4d
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/alloc/src/boxed.rs:1866:9
  45:     0x7fd908b252f3 - std::sys::unix::thread::Thread::new::thread_start::h9744b257b405e53e
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys/unix/thread.rs:108:17
  46:     0x7fd908a55609 - start_thread
  47:     0x7fd90896e163 - clone
  48:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (4dd8b420c 2022-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
             3: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             4: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path
             6: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             7: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
             8: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
             9: core::fmt::write
                       at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/core/src/fmt/mod.rs:1194:17
            10: core::fmt::Write::write_fmt
                       at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/core/src/fmt/mod.rs:186:9
            11: alloc::fmt::format
                       at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/alloc/src/fmt.rs:597:5
            12: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_field
            13: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
            14: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
            15: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
            16: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
            17: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
            18: rustc_typeck::check::check::check_fn
            19: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
            20: rustc_typeck::check::typeck
            21: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
            22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
            23: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>
            24: rustc_typeck::check::typeck_item_bodies
            25: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
            26: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
            27: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
            28: rustc_typeck::check_crate
            29: rustc_interface::passes::analysis
            30: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
            31: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            32: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            33: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
            34: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
            35: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
            36: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            37: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            38: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            39: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/alloc/src/boxed.rs:1866:9
            40: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/alloc/src/boxed.rs:1866:9
            41: std::sys::unix::thread::Thread::new::thread_start
                       at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys/unix/thread.rs:108:17
            42: start_thread
            43: clone
          

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1358:13
stack backtrace:
   0:     0x7fd908b17ebd - std::backtrace_rs::backtrace::libunwind::trace::h2110ce138777722b
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fd908b17ebd - std::backtrace_rs::backtrace::trace_unsynchronized::h8581d1b0ad43c04f
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd908b17ebd - std::sys_common::backtrace::_print_fmt::h61a0a5b1306ac3f8
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fd908b17ebd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h926b8315f8994dd9
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fd908b73bfc - core::fmt::write::h3f4cb6a6cbebab0c
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/core/src/fmt/mod.rs:1194:17
   5:     0x7fd908b095b1 - std::io::Write::write_fmt::h473895de5b528a4c
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/io/mod.rs:1655:15
   6:     0x7fd908b1abd5 - std::sys_common::backtrace::_print::h1e966216ddc81c55
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fd908b1abd5 - std::sys_common::backtrace::print::h506321a44638b03e
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fd908b1abd5 - std::panicking::default_hook::{{closure}}::hdcf8a0182858e663
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/panicking.rs:295:22
   9:     0x7fd908b1a849 - std::panicking::default_hook::hd68b372c6d095666
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/panicking.rs:314:9
  10:     0x7fd9092bfe41 - rustc_driver[8bde46e801238545]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fd908b1b3a6 - std::panicking::rust_panic_with_hook::haab191f924575952
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/panicking.rs:702:17
  12:     0x7fd90a4168e1 - std[ed3f406e496fe960]::panicking::begin_panic::<rustc_errors[2eb52f99f4960018]::ExplicitBug>::{closure#0}
  13:     0x7fd90a4156d6 - std[ed3f406e496fe960]::sys_common::backtrace::__rust_end_short_backtrace::<std[ed3f406e496fe960]::panicking::begin_panic<rustc_errors[2eb52f99f4960018]::ExplicitBug>::{closure#0}, !>
  14:     0x7fd90a42e736 - std[ed3f406e496fe960]::panicking::begin_panic::<rustc_errors[2eb52f99f4960018]::ExplicitBug>
  15:     0x7fd90a42b7e6 - std[ed3f406e496fe960]::panic::panic_any::<rustc_errors[2eb52f99f4960018]::ExplicitBug>
  16:     0x7fd90bc0cc47 - <rustc_errors[2eb52f99f4960018]::HandlerInner as core[90820e35db584f87]::ops::drop::Drop>::drop
  17:     0x7fd90b345168 - core[90820e35db584f87]::ptr::drop_in_place::<rustc_session[c7e02bcec93d4126]::parse::ParseSess>
  18:     0x7fd90b347473 - <alloc[94dc7c2f62e24003]::rc::Rc<rustc_session[c7e02bcec93d4126]::session::Session> as core[90820e35db584f87]::ops::drop::Drop>::drop
  19:     0x7fd90b34400d - core[90820e35db584f87]::ptr::drop_in_place::<rustc_interface[c5c78a653a9ca231]::interface::Compiler>
  20:     0x7fd90b343e4f - rustc_span[b93d47c7c5255e26]::with_source_map::<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_interface[c5c78a653a9ca231]::interface::create_compiler_and_run<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>::{closure#1}>
  21:     0x7fd90b32e4f4 - rustc_interface[c5c78a653a9ca231]::interface::create_compiler_and_run::<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>
  22:     0x7fd90b3189a2 - <scoped_tls[bc6bf2a250b07188]::ScopedKey<rustc_span[b93d47c7c5255e26]::SessionGlobals>>::set::<rustc_interface[c5c78a653a9ca231]::interface::run_compiler<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>
  23:     0x7fd90b33e13f - std[ed3f406e496fe960]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c5c78a653a9ca231]::util::run_in_thread_pool_with_globals<rustc_interface[c5c78a653a9ca231]::interface::run_compiler<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>
  24:     0x7fd90b330959 - <<std[ed3f406e496fe960]::thread::Builder>::spawn_unchecked_<rustc_interface[c5c78a653a9ca231]::util::run_in_thread_pool_with_globals<rustc_interface[c5c78a653a9ca231]::interface::run_compiler<core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>, rustc_driver[8bde46e801238545]::run_compiler::{closure#1}>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>::{closure#0}, core[90820e35db584f87]::result::Result<(), rustc_errors[2eb52f99f4960018]::ErrorGuaranteed>>::{closure#1} as core[90820e35db584f87]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7fd908b252f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfeaf6f240b8eb3a3
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/alloc/src/boxed.rs:1866:9
  26:     0x7fd908b252f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h18f279dec3207e4d
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/alloc/src/boxed.rs:1866:9
  27:     0x7fd908b252f3 - std::sys::unix::thread::Thread::new::thread_start::h9744b257b405e53e
                               at /rustc/4dd8b420c027001e47b0d811a7e55e2fe1de1395/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7fd908a55609 - start_thread
  29:     0x7fd90896e163 - clone
  30:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (4dd8b420c 2022-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: could not compile `playground`
