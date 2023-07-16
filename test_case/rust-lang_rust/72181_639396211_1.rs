`
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', src/librustc_middle/ty/mod.rs:2359:19
stack backtrace:
   0:     0x7f2188294d57 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he2fbf5405bd3f46a
   1:     0x7f21882e1a3d - core::fmt::write::h65255e7874586eb2
   2:     0x7f2188265b95 - std::io::Write::write_fmt::h3c49d63e356d89fe
   3:     0x7f2188271980 - std::panicking::default_hook::{{closure}}::h6b88f05100c9e449
   4:     0x7f2188271694 - std::panicking::default_hook::h57a663a807afe943
   5:     0x7f2189c298a3 - rustc_driver::report_ice::h34c4499908ba0b29
   6:     0x7f218827206c - std::panicking::rust_panic_with_hook::haef04471b94d420c
   7:     0x7f2188271c3b - rust_begin_unwind
   8:     0x7f21882df7e1 - core::panicking::panic_fmt::h95ae6ee01add471a
   9:     0x7f21882df7a2 - core::panicking::panic_bounds_check::h50981b2f442979f1
  10:     0x7f218d9f15c8 - rustc_middle::ty::AdtDef::discriminant_def_for_variant::hb5d4a4e0979d34e0
  11:     0x7f218c6ca1e2 - rustc_middle::ty::sty::<impl rustc_middle::ty::TyS>::discriminant_for_variant::he55d7c41489848d7
  12:     0x7f218c7160c7 - rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::read_discriminant::h5f5c38f26e8b8ed8
  13:     0x7f218c79dbdd - rustc_mir::const_eval::destructure_const::h1f2cd6036022dfb8
  14:     0x7f218c32db46 - core::ops::function::FnOnce::call_once::h8c20fbb5bf3c1dd6
  15:     0x7f218d68c8ad - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::destructure_const>::compute::h3615046505938230
  16:     0x7f218d71cb09 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::ha13979abed354d04
  17:     0x7f218d88a1b6 - rustc_data_structures::stack::ensure_sufficient_stack::hb28e752bbf70f332
  18:     0x7f218d76be7a - rustc_query_system::query::plumbing::get_query_impl::h4795959616542474
  19:     0x7f218d663492 - rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_const::h42bd810d2dabec53
  20:     0x7f218d663db3 - rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_const::h42bd810d2dabec53
  21:     0x7f218d65472c - rustc_middle::mir::pretty_print_const::hd7cbc3a530cb0234
  22:     0x7f21882e1a3d - core::fmt::write::h65255e7874586eb2
  23:     0x7f21882e2db2 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h2cfc8f96fe0dc0bb
  24:     0x7f218d89f974 - <&T as core::fmt::Debug>::fmt::h61826e44f28f4f4a
  25:     0x7f21882e1a3d - core::fmt::write::h65255e7874586eb2
  26:     0x7f21882e2db2 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h2cfc8f96fe0dc0bb
  27:     0x7f218d651aa5 - <rustc_middle::mir::Operand as core::fmt::Debug>::fmt::hf2307fe080426e27
  28:     0x7f21882e1a3d - core::fmt::write::h65255e7874586eb2
  29:     0x7f21882e2db2 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h2cfc8f96fe0dc0bb
  30:     0x7f218d65223c - <rustc_middle::mir::Rvalue as core::fmt::Debug>::fmt::hd0f3169c5295fd5a
  31:     0x7f21882e1a3d - core::fmt::write::h65255e7874586eb2
  32:     0x7f21882e2db2 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h2cfc8f96fe0dc0bb
  33:     0x7f218d6513a8 - <rustc_middle::mir::Statement as core::fmt::Debug>::fmt::hc9cc86a23e746649
  34:     0x7f21882e19aa - core::fmt::write::h65255e7874586eb2
  35:     0x7f21882bf6ca - alloc::fmt::format::hc8afcfd973bb1a1f
  36:     0x7f218c366f71 - rustc_mir::util::pretty::write_mir_pretty::h93c9145185ddab0f
  37:     0x7f218c862c23 - rustc_mir::transform::dump_mir::emit_mir::h514eb69ae7eaa045
  38:     0x7f2189de7604 - rustc_interface::passes::start_codegen::h64d9c511ec596bff
  39:     0x7f2189e156d2 - rustc_middle::ty::context::tls::enter_global::hd5d9719ecc10b8ad
  40:     0x7f2189e0b77e - rustc_interface::queries::Queries::ongoing_codegen::h5a481c30c4739353
  41:     0x7f2189c37a0b - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h46a369817e38d2d4
  42:     0x7f2189ca17ca - rustc_span::with_source_map::ha1e1d15982d868c1
  43:     0x7f2189c39b49 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hfd5622ef3ce026be
  44:     0x7f2189c1334e - scoped_tls::ScopedKey<T>::set::hd3c1397285b10bd3
  45:     0x7f2189c3a576 - std::sys_common::backtrace::__rust_begin_short_backtrace::h3df4554cf04b61c9
  46:     0x7f2189c0717e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb0a109416f61afc2
  47:     0x7f2188279d7a - std::sys::unix::thread::Thread::new::thread_start::hb2effada6f868308
  48:     0x7f2187fa0422 - start_thread
  49:     0x7f21880bebf3 - __GI___clone
  50:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.45.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z mir-opt-level=1

query stack during panic:
#0 [destructure_const] destructure constant
end of query stack
