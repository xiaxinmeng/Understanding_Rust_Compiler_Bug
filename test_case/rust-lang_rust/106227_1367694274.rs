plain
   Compiling gimli v0.26.1
   Compiling regex v1.5.6
   Compiling regex-automata v0.1.10
   Compiling object v0.29.0
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_data_structures/src/graph/dominators/mod.rs:138:21
   0:     0x7f50838809c6 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hafd81a0b5d487c53
   0:     0x7f50838809c6 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hafd81a0b5d487c53
   1:     0x7f50838eefb8 - core::fmt::write::h4a778e35e4f38324
   2:     0x7f5083871f81 - std::io::Write::write_fmt::ha216e80cfbe2a850
   3:     0x7f5083880795 - std::sys_common::backtrace::print::h1b9ed32e93c301a6
   4:     0x7f5083883cd7 - std::panicking::default_hook::{{closure}}::h1690eea152a3b836
   5:     0x7f5083883a36 - std::panicking::default_hook::h5e9cc4e411668de1
   6:     0x7f5084252142 - rustc_driver[54a292329d61ee30]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5083884603 - std::panicking::rust_panic_with_hook::h0e85789db00c4c44
   8:     0x7f50838842f2 - std::panicking::begin_panic_handler::{{closure}}::h27551f22330a5e57
   9:     0x7f5083880f6c - std::sys_common::backtrace::__rust_end_short_backtrace::h0c44fb4de016d510
  11:     0x7f5083836123 - core::panicking::panic_fmt::hea4f8765df0a9f41
  11:     0x7f5083836123 - core::panicking::panic_fmt::hea4f8765df0a9f41
  12:     0x7f50838361fd - core::panicking::panic::h6c8c2e19c88621fe
  13:     0x7f5084c93669 - rustc_data_structures[c12f58c5cf5ee10e]::graph::dominators::dominators::<&&rustc_middle[3fee2c4d4e9f0dfe]::mir::basic_blocks::BasicBlocks>
  14:     0x7f5084d0ed54 - <&mut <rustc_mir_transform[95d13cbdfb44c867]::ctfe_limit::CtfeLimit as rustc_middle[3fee2c4d4e9f0dfe]::mir::MirPass>::run_pass::{closure#0} as core[cada5d954255392c]::ops::function::FnMut<((rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlock, &rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlockData),)>>::call_mut
  15:     0x7f5084c4c93f - <core[cada5d954255392c]::iter::adapters::map::Map<core[cada5d954255392c]::iter::adapters::enumerate::Enumerate<core[cada5d954255392c]::slice::iter::Iter<rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlockData>>, <rustc_index[3c4177419627218c]::vec::IndexVec<rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlock, rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlockData>>::iter_enumerated::{closure#0}> as core[cada5d954255392c]::iter::traits::iterator::Iterator>::try_fold::<(), core[cada5d954255392c]::iter::traits::iterator::Iterator::find_map::check<(rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlock, &rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlockData), rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlock, &mut <rustc_mir_transform[95d13cbdfb44c867]::ctfe_limit::CtfeLimit as rustc_middle[3fee2c4d4e9f0dfe]::mir::MirPass>::run_pass::{closure#0}>::{closure#0}, core[cada5d954255392c]::ops::control_flow::ControlFlow<rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlock>>
  16:     0x7f5084d3e9ab - <alloc[7b8a314fc46dcffe]::vec::Vec<rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlock> as alloc[7b8a314fc46dcffe]::vec::spec_from_iter::SpecFromIter<rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlock, core[cada5d954255392c]::iter::adapters::filter_map::FilterMap<core[cada5d954255392c]::iter::adapters::map::Map<core[cada5d954255392c]::iter::adapters::enumerate::Enumerate<core[cada5d954255392c]::slice::iter::Iter<rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlockData>>, <rustc_index[3c4177419627218c]::vec::IndexVec<rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlock, rustc_middle[3fee2c4d4e9f0dfe]::mir::BasicBlockData>>::iter_enumerated::{closure#0}>, <rustc_mir_transform[95d13cbdfb44c867]::ctfe_limit::CtfeLimit as rustc_middle[3fee2c4d4e9f0dfe]::mir::MirPass>::run_pass::{closure#0}>>>::from_iter
  17:     0x7f5084d2e459 - <rustc_mir_transform[95d13cbdfb44c867]::ctfe_limit::CtfeLimit as rustc_middle[3fee2c4d4e9f0dfe]::mir::MirPass>::run_pass
  18:     0x7f5084d9b144 - rustc_mir_transform[95d13cbdfb44c867]::pass_manager::run_passes_inner
  19:     0x7f5084c2c68e - rustc_mir_transform[95d13cbdfb44c867]::run_analysis_to_runtime_passes
  20:     0x7f5084c2bf0a - rustc_mir_transform[95d13cbdfb44c867]::mir_drops_elaborated_and_const_checked
  21:     0x7f5085eeba51 - rustc_query_system[4ff46d1e41d78cab]::query::plumbing::try_execute_query::<rustc_query_impl[bba56a74d3a8740d]::plumbing::QueryCtxt, rustc_query_system[4ff46d1e41d78cab]::query::caches::DefaultCache<rustc_middle[3fee2c4d4e9f0dfe]::ty::WithOptConstParam<rustc_span[4b119e7221467786]::def_id::LocalDefId>, &rustc_data_structures[c12f58c5cf5ee10e]::steal::Steal<rustc_middle[3fee2c4d4e9f0dfe]::mir::Body>>>
  22:     0x7f5086053bc4 - rustc_query_system[4ff46d1e41d78cab]::query::plumbing::get_query::<rustc_query_impl[bba56a74d3a8740d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[bba56a74d3a8740d]::plumbing::QueryCtxt, rustc_middle[3fee2c4d4e9f0dfe]::dep_graph::dep_node::DepKind>
  23:     0x7f5085b68f63 - <rustc_query_impl[bba56a74d3a8740d]::Queries as rustc_middle[3fee2c4d4e9f0dfe]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  24:     0x7f508438ee04 - <rustc_session[8fecdfce2d84b36e]::session::Session>::time::<(), rustc_interface[700f389ea52edc67]::passes::analysis::{closure#3}>
  25:     0x7f50843aa932 - rustc_interface[700f389ea52edc67]::passes::analysis
  26:     0x7f5085f2f69d - rustc_query_system[4ff46d1e41d78cab]::query::plumbing::try_execute_query::<rustc_query_impl[bba56a74d3a8740d]::plumbing::QueryCtxt, rustc_query_system[4ff46d1e41d78cab]::query::caches::DefaultCache<(), core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>>
  27:     0x7f50860585c9 - rustc_query_system[4ff46d1e41d78cab]::query::plumbing::get_query::<rustc_query_impl[bba56a74d3a8740d]::queries::analysis, rustc_query_impl[bba56a74d3a8740d]::plumbing::QueryCtxt, rustc_middle[3fee2c4d4e9f0dfe]::dep_graph::dep_node::DepKind>
  28:     0x7f5085b5f39a - <rustc_query_impl[bba56a74d3a8740d]::Queries as rustc_middle[3fee2c4d4e9f0dfe]::ty::query::QueryEngine>::analysis
  29:     0x7f50842ac014 - <rustc_interface[700f389ea52edc67]::passes::QueryContext>::enter::<rustc_driver[54a292329d61ee30]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>
  30:     0x7f50842b5a1a - <rustc_interface[700f389ea52edc67]::interface::Compiler>::enter::<rustc_driver[54a292329d61ee30]::run_compiler::{closure#1}::{closure#2}, core[cada5d954255392c]::result::Result<core[cada5d954255392c]::option::Option<rustc_interface[700f389ea52edc67]::queries::Linker>, rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>
  31:     0x7f50842537cb - rustc_span[4b119e7221467786]::with_source_map::<core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>, rustc_interface[700f389ea52edc67]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>, rustc_driver[54a292329d61ee30]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  32:     0x7f50842aaf8a - <scoped_tls[5706bf77a85fe418]::ScopedKey<rustc_span[4b119e7221467786]::SessionGlobals>>::set::<rustc_interface[700f389ea52edc67]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>, rustc_driver[54a292329d61ee30]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>
  33:     0x7f5084270fa0 - std[153e7eca4933144d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[700f389ea52edc67]::util::run_in_thread_pool_with_globals<rustc_interface[700f389ea52edc67]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>, rustc_driver[54a292329d61ee30]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>
  34:     0x7f50842b6486 - std[153e7eca4933144d]::panicking::try::<core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>, core[cada5d954255392c]::panic::unwind_safe::AssertUnwindSafe<<std[153e7eca4933144d]::thread::Builder>::spawn_unchecked_<rustc_interface[700f389ea52edc67]::util::run_in_thread_pool_with_globals<rustc_interface[700f389ea52edc67]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>, rustc_driver[54a292329d61ee30]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f50842639c4 - <<std[153e7eca4933144d]::thread::Builder>::spawn_unchecked_<rustc_interface[700f389ea52edc67]::util::run_in_thread_pool_with_globals<rustc_interface[700f389ea52edc67]::interface::run_compiler<core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>, rustc_driver[54a292329d61ee30]::run_compiler::{closure#1}>::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cada5d954255392c]::result::Result<(), rustc_errors[e6ef93a3d7f5133c]::ErrorGuaranteed>>::{closure#1} as core[cada5d954255392c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f5083891c3e - std::sys::unix::thread::Thread::new::thread_start::h5d9f8d6d1fd775b4
  37:     0x7f5083626b43 - <unknown>
  38:     0x7f50836b8a00 - <unknown>
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (33e84e27b 2022-12-30) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -Z binary-dep-depinfo -Z unstable-options -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `expr::<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.102/src/expr.rs:783:1: 783:10>::DUMMY`
#1 [analysis] running analysis passes on this crate
error: could not compile `syn`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:19:10
