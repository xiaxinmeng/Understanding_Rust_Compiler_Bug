plain
failures:

---- [incremental] incremental/issue-49595/issue-49595.rs stdout ----

error in revision `cfail3`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-49595/issue-49595.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue-49595/issue-49595.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue-49595" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue-49595/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_borrowck/src/lib.rs:129:49
stack backtrace:
   0:     0x7f65ea04a5a0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h393229cb0065b886
   1:     0x7f65ea0b872f - core::fmt::write::h677214c1cd975f79
   2:     0x7f65ea037045 - std::io::Write::write_fmt::hb631c781a7adfa78
   3:     0x7f65ea04ead2 - std::panicking::default_hook::{{closure}}::h99f95d393ec6635c
   4:     0x7f65ea04e669 - std::panicking::default_hook::hf60417fd3dff6c86
   5:     0x7f65eaafe041 - rustc_driver[62bf47634bdbd9f7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f65ea04f3b3 - std::panicking::rust_panic_with_hook::h01fe1aea36d7a8c7
   7:     0x7f65ea04f207 - std::panicking::begin_panic_handler::{{closure}}::he30fd1f04d713a50
   8:     0x7f65ea04aa94 - std::sys_common::backtrace::__rust_end_short_backtrace::h833047b4414053a4
   9:     0x7f65ea04eee9 - rust_begin_unwind
  10:     0x7f65ea0018a3 - core::panicking::panic_fmt::hf71702dc2ea26b74
  11:     0x7f65eba27f0f - <rustc_data_structures[1e749ea147e33a28]::steal::Steal<rustc_middle[3933660198651f53]::mir::Body>>::borrow
  12:     0x7f65ebb1f068 - <rustc_infer[3d13a5b4410eed6a]::infer::InferCtxtBuilder>::enter::<rustc_middle[3933660198651f53]::mir::query::BorrowCheckResult, rustc_borrowck[d533252822d7ccfe]::mir_borrowck::{closure#0}>
  13:     0x7f65ebc4153e - rustc_borrowck[d533252822d7ccfe]::mir_borrowck
  14:     0x7f65ebc103fb - <rustc_borrowck[d533252822d7ccfe]::provide::{closure#0} as core[b4ba87579927ebc]::ops::function::FnOnce<(rustc_middle[3933660198651f53]::ty::context::TyCtxt, rustc_span[528afad43cd5e7c0]::def_id::LocalDefId)>>::call_once
  15:     0x7f65ec20ae88 - <rustc_middle[3933660198651f53]::dep_graph::dep_node::DepKind as rustc_query_system[7b59b85bcf5e986d]::dep_graph::DepKind>::with_deps::<rustc_query_system[7b59b85bcf5e986d]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_span[528afad43cd5e7c0]::def_id::LocalDefId, &rustc_middle[3933660198651f53]::mir::query::BorrowCheckResult>::{closure#1}, &rustc_middle[3933660198651f53]::mir::query::BorrowCheckResult>
  16:     0x7f65ec0b6986 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_span[528afad43cd5e7c0]::def_id::LocalDefId, &rustc_middle[3933660198651f53]::mir::query::BorrowCheckResult>
  17:     0x7f65ec067000 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::try_execute_query::<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_query_system[7b59b85bcf5e986d]::query::caches::DefaultCache<rustc_span[528afad43cd5e7c0]::def_id::LocalDefId, &rustc_middle[3933660198651f53]::mir::query::BorrowCheckResult>>
  18:     0x7f65ec12a033 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::get_query::<rustc_query_impl[edb3cc62f52f5dae]::queries::mir_borrowck, rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt>
  19:     0x7f65ed1bc2a4 - <rustc_middle[3933660198651f53]::ty::context::TyCtxt>::mir_borrowck_opt_const_arg
  20:     0x7f65ebcc71fa - rustc_const_eval[f579435e1602e3dc]::const_eval::eval_queries::eval_to_allocation_raw_provider
  21:     0x7f65ec202f7e - <rustc_middle[3933660198651f53]::dep_graph::dep_node::DepKind as rustc_query_system[7b59b85bcf5e986d]::dep_graph::DepKind>::with_deps::<rustc_query_system[7b59b85bcf5e986d]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_middle[3933660198651f53]::ty::ParamEnvAnd<rustc_middle[3933660198651f53]::mir::interpret::GlobalId>, core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>>::{closure#1}, core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>>
  22:     0x7f65ec0a9b51 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_middle[3933660198651f53]::ty::ParamEnvAnd<rustc_middle[3933660198651f53]::mir::interpret::GlobalId>, core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>>
  23:     0x7f65ec2c656c - rustc_data_structures[1e749ea147e33a28]::stack::ensure_sufficient_stack::<core[b4ba87579927ebc]::option::Option<(core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>, rustc_query_system[7b59b85bcf5e986d]::dep_graph::graph::DepNodeIndex)>, rustc_query_system[7b59b85bcf5e986d]::query::plumbing::execute_job<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_middle[3933660198651f53]::ty::ParamEnvAnd<rustc_middle[3933660198651f53]::mir::interpret::GlobalId>, core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>>::{closure#2}>
  24:     0x7f65ec15c805 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::get_query::<rustc_query_impl[edb3cc62f52f5dae]::queries::eval_to_allocation_raw, rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt>
  25:     0x7f65ec297296 - <rustc_query_impl[edb3cc62f52f5dae]::Queries as rustc_middle[3933660198651f53]::ty::query::QueryEngine>::eval_to_allocation_raw
  26:     0x7f65ebcc6e4c - rustc_const_eval[f579435e1602e3dc]::const_eval::eval_queries::eval_to_allocation_raw_provider
  27:     0x7f65ec202f7e - <rustc_middle[3933660198651f53]::dep_graph::dep_node::DepKind as rustc_query_system[7b59b85bcf5e986d]::dep_graph::DepKind>::with_deps::<rustc_query_system[7b59b85bcf5e986d]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_middle[3933660198651f53]::ty::ParamEnvAnd<rustc_middle[3933660198651f53]::mir::interpret::GlobalId>, core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>>::{closure#1}, core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>>
  28:     0x7f65ec0a9b51 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_middle[3933660198651f53]::ty::ParamEnvAnd<rustc_middle[3933660198651f53]::mir::interpret::GlobalId>, core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>>
  29:     0x7f65ec2c656c - rustc_data_structures[1e749ea147e33a28]::stack::ensure_sufficient_stack::<core[b4ba87579927ebc]::option::Option<(core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>, rustc_query_system[7b59b85bcf5e986d]::dep_graph::graph::DepNodeIndex)>, rustc_query_system[7b59b85bcf5e986d]::query::plumbing::execute_job<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_middle[3933660198651f53]::ty::ParamEnvAnd<rustc_middle[3933660198651f53]::mir::interpret::GlobalId>, core[b4ba87579927ebc]::result::Result<rustc_middle[3933660198651f53]::mir::interpret::value::ConstAlloc, rustc_middle[3933660198651f53]::mir::interpret::error::ErrorHandled>>::{closure#2}>
  30:     0x7f65ec15c805 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::get_query::<rustc_query_impl[edb3cc62f52f5dae]::queries::eval_to_allocation_raw, rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt>
  31:     0x7f65ec297296 - <rustc_query_impl[edb3cc62f52f5dae]::Queries as rustc_middle[3933660198651f53]::ty::query::QueryEngine>::eval_to_allocation_raw
  32:     0x7f65eb0a95ad - <rustc_const_eval[f579435e1602e3dc]::interpret::eval_context::InterpCx<rustc_mir_transform[da0cd0f5dc3bacf0]::const_prop::ConstPropMachine>>::mir_const_to_op
  33:     0x7f65eb0365db - <rustc_mir_transform[da0cd0f5dc3bacf0]::const_prop::ConstPropagator>::eval_constant
  34:     0x7f65eb038c2b - <rustc_mir_transform[da0cd0f5dc3bacf0]::const_prop::ConstPropagator as rustc_middle[3933660198651f53]::mir::visit::MutVisitor>::visit_constant
  35:     0x7f65eb03b231 - <rustc_mir_transform[da0cd0f5dc3bacf0]::const_prop::ConstPropagator as rustc_middle[3933660198651f53]::mir::visit::MutVisitor>::visit_statement
  36:     0x7f65eb038aa9 - <rustc_mir_transform[da0cd0f5dc3bacf0]::const_prop::ConstPropagator as rustc_middle[3933660198651f53]::mir::visit::MutVisitor>::visit_body
  37:     0x7f65eb0340e4 - <rustc_mir_transform[da0cd0f5dc3bacf0]::const_prop::ConstProp as rustc_middle[3933660198651f53]::mir::MirPass>::run_pass
  38:     0x7f65eb0eb025 - rustc_mir_transform[da0cd0f5dc3bacf0]::pass_manager::run_passes
  39:     0x7f65eb09265a - rustc_mir_transform[da0cd0f5dc3bacf0]::optimized_mir
  40:     0x7f65ec1fc141 - <rustc_middle[3933660198651f53]::dep_graph::dep_node::DepKind as rustc_query_system[7b59b85bcf5e986d]::dep_graph::DepKind>::with_deps::<<rustc_query_system[7b59b85bcf5e986d]::dep_graph::graph::DepGraph<rustc_middle[3933660198651f53]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[3933660198651f53]::ty::context::TyCtxt, rustc_span[528afad43cd5e7c0]::def_id::DefId, &rustc_middle[3933660198651f53]::mir::Body>::{closure#0}, &rustc_middle[3933660198651f53]::mir::Body>
  41:     0x7f65ec3868b0 - <rustc_query_system[7b59b85bcf5e986d]::dep_graph::graph::DepGraph<rustc_middle[3933660198651f53]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[3933660198651f53]::ty::context::TyCtxt, rustc_span[528afad43cd5e7c0]::def_id::DefId, &rustc_middle[3933660198651f53]::mir::Body>
  42:     0x7f65ec2dfc4f - rustc_data_structures[1e749ea147e33a28]::stack::ensure_sufficient_stack::<(&rustc_middle[3933660198651f53]::mir::Body, rustc_query_system[7b59b85bcf5e986d]::dep_graph::graph::DepNodeIndex), rustc_query_system[7b59b85bcf5e986d]::query::plumbing::execute_job<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_span[528afad43cd5e7c0]::def_id::DefId, &rustc_middle[3933660198651f53]::mir::Body>::{closure#3}>
  43:     0x7f65ec084435 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::try_execute_query::<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_query_system[7b59b85bcf5e986d]::query::caches::DefaultCache<rustc_span[528afad43cd5e7c0]::def_id::DefId, &rustc_middle[3933660198651f53]::mir::Body>>
  44:     0x7f65ec12dd2c - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::get_query::<rustc_query_impl[edb3cc62f52f5dae]::queries::optimized_mir, rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt>
  45:     0x7f65ed1d0405 - <rustc_middle[3933660198651f53]::ty::context::TyCtxt>::instance_mir
  46:     0x7f65eaf0d401 - rustc_monomorphize[39897594b11b5b2f]::collector::collect_neighbours
  47:     0x7f65eaf12506 - rustc_data_structures[1e749ea147e33a28]::stack::ensure_sufficient_stack::<(), rustc_monomorphize[39897594b11b5b2f]::collector::collect_items_rec::{closure#0}>
  48:     0x7f65eaf0449e - rustc_monomorphize[39897594b11b5b2f]::collector::collect_items_rec
  49:     0x7f65eaf04777 - rustc_monomorphize[39897594b11b5b2f]::collector::collect_items_rec
  50:     0x7f65eaf04777 - rustc_monomorphize[39897594b11b5b2f]::collector::collect_items_rec
  51:     0x7f65eaf1e5cf - <rustc_session[1355c1b6e1ecdbff]::session::Session>::time::<(), rustc_monomorphize[39897594b11b5b2f]::collector::collect_crate_mono_items::{closure#1}>
  52:     0x7f65eaf02d8f - rustc_monomorphize[39897594b11b5b2f]::collector::collect_crate_mono_items
  53:     0x7f65eaf3ce1f - rustc_monomorphize[39897594b11b5b2f]::partitioning::collect_and_partition_mono_items
  54:     0x7f65ec2026f4 - <rustc_middle[3933660198651f53]::dep_graph::dep_node::DepKind as rustc_query_system[7b59b85bcf5e986d]::dep_graph::DepKind>::with_deps::<<rustc_query_system[7b59b85bcf5e986d]::dep_graph::graph::DepGraph<rustc_middle[3933660198651f53]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[3933660198651f53]::ty::context::TyCtxt, (), (&std[2d571fadfbb17085]::collections::hash::set::HashSet<rustc_span[528afad43cd5e7c0]::def_id::DefId, core[b4ba87579927ebc]::hash::BuildHasherDefault<rustc_hash[db19c81e730011c1]::FxHasher>>, &[rustc_middle[3933660198651f53]::mir::mono::CodegenUnit])>::{closure#0}, (&std[2d571fadfbb17085]::collections::hash::set::HashSet<rustc_span[528afad43cd5e7c0]::def_id::DefId, core[b4ba87579927ebc]::hash::BuildHasherDefault<rustc_hash[db19c81e730011c1]::FxHasher>>, &[rustc_middle[3933660198651f53]::mir::mono::CodegenUnit])>
  55:     0x7f65ec3b5836 - <rustc_query_system[7b59b85bcf5e986d]::dep_graph::graph::DepGraph<rustc_middle[3933660198651f53]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[3933660198651f53]::ty::context::TyCtxt, (), (&std[2d571fadfbb17085]::collections::hash::set::HashSet<rustc_span[528afad43cd5e7c0]::def_id::DefId, core[b4ba87579927ebc]::hash::BuildHasherDefault<rustc_hash[db19c81e730011c1]::FxHasher>>, &[rustc_middle[3933660198651f53]::mir::mono::CodegenUnit])>
  56:     0x7f65ec2e3a67 - rustc_data_structures[1e749ea147e33a28]::stack::ensure_sufficient_stack::<((&std[2d571fadfbb17085]::collections::hash::set::HashSet<rustc_span[528afad43cd5e7c0]::def_id::DefId, core[b4ba87579927ebc]::hash::BuildHasherDefault<rustc_hash[db19c81e730011c1]::FxHasher>>, &[rustc_middle[3933660198651f53]::mir::mono::CodegenUnit]), rustc_query_system[7b59b85bcf5e986d]::dep_graph::graph::DepNodeIndex), rustc_query_system[7b59b85bcf5e986d]::query::plumbing::execute_job<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, (), (&std[2d571fadfbb17085]::collections::hash::set::HashSet<rustc_span[528afad43cd5e7c0]::def_id::DefId, core[b4ba87579927ebc]::hash::BuildHasherDefault<rustc_hash[db19c81e730011c1]::FxHasher>>, &[rustc_middle[3933660198651f53]::mir::mono::CodegenUnit])>::{closure#3}>
  57:     0x7f65ec0a7da0 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::try_execute_query::<rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt, rustc_query_system[7b59b85bcf5e986d]::query::caches::DefaultCache<(), (&std[2d571fadfbb17085]::collections::hash::set::HashSet<rustc_span[528afad43cd5e7c0]::def_id::DefId, core[b4ba87579927ebc]::hash::BuildHasherDefault<rustc_hash[db19c81e730011c1]::FxHasher>>, &[rustc_middle[3933660198651f53]::mir::mono::CodegenUnit])>>
  58:     0x7f65ec183065 - rustc_query_system[7b59b85bcf5e986d]::query::plumbing::get_query::<rustc_query_impl[edb3cc62f52f5dae]::queries::collect_and_partition_mono_items, rustc_query_impl[edb3cc62f52f5dae]::plumbing::QueryCtxt>
  59:     0x7f65ec29809e - <rustc_query_impl[edb3cc62f52f5dae]::Queries as rustc_middle[3933660198651f53]::ty::query::QueryEngine>::collect_and_partition_mono_items
  60:     0x7f65ead48692 - rustc_codegen_ssa[5fcf3d34189aed3]::base::codegen_crate::<rustc_codegen_llvm[216c041f4ac6a572]::LlvmCodegenBackend>
  61:     0x7f65eadf90d9 - <rustc_codegen_llvm[216c041f4ac6a572]::LlvmCodegenBackend as rustc_codegen_ssa[5fcf3d34189aed3]::traits::backend::CodegenBackend>::codegen_crate
  62:     0x7f65eac55261 - <rustc_session[1355c1b6e1ecdbff]::session::Session>::time::<alloc[ec20dd9d2ef7ab9d]::boxed::Box<dyn core[b4ba87579927ebc]::any::Any>, rustc_interface[66dcfa2d883a7a4b]::passes::start_codegen::{closure#0}>
  63:     0x7f65ead0d5fc - <rustc_interface[66dcfa2d883a7a4b]::passes::QueryContext>::enter::<<rustc_interface[66dcfa2d883a7a4b]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b4ba87579927ebc]::result::Result<alloc[ec20dd9d2ef7ab9d]::boxed::Box<dyn core[b4ba87579927ebc]::any::Any>, rustc_errors[e80c27fc0d458f96]::ErrorReported>>
  64:     0x7f65eac9fa78 - <rustc_interface[66dcfa2d883a7a4b]::queries::Queries>::ongoing_codegen
  65:     0x7f65eab2b97b - <rustc_interface[66dcfa2d883a7a4b]::interface::Compiler>::enter::<rustc_driver[62bf47634bdbd9f7]::run_compiler::{closure#1}::{closure#2}, core[b4ba87579927ebc]::result::Result<core[b4ba87579927ebc]::option::Option<rustc_interface[66dcfa2d883a7a4b]::queries::Linker>, rustc_errors[e80c27fc0d458f96]::ErrorReported>>
  66:     0x7f65eaae08db - rustc_span[528afad43cd5e7c0]::with_source_map::<core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>, rustc_interface[66dcfa2d883a7a4b]::interface::create_compiler_and_run<core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>, rustc_driver[62bf47634bdbd9f7]::run_compiler::{closure#1}>::{closure#1}>
  67:     0x7f65eab2a1dd - rustc_interface[66dcfa2d883a7a4b]::interface::create_compiler_and_run::<core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>, rustc_driver[62bf47634bdbd9f7]::run_compiler::{closure#1}>
  68:     0x7f65eab68cee - <scoped_tls[a5bdfc34659b1e41]::ScopedKey<rustc_span[528afad43cd5e7c0]::SessionGlobals>>::set::<rustc_interface[66dcfa2d883a7a4b]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[66dcfa2d883a7a4b]::interface::run_compiler<core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>, rustc_driver[62bf47634bdbd9f7]::run_compiler::{closure#1}>::{closure#0}, core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>>::{closure#0}::{closure#0}, core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>>
  69:     0x7f65eab31fb5 - std[2d571fadfbb17085]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[66dcfa2d883a7a4b]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[66dcfa2d883a7a4b]::interface::run_compiler<core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>, rustc_driver[62bf47634bdbd9f7]::run_compiler::{closure#1}>::{closure#0}, core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>>::{closure#0}, core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>>
  70:     0x7f65eab6c1e1 - std[2d571fadfbb17085]::panicking::try::<core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>, core[b4ba87579927ebc]::panic::unwind_safe::AssertUnwindSafe<<std[2d571fadfbb17085]::thread::Builder>::spawn_unchecked_<rustc_interface[66dcfa2d883a7a4b]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[66dcfa2d883a7a4b]::interface::run_compiler<core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>, rustc_driver[62bf47634bdbd9f7]::run_compiler::{closure#1}>::{closure#0}, core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>>::{closure#0}, core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>>::{closure#1}::{closure#0}>>
  71:     0x7f65eab27c4a - <<std[2d571fadfbb17085]::thread::Builder>::spawn_unchecked_<rustc_interface[66dcfa2d883a7a4b]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[66dcfa2d883a7a4b]::interface::run_compiler<core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>, rustc_driver[62bf47634bdbd9f7]::run_compiler::{closure#1}>::{closure#0}, core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>>::{closure#0}, core[b4ba87579927ebc]::result::Result<(), rustc_errors[e80c27fc0d458f96]::ErrorReported>>::{closure#1} as core[b4ba87579927ebc]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  72:     0x7f65ea05d2d3 - std::sys::unix::thread::Thread::new::thread_start::h350d1436c6d3304b
  73:     0x7f65e43cc609 - start_thread
  74:     0x7f65e9ec3293 - clone
  75:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (ccb4dbfc8 2022-02-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `lit_test::lit_test`
#1 [eval_to_allocation_raw] const-evaluating + checking `lit_test::lit_test::promoted[1]`
#2 [eval_to_allocation_raw] const-evaluating + checking `lit_test::lit_test::promoted[1]`
#3 [optimized_mir] optimizing MIR for `lit_test::lit_test`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items

------------------------------------------


