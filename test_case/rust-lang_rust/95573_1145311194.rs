plain
 finished in 0.610 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 154 tests
......................................F.....F.FFFFFFFF.FFFFFF...FFFF....F.FF....F....... 88/154
..............................................F.....F......F..F...

---- [incremental] src/test/incremental/change_symbol_export_status.rs stdout ----


error in revision `rpass4`: compilation failed!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass4" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-Zincremental-relative-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Illegal read of: 3', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:449:25
stack backtrace:
   0:     0x7f95d86e68ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0881b21dd197e011
   1:     0x7f95d874d2b8 - core::fmt::write::hdac112031f6b75eb
   2:     0x7f95d86d6711 - std::io::Write::write_fmt::hadf3430d0db3cffe
   3:     0x7f95d86e98be - std::panicking::default_hook::{{closure}}::hcfeb2b2a80489f0c
   4:     0x7f95d86e95a9 - std::panicking::default_hook::ha60145f0ba84e0b9
   5:     0x7f95d9208871 - rustc_driver[a78b23ba4854320d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f95d86ea022 - std::panicking::rust_panic_with_hook::h891fcbc5debc7293
   7:     0x7f95d86e9e57 - std::panicking::begin_panic_handler::{{closure}}::h9441107c35327431
   8:     0x7f95d86e6dc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h87501af48117d3bd
   9:     0x7f95d86e9b39 - rust_begin_unwind
  10:     0x7f95d869e013 - core::panicking::panic_fmt::h9b964ed5e908acf4
  11:     0x7f95daa9dfbf - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::read_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  12:     0x7f95da9b32bb - <rustc_span[927f4c5710008e1d]::span_encoding::Span as rustc_serialize[df0445aa1c2096e3]::serialize::Decodable<rustc_query_impl[b7644ba039beafc]::on_disk_cache::CacheDecoder>>::decode
  13:     0x7f95da96f61f - <rustc_query_impl[b7644ba039beafc]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[927f4c5710008e1d]::span_encoding::Span>
  14:     0x7f95dae80eb0 - <<rustc_query_impl[b7644ba039beafc]::queries::def_span as rustc_query_system[65f28d2dfe26dc45]::query::config::QueryDescription<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core[d1740fe21c5c331d]::ops::function::FnOnce<(rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  15:     0x7f95daab71b8 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>::{closure#0}, core[d1740fe21c5c331d]::option::Option<rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  16:     0x7f95dadf9c05 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>
  17:     0x7f95dad689ca - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  18:     0x7f95dae77fbe - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::def_span, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  19:     0x7f95da9f0fab - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::def_span
  20:     0x7f95d9f053a3 - rustc_ty_utils[c44fae3b11319949]::ty::param_env
  21:     0x7f95daab7c30 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>::{closure#1}, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  22:     0x7f95dadfaff6 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  23:     0x7f95dad6d2bb - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>>
  24:     0x7f95dae7b53f - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::param_env, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  25:     0x7f95da9f9819 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::param_env
  26:     0x7f95d9d9cece - <rustc_infer[8de38d22705b6c11]::infer::InferCtxtBuilder>::enter::<&rustc_middle[282e8596a57d1897]::ty::context::TypeckResults, <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}>
  27:     0x7f95d9c1211e - <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  28:     0x7f95d9cbe8fd - rustc_typeck[1d287365f0cbc09d]::check::typeck
  29:     0x7f95daaa33ad - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  30:     0x7f95dab64e6b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  31:     0x7f95dad54c9c - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>>
  32:     0x7f95dae77a77 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  33:     0x7f95da9e5e34 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck
  34:     0x7f95d9e6a42b - <rustc_middle[282e8596a57d1897]::hir::map::Map>::par_body_owners::<rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies::{closure#0}>
  35:     0x7f95d9cc370d - rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies
  36:     0x7f95daaac124 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>::{closure#0}, ()>
  37:     0x7f95dabb6d3b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>
  38:     0x7f95dad9b9fc - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), ()>>
  39:     0x7f95dae3b6d5 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck_item_bodies, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  40:     0x7f95da9e58de - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7f95d9e2788a - <rustc_session[717a7bb12563d11d]::session::Session>::time::<(), rustc_typeck[1d287365f0cbc09d]::check_crate::{closure#7}>
  42:     0x7f95d9d1a82e - rustc_typeck[1d287365f0cbc09d]::check_crate
  43:     0x7f95d935ab31 - rustc_interface[eb05bab285f8dfb4]::passes::analysis
  44:     0x7f95daaaa7e4 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  45:     0x7f95daba6b8b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  46:     0x7f95dad90dbf - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>>
  47:     0x7f95dae77e52 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::analysis, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  48:     0x7f95da9c9d4e - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::analysis
  49:     0x7f95d9279c94 - <rustc_interface[eb05bab285f8dfb4]::passes::QueryContext>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  50:     0x7f95d9226eae - <rustc_interface[eb05bab285f8dfb4]::interface::Compiler>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}, core[d1740fe21c5c331d]::result::Result<core[d1740fe21c5c331d]::option::Option<rustc_interface[eb05bab285f8dfb4]::queries::Linker>, rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  51:     0x7f95d920a0eb - rustc_span[927f4c5710008e1d]::with_source_map::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_interface[eb05bab285f8dfb4]::interface::create_compiler_and_run<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#1}>
  52:     0x7f95d9228074 - <scoped_tls[53f8922ada299dc0]::ScopedKey<rustc_span[927f4c5710008e1d]::SessionGlobals>>::set::<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  53:     0x7f95d927d3e9 - std[ff31b0db6279a1a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  54:     0x7f95d92291a1 - std[ff31b0db6279a1a]::panicking::try::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, core[d1740fe21c5c331d]::panic::unwind_safe::AssertUnwindSafe<<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  55:     0x7f95d927df92 - <<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1} as core[d1740fe21c5c331d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f95d86f53b3 - std::sys::unix::thread::Thread::new::thread_start::h01a569bcefc1fd2a
  57:     0x7f95d2c48609 - start_thread
  58:     0x7f95d855b133 - clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (aac9a70fc 2022-06-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:184:59
stack backtrace:
   0:     0x7f95d86e68ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0881b21dd197e011
   1:     0x7f95d874d2b8 - core::fmt::write::hdac112031f6b75eb
   2:     0x7f95d86d6711 - std::io::Write::write_fmt::hadf3430d0db3cffe
   3:     0x7f95d86e98be - std::panicking::default_hook::{{closure}}::hcfeb2b2a80489f0c
   4:     0x7f95d86e95a9 - std::panicking::default_hook::ha60145f0ba84e0b9
   5:     0x7f95d9208871 - rustc_driver[a78b23ba4854320d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f95d86ea022 - std::panicking::rust_panic_with_hook::h891fcbc5debc7293
   7:     0x7f95d86e9e19 - std::panicking::begin_panic_handler::{{closure}}::h9441107c35327431
   8:     0x7f95d86e6dc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h87501af48117d3bd
   9:     0x7f95d86e9b39 - rust_begin_unwind
  10:     0x7f95d869e013 - core::panicking::panic_fmt::h9b964ed5e908acf4
  11:     0x7f95d869dedd - core::panicking::panic::h113631e2710c4ec6
  12:     0x7f95dad69096 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  13:     0x7f95dae77fbe - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::def_span, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  14:     0x7f95da9f0fab - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::def_span
  15:     0x7f95daae031d - <rustc_span[927f4c5710008e1d]::def_id::DefId as rustc_query_impl[b7644ba039beafc]::keys::Key>::default_span
  16:     0x7f95daae0207 - <rustc_span[927f4c5710008e1d]::def_id::LocalDefId as rustc_query_impl[b7644ba039beafc]::keys::Key>::default_span
  17:     0x7f95dacce867 - rustc_query_impl[b7644ba039beafc]::make_query::typeck
  18:     0x7f95dace9e5d - <rustc_query_system[65f28d2dfe26dc45]::query::plumbing::QueryState<rustc_span[927f4c5710008e1d]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  19:     0x7f95da9c383e - <rustc_query_impl[b7644ba039beafc]::Queries>::try_collect_active_jobs
  20:     0x7f95dac8bb5d - rustc_query_system[65f28d2dfe26dc45]::query::job::print_query_stack::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  21:     0x7f95d93e5b64 - rustc_interface[eb05bab285f8dfb4]::interface::try_print_query_stack
  22:     0x7f95d9209517 - rustc_driver[a78b23ba4854320d]::report_ice
  23:     0x7f95d86ea022 - std::panicking::rust_panic_with_hook::h891fcbc5debc7293
  24:     0x7f95d86e9e57 - std::panicking::begin_panic_handler::{{closure}}::h9441107c35327431
  25:     0x7f95d86e6dc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h87501af48117d3bd
  26:     0x7f95d86e9b39 - rust_begin_unwind
  27:     0x7f95d869e013 - core::panicking::panic_fmt::h9b964ed5e908acf4
  28:     0x7f95daa9dfbf - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::read_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  29:     0x7f95da9b32bb - <rustc_span[927f4c5710008e1d]::span_encoding::Span as rustc_serialize[df0445aa1c2096e3]::serialize::Decodable<rustc_query_impl[b7644ba039beafc]::on_disk_cache::CacheDecoder>>::decode
  30:     0x7f95da96f61f - <rustc_query_impl[b7644ba039beafc]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[927f4c5710008e1d]::span_encoding::Span>
  31:     0x7f95dae80eb0 - <<rustc_query_impl[b7644ba039beafc]::queries::def_span as rustc_query_system[65f28d2dfe26dc45]::query::config::QueryDescription<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core[d1740fe21c5c331d]::ops::function::FnOnce<(rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  32:     0x7f95daab71b8 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>::{closure#0}, core[d1740fe21c5c331d]::option::Option<rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  33:     0x7f95dadf9c05 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>
  34:     0x7f95dad689ca - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  35:     0x7f95dae77fbe - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::def_span, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  36:     0x7f95da9f0fab - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::def_span
  37:     0x7f95d9f053a3 - rustc_ty_utils[c44fae3b11319949]::ty::param_env
  38:     0x7f95daab7c30 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>::{closure#1}, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  39:     0x7f95dadfaff6 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  40:     0x7f95dad6d2bb - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>>
  41:     0x7f95dae7b53f - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::param_env, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  42:     0x7f95da9f9819 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::param_env
  43:     0x7f95d9d9cece - <rustc_infer[8de38d22705b6c11]::infer::InferCtxtBuilder>::enter::<&rustc_middle[282e8596a57d1897]::ty::context::TypeckResults, <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}>
  44:     0x7f95d9c1211e - <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  45:     0x7f95d9cbe8fd - rustc_typeck[1d287365f0cbc09d]::check::typeck
  46:     0x7f95daaa33ad - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  47:     0x7f95dab64e6b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  48:     0x7f95dad54c9c - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>>
  49:     0x7f95dae77a77 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  50:     0x7f95da9e5e34 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck
  51:     0x7f95d9e6a42b - <rustc_middle[282e8596a57d1897]::hir::map::Map>::par_body_owners::<rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies::{closure#0}>
  52:     0x7f95d9cc370d - rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies
  53:     0x7f95daaac124 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>::{closure#0}, ()>
  54:     0x7f95dabb6d3b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>
  55:     0x7f95dad9b9fc - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), ()>>
  56:     0x7f95dae3b6d5 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck_item_bodies, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  57:     0x7f95da9e58de - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck_item_bodies
  58:     0x7f95d9e2788a - <rustc_session[717a7bb12563d11d]::session::Session>::time::<(), rustc_typeck[1d287365f0cbc09d]::check_crate::{closure#7}>
  59:     0x7f95d9d1a82e - rustc_typeck[1d287365f0cbc09d]::check_crate
  60:     0x7f95d935ab31 - rustc_interface[eb05bab285f8dfb4]::passes::analysis
  61:     0x7f95daaaa7e4 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  62:     0x7f95daba6b8b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  63:     0x7f95dad90dbf - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>>
  64:     0x7f95dae77e52 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::analysis, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  65:     0x7f95da9c9d4e - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::analysis
  66:     0x7f95d9279c94 - <rustc_interface[eb05bab285f8dfb4]::passes::QueryContext>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  67:     0x7f95d9226eae - <rustc_interface[eb05bab285f8dfb4]::interface::Compiler>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}, core[d1740fe21c5c331d]::result::Result<core[d1740fe21c5c331d]::option::Option<rustc_interface[eb05bab285f8dfb4]::queries::Linker>, rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  68:     0x7f95d920a0eb - rustc_span[927f4c5710008e1d]::with_source_map::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_interface[eb05bab285f8dfb4]::interface::create_compiler_and_run<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#1}>
  69:     0x7f95d9228074 - <scoped_tls[53f8922ada299dc0]::ScopedKey<rustc_span[927f4c5710008e1d]::SessionGlobals>>::set::<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  70:     0x7f95d927d3e9 - std[ff31b0db6279a1a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  71:     0x7f95d92291a1 - std[ff31b0db6279a1a]::panicking::try::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, core[d1740fe21c5c331d]::panic::unwind_safe::AssertUnwindSafe<<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  72:     0x7f95d927df92 - <<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1} as core[d1740fe21c5c331d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  73:     0x7f95d86f53b3 - std::sys::unix::thread::Thread::new::thread_start::h01a569bcefc1fd2a
  74:     0x7f95d2c48609 - start_thread
  75:     0x7f95d855b133 - clone
  76:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (aac9a70fc 2022-06-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [incremental] src/test/incremental/hashes/exported_vs_not.rs stdout ----

error in revision `cfail5`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail5" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-relative-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Illegal read of: 3', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:449:25
stack backtrace:
   0:     0x7f478cb9c8ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0881b21dd197e011
   1:     0x7f478cc032b8 - core::fmt::write::hdac112031f6b75eb
   2:     0x7f478cb8c711 - std::io::Write::write_fmt::hadf3430d0db3cffe
   3:     0x7f478cb9f8be - std::panicking::default_hook::{{closure}}::hcfeb2b2a80489f0c
   4:     0x7f478cb9f5a9 - std::panicking::default_hook::ha60145f0ba84e0b9
   5:     0x7f478d6be871 - rustc_driver[a78b23ba4854320d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f478cba0022 - std::panicking::rust_panic_with_hook::h891fcbc5debc7293
   7:     0x7f478cb9fe57 - std::panicking::begin_panic_handler::{{closure}}::h9441107c35327431
   8:     0x7f478cb9cdc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h87501af48117d3bd
   9:     0x7f478cb9fb39 - rust_begin_unwind
  10:     0x7f478cb54013 - core::panicking::panic_fmt::h9b964ed5e908acf4
  11:     0x7f478ef53fbf - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::read_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  12:     0x7f478ee692bb - <rustc_span[927f4c5710008e1d]::span_encoding::Span as rustc_serialize[df0445aa1c2096e3]::serialize::Decodable<rustc_query_impl[b7644ba039beafc]::on_disk_cache::CacheDecoder>>::decode
  13:     0x7f478ee2561f - <rustc_query_impl[b7644ba039beafc]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[927f4c5710008e1d]::span_encoding::Span>
  14:     0x7f478f336eb0 - <<rustc_query_impl[b7644ba039beafc]::queries::def_span as rustc_query_system[65f28d2dfe26dc45]::query::config::QueryDescription<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core[d1740fe21c5c331d]::ops::function::FnOnce<(rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  15:     0x7f478ef6d1b8 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>::{closure#0}, core[d1740fe21c5c331d]::option::Option<rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  16:     0x7f478f2afc05 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>
  17:     0x7f478f21e9ca - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  18:     0x7f478f32dfbe - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::def_span, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  19:     0x7f478eea6fab - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::def_span
  20:     0x7f478e3bb3a3 - rustc_ty_utils[c44fae3b11319949]::ty::param_env
  21:     0x7f478ef6dc30 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>::{closure#1}, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  22:     0x7f478f2b0ff6 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  23:     0x7f478f2232bb - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>>
  24:     0x7f478f33153f - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::param_env, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  25:     0x7f478eeaf819 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::param_env
  26:     0x7f478e252ece - <rustc_infer[8de38d22705b6c11]::infer::InferCtxtBuilder>::enter::<&rustc_middle[282e8596a57d1897]::ty::context::TypeckResults, <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}>
  27:     0x7f478e0c811e - <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  28:     0x7f478e1748fd - rustc_typeck[1d287365f0cbc09d]::check::typeck
  29:     0x7f478ef593ad - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  30:     0x7f478f01ae6b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  31:     0x7f478f20ac9c - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>>
  32:     0x7f478f32da77 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  33:     0x7f478ee9be34 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck
  34:     0x7f478e32042b - <rustc_middle[282e8596a57d1897]::hir::map::Map>::par_body_owners::<rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies::{closure#0}>
  35:     0x7f478e17970d - rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies
  36:     0x7f478ef62124 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>::{closure#0}, ()>
  37:     0x7f478f06cd3b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>
  38:     0x7f478f2519fc - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), ()>>
  39:     0x7f478f2f16d5 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck_item_bodies, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  40:     0x7f478ee9b8de - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7f478e2dd88a - <rustc_session[717a7bb12563d11d]::session::Session>::time::<(), rustc_typeck[1d287365f0cbc09d]::check_crate::{closure#7}>
  42:     0x7f478e1d082e - rustc_typeck[1d287365f0cbc09d]::check_crate
  43:     0x7f478d810b31 - rustc_interface[eb05bab285f8dfb4]::passes::analysis
  44:     0x7f478ef607e4 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  45:     0x7f478f05cb8b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  46:     0x7f478f246dbf - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>>
  47:     0x7f478f32de52 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::analysis, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  48:     0x7f478ee7fd4e - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::analysis
  49:     0x7f478d72fc94 - <rustc_interface[eb05bab285f8dfb4]::passes::QueryContext>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  50:     0x7f478d6dceae - <rustc_interface[eb05bab285f8dfb4]::interface::Compiler>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}, core[d1740fe21c5c331d]::result::Result<core[d1740fe21c5c331d]::option::Option<rustc_interface[eb05bab285f8dfb4]::queries::Linker>, rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  51:     0x7f478d6c00eb - rustc_span[927f4c5710008e1d]::with_source_map::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_interface[eb05bab285f8dfb4]::interface::create_compiler_and_run<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#1}>
  52:     0x7f478d6de074 - <scoped_tls[53f8922ada299dc0]::ScopedKey<rustc_span[927f4c5710008e1d]::SessionGlobals>>::set::<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  53:     0x7f478d7333e9 - std[ff31b0db6279a1a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  54:     0x7f478d6df1a1 - std[ff31b0db6279a1a]::panicking::try::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, core[d1740fe21c5c331d]::panic::unwind_safe::AssertUnwindSafe<<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  55:     0x7f478d733f92 - <<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1} as core[d1740fe21c5c331d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f478cbab3b3 - std::sys::unix::thread::Thread::new::thread_start::h01a569bcefc1fd2a
  57:     0x7f47870fe609 - start_thread
  58:     0x7f478ca11133 - clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (aac9a70fc 2022-06-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:184:59
stack backtrace:
   0:     0x7f478cb9c8ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0881b21dd197e011
   1:     0x7f478cc032b8 - core::fmt::write::hdac112031f6b75eb
   2:     0x7f478cb8c711 - std::io::Write::write_fmt::hadf3430d0db3cffe
   3:     0x7f478cb9f8be - std::panicking::default_hook::{{closure}}::hcfeb2b2a80489f0c
   4:     0x7f478cb9f5a9 - std::panicking::default_hook::ha60145f0ba84e0b9
   5:     0x7f478d6be871 - rustc_driver[a78b23ba4854320d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f478cba0022 - std::panicking::rust_panic_with_hook::h891fcbc5debc7293
   7:     0x7f478cb9fe19 - std::panicking::begin_panic_handler::{{closure}}::h9441107c35327431
   8:     0x7f478cb9cdc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h87501af48117d3bd
   9:     0x7f478cb9fb39 - rust_begin_unwind
  10:     0x7f478cb54013 - core::panicking::panic_fmt::h9b964ed5e908acf4
  11:     0x7f478cb53edd - core::panicking::panic::h113631e2710c4ec6
  12:     0x7f478f21f096 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  13:     0x7f478f32dfbe - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::def_span, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  14:     0x7f478eea6fab - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::def_span
  15:     0x7f478ef9631d - <rustc_span[927f4c5710008e1d]::def_id::DefId as rustc_query_impl[b7644ba039beafc]::keys::Key>::default_span
  16:     0x7f478ef96207 - <rustc_span[927f4c5710008e1d]::def_id::LocalDefId as rustc_query_impl[b7644ba039beafc]::keys::Key>::default_span
  17:     0x7f478f184867 - rustc_query_impl[b7644ba039beafc]::make_query::typeck
  18:     0x7f478f19fe5d - <rustc_query_system[65f28d2dfe26dc45]::query::plumbing::QueryState<rustc_span[927f4c5710008e1d]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  19:     0x7f478ee7983e - <rustc_query_impl[b7644ba039beafc]::Queries>::try_collect_active_jobs
  20:     0x7f478f141b5d - rustc_query_system[65f28d2dfe26dc45]::query::job::print_query_stack::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  21:     0x7f478d89bb64 - rustc_interface[eb05bab285f8dfb4]::interface::try_print_query_stack
  22:     0x7f478d6bf517 - rustc_driver[a78b23ba4854320d]::report_ice
  23:     0x7f478cba0022 - std::panicking::rust_panic_with_hook::h891fcbc5debc7293
  24:     0x7f478cb9fe57 - std::panicking::begin_panic_handler::{{closure}}::h9441107c35327431
  25:     0x7f478cb9cdc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h87501af48117d3bd
  26:     0x7f478cb9fb39 - rust_begin_unwind
  27:     0x7f478cb54013 - core::panicking::panic_fmt::h9b964ed5e908acf4
  28:     0x7f478ef53fbf - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::read_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  29:     0x7f478ee692bb - <rustc_span[927f4c5710008e1d]::span_encoding::Span as rustc_serialize[df0445aa1c2096e3]::serialize::Decodable<rustc_query_impl[b7644ba039beafc]::on_disk_cache::CacheDecoder>>::decode
  30:     0x7f478ee2561f - <rustc_query_impl[b7644ba039beafc]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[927f4c5710008e1d]::span_encoding::Span>
  31:     0x7f478f336eb0 - <<rustc_query_impl[b7644ba039beafc]::queries::def_span as rustc_query_system[65f28d2dfe26dc45]::query::config::QueryDescription<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core[d1740fe21c5c331d]::ops::function::FnOnce<(rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  32:     0x7f478ef6d1b8 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>::{closure#0}, core[d1740fe21c5c331d]::option::Option<rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  33:     0x7f478f2afc05 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>
  34:     0x7f478f21e9ca - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  35:     0x7f478f32dfbe - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::def_span, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  36:     0x7f478eea6fab - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::def_span
  37:     0x7f478e3bb3a3 - rustc_ty_utils[c44fae3b11319949]::ty::param_env
  38:     0x7f478ef6dc30 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>::{closure#1}, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  39:     0x7f478f2b0ff6 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  40:     0x7f478f2232bb - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>>
  41:     0x7f478f33153f - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::param_env, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  42:     0x7f478eeaf819 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::param_env
  43:     0x7f478e252ece - <rustc_infer[8de38d22705b6c11]::infer::InferCtxtBuilder>::enter::<&rustc_middle[282e8596a57d1897]::ty::context::TypeckResults, <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}>
  44:     0x7f478e0c811e - <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  45:     0x7f478e1748fd - rustc_typeck[1d287365f0cbc09d]::check::typeck
  46:     0x7f478ef593ad - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  47:     0x7f478f01ae6b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  48:     0x7f478f20ac9c - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>>
  49:     0x7f478f32da77 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  50:     0x7f478ee9be34 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck
  51:     0x7f478e32042b - <rustc_middle[282e8596a57d1897]::hir::map::Map>::par_body_owners::<rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies::{closure#0}>
  52:     0x7f478e17970d - rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies
  53:     0x7f478ef62124 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>::{closure#0}, ()>
  54:     0x7f478f06cd3b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>
  55:     0x7f478f2519fc - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), ()>>
  56:     0x7f478f2f16d5 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck_item_bodies, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  57:     0x7f478ee9b8de - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck_item_bodies
  58:     0x7f478e2dd88a - <rustc_session[717a7bb12563d11d]::session::Session>::time::<(), rustc_typeck[1d287365f0cbc09d]::check_crate::{closure#7}>
  59:     0x7f478e1d082e - rustc_typeck[1d287365f0cbc09d]::check_crate
  60:     0x7f478d810b31 - rustc_interface[eb05bab285f8dfb4]::passes::analysis
  61:     0x7f478ef607e4 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  62:     0x7f478f05cb8b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  63:     0x7f478f246dbf - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>>
  64:     0x7f478f32de52 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::analysis, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  65:     0x7f478ee7fd4e - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::analysis
  66:     0x7f478d72fc94 - <rustc_interface[eb05bab285f8dfb4]::passes::QueryContext>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  67:     0x7f478d6dceae - <rustc_interface[eb05bab285f8dfb4]::interface::Compiler>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}, core[d1740fe21c5c331d]::result::Result<core[d1740fe21c5c331d]::option::Option<rustc_interface[eb05bab285f8dfb4]::queries::Linker>, rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  68:     0x7f478d6c00eb - rustc_span[927f4c5710008e1d]::with_source_map::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_interface[eb05bab285f8dfb4]::interface::create_compiler_and_run<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#1}>
  69:     0x7f478d6de074 - <scoped_tls[53f8922ada299dc0]::ScopedKey<rustc_span[927f4c5710008e1d]::SessionGlobals>>::set::<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  70:     0x7f478d7333e9 - std[ff31b0db6279a1a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  71:     0x7f478d6df1a1 - std[ff31b0db6279a1a]::panicking::try::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, core[d1740fe21c5c331d]::panic::unwind_safe::AssertUnwindSafe<<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  72:     0x7f478d733f92 - <<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1} as core[d1740fe21c5c331d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  73:     0x7f478cbab3b3 - std::sys::unix::thread::Thread::new::thread_start::h01a569bcefc1fd2a
  74:     0x7f47870fe609 - start_thread
  75:     0x7f478ca11133 - clone
  76:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (aac9a70fc 2022-06-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [incremental] src/test/incremental/hashes/closure_expressions.rs stdout ----

error in revision `cfail5`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail5" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-relative-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Illegal read of: 3', /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:449:25
stack backtrace:
   0:     0x7fe488da08ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0881b21dd197e011
   1:     0x7fe488e072b8 - core::fmt::write::hdac112031f6b75eb
   2:     0x7fe488d90711 - std::io::Write::write_fmt::hadf3430d0db3cffe
   3:     0x7fe488da38be - std::panicking::default_hook::{{closure}}::hcfeb2b2a80489f0c
   4:     0x7fe488da35a9 - std::panicking::default_hook::ha60145f0ba84e0b9
   5:     0x7fe4898c2871 - rustc_driver[a78b23ba4854320d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe488da4022 - std::panicking::rust_panic_with_hook::h891fcbc5debc7293
   7:     0x7fe488da3e57 - std::panicking::begin_panic_handler::{{closure}}::h9441107c35327431
   8:     0x7fe488da0dc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h87501af48117d3bd
   9:     0x7fe488da3b39 - rust_begin_unwind
  10:     0x7fe488d58013 - core::panicking::panic_fmt::h9b964ed5e908acf4
  11:     0x7fe48b157fbf - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::read_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  12:     0x7fe48b06d2bb - <rustc_span[927f4c5710008e1d]::span_encoding::Span as rustc_serialize[df0445aa1c2096e3]::serialize::Decodable<rustc_query_impl[b7644ba039beafc]::on_disk_cache::CacheDecoder>>::decode
  13:     0x7fe48b02961f - <rustc_query_impl[b7644ba039beafc]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[927f4c5710008e1d]::span_encoding::Span>
  14:     0x7fe48b53aeb0 - <<rustc_query_impl[b7644ba039beafc]::queries::def_span as rustc_query_system[65f28d2dfe26dc45]::query::config::QueryDescription<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core[d1740fe21c5c331d]::ops::function::FnOnce<(rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  15:     0x7fe48b1711b8 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>::{closure#0}, core[d1740fe21c5c331d]::option::Option<rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  16:     0x7fe48b4b3c05 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>
  17:     0x7fe48b4229ca - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  18:     0x7fe48b531fbe - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::def_span, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  19:     0x7fe48b0aafab - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::def_span
  20:     0x7fe48a5bf3a3 - rustc_ty_utils[c44fae3b11319949]::ty::param_env
  21:     0x7fe48b171c30 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>::{closure#1}, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  22:     0x7fe48b4b4ff6 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  23:     0x7fe48b4272bb - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>>
  24:     0x7fe48b53553f - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::param_env, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  25:     0x7fe48b0b3819 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::param_env
  26:     0x7fe48a456ece - <rustc_infer[8de38d22705b6c11]::infer::InferCtxtBuilder>::enter::<&rustc_middle[282e8596a57d1897]::ty::context::TypeckResults, <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}>
  27:     0x7fe48a2cc11e - <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  28:     0x7fe48a3788fd - rustc_typeck[1d287365f0cbc09d]::check::typeck
  29:     0x7fe48b15d3ad - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  30:     0x7fe48b21ee6b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  31:     0x7fe48b40ec9c - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>>
  32:     0x7fe48b3ca2fc - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::force_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  33:     0x7fe48b2a3585 - rustc_query_impl[b7644ba039beafc]::query_callbacks::typeck::force_from_dep_node
  34:     0x7fe48c27acc6 - <rustc_middle[282e8596a57d1897]::ty::context::TyCtxt as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepContext>::try_force_from_dep_node
  35:     0x7fe48b1f5d4f - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  36:     0x7fe48b1ab46a - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  37:     0x7fe48b3cfccb - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::ensure_must_run::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, rustc_span[927f4c5710008e1d]::def_id::LocalDefId>
  38:     0x7fe48b531a16 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  39:     0x7fe48b09fe34 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck
  40:     0x7fe48a52442b - <rustc_middle[282e8596a57d1897]::hir::map::Map>::par_body_owners::<rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies::{closure#0}>
  41:     0x7fe48a37d70d - rustc_typeck[1d287365f0cbc09d]::check::typeck_item_bodies
  42:     0x7fe48b166124 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>::{closure#0}, ()>
  43:     0x7fe48b270d3b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), ()>
  44:     0x7fe48b4559fc - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), ()>>
  45:     0x7fe48b4f56d5 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::typeck_item_bodies, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  46:     0x7fe48b09f8de - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::typeck_item_bodies
  47:     0x7fe48a4e188a - <rustc_session[717a7bb12563d11d]::session::Session>::time::<(), rustc_typeck[1d287365f0cbc09d]::check_crate::{closure#7}>
  48:     0x7fe48a3d482e - rustc_typeck[1d287365f0cbc09d]::check_crate
  49:     0x7fe489a14b31 - rustc_interface[eb05bab285f8dfb4]::passes::analysis
  50:     0x7fe48b1647e4 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  51:     0x7fe48b260b8b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, (), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  52:     0x7fe48b44adbf - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<(), core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>>
  53:     0x7fe48b531e52 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::analysis, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  54:     0x7fe48b083d4e - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::analysis
  55:     0x7fe489933c94 - <rustc_interface[eb05bab285f8dfb4]::passes::QueryContext>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  56:     0x7fe4898e0eae - <rustc_interface[eb05bab285f8dfb4]::interface::Compiler>::enter::<rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}::{closure#2}, core[d1740fe21c5c331d]::result::Result<core[d1740fe21c5c331d]::option::Option<rustc_interface[eb05bab285f8dfb4]::queries::Linker>, rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  57:     0x7fe4898c40eb - rustc_span[927f4c5710008e1d]::with_source_map::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_interface[eb05bab285f8dfb4]::interface::create_compiler_and_run<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#1}>
  58:     0x7fe4898e2074 - <scoped_tls[53f8922ada299dc0]::ScopedKey<rustc_span[927f4c5710008e1d]::SessionGlobals>>::set::<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  59:     0x7fe4899373e9 - std[ff31b0db6279a1a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>
  60:     0x7fe4898e31a1 - std[ff31b0db6279a1a]::panicking::try::<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, core[d1740fe21c5c331d]::panic::unwind_safe::AssertUnwindSafe<<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  61:     0x7fe489937f92 - <<std[ff31b0db6279a1a]::thread::Builder>::spawn_unchecked_<rustc_interface[eb05bab285f8dfb4]::util::run_in_thread_pool_with_globals<rustc_interface[eb05bab285f8dfb4]::interface::run_compiler<core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>, rustc_driver[a78b23ba4854320d]::run_compiler::{closure#1}>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#0}, core[d1740fe21c5c331d]::result::Result<(), rustc_errors[337b10e14b46bf0a]::ErrorGuaranteed>>::{closure#1} as core[d1740fe21c5c331d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7fe488daf3b3 - std::sys::unix::thread::Thread::new::thread_start::h01a569bcefc1fd2a
  63:     0x7fe483302609 - start_thread
  64:     0x7fe488c15133 - clone
  65:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (aac9a70fc 2022-06-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:184:59
stack backtrace:
   0:     0x7fe488da08ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0881b21dd197e011
   1:     0x7fe488e072b8 - core::fmt::write::hdac112031f6b75eb
   2:     0x7fe488d90711 - std::io::Write::write_fmt::hadf3430d0db3cffe
   3:     0x7fe488da38be - std::panicking::default_hook::{{closure}}::hcfeb2b2a80489f0c
   4:     0x7fe488da35a9 - std::panicking::default_hook::ha60145f0ba84e0b9
   5:     0x7fe4898c2871 - rustc_driver[a78b23ba4854320d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe488da4022 - std::panicking::rust_panic_with_hook::h891fcbc5debc7293
   7:     0x7fe488da3e19 - std::panicking::begin_panic_handler::{{closure}}::h9441107c35327431
   8:     0x7fe488da0dc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h87501af48117d3bd
   9:     0x7fe488da3b39 - rust_begin_unwind
  10:     0x7fe488d58013 - core::panicking::panic_fmt::h9b964ed5e908acf4
  11:     0x7fe488d57edd - core::panicking::panic::h113631e2710c4ec6
  12:     0x7fe48b423096 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  13:     0x7fe48b531fbe - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::def_span, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  14:     0x7fe48b0aafab - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::def_span
  15:     0x7fe48b19a31d - <rustc_span[927f4c5710008e1d]::def_id::DefId as rustc_query_impl[b7644ba039beafc]::keys::Key>::default_span
  16:     0x7fe48b19a207 - <rustc_span[927f4c5710008e1d]::def_id::LocalDefId as rustc_query_impl[b7644ba039beafc]::keys::Key>::default_span
  17:     0x7fe48b388867 - rustc_query_impl[b7644ba039beafc]::make_query::typeck
  18:     0x7fe48b3a3e5d - <rustc_query_system[65f28d2dfe26dc45]::query::plumbing::QueryState<rustc_span[927f4c5710008e1d]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  19:     0x7fe48b07d83e - <rustc_query_impl[b7644ba039beafc]::Queries>::try_collect_active_jobs
  20:     0x7fe48b345b5d - rustc_query_system[65f28d2dfe26dc45]::query::job::print_query_stack::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  21:     0x7fe489a9fb64 - rustc_interface[eb05bab285f8dfb4]::interface::try_print_query_stack
  22:     0x7fe4898c3517 - rustc_driver[a78b23ba4854320d]::report_ice
  23:     0x7fe488da4022 - std::panicking::rust_panic_with_hook::h891fcbc5debc7293
  24:     0x7fe488da3e57 - std::panicking::begin_panic_handler::{{closure}}::h9441107c35327431
  25:     0x7fe488da0dc4 - std::sys_common::backtrace::__rust_end_short_backtrace::h87501af48117d3bd
  26:     0x7fe488da3b39 - rust_begin_unwind
  27:     0x7fe488d58013 - core::panicking::panic_fmt::h9b964ed5e908acf4
  28:     0x7fe48b157fbf - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::read_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  29:     0x7fe48b06d2bb - <rustc_span[927f4c5710008e1d]::span_encoding::Span as rustc_serialize[df0445aa1c2096e3]::serialize::Decodable<rustc_query_impl[b7644ba039beafc]::on_disk_cache::CacheDecoder>>::decode
  30:     0x7fe48b02961f - <rustc_query_impl[b7644ba039beafc]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[927f4c5710008e1d]::span_encoding::Span>
  31:     0x7fe48b53aeb0 - <<rustc_query_impl[b7644ba039beafc]::queries::def_span as rustc_query_system[65f28d2dfe26dc45]::query::config::QueryDescription<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core[d1740fe21c5c331d]::ops::function::FnOnce<(rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  32:     0x7fe48b1711b8 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>::{closure#0}, core[d1740fe21c5c331d]::option::Option<rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  33:     0x7fe48b4b3c05 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>
  34:     0x7fe48b4229ca - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_span[927f4c5710008e1d]::span_encoding::Span>>
  35:     0x7fe48b531fbe - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::def_span, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  36:     0x7fe48b0aafab - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::def_span
  37:     0x7fe48a5bf3a3 - rustc_ty_utils[c44fae3b11319949]::ty::param_env
  38:     0x7fe48b171c30 - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>::{closure#1}, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  39:     0x7fe48b4b4ff6 - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>
  40:     0x7fe48b4272bb - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::DefId, rustc_middle[282e8596a57d1897]::ty::ParamEnv>>
  41:     0x7fe48b53553f - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::get_query::<rustc_query_impl[b7644ba039beafc]::queries::param_env, rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt>
  42:     0x7fe48b0b3819 - <rustc_query_impl[b7644ba039beafc]::Queries as rustc_middle[282e8596a57d1897]::ty::query::QueryEngine>::param_env
  43:     0x7fe48a456ece - <rustc_infer[8de38d22705b6c11]::infer::InferCtxtBuilder>::enter::<&rustc_middle[282e8596a57d1897]::ty::context::TypeckResults, <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}>
  44:     0x7fe48a2cc11e - <rustc_typeck[1d287365f0cbc09d]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[1d287365f0cbc09d]::check::typeck_with_fallback<rustc_typeck[1d287365f0cbc09d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  45:     0x7fe48a3788fd - rustc_typeck[1d287365f0cbc09d]::check::typeck
  46:     0x7fe48b15d3ad - <rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind as rustc_query_system[65f28d2dfe26dc45]::dep_graph::DepKind>::with_deps::<<rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  47:     0x7fe48b21ee6b - <rustc_query_system[65f28d2dfe26dc45]::dep_graph::graph::DepGraph<rustc_middle[282e8596a57d1897]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[282e8596a57d1897]::ty::context::TyCtxt, rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>
  48:     0x7fe48b40ec9c - rustc_query_system[65f28d2dfe26dc45]::query::plumbing::try_execute_query::<rustc_query_impl[b7644ba039beafc]::plumbing::QueryCtxt, rustc_query_system[65f28d2dfe26dc45]::query::caches::DefaultCache<rustc_span[927f4c5710008e1d]::def_id::LocalDefId, &rustc_middle[282e8596a57d1897]::ty::context::TypeckResults>>
