
stack backtrace:
   0:     0x7f5dfcb6c9a0 - std::backtrace_rs::backtrace::libunwind::trace::he615646ea344481f
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f5dfcb6c9a0 - std::backtrace_rs::backtrace::trace_unsynchronized::h6ea8eaac68705b9c
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f5dfcb6c9a0 - std::sys_common::backtrace::_print_fmt::h7ac486a935ce0bf7
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f5dfcb6c9a0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1b5a095d3db2e28f
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f5dfcbc898e - core::fmt::write::h445545b92224a1cd
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f5dfcb5cb15 - std::io::Write::write_fmt::h55a43474c6520b00
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/io/mod.rs:1682:15
   6:     0x7f5dfcb6c765 - std::sys_common::backtrace::_print::h65d20526fdb736b0
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f5dfcb6c765 - std::sys_common::backtrace::print::h6555fbe12a1cc41b
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f5dfcb6f56f - std::panicking::default_hook::{{closure}}::hbdf58083140e7ac6
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:267:22
   9:     0x7f5dfcb6f2aa - std::panicking::default_hook::haef8271c56b74d85
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:286:9
  10:     0x7f5dfcb6fd78 - std::panicking::rust_panic_with_hook::hfd45b6b6c12d9fa5
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:688:13
  11:     0x7f5dfcb6fad1 - std::panicking::begin_panic_handler::{{closure}}::hf591e8609a75bd4b
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:577:13
  12:     0x7f5dfcb6ce4c - std::sys_common::backtrace::__rust_end_short_backtrace::h81899558795e4ff7
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7f5dfcb6f832 - rust_begin_unwind
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:575:5
  14:     0x7f5dfcbc5373 - core::panicking::panic_fmt::h4235fa9b4675b332
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/panicking.rs:65:14
  15:     0x7f5dfe3ff094 - <rustc_middle[1bbcd7fc39987e50]::mir::interpret::allocation::ConstAllocation as rustc_serialize[a15cbf6ce3209308]::serialize::Decodable<rustc_query_impl[969f89a88b0ee9ca]::on_disk_cache::CacheDecoder>>::decode
  16:     0x7f5dfe3ff8f4 - <rustc_query_impl[969f89a88b0ee9ca]::on_disk_cache::CacheDecoder as rustc_type_ir[1fd57c2de0aaa4ec]::codec::TyDecoder>::decode_alloc_id
  17:     0x7f5dfe3ff67d - <(rustc_target[1afba1ed92600763]::abi::Size, rustc_middle[1bbcd7fc39987e50]::mir::interpret::AllocId) as rustc_serialize[a15cbf6ce3209308]::serialize::Decodable<rustc_query_impl[969f89a88b0ee9ca]::on_disk_cache::CacheDecoder>>::decode
  18:     0x7f5dfe3ff331 - <alloc[d987cf4402e5b40a]::vec::Vec<(rustc_target[1afba1ed92600763]::abi::Size, rustc_middle[1bbcd7fc39987e50]::mir::interpret::AllocId)> as rustc_serialize[a15cbf6ce3209308]::serialize::Decodable<rustc_query_impl[969f89a88b0ee9ca]::on_disk_cache::CacheDecoder>>::decode
  19:     0x7f5dfe3fee62 - <rustc_middle[1bbcd7fc39987e50]::mir::interpret::allocation::ConstAllocation as rustc_serialize[a15cbf6ce3209308]::serialize::Decodable<rustc_query_impl[969f89a88b0ee9ca]::on_disk_cache::CacheDecoder>>::decode
  20:     0x7f5dfe3ff8f4 - <rustc_query_impl[969f89a88b0ee9ca]::on_disk_cache::CacheDecoder as rustc_type_ir[1fd57c2de0aaa4ec]::codec::TyDecoder>::decode_alloc_id
  21:     0x7f5dfea42760 - <rustc_middle[1bbcd7fc39987e50]::mir::interpret::value::ConstValue as rustc_serialize[a15cbf6ce3209308]::serialize::Decodable<rustc_query_impl[969f89a88b0ee9ca]::on_disk_cache::CacheDecoder>>::decode
  22:     0x7f5dfea42441 - <core[b97a30f8df81432d]::result::Result<rustc_middle[1bbcd7fc39987e50]::mir::interpret::value::ConstValue, rustc_middle[1bbcd7fc39987e50]::mir::interpret::error::ErrorHandled> as rustc_serialize[a15cbf6ce3209308]::serialize::Decodable<rustc_query_impl[969f89a88b0ee9ca]::on_disk_cache::CacheDecoder>>::decode
  23:     0x7f5dfea42246 - <rustc_query_impl[969f89a88b0ee9ca]::on_disk_cache::OnDiskCache>::try_load_query_result::<core[b97a30f8df81432d]::result::Result<rustc_middle[1bbcd7fc39987e50]::mir::interpret::value::ConstValue, rustc_middle[1bbcd7fc39987e50]::mir::interpret::error::ErrorHandled>>
  24:     0x7f5dfea41db0 - rustc_query_impl[969f89a88b0ee9ca]::plumbing::try_load_from_disk::<core[b97a30f8df81432d]::result::Result<rustc_middle[1bbcd7fc39987e50]::mir::interpret::value::ConstValue, rustc_middle[1bbcd7fc39987e50]::mir::interpret::error::ErrorHandled>>
  25:     0x7f5dfea41b14 - rustc_query_system[330b635d327008f7]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[969f89a88b0ee9ca]::plumbing::QueryCtxt, rustc_middle[1bbcd7fc39987e50]::ty::ParamEnvAnd<rustc_middle[1bbcd7fc39987e50]::mir::interpret::GlobalId>, core[b97a30f8df81432d]::result::Result<rustc_middle[1bbcd7fc39987e50]::mir::interpret::value::ConstValue, rustc_middle[1bbcd7fc39987e50]::mir::interpret::error::ErrorHandled>>
  26:     0x7f5dfea3fd3f - rustc_query_system[330b635d327008f7]::query::plumbing::try_execute_query::<rustc_query_impl[969f89a88b0ee9ca]::plumbing::QueryCtxt, rustc_query_system[330b635d327008f7]::query::caches::DefaultCache<rustc_middle[1bbcd7fc39987e50]::ty::ParamEnvAnd<rustc_middle[1bbcd7fc39987e50]::mir::interpret::GlobalId>, core[b97a30f8df81432d]::result::Result<rustc_middle[1bbcd7fc39987e50]::mir::interpret::value::ConstValue, rustc_middle[1bbcd7fc39987e50]::mir::interpret::error::ErrorHandled>>>
  27:     0x7f5dff70e149 - <rustc_query_impl[969f89a88b0ee9ca]::Queries as rustc_middle[1bbcd7fc39987e50]::ty::query::QueryEngine>::eval_to_const_value_raw
  28:     0x7f5dfe8a0aa8 - <rustc_middle[1bbcd7fc39987e50]::ty::context::TyCtxt>::const_eval_resolve
  29:     0x7f5dfe8908a3 - rustc_monomorphize[a032594a6bc26d56]::collector::collect_neighbours
  30:     0x7f5dfe88a5c6 - rustc_monomorphize[a032594a6bc26d56]::collector::collect_items_rec
  31:     0x7f5dfee3941c - <rustc_session[87e5e219eb43521c]::session::Session>::time::<(), rustc_monomorphize[a032594a6bc26d56]::collector::collect_crate_mono_items::{closure#1}>
  32:     0x7f5dfee38f19 - rustc_monomorphize[a032594a6bc26d56]::collector::collect_crate_mono_items
  33:     0x7f5dfee38293 - rustc_monomorphize[a032594a6bc26d56]::partitioning::collect_and_partition_mono_items
  34:     0x7f5dff506aa7 - <rustc_query_system[330b635d327008f7]::dep_graph::graph::DepGraph<rustc_middle[1bbcd7fc39987e50]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[1bbcd7fc39987e50]::ty::context::TyCtxt, (), (&std[bbad73ae434e23e5]::collections::hash::set::HashSet<rustc_span[14998722174c1bca]::def_id::DefId, core[b97a30f8df81432d]::hash::BuildHasherDefault<rustc_hash[d2515752935e9cb7]::FxHasher>>, &[rustc_middle[1bbcd7fc39987e50]::mir::mono::CodegenUnit])>
  35:     0x7f5dff505c40 - rustc_query_system[330b635d327008f7]::query::plumbing::try_execute_query::<rustc_query_impl[969f89a88b0ee9ca]::plumbing::QueryCtxt, rustc_query_system[330b635d327008f7]::query::caches::DefaultCache<(), (&std[bbad73ae434e23e5]::collections::hash::set::HashSet<rustc_span[14998722174c1bca]::def_id::DefId, core[b97a30f8df81432d]::hash::BuildHasherDefault<rustc_hash[d2515752935e9cb7]::FxHasher>>, &[rustc_middle[1bbcd7fc39987e50]::mir::mono::CodegenUnit])>>
  36:     0x7f5dff50562b - rustc_query_system[330b635d327008f7]::query::plumbing::get_query::<rustc_query_impl[969f89a88b0ee9ca]::queries::collect_and_partition_mono_items, rustc_query_impl[969f89a88b0ee9ca]::plumbing::QueryCtxt>
  37:     0x7f5dff50556e - <rustc_query_impl[969f89a88b0ee9ca]::Queries as rustc_middle[1bbcd7fc39987e50]::ty::query::QueryEngine>::collect_and_partition_mono_items
  38:     0x7f5dff433e0a - rustc_codegen_ssa[737ec7dcbddbd4a0]::base::codegen_crate::<rustc_codegen_llvm[8e98f4afd2a755cb]::LlvmCodegenBackend>
  39:     0x7f5dff433bc2 - <rustc_codegen_llvm[8e98f4afd2a755cb]::LlvmCodegenBackend as rustc_codegen_ssa[737ec7dcbddbd4a0]::traits::backend::CodegenBackend>::codegen_crate
  40:     0x7f5dff0c8d69 - <rustc_session[87e5e219eb43521c]::session::Session>::time::<alloc[d987cf4402e5b40a]::boxed::Box<dyn core[b97a30f8df81432d]::any::Any>, rustc_interface[ee1a3f92e887e004]::passes::start_codegen::{closure#0}>
  41:     0x7f5dff0c86ba - <rustc_interface[ee1a3f92e887e004]::passes::QueryContext>::enter::<<rustc_interface[ee1a3f92e887e004]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b97a30f8df81432d]::result::Result<alloc[d987cf4402e5b40a]::boxed::Box<dyn core[b97a30f8df81432d]::any::Any>, rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  42:     0x7f5dff087ea3 - <rustc_interface[ee1a3f92e887e004]::queries::Queries>::ongoing_codegen
  43:     0x7f5dff086fb2 - <rustc_interface[ee1a3f92e887e004]::interface::Compiler>::enter::<rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}::{closure#2}, core[b97a30f8df81432d]::result::Result<core[b97a30f8df81432d]::option::Option<rustc_interface[ee1a3f92e887e004]::queries::Linker>, rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  44:     0x7f5dff07eac2 - rustc_span[14998722174c1bca]::with_source_map::<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  45:     0x7f5dff07e5b9 - <scoped_tls[23afcff80b89ba49]::ScopedKey<rustc_span[14998722174c1bca]::SessionGlobals>>::set::<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  46:     0x7f5dff07dbc8 - std[bbad73ae434e23e5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ee1a3f92e887e004]::util::run_in_thread_pool_with_globals<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>
  47:     0x7f5dff07d8ec - <<std[bbad73ae434e23e5]::thread::Builder>::spawn_unchecked_<rustc_interface[ee1a3f92e887e004]::util::run_in_thread_pool_with_globals<rustc_interface[ee1a3f92e887e004]::interface::run_compiler<core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>, rustc_driver[9c4183344b2d0066]::run_compiler::{closure#1}>::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b97a30f8df81432d]::result::Result<(), rustc_errors[21897ed46328f955]::ErrorGuaranteed>>::{closure#1} as core[b97a30f8df81432d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7f5e00a276a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4273f95ec44459b3
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/boxed.rs:1987:9
  49:     0x7f5e00a276a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h70f28fa4ddc269e5
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/alloc/src/boxed.rs:1987:9
  50:     0x7f5e00a276a3 - std::sys::unix::thread::Thread::new::thread_start::h85a9c16b988e2bd0
                               at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/sys/unix/thread.rs:108:17
  51:     0x7f5dfc88cded - start_thread
  52:     0x7f5dfc912370 - __GI___clone3
  53:                0x0 - <unknown>
