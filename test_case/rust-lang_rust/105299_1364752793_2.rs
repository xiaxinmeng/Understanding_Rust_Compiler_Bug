text
   Compiling playground v0.0.1 (/playground)
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: NoSolution', compiler/rustc_borrowck/src/type_check/liveness/trace.rs:574:88
stack backtrace:
   0:     0x7efcb7fab9a0 - std::backtrace_rs::backtrace::libunwind::trace::he615646ea344481f
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7efcb7fab9a0 - std::backtrace_rs::backtrace::trace_unsynchronized::h6ea8eaac68705b9c
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7efcb7fab9a0 - std::sys_common::backtrace::_print_fmt::h7ac486a935ce0bf7
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7efcb7fab9a0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1b5a095d3db2e28f
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7efcb800798e - core::fmt::write::h445545b92224a1cd
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/fmt/mod.rs:1209:17
   5:     0x7efcb7f9bb15 - std::io::Write::write_fmt::h55a43474c6520b00
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/io/mod.rs:1682:15
   6:     0x7efcb7fab765 - std::sys_common::backtrace::_print::h65d20526fdb736b0
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7efcb7fab765 - std::sys_common::backtrace::print::h6555fbe12a1cc41b
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7efcb7fae56f - std::panicking::default_hook::{{closure}}::hbdf58083140e7ac6
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:267:22
   9:     0x7efcb7fae2aa - std::panicking::default_hook::haef8271c56b74d85
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:286:9
  10:     0x7efcb7faed78 - std::panicking::rust_panic_with_hook::hfd45b6b6c12d9fa5
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:688:13
  11:     0x7efcb7faeb17 - std::panicking::begin_panic_handler::{{closure}}::hf591e8609a75bd4b
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:579:13
  12:     0x7efcb7fabe4c - std::sys_common::backtrace::__rust_end_short_backtrace::h81899558795e4ff7
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7efcb7fae832 - rust_begin_unwind
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:575:5
  14:     0x7efcb8004373 - core::panicking::panic_fmt::h4235fa9b4675b332
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/panicking.rs:65:14
  15:     0x7efcb80048e3 - core::result::unwrap_failed::ha17dbf463031a5e1
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/result.rs:1791:5
  16:     0x7efcb9f9aefa - rustc_borrowck[60153a3fde5c674a]::type_check::liveness::trace::trace
  17:     0x7efcb9f6bc67 - rustc_borrowck[60153a3fde5c674a]::type_check::liveness::generate
  18:     0x7efcb9f66f76 - rustc_borrowck[60153a3fde5c674a]::type_check::type_check
  19:     0x7efcb9f5730a - rustc_borrowck[60153a3fde5c674a]::nll::compute_regions
  20:     0x7efcb9f3865f - rustc_borrowck[60153a3fde5c674a]::do_mir_borrowck
  21:     0x7efcb9f1e996 - rustc_borrowck[60153a3fde5c674a]::mir_borrowck
  22:     0x7efcba29651c - rustc_query_system[330b635d327008f7]::query::plumbing::try_execute_query::<rustc_query_impl[969f89a88b0ee9ca]::plumbing::QueryCtxt, rustc_query_system[330b635d327008f7]::query::caches::DefaultCache<rustc_span[14998722174c1bca]::def_id::LocalDefId, &rustc_middle[1bbcd7fc39987e50]::mir::query::BorrowCheckResult>>
  23:     0x7efcb954e072 - rustc_data_structures[e8a5264856147ea5]::sync::par_for_each_in::<&[rustc_span[14998722174c1bca]::def_id::LocalDefId], <rustc_middle[1bbcd7fc39987e50]::hir::map::Map>::par_body_owners<rustc_interface[ee1a3f92e887e004]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  24:     0x7efcb954cb4f - <rustc_session[87e5e219eb43521c]::session::Session>::time::<(), rustc_interface[ee1a3f92e887e004]::passes::analysis::{closure#2}>
  25:     0x7efcb954aa87 - rustc_interface[ee1a3f92e887e004]::passes::analysis
  26:     0x7efcba987ac4 - rustc_query_system[330b635d327008f7]::query::plumbing::try_execute_query::<rustc_query_impl[969f89a88b0ee9ca]::plumbing::QueryCtxt, rustc_query_system[330b635d327008f7]::query::caches::DefaultCache<(), core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>>
  27:     0x7efcba9877c7 - rustc_query_system[330b635d327008f7]::query::plumbing::get_query::<rustc_query_impl[969f89a88b0ee9ca]::queries::analysis, rustc_query_impl[969f89a88b0ee9ca]::plumbing::QueryCtxt>
  28:     0x7efcba4c9acd - <rustc_interface[ee1a3f92e887e004]::passes::QueryContext>::enter::<rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  29:     0x7efcba4c5f7f - <rustc_interface[ee1a3f92e887e004]::interface::Compiler>::enter::<rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}::{closure#2}, core[b97a30f8df81432d]::result::Result<core[b97a30f8df81432d]::option::Option<rustc_interface[ee1a3f92e887e004]::queries::Linker>, rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  30:     0x7efcba4bdac2 - rustc_span[14998722174c1bca]::with_source_map::<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  31:     0x7efcba4bd5b9 - <scoped_tls[23afcff80b89ba49]::ScopedKey<rustc_span[14998722174c1bca]::SessionGlobals>>::set::<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  32:     0x7efcba4bcbc8 - std[bbad73ae434e23e5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ee1a3f92e887e004]::util::run_in_thread_pool_with_globals<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  33:     0x7efcba4bc8ec - <<std[bbad73ae434e23e5]::thread::Builder>::spawn_unchecked_<rustc_interface[ee1a3f92e887e004]::util::run_in_thread_pool_with_globals<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#1} as core[b97a30f8df81432d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7efcbbe666a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4273f95ec44459b3
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/boxed.rs:1987:9
  35:     0x7efcbbe666a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h70f28fa4ddc269e5
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/boxed.rs:1987:9
  36:     0x7efcbbe666a3 - std::sys::unix::thread::Thread::new::thread_start::h85a9c16b988e2bd0
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys/unix/thread.rs:108:17
  37:     0x7efcb7e8a609 - start_thread
  38:     0x7efcb7dad133 - clone
  39:                0x0 - <unknown>
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: unexpected ambiguity: Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [Binder(TraitPredicate(<[T] as Foo>, polarity:Positive), []), Binder(TraitPredicate(<[T] as std::clone::Clone>, polarity:Positive), []), Binder(TraitPredicate(<[T] as std::marker::Sized>, polarity:Positive), []), Binder(TraitPredicate(<T as std::clone::Clone>, polarity:Positive), []), Binder(TraitPredicate(<T as std::marker::Sized>, polarity:Positive), [])], reveal: UserFacing, constness: NotConst }, value: ProjectionTy { substs: [[T]], item_def_id: DefId(5:859 ~ alloc[d987]::borrow::ToOwned::Owned) } } } Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: QueryResponse { var_values: CanonicalVarValues { var_values: [] }, region_constraints: QueryRegionConstraints { outlives: [], member_constraints: [] }, certainty: Ambiguous, opaque_types: [], value: NormalizationResult { normalized_ty: ^0 } } }
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/query/normalize.rs:260:34

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1553:13
stack backtrace:
   0:     0x7efcbbe5ce20 - std::backtrace_rs::backtrace::libunwind::trace::he615646ea344481f
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7efcbbe5ce20 - std::backtrace_rs::backtrace::trace_unsynchronized::h6ea8eaac68705b9c
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7efcbbe5ce20 - std::sys_common::backtrace::_print_fmt::h7ac486a935ce0bf7
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7efcbbe5ce20 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1b5a095d3db2e28f
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7efcb800798e - core::fmt::write::h445545b92224a1cd
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/fmt/mod.rs:1209:17
   5:     0x7efcbbe50ad5 - std::io::Write::write_fmt::h55a43474c6520b00
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/io/mod.rs:1682:15
   6:     0x7efcbbe5cbe5 - std::sys_common::backtrace::_print::h65d20526fdb736b0
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7efcbbe5cbe5 - std::sys_common::backtrace::print::h6555fbe12a1cc41b
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7efcbbe5ef3f - std::panicking::default_hook::{{closure}}::hbdf58083140e7ac6
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:267:22
   9:     0x7efcbbe5ec7a - std::panicking::default_hook::haef8271c56b74d85
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:286:9
  10:     0x7efcbb122cd4 - <rustc_driver[9c4183344b2d0066]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[b97a30f8df81432d]::ops::function::FnOnce<(&core[b97a30f8df81432d]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7efcbbe5f729 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h44df53ea2a13204b
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/boxed.rs:2001:9
  12:     0x7efcbbe5f729 - std::panicking::rust_panic_with_hook::hfd45b6b6c12d9fa5
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:692:13
  13:     0x7efcbb1510b1 - std[bbad73ae434e23e5]::panicking::begin_panic::<rustc_errors[21897ed46328f955]::ExplicitBug>::{closure#0}
  14:     0x7efcbb14f846 - std[bbad73ae434e23e5]::sys_common::backtrace::__rust_end_short_backtrace::<std[bbad73ae434e23e5]::panicking::begin_panic<rustc_errors[21897ed46328f955]::ExplicitBug>::{closure#0}, !>
  15:     0x7efcbb149636 - std[bbad73ae434e23e5]::panicking::begin_panic::<rustc_errors[21897ed46328f955]::ExplicitBug>
  16:     0x7efcbb14bcc6 - std[bbad73ae434e23e5]::panic::panic_any::<rustc_errors[21897ed46328f955]::ExplicitBug>
  17:     0x7efcba6c4b71 - <rustc_errors[21897ed46328f955]::HandlerInner>::flush_delayed::<alloc[d987cf4402e5b40a]::vec::Vec<rustc_errors[21897ed46328f955]::diagnostic::Diagnostic>, &str>
  18:     0x7efcba6c3bb2 - <rustc_errors[21897ed46328f955]::HandlerInner as core[b97a30f8df81432d]::ops::drop::Drop>::drop
  19:     0x7efcba50cd58 - core[b97a30f8df81432d]::ptr::drop_in_place::<rustc_session[87e5e219eb43521c]::parse::ParseSess>
  20:     0x7efcba4bedf8 - core[b97a30f8df81432d]::ptr::drop_in_place::<rustc_session[87e5e219eb43521c]::session::Session>
  21:     0x7efcba4beb19 - core[b97a30f8df81432d]::ptr::drop_in_place::<rustc_interface[ee1a3f92e887e004]::interface::Compiler>
  22:     0x7efcba4bdf2e - rustc_span[14998722174c1bca]::with_source_map::<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  23:     0x7efcba4bd5b9 - <scoped_tls[23afcff80b89ba49]::ScopedKey<rustc_span[14998722174c1bca]::SessionGlobals>>::set::<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  24:     0x7efcba4bcbc8 - std[bbad73ae434e23e5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ee1a3f92e887e004]::util::run_in_thread_pool_with_globals<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  25:     0x7efcba4bc8ec - <<std[bbad73ae434e23e5]::thread::Builder>::spawn_unchecked_<rustc_interface[ee1a3f92e887e004]::util::run_in_thread_pool_with_globals<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#1} as core[b97a30f8df81432d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x7efcbbe666a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4273f95ec44459b3
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/boxed.rs:1987:9
  27:     0x7efcbbe666a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h70f28fa4ddc269e5
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/boxed.rs:1987:9
  28:     0x7efcbbe666a3 - std::sys::unix::thread::Thread::new::thread_start::h85a9c16b988e2bd0
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys/unix/thread.rs:108:17
  29:     0x7efcb7e8a609 - start_thread
  30:     0x7efcb7dad133 - clone
  31:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0 (69f9c33d7 2022-12-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread '<unnamed>' panicked at 'panic in a function that cannot unwind', library/core/src/panicking.rs:90:58
stack backtrace:
   0:     0x7efcb7fab9a0 - std::backtrace_rs::backtrace::libunwind::trace::he615646ea344481f
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7efcb7fab9a0 - std::backtrace_rs::backtrace::trace_unsynchronized::h6ea8eaac68705b9c
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7efcb7fab9a0 - std::sys_common::backtrace::_print_fmt::h7ac486a935ce0bf7
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7efcb7fab9a0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1b5a095d3db2e28f
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7efcb800798e - core::fmt::write::h445545b92224a1cd
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/fmt/mod.rs:1209:17
   5:     0x7efcb7f9bb15 - std::io::Write::write_fmt::h55a43474c6520b00
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/io/mod.rs:1682:15
   6:     0x7efcb7fab765 - std::sys_common::backtrace::_print::h65d20526fdb736b0
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7efcb7fab765 - std::sys_common::backtrace::print::h6555fbe12a1cc41b
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7efcb7fae56f - std::panicking::default_hook::{{closure}}::hbdf58083140e7ac6
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:267:22
   9:     0x7efcb7fae2aa - std::panicking::default_hook::haef8271c56b74d85
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:286:9
  10:     0x7efcb7faed78 - std::panicking::rust_panic_with_hook::hfd45b6b6c12d9fa5
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:688:13
  11:     0x7efcb7faead1 - std::panicking::begin_panic_handler::{{closure}}::hf591e8609a75bd4b
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:577:13
  12:     0x7efcb7fabe4c - std::sys_common::backtrace::__rust_end_short_backtrace::h81899558795e4ff7
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7efcb7fae832 - rust_begin_unwind
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:575:5
  14:     0x7efcb80043f3 - core::panicking::panic_str_nounwind::h2959ea310e934ac1
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/panicking.rs:93:14
  15:     0x7efcb8004563 - core::panicking::panic_no_unwind::h31a06247391473ac
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/panicking.rs:164:5
  16:     0x7efcba4bdf45 - rustc_span[14998722174c1bca]::with_source_map::<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  17:     0x7efcba4bd5b9 - <scoped_tls[23afcff80b89ba49]::ScopedKey<rustc_span[14998722174c1bca]::SessionGlobals>>::set::<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  18:     0x7efcba4bcbc8 - std[bbad73ae434e23e5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ee1a3f92e887e004]::util::run_in_thread_pool_with_globals<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  19:     0x7efcba4bc8ec - <<std[bbad73ae434e23e5]::thread::Builder>::spawn_unchecked_<rustc_interface[ee1a3f92e887e004]::util::run_in_thread_pool_with_globals<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#1} as core[b97a30f8df81432d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7efcbbe666a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4273f95ec44459b3
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/boxed.rs:1987:9
  21:     0x7efcbbe666a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h70f28fa4ddc269e5
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/boxed.rs:1987:9
  22:     0x7efcbbe666a3 - std::sys::unix::thread::Thread::new::thread_start::h85a9c16b988e2bd0
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys/unix/thread.rs:108:17
  23:     0x7efcb7e8a609 - start_thread
  24:     0x7efcb7dad133 - clone
  25:                0x0 - <unknown>
thread panicked while panicking. aborting.
error: could not compile `playground`

