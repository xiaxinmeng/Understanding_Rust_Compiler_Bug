
error: unconstrained generic constant
  --> <source>:15:26
   |
15 |             Some(x) => x.hmm(&res),
   |                          ^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::W]:`

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1039:27
stack backtrace:
   0:     0x7f3ffa3f4f30 - std::backtrace_rs::backtrace::libunwind::trace::hfa838fc631229987
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f3ffa3f4f30 - std::backtrace_rs::backtrace::trace_unsynchronized::h93a23e36ec026219
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f3ffa3f4f30 - std::sys_common::backtrace::_print_fmt::hba56c7f796a4152f
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f3ffa3f4f30 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h214637f1e26310e1
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f3ffa4629cc - core::fmt::write::h7aa6cd0067dca82a
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/core/src/fmt/mod.rs:1110:17
   5:     0x7f3ffa3e6905 - std::io::Write::write_fmt::heb07fc0616bbd06d
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/io/mod.rs:1588:15
   6:     0x7f3ffa3f8dbb - std::sys_common::backtrace::_print::h2c2441c37e894fb5
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f3ffa3f8dbb - std::sys_common::backtrace::print::h4fb679ac439362ea
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f3ffa3f8dbb - std::panicking::default_hook::{{closure}}::h56bbadec2356e5d2
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/panicking.rs:208:50
   9:     0x7f3ffa3f8891 - std::panicking::default_hook::hb25822b45f6fdc4e
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/panicking.rs:225:9
  10:     0x7f3ffac0023d - rustc_driver::report_ice::h76bc4f5444182048
  11:     0x7f3ffa3f95e9 - std::panicking::rust_panic_with_hook::h4da5578e7277d2d4
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/panicking.rs:626:17
  12:     0x7f3ffbb77475 - std::panicking::begin_panic::{{closure}}::hc88daeb5c009ae76
  13:     0x7f3ffbb7742d - std::sys_common::backtrace::__rust_end_short_backtrace::h84a524526011be00
  14:     0x7f3ffbb78e09 - std::panicking::begin_panic::h33279cf97ed484d9
  15:     0x7f3ffcfe65ad - rustc_errors::HandlerInner::emit_diagnostic::h76780c4d20460946
  16:     0x7f3ffcfdaa10 - rustc_errors::diagnostic_builder::DiagnosticBuilder::emit::h0d62b3ea3765c3ae
  17:     0x7f3ffb8d6175 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error::h8ca4ee57ee8d264f
  18:     0x7f3ffb8d508e - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors::h8849a95ac2a470d5
  19:     0x7f3ffbd83792 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types::h89f69c415052d283
  20:     0x7f3ffbd821b7 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_method_argument_types::h792ac239b2f5c8a9
  21:     0x7f3ffbd6f1fe - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h8d1f807cdea5b1f2
  22:     0x7f3ffbd6e3f5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h7392d37e694948a0
  23:     0x7f3ffbd62ef9 - rustc_typeck::check::_match::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_match::h039eaaf8da6dee99
  24:     0x7f3ffbd6fb02 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h8d1f807cdea5b1f2
  25:     0x7f3ffbd6e3f5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h7392d37e694948a0
  26:     0x7f3ffbd85792 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected::h7aa84a10b232436e
  27:     0x7f3ffbd6e3f5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h7392d37e694948a0
  28:     0x7f3ffbd796e3 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr::hcc862e1ed7fc889d
  29:     0x7f3ffbe585a9 - rustc_typeck::check::check::check_fn::h3335df8aabd02e64
  30:     0x7f3ffbdf7abf - rustc_typeck::check::inherited::InheritedBuilder::enter::h5b9d92bda223182e
  31:     0x7f3ffbe508f7 - rustc_typeck::check::typeck::hdf6c1204a66d0d0e
  32:     0x7f3ffbf7f14e - rustc_query_system::query::plumbing::get_query_impl::h71d16d63d272bfdd
  33:     0x7f3ffbfd707d - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck::h782dd09a7500f3f1
  34:     0x7f3ffbdc97aa - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners::h2d3bb0a997bcab19
  35:     0x7f3ffc92413c - rustc_typeck::check::typeck_item_bodies::hddbcdf3536483b98
  36:     0x7f3ffca25976 - rustc_query_system::query::plumbing::get_query_impl::hebfadea87ef89ff3
  37:     0x7f3ffcab5b09 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck_item_bodies::hfe37ccaef674a2a2
  38:     0x7f3ffc8d513f - rustc_session::utils::<impl rustc_session::session::Session>::time::hd4175fdfc4364b5a
  39:     0x7f3ffc9526e7 - rustc_typeck::check_crate::h1507f8a8066b13df
  40:     0x7f3ffc6adb4f - rustc_interface::passes::analysis::h66dd9a05fe8e645a
  41:     0x7f3ffca12af3 - rustc_query_system::query::plumbing::get_query_impl::h9aeea1401fc08680
  42:     0x7f3ffcab3e19 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::hfde1ec9e545fab07
  43:     0x7f3ffc697fa7 - rustc_interface::passes::QueryContext::enter::h2fbbc48960ce5abf
  44:     0x7f3ffc673573 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hae97425be50638ea
  45:     0x7f3ffc672179 - rustc_span::with_source_map::he9f59604b4181090
  46:     0x7f3ffc6744a0 - rustc_interface::interface::create_compiler_and_run::h5ef56071a85cc477
  47:     0x7f3ffc6727d9 - rustc_span::with_session_globals::h2cbe1135abeaa753
  48:     0x7f3ffc69836f - std::sys_common::backtrace::__rust_begin_short_backtrace::hf021a25e992e2a35
  49:     0x7f3ffc66adf5 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc1d5236c15af366c
  50:     0x7f3ffa405ac7 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd1f9b751a68dd2ac
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/alloc/src/boxed.rs:1575:9
  51:     0x7f3ffa405ac7 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h22d732940e85619f
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/alloc/src/boxed.rs:1575:9
  52:     0x7f3ffa405ac7 - std::sys::unix::thread::Thread::new::thread_start::h04c7a9e17ed1032c
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys/unix/thread.rs:71:17
  53:     0x7f3ffa32c299 - start_thread
  54:     0x7f3ffa243053 - clone
  55:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (16e18395c 2021-06-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug --crate-type rlib

query stack during panic:
#0 [typeck] type-checking `<impl at a.rs:10:1: 19:2>::hmm`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack

