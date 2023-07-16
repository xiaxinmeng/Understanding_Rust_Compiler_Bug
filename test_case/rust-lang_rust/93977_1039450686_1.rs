rust
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling alloc v0.0.0 (/home/simon/projects/rust/library/alloc)
thread 'rustc' panicked at 'no entry found for key', compiler/rustc_metadata/src/rmeta/decoder.rs:1617:13
stack backtrace:
   0:     0x7f6180be8ccc - std::backtrace_rs::backtrace::libunwind::trace::hc419169bf6ef9454
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f6180be8ccc - std::backtrace_rs::backtrace::trace_unsynchronized::h9d59fe14733c6869
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f6180be8ccc - std::sys_common::backtrace::_print_fmt::h7b4753f1990145e4
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f6180be8ccc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he933bf90d5f823a3
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f6180c49b5c - core::fmt::write::h4f22e68bd4c318cf
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/core/src/fmt/mod.rs:1168:17
   5:     0x7f6180bd8143 - std::io::Write::write_fmt::h91e45975cc2ac4f4
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/io/mod.rs:1660:15
   6:     0x7f6180becdd2 - std::sys_common::backtrace::_print::h53eaa013323ce8bd
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f6180becdd2 - std::sys_common::backtrace::print::hddd71819e54744d2
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f6180becdd2 - std::panicking::default_hook::{{closure}}::ha7bb60f433497f90
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/panicking.rs:211:50
   9:     0x7f6180bec9b5 - std::panicking::default_hook::he08c1b6dbe4ebf2d
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/panicking.rs:228:9
  10:     0x7f61813d95f1 - rustc_driver[6192c8833b08d212]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f6180bed585 - std::panicking::rust_panic_with_hook::h4a7addec02e7dc8c
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/panicking.rs:610:17
  12:     0x7f6180bed280 - std::panicking::begin_panic_handler::{{closure}}::hd61a91341efd4038
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/panicking.rs:502:13
  13:     0x7f6180be9174 - std::sys_common::backtrace::__rust_end_short_backtrace::hf785c5cc16e3ea16
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/sys_common/backtrace.rs:139:18
  14:     0x7f6180becfb9 - rust_begin_unwind
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/panicking.rs:498:5
  15:     0x7f6180bb5041 - core::panicking::panic_fmt::ha2f5c0f1190ddcde
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/core/src/panicking.rs:107:14
  16:     0x7f6180c469e1 - core::panicking::panic_display::ha88d4cfddda37f53
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/core/src/panicking.rs:63:5
  17:     0x7f6180c4698b - core::panicking::panic_str::h1d816edb2312ac8d
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/core/src/panicking.rs:55:5
  18:     0x7f6180bb4f36 - core::option::expect_failed::hfe692c498deeb864
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/core/src/option.rs:1817:5
  19:     0x7f6182037a6e - <rustc_metadata[72dae10b32ad85]::creader::CStore as rustc_session[d69b998f511832b7]::cstore::CrateStore>::expn_hash_to_expn_id
  20:     0x7f6183869088 - <rustc_span[257a5d576bad4151]::hygiene::ExpnId as rustc_serialize[e0a901503b8dbcae]::serialize::Decodable<rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder>>::decode
  21:     0x7f6183869675 - <rustc_span[257a5d576bad4151]::hygiene::SyntaxContextData as rustc_serialize[e0a901503b8dbcae]::serialize::Decodable<rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder>>::decode
  22:     0x7f6182d508b1 - <rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder as rustc_middle[57c2e03db41ceede]::ty::codec::TyDecoder>::with_position::<<rustc_span[257a5d576bad4151]::hygiene::SyntaxContext as rustc_serialize[e0a901503b8dbcae]::serialize::Decodable<rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder>>::decode::{closure#0}::{closure#0}, core[5ae97c210862f887]::result::Result<rustc_span[257a5d576bad4151]::hygiene::SyntaxContextData, alloc[5dfd09a990d64525]::string::String>>
  23:     0x7f6182d894c1 - <rustc_span[257a5d576bad4151]::span_encoding::Span as rustc_serialize[e0a901503b8dbcae]::serialize::Decodable<rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder>>::decode
  24:     0x7f6182d8608b - <rustc_middle[57c2e03db41ceede]::ty::VariantDef as rustc_serialize[e0a901503b8dbcae]::serialize::Decodable<rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder>>::decode
  25:     0x7f6182d49d97 - <rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder as rustc_serialize[e0a901503b8dbcae]::serialize::Decoder>::read_seq::<alloc[5dfd09a990d64525]::vec::Vec<rustc_middle[57c2e03db41ceede]::ty::VariantDef>, <alloc[5dfd09a990d64525]::vec::Vec<rustc_middle[57c2e03db41ceede]::ty::VariantDef> as rustc_serialize[e0a901503b8dbcae]::serialize::Decodable<rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
  26:     0x7f61837f0347 - <rustc_middle[57c2e03db41ceede]::ty::adt::AdtDef as rustc_serialize[e0a901503b8dbcae]::serialize::Decodable<rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder>>::decode
  27:     0x7f6182d2ec4b - <rustc_middle[57c2e03db41ceede]::ty::sty::TyKind as rustc_serialize[e0a901503b8dbcae]::serialize::Decodable<rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder>>::decode
  28:     0x7f6182d7f2c1 - <&rustc_middle[57c2e03db41ceede]::ty::TyS as rustc_serialize[e0a901503b8dbcae]::serialize::Decodable<rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::CacheDecoder>>::decode
  29:     0x7f618380d029 - <rustc_query_impl[18d0a3fa2bdd5685]::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle[57c2e03db41ceede]::ty::TyS>
  30:     0x7f6182cce098 - rustc_query_system[89fe9b701e431ad8]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt, rustc_span[257a5d576bad4151]::def_id::DefId, &rustc_middle[57c2e03db41ceede]::ty::TyS>
  31:     0x7f6182d19ccc - <rustc_query_impl[18d0a3fa2bdd5685]::Queries as rustc_middle[57c2e03db41ceede]::ty::query::QueryEngine>::type_of
  32:     0x7f61828cc9d1 - <rustc_typeck[3061d5ce007a7ddd]::outlives::implicit_infer::InferVisitor as rustc_hir[486658192172cdf0]::itemlikevisit::ItemLikeVisitor>::visit_item
  33:     0x7f61828a425b - <rustc_middle[57c2e03db41ceede]::hir::map::Map>::visit_all_item_likes::<rustc_typeck[3061d5ce007a7ddd]::outlives::implicit_infer::InferVisitor>
  34:     0x7f6183556517 - rustc_typeck[3061d5ce007a7ddd]::outlives::inferred_outlives_crate
  35:     0x7f6181ce3d80 - <rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind as rustc_query_system[89fe9b701e431ad8]::dep_graph::DepKind>::with_deps::<<rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[57c2e03db41ceede]::ty::context::TyCtxt, (), rustc_middle[57c2e03db41ceede]::ty::CratePredicatesMap>::{closure#0}, rustc_middle[57c2e03db41ceede]::ty::CratePredicatesMap>
  36:     0x7f6181db62e9 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[57c2e03db41ceede]::ty::context::TyCtxt, (), rustc_middle[57c2e03db41ceede]::ty::CratePredicatesMap>
  37:     0x7f6181cd09fc - rustc_data_structures[5bcc8dcfb4a3daf1]::stack::ensure_sufficient_stack::<(rustc_middle[57c2e03db41ceede]::ty::CratePredicatesMap, rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepNodeIndex), rustc_query_system[89fe9b701e431ad8]::query::plumbing::execute_job<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt, (), rustc_middle[57c2e03db41ceede]::ty::CratePredicatesMap>::{closure#3}>
  38:     0x7f61836f906b - rustc_query_system[89fe9b701e431ad8]::query::plumbing::try_execute_query::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt, rustc_query_system[89fe9b701e431ad8]::query::caches::ArenaCache<(), rustc_middle[57c2e03db41ceede]::ty::CratePredicatesMap>>
  39:     0x7f6181c755aa - rustc_query_system[89fe9b701e431ad8]::query::plumbing::force_query::<rustc_query_impl[18d0a3fa2bdd5685]::queries::inferred_outlives_crate, rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  40:     0x7f6181d233c6 - rustc_query_impl[18d0a3fa2bdd5685]::query_callbacks::inferred_outlives_crate::force_from_dep_node
  41:     0x7f6183ad7b21 - <rustc_middle[57c2e03db41ceede]::ty::context::TyCtxt as rustc_query_system[89fe9b701e431ad8]::dep_graph::DepContext>::try_force_from_dep_node
  42:     0x7f6182d5ef47 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  43:     0x7f6182d5e6f6 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  44:     0x7f6182d5e6f6 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  45:     0x7f6182d5e6f6 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  46:     0x7f6182d5e6f6 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  47:     0x7f6182d5e6f6 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  48:     0x7f6182d5e6f6 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  49:     0x7f6182d5e6f6 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  50:     0x7f6182cce5d6 - rustc_query_system[89fe9b701e431ad8]::query::plumbing::ensure_must_run::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt, rustc_span[257a5d576bad4151]::def_id::LocalDefId, ()>
  51:     0x7f618377f91a - rustc_query_system[89fe9b701e431ad8]::query::plumbing::get_query::<rustc_query_impl[18d0a3fa2bdd5685]::queries::check_mod_const_bodies, rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  52:     0x7f61832b34fd - <core[5ae97c210862f887]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[dcd6c118f3bc6900]::passes::analysis::{closure#0}::{closure#1}> as core[5ae97c210862f887]::ops::function::FnOnce<()>>::call_once
  53:     0x7f61832b003e - <rustc_session[d69b998f511832b7]::session::Session>::time::<(), rustc_interface[dcd6c118f3bc6900]::passes::analysis::{closure#0}>
  54:     0x7f618328ab6e - rustc_interface[dcd6c118f3bc6900]::passes::analysis
  55:     0x7f6183853446 - <rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepGraph<rustc_middle[57c2e03db41ceede]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[57c2e03db41ceede]::ty::context::TyCtxt, (), core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>>
  56:     0x7f61837b09a3 - rustc_data_structures[5bcc8dcfb4a3daf1]::stack::ensure_sufficient_stack::<(core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>, rustc_query_system[89fe9b701e431ad8]::dep_graph::graph::DepNodeIndex), rustc_query_system[89fe9b701e431ad8]::query::plumbing::execute_job<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt, (), core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>>::{closure#3}>
  57:     0x7f6183731f66 - rustc_query_system[89fe9b701e431ad8]::query::plumbing::try_execute_query::<rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt, rustc_query_system[89fe9b701e431ad8]::query::caches::DefaultCache<(), core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>>>
  58:     0x7f618378ae75 - rustc_query_system[89fe9b701e431ad8]::query::plumbing::get_query::<rustc_query_impl[18d0a3fa2bdd5685]::queries::analysis, rustc_query_impl[18d0a3fa2bdd5685]::plumbing::QueryCtxt>
  59:     0x7f618326bab0 - <rustc_interface[dcd6c118f3bc6900]::interface::Compiler>::enter::<rustc_driver[6192c8833b08d212]::run_compiler::{closure#1}::{closure#2}, core[5ae97c210862f887]::result::Result<core[5ae97c210862f887]::option::Option<rustc_interface[dcd6c118f3bc6900]::queries::Linker>, rustc_errors[3fa4551cef5b7d38]::ErrorReported>>
  60:     0x7f618324d7bc - rustc_span[257a5d576bad4151]::with_source_map::<core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>, rustc_interface[dcd6c118f3bc6900]::interface::create_compiler_and_run<core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>, rustc_driver[6192c8833b08d212]::run_compiler::{closure#1}>::{closure#1}>
  61:     0x7f618326ae8e - rustc_interface[dcd6c118f3bc6900]::interface::create_compiler_and_run::<core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>, rustc_driver[6192c8833b08d212]::run_compiler::{closure#1}>
  62:     0x7f618324ed1b - <scoped_tls[bc78695e5e7f043a]::ScopedKey<rustc_span[257a5d576bad4151]::SessionGlobals>>::set::<rustc_interface[dcd6c118f3bc6900]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[dcd6c118f3bc6900]::interface::run_compiler<core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>, rustc_driver[6192c8833b08d212]::run_compiler::{closure#1}>::{closure#0}, core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>>::{closure#0}::{closure#0}, core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>>
  63:     0x7f618324eb15 - std[b709ef98f7a12fb4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dcd6c118f3bc6900]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[dcd6c118f3bc6900]::interface::run_compiler<core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>, rustc_driver[6192c8833b08d212]::run_compiler::{closure#1}>::{closure#0}, core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>>::{closure#0}, core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>>
  64:     0x7f618327b429 - <<std[b709ef98f7a12fb4]::thread::Builder>::spawn_unchecked<rustc_interface[dcd6c118f3bc6900]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[dcd6c118f3bc6900]::interface::run_compiler<core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>, rustc_driver[6192c8833b08d212]::run_compiler::{closure#1}>::{closure#0}, core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>>::{closure#0}, core[5ae97c210862f887]::result::Result<(), rustc_errors[3fa4551cef5b7d38]::ErrorReported>>::{closure#1} as core[5ae97c210862f887]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  65:     0x7f6180bf91a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h2f7659ee59dd0ada
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/alloc/src/boxed.rs:1854:9
  66:     0x7f6180bf91a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h95b83a7b24076276
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/alloc/src/boxed.rs:1854:9
  67:     0x7f6180bf91a3 - std::sys::unix::thread::Thread::new::thread_start::h1cd6c1faf6f6ab03
                               at /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/library/std/src/sys/unix/thread.rs:108:17
  68:     0x7f6180b08259 - start_thread
  69:     0x7f6180a245e3 - __GI___clone
  70:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-beta.5 (28c8a34e1 2022-01-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental -C symbol-mangling-version=legacy -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib

note: some of the compiler flags provided by cargo are hidden
