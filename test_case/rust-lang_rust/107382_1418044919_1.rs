
error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:198:90: Failed to normalize <&[u8] as rkyv::Archive>::Resolver, maybe try to call `try_normalize_erasing_regions` instead

thread 'rustc' panicked at 'Box<dyn Any>', /private/tmp/nix-build-rustc-1.67.0.drv-0/rustc-1.67.0-src/compiler/rustc_errors/src/lib.rs:1576:9
stack backtrace:
   0:        0x10153b16c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5bd648d504842b6c
   1:        0x1015b7348 - core::fmt::write::h8a30860ecea56bd6
   2:        0x10155f880 - std::io::Write::write_fmt::h274353ec696abd45
   3:        0x10153af7c - std::sys_common::backtrace::print::hae9327e0193c2484
   4:        0x101530234 - std::panicking::default_hook::{{closure}}::h6c2d4b88437caf03
   5:        0x10152ffcc - std::panicking::default_hook::heb240acabf7815b4
   6:        0x1055df804 - rustc_driver[b35e9b6b737ab360]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x101530884 - std::panicking::rust_panic_with_hook::hfdebfa07438c439a
   8:        0x107371fec - std[d9acd6070f73bcf6]::panicking::begin_panic::<rustc_errors[491d78d41d29fe12]::ExplicitBug>::{closure#0}
   9:        0x10737185c - std[d9acd6070f73bcf6]::sys_common::backtrace::__rust_end_short_backtrace::<std[d9acd6070f73bcf6]::panicking::begin_panic<rustc_errors[491d78d41d29fe12]::ExplicitBug>::{closure#0}, !>
  10:        0x10789d4e8 - std[d9acd6070f73bcf6]::panicking::begin_panic::<rustc_errors[491d78d41d29fe12]::ExplicitBug>
  11:        0x107371850 - std[d9acd6070f73bcf6]::panic::panic_any::<rustc_errors[491d78d41d29fe12]::ExplicitBug>
  12:        0x10736941c - <rustc_errors[491d78d41d29fe12]::HandlerInner>::bug::<&alloc[21e6e732353c4023]::string::String>
  13:        0x107368f6c - <rustc_errors[491d78d41d29fe12]::Handler>::bug::<&alloc[21e6e732353c4023]::string::String>
  14:        0x1073339b0 - rustc_middle[c22d6cdeb3c0aded]::ty::context::tls::with_context_opt::<rustc_middle[c22d6cdeb3c0aded]::ty::context::tls::with_opt<rustc_middle[c22d6cdeb3c0aded]::util::bug::opt_span_bug_fmt<rustc_span[80f08c829b755ffc]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:        0x107333a20 - rustc_middle[c22d6cdeb3c0aded]::util::bug::opt_span_bug_fmt::<rustc_span[80f08c829b755ffc]::span_encoding::Span>
  16:        0x10789986c - rustc_middle[c22d6cdeb3c0aded]::util::bug::bug_fmt
  17:        0x107383150 - <rustc_middle[c22d6cdeb3c0aded]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::normalize_generic_arg_after_erasing_regions
  18:        0x1073831f8 - <rustc_middle[c22d6cdeb3c0aded]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[c22d6cdeb3c0aded]::ty::fold::TypeFolder>::fold_ty
  19:        0x105c37f10 - <core[ba9993f3380e93da]::iter::adapters::map::Map<core[ba9993f3380e93da]::iter::adapters::enumerate::Enumerate<core[ba9993f3380e93da]::slice::iter::Iter<rustc_middle[c22d6cdeb3c0aded]::ty::FieldDef>>, <rustc_mir_dataflow[a5a75b4bdf613a7b]::elaborate_drops::DropCtxt<rustc_mir_transform[73631a7cd162c963]::elaborate_drops::Elaborator>>::move_paths_for_fields::{closure#0}> as core[ba9993f3380e93da]::iter::traits::iterator::Iterator>::fold::<(), core[ba9993f3380e93da]::iter::traits::iterator::Iterator::for_each::call<(rustc_middle[c22d6cdeb3c0aded]::mir::syntax::Place, core[ba9993f3380e93da]::option::Option<rustc_mir_dataflow[a5a75b4bdf613a7b]::move_paths::MovePathIndex>), <alloc[21e6e732353c4023]::vec::Vec<(rustc_middle[c22d6cdeb3c0aded]::mir::syntax::Place, core[ba9993f3380e93da]::option::Option<rustc_mir_dataflow[a5a75b4bdf613a7b]::move_paths::MovePathIndex>)>>::extend_trusted<core[ba9993f3380e93da]::iter::adapters::map::Map<core[ba9993f3380e93da]::iter::adapters::enumerate::Enumerate<core[ba9993f3380e93da]::slice::iter::Iter<rustc_middle[c22d6cdeb3c0aded]::ty::FieldDef>>, <rustc_mir_dataflow[a5a75b4bdf613a7b]::elaborate_drops::DropCtxt<rustc_mir_transform[73631a7cd162c963]::elaborate_drops::Elaborator>>::move_paths_for_fields::{closure#0}>>::{closure#0}>::{closure#0}>
  20:        0x105d1b480 - <alloc[21e6e732353c4023]::vec::Vec<(rustc_middle[c22d6cdeb3c0aded]::mir::syntax::Place, core[ba9993f3380e93da]::option::Option<rustc_mir_dataflow[a5a75b4bdf613a7b]::move_paths::MovePathIndex>)> as alloc[21e6e732353c4023]::vec::spec_from_iter::SpecFromIter<(rustc_middle[c22d6cdeb3c0aded]::mir::syntax::Place, core[ba9993f3380e93da]::option::Option<rustc_mir_dataflow[a5a75b4bdf613a7b]::move_paths::MovePathIndex>), core[ba9993f3380e93da]::iter::adapters::map::Map<core[ba9993f3380e93da]::iter::adapters::enumerate::Enumerate<core[ba9993f3380e93da]::slice::iter::Iter<rustc_middle[c22d6cdeb3c0aded]::ty::FieldDef>>, <rustc_mir_dataflow[a5a75b4bdf613a7b]::elaborate_drops::DropCtxt<rustc_mir_transform[73631a7cd162c963]::elaborate_drops::Elaborator>>::move_paths_for_fields::{closure#0}>>>::from_iter
  21:        0x105cfc26c - <rustc_mir_dataflow[a5a75b4bdf613a7b]::elaborate_drops::DropCtxt<rustc_mir_transform[73631a7cd162c963]::elaborate_drops::Elaborator>>::open_drop_for_adt
  22:        0x105cfbab0 - <rustc_mir_dataflow[a5a75b4bdf613a7b]::elaborate_drops::DropCtxt<rustc_mir_transform[73631a7cd162c963]::elaborate_drops::Elaborator>>::elaborate_drop
  23:        0x105c7b950 - <rustc_mir_transform[73631a7cd162c963]::elaborate_drops::ElaborateDrops as rustc_middle[c22d6cdeb3c0aded]::mir::MirPass>::run_pass
  24:        0x105bc1ce8 - rustc_mir_transform[73631a7cd162c963]::pass_manager::run_passes_inner
  25:        0x105c602d4 - rustc_mir_transform[73631a7cd162c963]::run_analysis_to_runtime_passes
  26:        0x105c5fc44 - rustc_mir_transform[73631a7cd162c963]::mir_drops_elaborated_and_const_checked
  27:        0x10679f540 - <rustc_middle[c22d6cdeb3c0aded]::dep_graph::dep_node::DepKind as rustc_query_system[78dadc92d2b94d9e]::dep_graph::DepKind>::with_deps::<<rustc_query_system[78dadc92d2b94d9e]::dep_graph::graph::DepGraph<rustc_middle[c22d6cdeb3c0aded]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[c22d6cdeb3c0aded]::ty::context::TyCtxt, rustc_middle[c22d6cdeb3c0aded]::ty::WithOptConstParam<rustc_span[80f08c829b755ffc]::def_id::LocalDefId>, &rustc_data_structures[95d03e94522b1bae]::steal::Steal<rustc_middle[c22d6cdeb3c0aded]::mir::Body>>::{closure#0}, &rustc_data_structures[95d03e94522b1bae]::steal::Steal<rustc_middle[c22d6cdeb3c0aded]::mir::Body>>
  28:        0x106704b14 - <rustc_query_system[78dadc92d2b94d9e]::dep_graph::graph::DepGraph<rustc_middle[c22d6cdeb3c0aded]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[c22d6cdeb3c0aded]::ty::context::TyCtxt, rustc_middle[c22d6cdeb3c0aded]::ty::WithOptConstParam<rustc_span[80f08c829b755ffc]::def_id::LocalDefId>, &rustc_data_structures[95d03e94522b1bae]::steal::Steal<rustc_middle[c22d6cdeb3c0aded]::mir::Body>>
  29:        0x10658c4d4 - rustc_query_system[78dadc92d2b94d9e]::query::plumbing::try_execute_query::<rustc_query_impl[20a35905410c6796]::plumbing::QueryCtxt, rustc_query_system[78dadc92d2b94d9e]::query::caches::DefaultCache<rustc_middle[c22d6cdeb3c0aded]::ty::WithOptConstParam<rustc_span[80f08c829b755ffc]::def_id::LocalDefId>, &rustc_data_structures[95d03e94522b1bae]::steal::Steal<rustc_middle[c22d6cdeb3c0aded]::mir::Body>>>
  30:        0x1066b1788 - rustc_query_system[78dadc92d2b94d9e]::query::plumbing::get_query::<rustc_query_impl[20a35905410c6796]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[20a35905410c6796]::plumbing::QueryCtxt>
  31:        0x106857b74 - <rustc_query_impl[20a35905410c6796]::Queries as rustc_middle[c22d6cdeb3c0aded]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  32:        0x105c60ef8 - rustc_mir_transform[73631a7cd162c963]::optimized_mir
  33:        0x10672279c - <rustc_query_system[78dadc92d2b94d9e]::dep_graph::graph::DepGraph<rustc_middle[c22d6cdeb3c0aded]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[c22d6cdeb3c0aded]::ty::context::TyCtxt, rustc_span[80f08c829b755ffc]::def_id::DefId, &rustc_middle[c22d6cdeb3c0aded]::mir::Body>
  34:        0x1065c8610 - rustc_query_system[78dadc92d2b94d9e]::query::plumbing::try_execute_query::<rustc_query_impl[20a35905410c6796]::plumbing::QueryCtxt, rustc_query_system[78dadc92d2b94d9e]::query::caches::DefaultCache<rustc_span[80f08c829b755ffc]::def_id::DefId, &rustc_middle[c22d6cdeb3c0aded]::mir::Body>>
  35:        0x106682794 - rustc_query_system[78dadc92d2b94d9e]::query::plumbing::get_query::<rustc_query_impl[20a35905410c6796]::queries::optimized_mir, rustc_query_impl[20a35905410c6796]::plumbing::QueryCtxt>
  36:        0x106c25d7c - <rustc_metadata[bc087994ee38342d]::rmeta::encoder::EncodeContext>::encode_crate_root
  37:        0x106c3cea0 - rustc_metadata[bc087994ee38342d]::rmeta::encoder::encode_metadata_impl
  38:        0x106bb9c00 - rustc_data_structures[95d03e94522b1bae]::sync::join::<rustc_metadata[bc087994ee38342d]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[bc087994ee38342d]::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
  39:        0x106c3c71c - rustc_metadata[bc087994ee38342d]::rmeta::encoder::encode_metadata
  40:        0x106c18088 - rustc_metadata[bc087994ee38342d]::fs::encode_and_write_metadata
  41:        0x1056db174 - rustc_interface[ebcbd89a6b84ecc5]::passes::start_codegen
  42:        0x1056da0cc - <rustc_interface[ebcbd89a6b84ecc5]::passes::QueryContext>::enter::<<rustc_interface[ebcbd89a6b84ecc5]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ba9993f3380e93da]::result::Result<alloc[21e6e732353c4023]::boxed::Box<dyn core[ba9993f3380e93da]::any::Any>, rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>>
  43:        0x10569a2a0 - <rustc_interface[ebcbd89a6b84ecc5]::queries::Queries>::ongoing_codegen
  44:        0x1055b1b90 - rustc_span[80f08c829b755ffc]::with_source_map::<core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>, rustc_interface[ebcbd89a6b84ecc5]::interface::run_compiler<core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>, rustc_driver[b35e9b6b737ab360]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  45:        0x1055aa968 - <scoped_tls[8ddbc68b9cbb90b1]::ScopedKey<rustc_span[80f08c829b755ffc]::SessionGlobals>>::set::<rustc_interface[ebcbd89a6b84ecc5]::interface::run_compiler<core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>, rustc_driver[b35e9b6b737ab360]::run_compiler::{closure#1}>::{closure#0}, core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>>
  46:        0x1055f584c - std[d9acd6070f73bcf6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ebcbd89a6b84ecc5]::util::run_in_thread_pool_with_globals<rustc_interface[ebcbd89a6b84ecc5]::interface::run_compiler<core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>, rustc_driver[b35e9b6b737ab360]::run_compiler::{closure#1}>::{closure#0}, core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>>
  47:        0x1055e4600 - <<std[d9acd6070f73bcf6]::thread::Builder>::spawn_unchecked_<rustc_interface[ebcbd89a6b84ecc5]::util::run_in_thread_pool_with_globals<rustc_interface[ebcbd89a6b84ecc5]::interface::run_compiler<core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>, rustc_driver[b35e9b6b737ab360]::run_compiler::{closure#1}>::{closure#0}, core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ba9993f3380e93da]::result::Result<(), rustc_errors[491d78d41d29fe12]::ErrorGuaranteed>>::{closure#1} as core[ba9993f3380e93da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:        0x101559f94 - std::sys::unix::thread::Thread::new::thread_start::hcc1c30cde6d430b4
  49:        0x182c8206c - __pthread_deallocate

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0 (fc594f156 2023-01-24) (built from a source tarball) running on aarch64-apple-darwin

note: compiler flags: --crate-type lib -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `_::<impl at src/lib.rs:3:10: 3:17>::resolve`
#1 [optimized_mir] optimizing MIR for `_::<impl at src/lib.rs:3:10: 3:17>::resolve`
end of query stack
error: could not compile `ice-repro`
