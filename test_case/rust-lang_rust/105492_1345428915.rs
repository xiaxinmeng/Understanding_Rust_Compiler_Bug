plain
   Compiling object v0.29.0
[RUSTC-TIMING] rustc_std_workspace_alloc test:false 0.022
[RUSTC-TIMING] panic_abort test:false 0.043
[RUSTC-TIMING] panic_unwind test:false 0.073
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
             3: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             4: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             5: <rustc_middle::ty::context::TyCtxt>::def_path_str
             6: <rustc_hir::hir::Pat>::walk_::<<rustc_hir::hir::Pat>::walk_always<rustc_mir_build::thir::pattern::check_match::check_for_bindings_named_same_as_variants::{closure#0}>::{closure#0}>
             7: <rustc_mir_build::thir::pattern::check_match::MatchVisitor>::check_irrefutable
             8: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_local
             9: rustc_hir::intravisit::walk_expr::<rustc_mir_build::thir::pattern::check_match::MatchVisitor>
            10: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
            11: rustc_mir_build::thir::pattern::check_match::check_match
            12: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, ()>>
            13: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_match, rustc_query_impl::plumbing::QueryCtxt>
            14: rustc_data_structures::sync::par_for_each_in::<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_interface::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
            15: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#1}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
            16: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#1}>
            18: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
            19: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            20: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            21: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
            21: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
            22: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            23: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            24: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            25: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            26: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/alloc/src/boxed.rs:1990:9
            27: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/alloc/src/boxed.rs:1990:9
                       at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/sys/unix/thread.rs:108:17
            29: start_thread
            30: clone
          
          

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1610:13
stack backtrace:
   0:     0x7f37c7697010 - std::backtrace_rs::backtrace::libunwind::trace::ha182a275669c7be9
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f37c7697010 - std::backtrace_rs::backtrace::trace_unsynchronized::h75413296d21f67e2
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f37c7697010 - std::sys_common::backtrace::_print_fmt::h169238ea222f4208
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f37c7697010 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h87444059c5498e93
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f37c76f700e - core::fmt::write::hcb131ddd0c15f718
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f37c7688c05 - std::io::Write::write_fmt::h7a871bbd1b2c919b
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/io/mod.rs:1682:15
   6:     0x7f37c7696dd5 - std::sys_common::backtrace::_print::h90afda076b677b8e
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f37c7696dd5 - std::sys_common::backtrace::print::h0c6bf4c65b86a96b
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f37c7699b2f - std::panicking::default_hook::{{closure}}::h82eb55a48c8b1cec
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/panicking.rs:267:22
   9:     0x7f37c769986a - std::panicking::default_hook::h0f3ae919e54e4f28
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/panicking.rs:286:9
  10:     0x7f37c83f6d36 - rustc_driver[9cf50ef95d97bfe5]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f37c769a2f9 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h8ddc8652f49a66a2
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/alloc/src/boxed.rs:2024:9
  12:     0x7f37c769a2f9 - std::panicking::rust_panic_with_hook::h05c77ca20a252ab3
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/panicking.rs:692:13
  13:     0x7f37cdf70df3 - std[af4a3ff7bc24e988]::panicking::begin_panic::<rustc_errors[b92b891ac752ae31]::ExplicitBug>::{closure#0}
  14:     0x7f37cdf6d276 - std[af4a3ff7bc24e988]::sys_common::backtrace::__rust_end_short_backtrace::<std[af4a3ff7bc24e988]::panicking::begin_panic<rustc_errors[b92b891ac752ae31]::ExplicitBug>::{closure#0}, !>
  15:     0x7f37c828e586 - std[af4a3ff7bc24e988]::panicking::begin_panic::<rustc_errors[b92b891ac752ae31]::ExplicitBug>
  16:     0x7f37cdf62f86 - std[af4a3ff7bc24e988]::panic::panic_any::<rustc_errors[b92b891ac752ae31]::ExplicitBug>
  17:     0x7f37cdf66508 - <rustc_errors[b92b891ac752ae31]::HandlerInner as core[ae65f184c67bc919]::ops::drop::Drop>::drop
  18:     0x7f37c836e1e8 - core[ae65f184c67bc919]::ptr::drop_in_place::<rustc_session[1d8f9ffdab79f913]::parse::ParseSess>
  19:     0x7f37c836f9e8 - core[ae65f184c67bc919]::ptr::drop_in_place::<rustc_session[1d8f9ffdab79f913]::session::Session>
  20:     0x7f37c835e6f9 - core[ae65f184c67bc919]::ptr::drop_in_place::<rustc_interface[37bdbdcd124593a6]::interface::Compiler>
  21:     0x7f37c835b274 - rustc_span[b384040717ceefb]::with_source_map::<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_interface[37bdbdcd124593a6]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_driver[9cf50ef95d97bfe5]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  22:     0x7f37c83bedc8 - <scoped_tls[27cd29d9afbae785]::ScopedKey<rustc_span[b384040717ceefb]::SessionGlobals>>::set::<rustc_interface[37bdbdcd124593a6]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_driver[9cf50ef95d97bfe5]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>
  23:     0x7f37c8385b00 - std[af4a3ff7bc24e988]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[37bdbdcd124593a6]::util::run_in_thread_pool_with_globals<rustc_interface[37bdbdcd124593a6]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_driver[9cf50ef95d97bfe5]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>
  24:     0x7f37c8365ee4 - <<std[af4a3ff7bc24e988]::thread::Builder>::spawn_unchecked_<rustc_interface[37bdbdcd124593a6]::util::run_in_thread_pool_with_globals<rustc_interface[37bdbdcd124593a6]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_driver[9cf50ef95d97bfe5]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>::{closure#1} as core[ae65f184c67bc919]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7f37c76a40d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h80124674980fc741
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/alloc/src/boxed.rs:1990:9
  26:     0x7f37c76a40d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h5adf35c707622c31
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/alloc/src/boxed.rs:1990:9
  27:     0x7f37c76a40d3 - std::sys::unix::thread::Thread::new::thread_start::ha68eca8ef3e01ad3
                               at /rustc/6278fdf7a1ddd0f52f318ffdc19651902aa52154/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7f37c73d7609 - start_thread
  30:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (6278fdf7a 2022-12-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=riscv64-unknown-linux-gnu-gcc -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -C force-unwind-tables=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
