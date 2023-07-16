shell
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
    Checking common-io v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/common/io)
   Compiling common-meta-types v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/meta/types)
   Compiling common-meta-client v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/meta/client)
   Compiling common-config v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/config)
   Compiling common-storages-system v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/storages/system)
   Compiling databend-meta v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/meta/service)
    Checking common-datavalues v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/datavalues)
    Checking common-auth v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/common/auth)
    Checking common-datablocks v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/datablocks)
    Checking common-storage v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/common/storage)
    Checking common-sharing v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/sharing)
    Checking common-functions v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/functions)
    Checking common-pipeline-core v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/pipeline/core)
    Checking common-expression v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/expression)
    Checking common-pipeline-transforms v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/pipeline/transforms)
    Checking codegen v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/codegen)
    Checking common-functions-v2 v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/query/functions-v2)
    Checking common-meta-app v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/meta/app)
    Checking common-meta-sled-store v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/meta/sled-store)
    Checking common-proto-conv v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/meta/proto-conv)
    Checking common-meta-api v0.1.0 (/Users/bohu/github/datafuselabs/databend/src/meta/api)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `6971`', compiler/rustc_query_impl/src/[on_disk_cache.rs:544](http://on_disk_cache.rs:544/):5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.66 (7eef946f 2022-11-06)

query stack during panic:
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `96`,
 right: `410224`', compiler/rustc_query_impl/src/[on_disk_cache.rs:544](http://on_disk_cache.rs:544/):5
stack backtrace:
   0:        0x10372bce0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::heb219010e7c6ee67
   1:        0x10377681c - core::fmt::write::h44dbeb0020bd9987
   2:        0x10371e9e8 - std::io::Write::write_fmt::hf2fca6e6141ed53c
   3:        0x10372baf4 - std::sys_common::backtrace::print::h8e480b32c49056e5
   4:        0x10372e648 - std::panicking::default_hook::{{closure}}::hae2666e10ca5e3c3
   5:        0x10372e3a0 - std::panicking::default_hook::h5cacca55e83dc73d
   6:        0x1025193f4 - clippy_driver[e831aa7786165f58]::ICE_HOOK::{closure#0}::{closure#0}
   7:        0x10372ed58 - std::panicking::rust_panic_with_hook::hc5673dfe88cddbda
   8:        0x10372eb40 - std::panicking::begin_panic_handler::{{closure}}::h7298d974d1263a76
   9:        0x10372c148 - std::sys_common::backtrace::__rust_end_short_backtrace::hcd143c038b29ac9d
  10:        0x10372e894 - _rust_begin_unwind
  11:        0x1037a21d8 - core::panicking::panic_fmt::h7cb516df4f1051f6
  12:        0x103773c3c - core::panicking::assert_failed_inner::h9dd5ba3df3dadb16
  13:        0x10f878704 - core[6b7ccbd7903790]::panicking::assert_failed::<rustc_query_system[ee6682a9ec28d0da]::dep_graph::serialized::SerializedDepNodeIndex, rustc_query_system[ee6682a9ec28d0da]::dep_graph::serialized::SerializedDepNodeIndex>
  14:        0x10e8889f0 - <rustc_query_impl[9f5877b90dc70815]::on_disk_cache::OnDiskCache>::try_load_query_result::<rustc_span[3fd78b27b43bb270]::span_encoding::Span>
  15:        0x10e7a0d8c - <rustc_middle[965378d07c150375]::dep_graph::dep_node::DepKind as rustc_query_system[ee6682a9ec28d0da]::dep_graph::DepKind>::with_deps::<rustc_query_system[ee6682a9ec28d0da]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, rustc_span[3fd78b27b43bb270]::def_id::LocalDefId, rustc_span[3fd78b27b43bb270]::span_encoding::Span>::{closure#0}, core[6b7ccbd7903790]::option::Option<rustc_span[3fd78b27b43bb270]::span_encoding::Span>>
  16:        0x10e703da4 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, rustc_span[3fd78b27b43bb270]::def_id::DefId, rustc_span[3fd78b27b43bb270]::span_encoding::Span>
  17:        0x10e6821d4 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::try_execute_query::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, rustc_query_system[ee6682a9ec28d0da]::query::caches::DefaultCache<rustc_span[3fd78b27b43bb270]::def_id::DefId, rustc_span[3fd78b27b43bb270]::span_encoding::Span>>
  18:        0x10e73b41c - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::get_query::<rustc_query_impl[9f5877b90dc70815]::queries::def_span, rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt>
  19:        0x10e7d08a0 - <rustc_query_impl[9f5877b90dc70815]::Queries as rustc_middle[965378d07c150375]::ty::query::QueryEngine>::def_span
  20:        0x10e83ec34 - <rustc_span[3fd78b27b43bb270]::def_id::DefId as rustc_query_impl[9f5877b90dc70815]::keys::Key>::default_span
  21:        0x10e9328bc - rustc_query_impl[9f5877b90dc70815]::plumbing::create_query_frame::<rustc_span[3fd78b27b43bb270]::def_id::DefId>
  22:        0x10e85bcc8 - <rustc_query_impl[9f5877b90dc70815]::query_structs::opt_def_kind::{closure#0} as core[6b7ccbd7903790]::ops::function::FnOnce<(rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, &mut std[bf8102ec50f6ab2f]::collections::hash::map::HashMap<rustc_query_system[ee6682a9ec28d0da]::query::job::QueryJobId, rustc_query_system[ee6682a9ec28d0da]::query::job::QueryJobInfo, core[6b7ccbd7903790]::hash::BuildHasherDefault<rustc_hash[680f175f91322e64]::FxHasher>>)>>::call_once
  23:        0x10e842ae8 - rustc_query_system[ee6682a9ec28d0da]::query::job::print_query_stack::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt>
  24:        0x10b7ce77c - rustc_interface[323f62be639fa926]::interface::try_print_query_stack
  25:        0x1025197e4 - clippy_driver[e831aa7786165f58]::ICE_HOOK::{closure#0}::{closure#0}
  26:        0x10372ed58 - std::panicking::rust_panic_with_hook::hc5673dfe88cddbda
  27:        0x10372eb40 - std::panicking::begin_panic_handler::{{closure}}::h7298d974d1263a76
  28:        0x10372c148 - std::sys_common::backtrace::__rust_end_short_backtrace::hcd143c038b29ac9d
  29:        0x10372e894 - _rust_begin_unwind
  30:        0x1037a21d8 - core::panicking::panic_fmt::h7cb516df4f1051f6
  31:        0x103773c3c - core::panicking::assert_failed_inner::h9dd5ba3df3dadb16
  32:        0x10f878704 - core[6b7ccbd7903790]::panicking::assert_failed::<rustc_query_system[ee6682a9ec28d0da]::dep_graph::serialized::SerializedDepNodeIndex, rustc_query_system[ee6682a9ec28d0da]::dep_graph::serialized::SerializedDepNodeIndex>
  33:        0x10e885cd4 - <rustc_query_impl[9f5877b90dc70815]::on_disk_cache::OnDiskCache>::try_load_query_result::<core[6b7ccbd7903790]::option::Option<rustc_hir[9989958ecc1c32ac]::def::DefKind>>
  34:        0x10e9378dc - rustc_query_impl[9f5877b90dc70815]::plumbing::try_load_from_disk::<core[6b7ccbd7903790]::option::Option<rustc_hir[9989958ecc1c32ac]::def::DefKind>>
  35:        0x10e7a177c - <rustc_middle[965378d07c150375]::dep_graph::dep_node::DepKind as rustc_query_system[ee6682a9ec28d0da]::dep_graph::DepKind>::with_deps::<rustc_query_system[ee6682a9ec28d0da]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, rustc_span[3fd78b27b43bb270]::def_id::DefId, core[6b7ccbd7903790]::option::Option<rustc_hir[9989958ecc1c32ac]::def::DefKind>>::{closure#0}, core[6b7ccbd7903790]::option::Option<core[6b7ccbd7903790]::option::Option<rustc_hir[9989958ecc1c32ac]::def::DefKind>>>
  36:        0x10e700a38 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, rustc_span[3fd78b27b43bb270]::def_id::DefId, core[6b7ccbd7903790]::option::Option<rustc_hir[9989958ecc1c32ac]::def::DefKind>>
  37:        0x10e675c38 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::try_execute_query::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, rustc_query_system[ee6682a9ec28d0da]::query::caches::DefaultCache<rustc_span[3fd78b27b43bb270]::def_id::DefId, core[6b7ccbd7903790]::option::Option<rustc_hir[9989958ecc1c32ac]::def::DefKind>>>
  38:        0x10e722b90 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::get_query::<rustc_query_impl[9f5877b90dc70815]::queries::opt_def_kind, rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt>
  39:        0x10e0a0c2c - <rustc_middle[965378d07c150375]::ty::context::TyCtxt>::def_kind::<rustc_span[3fd78b27b43bb270]::def_id::LocalDefId>
  40:        0x10e092038 - rustc_passes[7782f14db13ae9be]::lang_items::get_lang_items
  41:        0x10e905ab8 - <rustc_query_system[ee6682a9ec28d0da]::dep_graph::graph::DepGraph<rustc_middle[965378d07c150375]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[965378d07c150375]::ty::context::TyCtxt, (), rustc_hir[9989958ecc1c32ac]::lang_items::LanguageItems>
  42:        0x10e63e6e0 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::try_execute_query::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, rustc_query_system[ee6682a9ec28d0da]::query::caches::ArenaCache<(), rustc_hir[9989958ecc1c32ac]::lang_items::LanguageItems>>
  43:        0x10e5f2cc8 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::force_query::<rustc_query_impl[9f5877b90dc70815]::queries::get_lang_items, rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt>
  44:        0x10e939a00 - rustc_query_impl[9f5877b90dc70815]::plumbing::force_from_dep_node::<rustc_query_impl[9f5877b90dc70815]::queries::get_lang_items>
  45:        0x10e8c8548 - <rustc_query_system[ee6682a9ec28d0da]::dep_graph::graph::DepGraph<rustc_middle[965378d07c150375]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt>
  46:        0x10e8c858c - <rustc_query_system[ee6682a9ec28d0da]::dep_graph::graph::DepGraph<rustc_middle[965378d07c150375]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt>
  47:        0x10e8977e8 - <rustc_query_system[ee6682a9ec28d0da]::dep_graph::graph::DepGraph<rustc_middle[965378d07c150375]::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt>
  48:        0x10e60f9f0 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::ensure_must_run::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, rustc_span[3fd78b27b43bb270]::def_id::LocalDefId, rustc_span[3fd78b27b43bb270]::def_id::LocalDefId>
  49:        0x10e725440 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::get_query::<rustc_query_impl[9f5877b90dc70815]::queries::check_mod_attrs, rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt>
  50:        0x10b78743c - <core[6b7ccbd7903790]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e78f64e65aa6125e]::sync::par_for_each_in<&[rustc_hir[9989958ecc1c32ac]::hir_id::OwnerId], <rustc_middle[965378d07c150375]::hir::map::Map>::par_for_each_module<rustc_interface[323f62be639fa926]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[6b7ccbd7903790]::ops::function::FnOnce<()>>::call_once
  51:        0x10b7cc4a8 - rustc_data_structures[e78f64e65aa6125e]::sync::par_for_each_in::<&[rustc_hir[9989958ecc1c32ac]::hir_id::OwnerId], <rustc_middle[965378d07c150375]::hir::map::Map>::par_for_each_module<rustc_interface[323f62be639fa926]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>::{closure#0}>
  52:        0x10b787dcc - <core[6b7ccbd7903790]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[323f62be639fa926]::passes::analysis::{closure#0}::{closure#1}> as core[6b7ccbd7903790]::ops::function::FnOnce<()>>::call_once
  53:        0x10b7433e0 - <rustc_session[9b62c70a0fc5bd55]::session::Session>::time::<(), rustc_interface[323f62be639fa926]::passes::analysis::{closure#0}>
  54:        0x10b775260 - rustc_interface[323f62be639fa926]::passes::analysis
  55:        0x10e901b90 - <rustc_query_system[ee6682a9ec28d0da]::dep_graph::graph::DepGraph<rustc_middle[965378d07c150375]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[965378d07c150375]::ty::context::TyCtxt, (), core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>>
  56:        0x10e6d1f00 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::try_execute_query::<rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt, rustc_query_system[ee6682a9ec28d0da]::query::caches::DefaultCache<(), core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>>>
  57:        0x10e73b2a4 - rustc_query_system[ee6682a9ec28d0da]::query::plumbing::get_query::<rustc_query_impl[9f5877b90dc70815]::queries::analysis, rustc_query_impl[9f5877b90dc70815]::plumbing::QueryCtxt>
  58:        0x10b673efc - <rustc_interface[323f62be639fa926]::passes::QueryContext>::enter::<rustc_driver[55d826ba0aec6bd8]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>>
  59:        0x10b6b08ac - rustc_span[3fd78b27b43bb270]::with_source_map::<core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>, rustc_interface[323f62be639fa926]::interface::run_compiler<core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>, rustc_driver[55d826ba0aec6bd8]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  60:        0x10b6a27b8 - <scoped_tls[835146d6f855dbcf]::ScopedKey<rustc_span[3fd78b27b43bb270]::SessionGlobals>>::set::<rustc_interface[323f62be639fa926]::interface::run_compiler<core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>, rustc_driver[55d826ba0aec6bd8]::run_compiler::{closure#1}>::{closure#0}, core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>>
  61:        0x10b677288 - std[bf8102ec50f6ab2f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[323f62be639fa926]::util::run_in_thread_pool_with_globals<rustc_interface[323f62be639fa926]::interface::run_compiler<core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>, rustc_driver[55d826ba0aec6bd8]::run_compiler::{closure#1}>::{closure#0}, core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>>
  62:        0x10b660bac - <<std[bf8102ec50f6ab2f]::thread::Builder>::spawn_unchecked_<rustc_interface[323f62be639fa926]::util::run_in_thread_pool_with_globals<rustc_interface[323f62be639fa926]::interface::run_compiler<core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>, rustc_driver[55d826ba0aec6bd8]::run_compiler::{closure#1}>::{closure#0}, core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6b7ccbd7903790]::result::Result<(), rustc_errors[80fd13a7dee11d14]::ErrorGuaranteed>>::{closure#1} as core[6b7ccbd7903790]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  63:        0x103736da4 - std::sys::unix::thread::Thread::new::thread_start::hde848c02a099d1a3
  64:        0x18c7d426c - __pthread_deallocate

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.66 (7eef946f 2022-11-06)

query stack during panic:
thread panicked while processing panic. aborting.
error: could not compile `common-functions-v2`

Caused by:
  process didn't exit successfully: `/Users/bohu/.rustup/toolchains/nightly-2022-11-07-aarch64-apple-darwin/bin/clippy-driver rustc --crate-name common_functions_v2 --edition=2021 src/query/functions-v2/src/[lib.rs](http://lib.rs/) --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C overflow-checks=off -C metadata=ac2802044bdd45ed -C extra-filename=-ac2802044bdd45ed --out-dir /Users/bohu/github/datafuselabs/databend/target/debug/deps -C incremental=/Users/bohu/github/datafuselabs/databend/target/debug/incremental -L dependency=/Users/bohu/github/datafuselabs/databend/target/debug/deps --extern base64=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libbase64-1d997eb6f82cfb3d.rmeta --extern bstr=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libbstr-025037e7bc17bf55.rmeta --extern bumpalo=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libbumpalo-83da5192582aa0d6.rmeta --extern bytes=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libbytes-37485cc8ead049d2.rmeta --extern chrono=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libchrono-d42f70232ce46eff.rmeta --extern chrono_tz=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libchrono_tz-5578b1b3d4b34964.rmeta --extern common_arrow=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libcommon_arrow-e0de7486ba0a6128.rmeta --extern common_exception=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libcommon_exception-69bac28c81b11daa.rmeta --extern common_expression=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libcommon_expression-f6964b08160fa55b.rmeta --extern common_hashtable=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libcommon_hashtable-c82d853ce61f0cbb.rmeta --extern common_io=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libcommon_io-63bc0588ce354250.rmeta --extern common_jsonb=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libcommon_jsonb-7be5fc0a27017f60.rmeta --extern crc32fast=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libcrc32fast-53d6145b6d969e82.rmeta --extern hex=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libhex-6c562c28d6a3c74e.rmeta --extern itertools=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libitertools-e578bca5703ccfe1.rmeta --extern match_template=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libmatch_template-1ff2cb48725c4790.dylib --extern num_traits=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libnum_traits-0a03bc806d57203a.rmeta --extern once_cell=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libonce_cell-e711c3afe01bcf2d.rmeta --extern ordered_float=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libordered_float-562585fe5f1e0ad1.rmeta --extern rand=/Users/bohu/github/datafuselabs/databend/target/debug/deps/librand-ee3f86e4709c57b9.rmeta --extern regex=/Users/bohu/github/datafuselabs/databend/target/debug/deps/libregex-0c78c5bb7d23b701.rmeta --extern serde=/Users/bohu/github/datafuselabs/databend/t
