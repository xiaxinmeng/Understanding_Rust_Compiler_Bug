plain
 finished in 0.626 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 154 tests
....................................F......F.F.F.F..F.....F............................. 88/154
failures:
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [incremental] src/test/incremental/hashes/closure_expressions.rs stdout ----
---- [incremental] src/test/incremental/hashes/closure_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zmir-opt-level=0" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_mir_transform/src/ffi_unwind_calls.rs:55:78
stack backtrace:
   0:     0x7f0544cdb97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d1b0ec045decce0
   1:     0x7f0544d41f88 - core::fmt::write::hd9d515704885fbc6
   2:     0x7f0544ccb801 - std::io::Write::write_fmt::hacfc552cb196af7c
   3:     0x7f0544cde9ce - std::panicking::default_hook::{{closure}}::h4f16d4a983fe151a
   4:     0x7f0544cde5fc - std::panicking::default_hook::h1c29e965239144a9
   5:     0x7f0545874fc1 - rustc_driver[eedaaef195cd2bca]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0544cdf22e - std::panicking::rust_panic_with_hook::h1fd6da4624751e3a
   7:     0x7f0544cdf027 - std::panicking::begin_panic_handler::{{closure}}::h3061509f74e25df9
   8:     0x7f0544cdbe94 - std::sys_common::backtrace::__rust_end_short_backtrace::h17b83daf94d218c7
   9:     0x7f0544cded09 - rust_begin_unwind
  10:     0x7f0544c93073 - core::panicking::panic_fmt::h6c25c0992172e8a7
  11:     0x7f0545d8880f - <rustc_data_structures[4b7b7fc83bfd84bf]::steal::Steal<rustc_middle[9d75a7afa4763910]::mir::Body>>::borrow
  12:     0x7f0545d086ff - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::has_ffi_unwind_calls
  13:     0x7f0546fcc60f - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>::{closure#1}, bool>
  14:     0x7f05472f9c30 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>
  15:     0x7f0547262864 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>>
  16:     0x7f0547346f34 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::has_ffi_unwind_calls, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  17:     0x7f05471cc694 - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::has_ffi_unwind_calls
  18:     0x7f0545d097f9 - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::required_panic_strategy
  19:     0x7f0546fc25c4 - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>::{closure#0}, core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  20:     0x7f05470c243b - <rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  21:     0x7f054729b771 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<(), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>>
  22:     0x7f054735e9c5 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::required_panic_strategy, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  23:     0x7f05471ccbee - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::required_panic_strategy
  24:     0x7f05477dc69b - <rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodeContext>::encode_crate_root
  25:     0x7f05477eed26 - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata_impl
  26:     0x7f05478b3d51 - rustc_data_structures[4b7b7fc83bfd84bf]::sync::join::<rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodedMetadata, ()>
  27:     0x7f05477ee1fe - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata
  28:     0x7f0545947c67 - <rustc_interface[153f877473f555b9]::passes::QueryContext>::enter::<<rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[70453ac7847f43d3]::result::Result<alloc[3bed6a74638318eb]::boxed::Box<dyn core[70453ac7847f43d3]::any::Any>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  29:     0x7f0545932f6e - <rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen
  30:     0x7f0545803680 - <rustc_interface[153f877473f555b9]::interface::Compiler>::enter::<rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}::{closure#2}, core[70453ac7847f43d3]::result::Result<core[70453ac7847f43d3]::option::Option<rustc_interface[153f877473f555b9]::queries::Linker>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  31:     0x7f05457e53cb - rustc_span[c893a8830d05c8cd]::with_source_map::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_interface[153f877473f555b9]::interface::create_compiler_and_run<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7f0545804829 - <scoped_tls[bef1a4b4d47eddc0]::ScopedKey<rustc_span[c893a8830d05c8cd]::SessionGlobals>>::set::<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  33:     0x7f05458602f9 - std[f41be0eaf76ae4e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  34:     0x7f0545818f21 - std[f41be0eaf76ae4e1]::panicking::try::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, core[70453ac7847f43d3]::panic::unwind_safe::AssertUnwindSafe<<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f054585aee2 - <<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1} as core[70453ac7847f43d3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f0544cea5b3 - std::sys::unix::thread::Thread::new::thread_start::hdf31f135ed490498
  37:     0x7f053f23c609 - start_thread
  38:     0x7f0544b4f133 - clone
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (ddb738607 2022-05-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z mir-opt-level=0 -Z incremental
query stack during panic:
query stack during panic:
#0 [has_ffi_unwind_calls] check if `add_type_ascription_to_parameter` contains FFI-unwind calls
#1 [required_panic_strategy] compute the required panic strategy for the current crate
------------------------------------------


---- [incremental] src/test/incremental/hashes/enum_constructors.rs stdout ----
---- [incremental] src/test/incremental/hashes/enum_constructors.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zmir-opt-level=0" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_mir_transform/src/ffi_unwind_calls.rs:55:78
stack backtrace:
   0:     0x7fcf2a7f897c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d1b0ec045decce0
   1:     0x7fcf2a85ef88 - core::fmt::write::hd9d515704885fbc6
   2:     0x7fcf2a7e8801 - std::io::Write::write_fmt::hacfc552cb196af7c
   3:     0x7fcf2a7fb9ce - std::panicking::default_hook::{{closure}}::h4f16d4a983fe151a
   4:     0x7fcf2a7fb5fc - std::panicking::default_hook::h1c29e965239144a9
   5:     0x7fcf2b391fc1 - rustc_driver[eedaaef195cd2bca]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fcf2a7fc22e - std::panicking::rust_panic_with_hook::h1fd6da4624751e3a
   7:     0x7fcf2a7fc027 - std::panicking::begin_panic_handler::{{closure}}::h3061509f74e25df9
   8:     0x7fcf2a7f8e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h17b83daf94d218c7
   9:     0x7fcf2a7fbd09 - rust_begin_unwind
  10:     0x7fcf2a7b0073 - core::panicking::panic_fmt::h6c25c0992172e8a7
  11:     0x7fcf2b8a580f - <rustc_data_structures[4b7b7fc83bfd84bf]::steal::Steal<rustc_middle[9d75a7afa4763910]::mir::Body>>::borrow
  12:     0x7fcf2b8256ff - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::has_ffi_unwind_calls
  13:     0x7fcf2cae960f - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>::{closure#1}, bool>
  14:     0x7fcf2ce16c30 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>
  15:     0x7fcf2cd7f864 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>>
  16:     0x7fcf2ce63f34 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::has_ffi_unwind_calls, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  17:     0x7fcf2cce9694 - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::has_ffi_unwind_calls
  18:     0x7fcf2b8267f9 - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::required_panic_strategy
  19:     0x7fcf2cadf5c4 - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>::{closure#0}, core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  20:     0x7fcf2cbdf43b - <rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  21:     0x7fcf2cdb8771 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<(), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>>
  22:     0x7fcf2ce7b9c5 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::required_panic_strategy, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  23:     0x7fcf2cce9bee - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::required_panic_strategy
  24:     0x7fcf2d2f969b - <rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodeContext>::encode_crate_root
  25:     0x7fcf2d30bd26 - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata_impl
  26:     0x7fcf2d3d0d51 - rustc_data_structures[4b7b7fc83bfd84bf]::sync::join::<rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodedMetadata, ()>
  27:     0x7fcf2d30b1fe - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata
  28:     0x7fcf2b464c67 - <rustc_interface[153f877473f555b9]::passes::QueryContext>::enter::<<rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[70453ac7847f43d3]::result::Result<alloc[3bed6a74638318eb]::boxed::Box<dyn core[70453ac7847f43d3]::any::Any>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  29:     0x7fcf2b44ff6e - <rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen
  30:     0x7fcf2b320680 - <rustc_interface[153f877473f555b9]::interface::Compiler>::enter::<rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}::{closure#2}, core[70453ac7847f43d3]::result::Result<core[70453ac7847f43d3]::option::Option<rustc_interface[153f877473f555b9]::queries::Linker>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  31:     0x7fcf2b3023cb - rustc_span[c893a8830d05c8cd]::with_source_map::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_interface[153f877473f555b9]::interface::create_compiler_and_run<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7fcf2b321829 - <scoped_tls[bef1a4b4d47eddc0]::ScopedKey<rustc_span[c893a8830d05c8cd]::SessionGlobals>>::set::<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  33:     0x7fcf2b37d2f9 - std[f41be0eaf76ae4e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  34:     0x7fcf2b335f21 - std[f41be0eaf76ae4e1]::panicking::try::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, core[70453ac7847f43d3]::panic::unwind_safe::AssertUnwindSafe<<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7fcf2b377ee2 - <<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1} as core[70453ac7847f43d3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fcf2a8075b3 - std::sys::unix::thread::Thread::new::thread_start::hdf31f135ed490498
  37:     0x7fcf24d59609 - start_thread
  38:     0x7fcf2a66c133 - clone
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (ddb738607 2022-05-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z mir-opt-level=0 -Z incremental
query stack during panic:
query stack during panic:
#0 [has_ffi_unwind_calls] check if `change_field_order_struct_like` contains FFI-unwind calls
#1 [required_panic_strategy] compute the required panic strategy for the current crate
------------------------------------------


---- [incremental] src/test/incremental/hashes/if_expressions.rs stdout ----
---- [incremental] src/test/incremental/hashes/if_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_mir_transform/src/ffi_unwind_calls.rs:55:78
stack backtrace:
   0:     0x7fb2358c697c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d1b0ec045decce0
   1:     0x7fb23592cf88 - core::fmt::write::hd9d515704885fbc6
   2:     0x7fb2358b6801 - std::io::Write::write_fmt::hacfc552cb196af7c
   3:     0x7fb2358c99ce - std::panicking::default_hook::{{closure}}::h4f16d4a983fe151a
   4:     0x7fb2358c95fc - std::panicking::default_hook::h1c29e965239144a9
   5:     0x7fb23645ffc1 - rustc_driver[eedaaef195cd2bca]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb2358ca22e - std::panicking::rust_panic_with_hook::h1fd6da4624751e3a
   7:     0x7fb2358ca027 - std::panicking::begin_panic_handler::{{closure}}::h3061509f74e25df9
   8:     0x7fb2358c6e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h17b83daf94d218c7
   9:     0x7fb2358c9d09 - rust_begin_unwind
  10:     0x7fb23587e073 - core::panicking::panic_fmt::h6c25c0992172e8a7
  11:     0x7fb23697380f - <rustc_data_structures[4b7b7fc83bfd84bf]::steal::Steal<rustc_middle[9d75a7afa4763910]::mir::Body>>::borrow
  12:     0x7fb2368f36ff - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::has_ffi_unwind_calls
  13:     0x7fb237bb760f - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>::{closure#1}, bool>
  14:     0x7fb237ee4c30 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>
  15:     0x7fb237e4d864 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>>
  16:     0x7fb237f31f34 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::has_ffi_unwind_calls, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  17:     0x7fb237db7694 - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::has_ffi_unwind_calls
  18:     0x7fb2368f47f9 - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::required_panic_strategy
  19:     0x7fb237bad5c4 - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>::{closure#0}, core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  20:     0x7fb237cad43b - <rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  21:     0x7fb237e86771 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<(), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>>
  22:     0x7fb237f499c5 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::required_panic_strategy, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  23:     0x7fb237db7bee - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::required_panic_strategy
  24:     0x7fb2383c769b - <rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodeContext>::encode_crate_root
  25:     0x7fb2383d9d26 - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata_impl
  26:     0x7fb23849ed51 - rustc_data_structures[4b7b7fc83bfd84bf]::sync::join::<rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodedMetadata, ()>
  27:     0x7fb2383d91fe - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata
  28:     0x7fb236532c67 - <rustc_interface[153f877473f555b9]::passes::QueryContext>::enter::<<rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[70453ac7847f43d3]::result::Result<alloc[3bed6a74638318eb]::boxed::Box<dyn core[70453ac7847f43d3]::any::Any>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  29:     0x7fb23651df6e - <rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen
  30:     0x7fb2363ee680 - <rustc_interface[153f877473f555b9]::interface::Compiler>::enter::<rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}::{closure#2}, core[70453ac7847f43d3]::result::Result<core[70453ac7847f43d3]::option::Option<rustc_interface[153f877473f555b9]::queries::Linker>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  31:     0x7fb2363d03cb - rustc_span[c893a8830d05c8cd]::with_source_map::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_interface[153f877473f555b9]::interface::create_compiler_and_run<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7fb2363ef829 - <scoped_tls[bef1a4b4d47eddc0]::ScopedKey<rustc_span[c893a8830d05c8cd]::SessionGlobals>>::set::<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  33:     0x7fb23644b2f9 - std[f41be0eaf76ae4e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  34:     0x7fb236403f21 - std[f41be0eaf76ae4e1]::panicking::try::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, core[70453ac7847f43d3]::panic::unwind_safe::AssertUnwindSafe<<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7fb236445ee2 - <<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1} as core[70453ac7847f43d3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fb2358d55b3 - std::sys::unix::thread::Thread::new::thread_start::hdf31f135ed490498
  37:     0x7fb22fe27609 - start_thread
  38:     0x7fb23573a133 - clone
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (ddb738607 2022-05-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [has_ffi_unwind_calls] check if `add_else_branch` contains FFI-unwind calls
#1 [required_panic_strategy] compute the required panic strategy for the current crate
------------------------------------------


---- [incremental] src/test/incremental/hashes/function_interfaces.rs stdout ----
---- [incremental] src/test/incremental/hashes/function_interfaces.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_mir_transform/src/ffi_unwind_calls.rs:55:78
stack backtrace:
   0:     0x7ff234f5c97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d1b0ec045decce0
   1:     0x7ff234fc2f88 - core::fmt::write::hd9d515704885fbc6
   2:     0x7ff234f4c801 - std::io::Write::write_fmt::hacfc552cb196af7c
   3:     0x7ff234f5f9ce - std::panicking::default_hook::{{closure}}::h4f16d4a983fe151a
   4:     0x7ff234f5f5fc - std::panicking::default_hook::h1c29e965239144a9
   5:     0x7ff235af5fc1 - rustc_driver[eedaaef195cd2bca]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff234f6022e - std::panicking::rust_panic_with_hook::h1fd6da4624751e3a
   7:     0x7ff234f60027 - std::panicking::begin_panic_handler::{{closure}}::h3061509f74e25df9
   8:     0x7ff234f5ce94 - std::sys_common::backtrace::__rust_end_short_backtrace::h17b83daf94d218c7
   9:     0x7ff234f5fd09 - rust_begin_unwind
  10:     0x7ff234f14073 - core::panicking::panic_fmt::h6c25c0992172e8a7
  11:     0x7ff23600980f - <rustc_data_structures[4b7b7fc83bfd84bf]::steal::Steal<rustc_middle[9d75a7afa4763910]::mir::Body>>::borrow
  12:     0x7ff235f896ff - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::has_ffi_unwind_calls
  13:     0x7ff23724d60f - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>::{closure#1}, bool>
  14:     0x7ff23757ac30 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>
  15:     0x7ff2374e3864 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>>
  16:     0x7ff2375c7f34 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::has_ffi_unwind_calls, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  17:     0x7ff23744d694 - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::has_ffi_unwind_calls
  18:     0x7ff235f8a7f9 - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::required_panic_strategy
  19:     0x7ff2372435c4 - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>::{closure#0}, core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  20:     0x7ff23734343b - <rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  21:     0x7ff23751c771 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<(), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>>
  22:     0x7ff2375df9c5 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::required_panic_strategy, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  23:     0x7ff23744dbee - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::required_panic_strategy
  24:     0x7ff237a5d69b - <rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodeContext>::encode_crate_root
  25:     0x7ff237a6fd26 - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata_impl
  26:     0x7ff237b34d51 - rustc_data_structures[4b7b7fc83bfd84bf]::sync::join::<rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodedMetadata, ()>
  27:     0x7ff237a6f1fe - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata
  28:     0x7ff235bc8c67 - <rustc_interface[153f877473f555b9]::passes::QueryContext>::enter::<<rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[70453ac7847f43d3]::result::Result<alloc[3bed6a74638318eb]::boxed::Box<dyn core[70453ac7847f43d3]::any::Any>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  29:     0x7ff235bb3f6e - <rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen
  30:     0x7ff235a84680 - <rustc_interface[153f877473f555b9]::interface::Compiler>::enter::<rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}::{closure#2}, core[70453ac7847f43d3]::result::Result<core[70453ac7847f43d3]::option::Option<rustc_interface[153f877473f555b9]::queries::Linker>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  31:     0x7ff235a663cb - rustc_span[c893a8830d05c8cd]::with_source_map::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_interface[153f877473f555b9]::interface::create_compiler_and_run<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7ff235a85829 - <scoped_tls[bef1a4b4d47eddc0]::ScopedKey<rustc_span[c893a8830d05c8cd]::SessionGlobals>>::set::<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  33:     0x7ff235ae12f9 - std[f41be0eaf76ae4e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  34:     0x7ff235a99f21 - std[f41be0eaf76ae4e1]::panicking::try::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, core[70453ac7847f43d3]::panic::unwind_safe::AssertUnwindSafe<<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7ff235adbee2 - <<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1} as core[70453ac7847f43d3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7ff234f6b5b3 - std::sys::unix::thread::Thread::new::thread_start::hdf31f135ed490498
  37:     0x7ff22f4bd609 - start_thread
  38:     0x7ff234dd0133 - clone
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (ddb738607 2022-05-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [has_ffi_unwind_calls] check if `trait_bound` contains FFI-unwind calls
#1 [required_panic_strategy] compute the required panic strategy for the current crate
------------------------------------------


---- [incremental] src/test/incremental/hashes/inherent_impls.rs stdout ----
---- [incremental] src/test/incremental/hashes/inherent_impls.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_mir_transform/src/ffi_unwind_calls.rs:55:78
stack backtrace:
   0:     0x7fd0d01c297c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d1b0ec045decce0
   1:     0x7fd0d0228f88 - core::fmt::write::hd9d515704885fbc6
   2:     0x7fd0d01b2801 - std::io::Write::write_fmt::hacfc552cb196af7c
   3:     0x7fd0d01c59ce - std::panicking::default_hook::{{closure}}::h4f16d4a983fe151a
   4:     0x7fd0d01c55fc - std::panicking::default_hook::h1c29e965239144a9
   5:     0x7fd0d0d5bfc1 - rustc_driver[eedaaef195cd2bca]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd0d01c622e - std::panicking::rust_panic_with_hook::h1fd6da4624751e3a
   7:     0x7fd0d01c6027 - std::panicking::begin_panic_handler::{{closure}}::h3061509f74e25df9
   8:     0x7fd0d01c2e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h17b83daf94d218c7
   9:     0x7fd0d01c5d09 - rust_begin_unwind
  10:     0x7fd0d017a073 - core::panicking::panic_fmt::h6c25c0992172e8a7
  11:     0x7fd0d126f80f - <rustc_data_structures[4b7b7fc83bfd84bf]::steal::Steal<rustc_middle[9d75a7afa4763910]::mir::Body>>::borrow
  12:     0x7fd0d11ef6ff - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::has_ffi_unwind_calls
  13:     0x7fd0d24b360f - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>::{closure#1}, bool>
  14:     0x7fd0d27e0c30 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>
  15:     0x7fd0d2749864 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>>
  16:     0x7fd0d282df34 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::has_ffi_unwind_calls, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  17:     0x7fd0d26b3694 - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::has_ffi_unwind_calls
  18:     0x7fd0d11f07f9 - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::required_panic_strategy
  19:     0x7fd0d24a95c4 - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>::{closure#0}, core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  20:     0x7fd0d25a943b - <rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  21:     0x7fd0d2782771 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<(), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>>
  22:     0x7fd0d28459c5 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::required_panic_strategy, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  23:     0x7fd0d26b3bee - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::required_panic_strategy
  24:     0x7fd0d2cc369b - <rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodeContext>::encode_crate_root
  25:     0x7fd0d2cd5d26 - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata_impl
  26:     0x7fd0d2d9ad51 - rustc_data_structures[4b7b7fc83bfd84bf]::sync::join::<rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodedMetadata, ()>
  27:     0x7fd0d2cd51fe - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata
  28:     0x7fd0d0e2ec67 - <rustc_interface[153f877473f555b9]::passes::QueryContext>::enter::<<rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[70453ac7847f43d3]::result::Result<alloc[3bed6a74638318eb]::boxed::Box<dyn core[70453ac7847f43d3]::any::Any>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  29:     0x7fd0d0e19f6e - <rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen
  30:     0x7fd0d0cea680 - <rustc_interface[153f877473f555b9]::interface::Compiler>::enter::<rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}::{closure#2}, core[70453ac7847f43d3]::result::Result<core[70453ac7847f43d3]::option::Option<rustc_interface[153f877473f555b9]::queries::Linker>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  31:     0x7fd0d0ccc3cb - rustc_span[c893a8830d05c8cd]::with_source_map::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_interface[153f877473f555b9]::interface::create_compiler_and_run<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7fd0d0ceb829 - <scoped_tls[bef1a4b4d47eddc0]::ScopedKey<rustc_span[c893a8830d05c8cd]::SessionGlobals>>::set::<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  33:     0x7fd0d0d472f9 - std[f41be0eaf76ae4e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  34:     0x7fd0d0cfff21 - std[f41be0eaf76ae4e1]::panicking::try::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, core[70453ac7847f43d3]::panic::unwind_safe::AssertUnwindSafe<<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7fd0d0d41ee2 - <<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1} as core[70453ac7847f43d3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fd0d01d15b3 - std::sys::unix::thread::Thread::new::thread_start::hdf31f135ed490498
  37:     0x7fd0ca723609 - start_thread
  38:     0x7fd0d0036133 - clone
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (ddb738607 2022-05-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [has_ffi_unwind_calls] check if `<impl at /checkout/src/test/incremental/hashes/inherent_impls.rs:118:1: 124:2>::method_privacy` contains FFI-unwind calls
#1 [required_panic_strategy] compute the required panic strategy for the current crate
------------------------------------------


---- [incremental] src/test/incremental/hashes/struct_constructors.rs stdout ----
---- [incremental] src/test/incremental/hashes/struct_constructors.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/struct_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/struct_constructors.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_mir_transform/src/ffi_unwind_calls.rs:55:78
stack backtrace:
   0:     0x7f77653fc97c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d1b0ec045decce0
   1:     0x7f7765462f88 - core::fmt::write::hd9d515704885fbc6
   2:     0x7f77653ec801 - std::io::Write::write_fmt::hacfc552cb196af7c
   3:     0x7f77653ff9ce - std::panicking::default_hook::{{closure}}::h4f16d4a983fe151a
   4:     0x7f77653ff5fc - std::panicking::default_hook::h1c29e965239144a9
   5:     0x7f7765f95fc1 - rustc_driver[eedaaef195cd2bca]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f776540022e - std::panicking::rust_panic_with_hook::h1fd6da4624751e3a
   7:     0x7f7765400027 - std::panicking::begin_panic_handler::{{closure}}::h3061509f74e25df9
   8:     0x7f77653fce94 - std::sys_common::backtrace::__rust_end_short_backtrace::h17b83daf94d218c7
   9:     0x7f77653ffd09 - rust_begin_unwind
  10:     0x7f77653b4073 - core::panicking::panic_fmt::h6c25c0992172e8a7
  11:     0x7f77664a980f - <rustc_data_structures[4b7b7fc83bfd84bf]::steal::Steal<rustc_middle[9d75a7afa4763910]::mir::Body>>::borrow
  12:     0x7f77664296ff - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::has_ffi_unwind_calls
  13:     0x7f77676ed60f - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>::{closure#1}, bool>
  14:     0x7f7767a1ac30 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>
  15:     0x7f7767983864 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>>
  16:     0x7f7767a67f34 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::has_ffi_unwind_calls, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  17:     0x7f77678ed694 - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::has_ffi_unwind_calls
  18:     0x7f776642a7f9 - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::required_panic_strategy
  19:     0x7f77676e35c4 - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>::{closure#0}, core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  20:     0x7f77677e343b - <rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  21:     0x7f77679bc771 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<(), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>>
  22:     0x7f7767a7f9c5 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::required_panic_strategy, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  23:     0x7f77678edbee - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::required_panic_strategy
  24:     0x7f7767efd69b - <rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodeContext>::encode_crate_root
  25:     0x7f7767f0fd26 - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata_impl
  26:     0x7f7767fd4d51 - rustc_data_structures[4b7b7fc83bfd84bf]::sync::join::<rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodedMetadata, ()>
  27:     0x7f7767f0f1fe - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata
  28:     0x7f7766068c67 - <rustc_interface[153f877473f555b9]::passes::QueryContext>::enter::<<rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[70453ac7847f43d3]::result::Result<alloc[3bed6a74638318eb]::boxed::Box<dyn core[70453ac7847f43d3]::any::Any>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  29:     0x7f7766053f6e - <rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen
  30:     0x7f7765f24680 - <rustc_interface[153f877473f555b9]::interface::Compiler>::enter::<rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}::{closure#2}, core[70453ac7847f43d3]::result::Result<core[70453ac7847f43d3]::option::Option<rustc_interface[153f877473f555b9]::queries::Linker>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  31:     0x7f7765f063cb - rustc_span[c893a8830d05c8cd]::with_source_map::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_interface[153f877473f555b9]::interface::create_compiler_and_run<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7f7765f25829 - <scoped_tls[bef1a4b4d47eddc0]::ScopedKey<rustc_span[c893a8830d05c8cd]::SessionGlobals>>::set::<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  33:     0x7f7765f812f9 - std[f41be0eaf76ae4e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  34:     0x7f7765f39f21 - std[f41be0eaf76ae4e1]::panicking::try::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, core[70453ac7847f43d3]::panic::unwind_safe::AssertUnwindSafe<<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f7765f7bee2 - <<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1} as core[70453ac7847f43d3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f776540b5b3 - std::sys::unix::thread::Thread::new::thread_start::hdf31f135ed490498
  37:     0x7f775f95d609 - start_thread
  38:     0x7f7765270133 - clone
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (ddb738607 2022-05-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [has_ffi_unwind_calls] check if `change_field_order_regular_struct` contains FFI-unwind calls
#1 [required_panic_strategy] compute the required panic strategy for the current crate
------------------------------------------


---- [incremental] src/test/incremental/hashes/trait_impls.rs stdout ----
---- [incremental] src/test/incremental/hashes/trait_impls.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/trait_impls.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_mir_transform/src/ffi_unwind_calls.rs:55:78
stack backtrace:
   0:     0x7f2951c4697c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d1b0ec045decce0
   1:     0x7f2951cacf88 - core::fmt::write::hd9d515704885fbc6
   2:     0x7f2951c36801 - std::io::Write::write_fmt::hacfc552cb196af7c
   3:     0x7f2951c499ce - std::panicking::default_hook::{{closure}}::h4f16d4a983fe151a
   4:     0x7f2951c495fc - std::panicking::default_hook::h1c29e965239144a9
   5:     0x7f29527dffc1 - rustc_driver[eedaaef195cd2bca]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2951c4a22e - std::panicking::rust_panic_with_hook::h1fd6da4624751e3a
   7:     0x7f2951c4a027 - std::panicking::begin_panic_handler::{{closure}}::h3061509f74e25df9
   8:     0x7f2951c46e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h17b83daf94d218c7
   9:     0x7f2951c49d09 - rust_begin_unwind
  10:     0x7f2951bfe073 - core::panicking::panic_fmt::h6c25c0992172e8a7
  11:     0x7f2952cf380f - <rustc_data_structures[4b7b7fc83bfd84bf]::steal::Steal<rustc_middle[9d75a7afa4763910]::mir::Body>>::borrow
  12:     0x7f2952c736ff - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::has_ffi_unwind_calls
  13:     0x7f2953f3760f - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>::{closure#1}, bool>
  14:     0x7f2954264c30 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>
  15:     0x7f29541cd864 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<rustc_span[c893a8830d05c8cd]::def_id::LocalDefId, bool>>
  16:     0x7f29542b1f34 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::has_ffi_unwind_calls, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  17:     0x7f2954137694 - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::has_ffi_unwind_calls
  18:     0x7f2952c747f9 - rustc_mir_transform[b4300423d89dc3ce]::ffi_unwind_calls::required_panic_strategy
  19:     0x7f2953f2d5c4 - <rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind as rustc_query_system[c3bd63d6abb2190f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>::{closure#0}, core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  20:     0x7f295402d43b - <rustc_query_system[c3bd63d6abb2190f]::dep_graph::graph::DepGraph<rustc_middle[9d75a7afa4763910]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[9d75a7afa4763910]::ty::context::TyCtxt, (), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>
  21:     0x7f2954206771 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::try_execute_query::<rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt, rustc_query_system[c3bd63d6abb2190f]::query::caches::DefaultCache<(), core[70453ac7847f43d3]::option::Option<rustc_target[efbbd183c08ef581]::spec::PanicStrategy>>>
  22:     0x7f29542c99c5 - rustc_query_system[c3bd63d6abb2190f]::query::plumbing::get_query::<rustc_query_impl[1b1e0e69084ce9c3]::queries::required_panic_strategy, rustc_query_impl[1b1e0e69084ce9c3]::plumbing::QueryCtxt>
  23:     0x7f2954137bee - <rustc_query_impl[1b1e0e69084ce9c3]::Queries as rustc_middle[9d75a7afa4763910]::ty::query::QueryEngine>::required_panic_strategy
  24:     0x7f295474769b - <rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodeContext>::encode_crate_root
  25:     0x7f2954759d26 - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata_impl
  26:     0x7f295481ed51 - rustc_data_structures[4b7b7fc83bfd84bf]::sync::join::<rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[61898d3075d11fc4]::rmeta::encoder::EncodedMetadata, ()>
  27:     0x7f29547591fe - rustc_metadata[61898d3075d11fc4]::rmeta::encoder::encode_metadata
  28:     0x7f29528b2c67 - <rustc_interface[153f877473f555b9]::passes::QueryContext>::enter::<<rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[70453ac7847f43d3]::result::Result<alloc[3bed6a74638318eb]::boxed::Box<dyn core[70453ac7847f43d3]::any::Any>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  29:     0x7f295289df6e - <rustc_interface[153f877473f555b9]::queries::Queries>::ongoing_codegen
  30:     0x7f295276e680 - <rustc_interface[153f877473f555b9]::interface::Compiler>::enter::<rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}::{closure#2}, core[70453ac7847f43d3]::result::Result<core[70453ac7847f43d3]::option::Option<rustc_interface[153f877473f555b9]::queries::Linker>, rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  31:     0x7f29527503cb - rustc_span[c893a8830d05c8cd]::with_source_map::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_interface[153f877473f555b9]::interface::create_compiler_and_run<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7f295276f829 - <scoped_tls[bef1a4b4d47eddc0]::ScopedKey<rustc_span[c893a8830d05c8cd]::SessionGlobals>>::set::<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  33:     0x7f29527cb2f9 - std[f41be0eaf76ae4e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>
  34:     0x7f2952783f21 - std[f41be0eaf76ae4e1]::panicking::try::<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, core[70453ac7847f43d3]::panic::unwind_safe::AssertUnwindSafe<<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f29527c5ee2 - <<std[f41be0eaf76ae4e1]::thread::Builder>::spawn_unchecked_<rustc_interface[153f877473f555b9]::util::run_in_thread_pool_with_globals<rustc_interface[153f877473f555b9]::interface::run_compiler<core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>, rustc_driver[eedaaef195cd2bca]::run_compiler::{closure#1}>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#0}, core[70453ac7847f43d3]::result::Result<(), rustc_errors[eea6c6a96b4f7f66]::ErrorGuaranteed>>::{closure#1} as core[70453ac7847f43d3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f2951c555b3 - std::sys::unix::thread::Thread::new::thread_start::hdf31f135ed490498
  37:     0x7f294c1a7609 - start_thread
  38:     0x7f2951aba133 - clone
  39:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (ddb738607 2022-05-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
#0 [has_ffi_unwind_calls] check if `<impl at /checkout/src/test/incremental/hashes/trait_impls.rs:373:1: 379:2>::method_name` contains FFI-unwind calls
#1 [required_panic_strategy] compute the required panic strategy for the current crate
------------------------------------------



