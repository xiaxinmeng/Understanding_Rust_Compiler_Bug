
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/00d5e42e776da900049fe19087bc9b0057ec70cd\compiler\rustc_hir\src\definitions.rs:452:14
stack backtrace:
   0:     0x7fff784a8c0f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4287336b592e4e30
   1:     0x7fff784d28ca - core::fmt::write::h8a2c40ddb66ccc71
   2:     0x7fff7849b5b8 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::hd15f09e3fd3eac54
   3:     0x7fff784ac366 - std::panicking::take_hook::hf14c76592f73c762
   4:     0x7fff784abe4c - std::panicking::take_hook::hf14c76592f73c762
   5:     0x7fff5ce5a945 - <rustc_metadata[dfdfc7113336e1f8]::native_libs::Collector as rustc_hir[c1b1b26d2913040d]::itemlikevisit::ItemLikeVisitor>::visit_trait_item
   6:     0x7fff784acc79 - std::panicking::rust_panic_with_hook::hbc0e9c80ca88eac0
   7:     0x7fff784ac6ef - rust_begin_unwind
   8:     0x7fff784a9547 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4287336b592e4e30
   9:     0x7fff784ac679 - rust_begin_unwind
  10:     0x7fff78508de0 - core::panicking::panic_fmt::hc1b1ca620e7a2c9f
  11:     0x7fff78508d2c - core::panicking::panic::hfef5cee9afdef02a
  12:     0x7fff6131a89a - <rustc_middle[68438b5bfe988d4e]::ty::context::TyCtxt>::def_path_hash_to_def_id
  13:     0x7fff61418fce - <rustc_query_system[993d4a9a8633e257]::dep_graph::dep_node::DepNode<rustc_middle[68438b5bfe988d4e]::dep_graph::dep_node::DepKind> as rustc_middle[68438b5bfe988d4e]::dep_graph::dep_node::DepNodeExt>::extract_def_id
  14:     0x7fff606fc5e4 - rustc_query_impl[2492acdb0e5eb4f6]::query_callbacks::hir_owner
  15:     0x7fff61301773 - <rustc_middle[68438b5bfe988d4e]::ty::context::TyCtxt as rustc_query_system[993d4a9a8633e257]::dep_graph::DepContext>::try_force_from_dep_node
  16:     0x7fff6067dabb - <rustc_query_impl[2492acdb0e5eb4f6]::Queries as rustc_middle[68438b5bfe988d4e]::ty::query::QueryEngine>::try_mark_green
  17:     0x7fff6067da97 - <rustc_query_impl[2492acdb0e5eb4f6]::Queries as rustc_middle[68438b5bfe988d4e]::ty::query::QueryEngine>::try_mark_green
  18:     0x7fff6067da97 - <rustc_query_impl[2492acdb0e5eb4f6]::Queries as rustc_middle[68438b5bfe988d4e]::ty::query::QueryEngine>::try_mark_green
  19:     0x7fff6067da97 - <rustc_query_impl[2492acdb0e5eb4f6]::Queries as rustc_middle[68438b5bfe988d4e]::ty::query::QueryEngine>::try_mark_green
  20:     0x7fff60655dda - <rustc_query_impl[2492acdb0e5eb4f6]::Queries as rustc_middle[68438b5bfe988d4e]::ty::query::QueryEngine>::try_mark_green
  21:     0x7fff60370147 - <rustc_mir_dataflow[b218101e43c2c97d]::framework::cursor::CursorPosition as core[3b5640218fb446b]::fmt::Debug>::fmt
  22:     0x7fff6050cda5 - <rustc_mir_dataflow[b218101e43c2c97d]::framework::cursor::CursorPosition as core[3b5640218fb446b]::fmt::Debug>::fmt
  23:     0x7fff5f865760 - <rustc_typeck[2dede4d9058b9b75]::outlives::explicit::ExplicitPredicatesMap as core[3b5640218fb446b]::fmt::Debug>::fmt
  24:     0x7fff5f82f840 - rustc_typeck[2dede4d9058b9b75]::check_crate
  25:     0x7fff5cfb72ff - rustc_interface[ff1d7c07a753e82f]::passes::analysis
  26:     0x7fff60603c7b - <rustc_span[abfcba77a0dab5e7]::def_id::DefIndex as rustc_query_impl[2492acdb0e5eb4f6]::profiling_support::SpecIntoSelfProfilingString>::spec_to_self_profile_string
  27:     0x7fff606b142e - <rustc_query_impl[2492acdb0e5eb4f6]::Queries as rustc_middle[68438b5bfe988d4e]::ty::query::QueryEngine>::try_mark_green
  28:     0x7fff605de5f1 - <rustc_span[abfcba77a0dab5e7]::def_id::DefIndex as rustc_query_impl[2492acdb0e5eb4f6]::profiling_support::SpecIntoSelfProfilingString>::spec_to_self_profile_string
  29:     0x7fff60487c29 - <rustc_mir_dataflow[b218101e43c2c97d]::framework::cursor::CursorPosition as core[3b5640218fb446b]::fmt::Debug>::fmt
  30:     0x7fff60521e79 - <rustc_mir_dataflow[b218101e43c2c97d]::framework::cursor::CursorPosition as core[3b5640218fb446b]::fmt::Debug>::fmt
  31:     0x7fff5cebadc0 - <rustc_middle[68438b5bfe988d4e]::ty::SymbolName as core[3b5640218fb446b]::fmt::Display>::fmt
  32:     0x7fff5cebb85b - <rustc_middle[68438b5bfe988d4e]::ty::SymbolName as core[3b5640218fb446b]::fmt::Display>::fmt
  33:     0x7fff5ce746e6 - rustc_driver[af3c8dcfa508860b]::pretty::print_after_hir_lowering
  34:     0x7fff5ced8a76 - <rustc_middle[68438b5bfe988d4e]::ty::SymbolName as core[3b5640218fb446b]::fmt::Display>::fmt
  35:     0x7fff5ce7ddb7 - <rustc_middle[68438b5bfe988d4e]::ty::SymbolName as core[3b5640218fb446b]::fmt::Display>::fmt
  36:     0x7fff5ce7b00c - <tracing_subscriber[281ba6a53a80a207]::util::TryInitError as core[3b5640218fb446b]::fmt::Display>::fmt
  37:     0x7fff5cef6668 - <rustc_driver[af3c8dcfa508860b]::args::Error as core[3b5640218fb446b]::fmt::Debug>::fmt
  38:     0x7fff784b933c - std::sys::windows::thread::Thread::new::he49a5cdb1d3d1cfc
  39:     0x7fffd6db7034 - BaseThreadInitThunk
  40:     0x7fffd6f22651 - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (00d5e42e7 2021-10-24) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C linker=rust-lld.exe -C incremental --crate-type lib --crate-type cdylib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
