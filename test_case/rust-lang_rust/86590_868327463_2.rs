
error: unconstrained generic constant
  --> <source>:15:26
   |
15 |             Some(x) => x.hmm(&res),
   |                          ^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::W]:`

error[E0308]: mismatched types
  --> <source>:15:30
   |
15 |             Some(x) => x.hmm(&res),
   |                              ^^^^ expected `Self::W`, found `Self::W`
   |
   = note: expected type `Self::W`
              found type `Self::W`

thread 'rustc' panicked at 'aborting after 2 errors due to `-Z treat-err-as-bug=2`', compiler/rustc_errors/src/lib.rs:1041:36
stack backtrace:
   0:     0x7fd1b69d5f30 - std::backtrace_rs::backtrace::libunwind::trace::hfa838fc631229987
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7fd1b69d5f30 - std::backtrace_rs::backtrace::trace_unsynchronized::h93a23e36ec026219
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd1b69d5f30 - std::sys_common::backtrace::_print_fmt::hba56c7f796a4152f
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7fd1b69d5f30 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h214637f1e26310e1
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7fd1b6a439cc - core::fmt::write::h7aa6cd0067dca82a
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/core/src/fmt/mod.rs:1110:17
   5:     0x7fd1b69c7905 - std::io::Write::write_fmt::heb07fc0616bbd06d
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/io/mod.rs:1588:15
   6:     0x7fd1b69d9dbb - std::sys_common::backtrace::_print::h2c2441c37e894fb5
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7fd1b69d9dbb - std::sys_common::backtrace::print::h4fb679ac439362ea
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7fd1b69d9dbb - std::panicking::default_hook::{{closure}}::h56bbadec2356e5d2
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/panicking.rs:208:50
   9:     0x7fd1b69d9891 - std::panicking::default_hook::hb25822b45f6fdc4e
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/panicking.rs:225:9
  10:     0x7fd1b71e123d - rustc_driver::report_ice::h76bc4f5444182048
  11:     0x7fd1b69da5e9 - std::panicking::rust_panic_with_hook::h4da5578e7277d2d4
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/panicking.rs:626:17
  12:     0x7fd1b69da0a7 - std::panicking::begin_panic_handler::{{closure}}::h003783ddb3cba4e8
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/panicking.rs:519:13
  13:     0x7fd1b69d640c - std::sys_common::backtrace::__rust_end_short_backtrace::hd138d2032731ed21
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys_common/backtrace.rs:141:18
  14:     0x7fd1b69da009 - rust_begin_unwind
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/panicking.rs:515:5
  15:     0x7fd1b69a2e9b - std::panicking::begin_panic_fmt::h2db7e4060dc9c373
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/panicking.rs:457:5
  16:     0x7fd1b95c78c4 - rustc_errors::HandlerInner::emit_diagnostic::h76780c4d20460946
  17:     0x7fd1b95bba10 - rustc_errors::diagnostic_builder::DiagnosticBuilder::emit::h0d62b3ea3765c3ae
  18:     0x7fd1b7ebc808 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error::h5474fa892f25752a
  19:     0x7fd1b7eb608e - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors::h8849a95ac2a470d5
  20:     0x7fd1b8364792 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types::h89f69c415052d283
  21:     0x7fd1b83631b7 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_method_argument_types::h792ac239b2f5c8a9
  22:     0x7fd1b83501fe - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h8d1f807cdea5b1f2
  23:     0x7fd1b834f3f5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h7392d37e694948a0
  24:     0x7fd1b8343ef9 - rustc_typeck::check::_match::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_match::h039eaaf8da6dee99
  25:     0x7fd1b8350b02 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h8d1f807cdea5b1f2
  26:     0x7fd1b834f3f5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h7392d37e694948a0
  27:     0x7fd1b8366792 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected::h7aa84a10b232436e
  28:     0x7fd1b834f3f5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h7392d37e694948a0
  29:     0x7fd1b835a6e3 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr::hcc862e1ed7fc889d
  30:     0x7fd1b84395a9 - rustc_typeck::check::check::check_fn::h3335df8aabd02e64
  31:     0x7fd1b83d8abf - rustc_typeck::check::inherited::InheritedBuilder::enter::h5b9d92bda223182e
  32:     0x7fd1b84318f7 - rustc_typeck::check::typeck::hdf6c1204a66d0d0e
  33:     0x7fd1b856014e - rustc_query_system::query::plumbing::get_query_impl::h71d16d63d272bfdd
  34:     0x7fd1b85b807d - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck::h782dd09a7500f3f1
  35:     0x7fd1b83aa7aa - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners::h2d3bb0a997bcab19
  36:     0x7fd1b8f0513c - rustc_typeck::check::typeck_item_bodies::hddbcdf3536483b98
  37:     0x7fd1b9006976 - rustc_query_system::query::plumbing::get_query_impl::hebfadea87ef89ff3
  38:     0x7fd1b9096b09 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck_item_bodies::hfe37ccaef674a2a2
  39:     0x7fd1b8eb613f - rustc_session::utils::<impl rustc_session::session::Session>::time::hd4175fdfc4364b5a
  40:     0x7fd1b8f336e7 - rustc_typeck::check_crate::h1507f8a8066b13df
  41:     0x7fd1b8c8eb4f - rustc_interface::passes::analysis::h66dd9a05fe8e645a
  42:     0x7fd1b8ff3af3 - rustc_query_system::query::plumbing::get_query_impl::h9aeea1401fc08680
  43:     0x7fd1b9094e19 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::hfde1ec9e545fab07
  44:     0x7fd1b8c78fa7 - rustc_interface::passes::QueryContext::enter::h2fbbc48960ce5abf
  45:     0x7fd1b8c54573 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hae97425be50638ea
  46:     0x7fd1b8c53179 - rustc_span::with_source_map::he9f59604b4181090
  47:     0x7fd1b8c554a0 - rustc_interface::interface::create_compiler_and_run::h5ef56071a85cc477
  48:     0x7fd1b8c537d9 - rustc_span::with_session_globals::h2cbe1135abeaa753
  49:     0x7fd1b8c7936f - std::sys_common::backtrace::__rust_begin_short_backtrace::hf021a25e992e2a35
  50:     0x7fd1b8c4bdf5 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc1d5236c15af366c
  51:     0x7fd1b69e6ac7 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd1f9b751a68dd2ac
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/alloc/src/boxed.rs:1575:9
  52:     0x7fd1b69e6ac7 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h22d732940e85619f
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/alloc/src/boxed.rs:1575:9
  53:     0x7fd1b69e6ac7 - std::sys::unix::thread::Thread::new::thread_start::h04c7a9e17ed1032c
                               at /rustc/16e18395ce33ca1ebfe60a591fb2f9317a75d822/library/std/src/sys/unix/thread.rs:71:17
  54:     0x7fd1b690d299 - start_thread
  55:     0x7fd1b6824053 - clone
  56:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (16e18395c 2021-06-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug=2 --crate-type rlib

query stack during panic:
#0 [typeck] type-checking `<impl at a.rs:10:1: 19:2>::hmm`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
