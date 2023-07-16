
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\compiler\rustc_hir\src\definitions.rs:452:14
stack backtrace:
   0:     0x7fffc7f392cf - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h53606b08c01ef688
   1:     0x7fffc7f6306a - core::fmt::write::he97f21b785b90134
   2:     0x7fffc7f2b448 - <std::io::IoSlice as core::fmt::Debug>::fmt::h967100caef347f28
   3:     0x7fffc7f3cd96 - std::panicking::take_hook::hb46205f0055becfd
   4:     0x7fffc7f3c797 - std::panicking::take_hook::hb46205f0055becfd
   5:     0x7ff6bc51c12e - <unknown>
   6:     0x7fffc7f3d6a9 - std::panicking::rust_panic_with_hook::hce0846b912adb5c5
   7:     0x7fffc7f3d11f - rust_begin_unwind
   8:     0x7fffc7f39bf7 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h53606b08c01ef688
   9:     0x7fffc7f3d0a9 - rust_begin_unwind
  10:     0x7fffc7f99510 - core::panicking::panic_fmt::h58715deff5969a45
  11:     0x7fffc7f9945c - core::panicking::panic::hc36befec1b90302c
  12:     0x7fffa5edb97e - <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id::h9f7b03ae96e81f81
  13:     0x7fffa6beeeb4 - rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::LocalDefId>::recover::he3ec8272ca40aa05
  14:     0x7fffa5e01dad - rustc_query_impl::query_callbacks::hir_owner::force_from_dep_node::h913d78c1ee69fb04
  15:     0x7fffa5e5cb34 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h79c2867594abd9a1
  16:     0x7fffa5e5cb0b - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h79c2867594abd9a1
  17:     0x7fffa5e5cb0b - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h79c2867594abd9a1
  18:     0x7fffa5e3497d - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h79c2867594abd9a1
  19:     0x7fffa5b1b81e - <rustc_mir_dataflow::framework::cursor::CursorPosition as core::fmt::Debug>::fmt::hdad43e681eb5eae7
  20:     0x7fffa5e22950 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h79c2867594abd9a1
  21:     0x7fffa50f764f - <<dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path::SubstsForAstPathCtxt as rustc_typeck::astconv::CreateSubstsForGenericArgsCtxt>::inferred_kind::h3effd8703dda5339
  22:     0x7fffa5053391 - <rustc_typeck::check::PlaceOp as core::fmt::Debug>::fmt::h2e3c14b4c6653d68
  23:     0x7fffa500da7f - rustc_typeck::check_crate::h07f2a7631cf584be
  24:     0x7fffa27c3e8d - rustc_interface::passes::analysis::hc0b219c9fceabc88
  25:     0x7fffa5ddb8eb - ZN16rustc_query_impl13on_disk_cache260_$LT$impl$u20$rustc_serialize..serialize..Decodable$LT$rustc_query_impl..on_disk_cache..CacheDecoder$GT$$u20$for$u20$$RF$std..collections..hash..set..HashSet$LT$rustc_span..def_id..LocalDefId$C$core..hash..BuildHasher
  26:     0x7fffa5e6c61d - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h79c2867594abd9a1
  27:     0x7fffa5db2972 - <rustc_span::def_id::LocalDefId as rustc_query_impl::profiling_support::SpecIntoSelfProfilingString>::spec_to_self_profile_string::h641e5a6d4a18de66
  28:     0x7fffa5b54849 - <rustc_mir_dataflow::framework::cursor::CursorPosition as core::fmt::Debug>::fmt::hdad43e681eb5eae7
  29:     0x7fffa5e1e095 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h79c2867594abd9a1
  30:     0x7fffa26ae59a - <tracing_subscriber::util::TryInitError as core::fmt::Display>::fmt::h8e44660347c50da5
  31:     0x7fffa2678944 - rustc_driver::pretty::print_after_hir_lowering::h16ae7f3f7d316762
  32:     0x7fffa26b07e6 - <tracing_subscriber::util::TryInitError as core::fmt::Display>::fmt::h8e44660347c50da5
  34:     0x7fffa26d9a58 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h11014d06e9af86ea
  35:     0x7fffc7f49d8c - std::sys::windows::thread::Thread::new::h4046847bccf80b57
  36:     0x7ff8261b7034 - BaseThreadInitThunk
  37:     0x7ff827982651 - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.57 (f1edd042 2021-11-29)

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
