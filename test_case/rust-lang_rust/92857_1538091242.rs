
thread 'rustc' panicked at 'index out of bounds: the len is 38 but the index is 38', /cargo/registry/src/index.crates.io-6f17d22bba15001f/ena-0.14.2/src/snapshot_vec.rs:199:10
stack backtrace:
   0:     0x7fc303151e71 - std::backtrace_rs::backtrace::libunwind::trace::hdca2f1d760ae3bfc
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc303151e71 - std::backtrace_rs::backtrace::trace_unsynchronized::hd02181015534cc1c
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc303151e71 - std::sys_common::backtrace::_print_fmt::h027d0dd83d9c6057
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fc303151e71 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc3454f8104aa16e4
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fc3031b21bf - core::fmt::rt::Argument::fmt::haca00ab3f8062a1f
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/core/src/fmt/rt.rs:138:9
   5:     0x7fc3031b21bf - core::fmt::write::he3e34afeea0f8a6d
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/core/src/fmt/mod.rs:1094:21
   6:     0x7fc303145141 - std::io::Write::write_fmt::h0dda8fe866ff6ad9
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/io/mod.rs:1712:15
   7:     0x7fc303151c85 - std::sys_common::backtrace::_print::hdee7445a3e0bc35e
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x7fc303151c85 - std::sys_common::backtrace::print::h88d45cc795ad4f3a
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x7fc303154917 - std::panicking::default_hook::{{closure}}::h0c28f70af069bb4e
  10:     0x7fc303154704 - std::panicking::default_hook::h9d9751edb4b25952
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/panicking.rs:288:9
  11:     0x7fc306371e4b - rustc_driver_impl[dde0876b390e6a89]::install_ice_hook::{closure#0}
  12:     0x7fc303155037 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hd829a428c0494b99
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/alloc/src/boxed.rs:1999:9
  13:     0x7fc303155037 - std::panicking::rust_panic_with_hook::hb6e77b312dba3d0e
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/panicking.rs:695:13
  14:     0x7fc303154db7 - std::panicking::begin_panic_handler::{{closure}}::h067991e3b1a9262c
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/panicking.rs:582:13
  15:     0x7fc3031522b6 - std::sys_common::backtrace::__rust_end_short_backtrace::hb367ce5d190896e5
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/sys_common/backtrace.rs:150:18
  16:     0x7fc303154b22 - rust_begin_unwind
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/panicking.rs:578:5
  17:     0x7fc3031ae443 - core::panicking::panic_fmt::h1648e922407c123f
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/core/src/panicking.rs:67:14
  18:     0x7fc3031ae5a2 - core::panicking::panic_bounds_check::hf3b4df6285bfac31
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/core/src/panicking.rs:162:5
  19:     0x7fc30434e0c6 - <ena[62d0be7b20f421c9]::unify::UnificationTable<ena[62d0be7b20f421c9]::unify::backing_vec::InPlace<rustc_middle[f12e14326fcb6d37]::infer::unify_key::RegionVidKey, &mut alloc[f90fa24b258e2697]::vec::Vec<ena[62d0be7b20f421c9]::unify::VarValue<rustc_middle[f12e14326fcb6d37]::infer::unify_key::RegionVidKey>>, &mut rustc_infer[81cc4442e2774718]::infer::undo_log::InferCtxtUndoLogs>>>::uninlined_get_root_key
  20:     0x7fc3043d4fab - <rustc_infer[81cc4442e2774718]::infer::equate::Equate as rustc_middle[f12e14326fcb6d37]::ty::relate::TypeRelation>::relate::<rustc_middle[f12e14326fcb6d37]::ty::sty::Region>
  21:     0x7fc3043d16c9 - rustc_middle[f12e14326fcb6d37]::ty::relate::super_relate_tys::<rustc_infer[81cc4442e2774718]::infer::equate::Equate>
  22:     0x7fc3043c77c5 - <rustc_infer[81cc4442e2774718]::infer::equate::Equate as rustc_middle[f12e14326fcb6d37]::ty::relate::TypeRelation>::tys
  23:     0x7fc304d677ca - <rustc_infer[81cc4442e2774718]::infer::InferCtxt>::commit_if_ok::<rustc_infer[81cc4442e2774718]::infer::InferOk<()>, rustc_middle[f12e14326fcb6d37]::ty::error::TypeError, <rustc_infer[81cc4442e2774718]::infer::at::Trace>::eq<rustc_middle[f12e14326fcb6d37]::ty::Ty>::{closure#0}>
  24:     0x7fc306631f32 - <rustc_infer[81cc4442e2774718]::infer::at::At>::eq::<rustc_middle[f12e14326fcb6d37]::ty::Ty>
  25:     0x7fc3066b06cd - <rustc_infer[81cc4442e2774718]::infer::InferCtxt>::can_eq::<rustc_middle[f12e14326fcb6d37]::ty::Ty>
  26:     0x7fc306685c43 - <rustc_infer[81cc4442e2774718]::infer::error_reporting::TypeErrCtxt>::expected_projection
  27:     0x7fc306684980 - <rustc_infer[81cc4442e2774718]::infer::error_reporting::TypeErrCtxt>::note_and_explain_type_err
  28:     0x7fc306698e82 - <rustc_infer[81cc4442e2774718]::infer::error_reporting::TypeErrCtxt>::note_type_err
  29:     0x7fc30668ed7a - <rustc_infer[81cc4442e2774718]::infer::error_reporting::TypeErrCtxt>::report_and_explain_type_error
  30:     0x7fc3064f52d0 - <rustc_hir_typeck[5439598e7a2cfc57]::fn_ctxt::FnCtxt>::report_arg_errors
  31:     0x7fc304542032 - <rustc_hir_typeck[5439598e7a2cfc57]::fn_ctxt::FnCtxt>::check_argument_types
  32:     0x7fc3045156d5 - <rustc_hir_typeck[5439598e7a2cfc57]::fn_ctxt::FnCtxt>::check_call
  33:     0x7fc30488e2d6 - <rustc_hir_typeck[5439598e7a2cfc57]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7fc3048d706f - <rustc_hir_typeck[5439598e7a2cfc57]::fn_ctxt::FnCtxt>::check_block_with_expected
  35:     0x7fc30488eaf1 - <rustc_hir_typeck[5439598e7a2cfc57]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  36:     0x7fc304e753e6 - <rustc_hir_typeck[5439598e7a2cfc57]::fn_ctxt::FnCtxt>::check_return_expr
  37:     0x7fc304e6d032 - rustc_hir_typeck[5439598e7a2cfc57]::check::check_fn
  38:     0x7fc304e549c4 - rustc_hir_typeck[5439598e7a2cfc57]::typeck
  39:     0x7fc304e4a3c2 - rustc_query_system[284fc37ba8dbaa13]::query::plumbing::try_execute_query::<rustc_query_impl[14a35b422f4e9e94]::queries::typeck, rustc_query_impl[14a35b422f4e9e94]::plumbing::QueryCtxt>
  40:     0x7fc3053f8480 - rustc_query_system[284fc37ba8dbaa13]::query::plumbing::try_execute_query::<rustc_query_impl[14a35b422f4e9e94]::queries::used_trait_imports, rustc_query_impl[14a35b422f4e9e94]::plumbing::QueryCtxt>
  41:     0x7fc3057ff342 - rustc_hir_analysis[df7ae5e86821b533]::check_crate
  42:     0x7fc3057f2025 - rustc_interface[5dd728079daf5b2f]::passes::analysis
  43:     0x7fc305af8227 - rustc_query_system[284fc37ba8dbaa13]::query::plumbing::try_execute_query::<rustc_query_impl[14a35b422f4e9e94]::queries::analysis, rustc_query_impl[14a35b422f4e9e94]::plumbing::QueryCtxt>
  44:     0x7fc305af7f43 - rustc_query_impl[14a35b422f4e9e94]::get_query::analysis
  45:     0x7fc30559f9bf - <rustc_middle[f12e14326fcb6d37]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[dde0876b390e6a89]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[1a45016e46435943]::result::Result<(), rustc_span[3e163d59592952e6]::ErrorGuaranteed>>
  46:     0x7fc30559eacf - <rustc_interface[5dd728079daf5b2f]::interface::Compiler>::enter::<rustc_driver_impl[dde0876b390e6a89]::run_compiler::{closure#1}::{closure#2}, core[1a45016e46435943]::result::Result<core[1a45016e46435943]::option::Option<rustc_interface[5dd728079daf5b2f]::queries::Linker>, rustc_span[3e163d59592952e6]::ErrorGuaranteed>>
  47:     0x7fc30559cb71 - rustc_span[3e163d59592952e6]::set_source_map::<core[1a45016e46435943]::result::Result<(), rustc_span[3e163d59592952e6]::ErrorGuaranteed>, rustc_interface[5dd728079daf5b2f]::interface::run_compiler<core[1a45016e46435943]::result::Result<(), rustc_span[3e163d59592952e6]::ErrorGuaranteed>, rustc_driver_impl[dde0876b390e6a89]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  48:     0x7fc30559c220 - std[d48aaa48ad184f7f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5dd728079daf5b2f]::util::run_in_thread_pool_with_globals<rustc_interface[5dd728079daf5b2f]::interface::run_compiler<core[1a45016e46435943]::result::Result<(), rustc_span[3e163d59592952e6]::ErrorGuaranteed>, rustc_driver_impl[dde0876b390e6a89]::run_compiler::{closure#1}>::{closure#0}, core[1a45016e46435943]::result::Result<(), rustc_span[3e163d59592952e6]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1a45016e46435943]::result::Result<(), rustc_span[3e163d59592952e6]::ErrorGuaranteed>>
  49:     0x7fc305c5aeb5 - <<std[d48aaa48ad184f7f]::thread::Builder>::spawn_unchecked_<rustc_interface[5dd728079daf5b2f]::util::run_in_thread_pool_with_globals<rustc_interface[5dd728079daf5b2f]::interface::run_compiler<core[1a45016e46435943]::result::Result<(), rustc_span[3e163d59592952e6]::ErrorGuaranteed>, rustc_driver_impl[dde0876b390e6a89]::run_compiler::{closure#1}>::{closure#0}, core[1a45016e46435943]::result::Result<(), rustc_span[3e163d59592952e6]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1a45016e46435943]::result::Result<(), rustc_span[3e163d59592952e6]::ErrorGuaranteed>>::{closure#1} as core[1a45016e46435943]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7fc30315f4d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf9d590878bec4a84
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/alloc/src/boxed.rs:1985:9
  51:     0x7fc30315f4d5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3b901df10d727b47
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/alloc/src/boxed.rs:1985:9
  52:     0x7fc30315f4d5 - std::sys::unix::thread::Thread::new::thread_start::hbbc99806fcfda4ba
                               at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/sys/unix/thread.rs:108:17
  53:     0x7fc30302e609 - start_thread
  54:     0x7fc302f51133 - clone
  55:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (c4190f2d3 2023-05-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `<impl at src/main.rs:42:1: 42:27>::load_callbacks`
#1 [used_trait_imports] finding used_trait_imports `<impl at src/main.rs:42:1: 42:27>::load_callbacks`
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: no errors reported for args
  --> src/main.rs:63:9
   |
63 |         Callbacks(callbacks)
   |         ^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &str>
              2: <rustc_hir_typeck::fn_ctxt::FnCtxt>::report_arg_errors
              3: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_argument_types
              4: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_call
              5: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
              6: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_block_with_expected
              7: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
              8: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_return_expr
              9: rustc_hir_typeck::check::check_fn
             10: rustc_hir_typeck::typeck
             11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt>
             12: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::used_trait_imports, rustc_query_impl::plumbing::QueryCtxt>
             13: rustc_hir_analysis::check_crate
             14: rustc_interface::passes::analysis
             15: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             16: rustc_query_impl::get_query::analysis
             17: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             18: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             19: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             20: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             21: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             22: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                        at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/alloc/src/boxed.rs:1985:9
             23: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                        at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/alloc/src/boxed.rs:1985:9
             24: std::sys::unix::thread::Thread::new::thread_start
                        at /rustc/c4190f2d3a46a59f435f7b42f58bc22b2f4d6917/library/std/src/sys/unix/thread.rs:108:17
             25: start_thread
             26: clone
           

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (c4190f2d3 2023-05-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: could not compile `playground` (bin "playground")
