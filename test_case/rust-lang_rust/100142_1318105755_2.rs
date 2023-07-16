
thread '<unnamed>' panicked at 'invalid enum variant tag while decoding `InstanceDef`, expected 0..9', /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/compiler/rustc_middle/src/ty/instance.rs:28:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.66 (e631891 2022-11-13)

query stack during panic:
thread '<unnamed>' panicked at 'Illegal read of: 45108', /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/compiler/rustc_query_system/src/dep_graph/graph.rs:450:25
stack backtrace:
   0:     0x7fc4af655e40 - std::backtrace_rs::backtrace::libunwind::trace::he4d5f2c13fd7ea58
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc4af655e40 - std::backtrace_rs::backtrace::trace_unsynchronized::ha872462b0d7f1993
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc4af655e40 - std::sys_common::backtrace::_print_fmt::hfa57ea34975e92c8
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fc4af655e40 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hdf4fb6a2aa638ae6
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fc4af6b82fe - core::fmt::write::h556296da8255a0fe
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fc4af646185 - std::io::Write::write_fmt::h1e63bc45d9e0b177
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/io/mod.rs:1682:15
   6:     0x7fc4af655c05 - std::sys_common::backtrace::_print::h18ecb2198a8971d0
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fc4af655c05 - std::sys_common::backtrace::print::he62f784f6f099fe2
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fc4af6589af - std::panicking::default_hook::{{closure}}::h220c4bc545c14f7e
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/panicking.rs:267:22
   9:     0x7fc4af6586ea - std::panicking::default_hook::hc6a46490400a51ce
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/panicking.rs:286:9
  10:     0x55f1c14560d0 - clippy_driver[ec9c12c8c9bb3289]::ICE_HOOK::{closure#0}::{closure#0}
  11:     0x7fc4af6591dd - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hba54d23f8fd826e2
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/alloc/src/boxed.rs:2032:9
  12:     0x7fc4af6591dd - std::panicking::rust_panic_with_hook::hec8b21fa74280350
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/panicking.rs:692:13
  13:     0x7fc4af658f57 - std::panicking::begin_panic_handler::{{closure}}::hfc7a6bbbe087d67f
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/panicking.rs:579:13
  14:     0x7fc4af6562ec - std::sys_common::backtrace::__rust_end_short_backtrace::h9522cd483b462760
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7fc4af658c72 - rust_begin_unwind
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/panicking.rs:575:5
  16:     0x7fc4af6b4cd3 - core::panicking::panic_fmt::hed336b84331644c6
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/core/src/panicking.rs:65:14
  17:     0x7fc4b0830f9b - <rustc_middle[ae550a8702fafb50]::dep_graph::dep_node::DepKind as rustc_query_system[bfe2dc31ee70e4dd]::dep_graph::DepKind>::read_deps::<<rustc_query_system[bfe2dc31ee70e4dd]::dep_gr
aph::graph::DepGraph<rustc_middle[ae550a8702fafb50]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  18:     0x7fc4b1fb01e7 - <rustc_middle[ae550a8702fafb50]::ty::print::pretty::FmtPrinter as rustc_middle[ae550a8702fafb50]::ty::print::Printer>::print_def_path
  19:     0x7fc4b1faf548 - <rustc_middle[ae550a8702fafb50]::ty::print::pretty::FmtPrinter as rustc_middle[ae550a8702fafb50]::ty::print::Printer>::print_def_path
  20:     0x7fc4b1457608 - <rustc_middle[ae550a8702fafb50]::ty::context::TyCtxt>::def_path_str_with_substs
  21:     0x7fc4b2cb4a17 - rustc_middle[ae550a8702fafb50]::query::descs::optimized_mir
  22:     0x7fc4b31274f4 - rustc_query_impl[65526abc69128445]::plumbing::create_query_frame::<rustc_span[d217bca3c4113f56]::def_id::DefId>
  23:     0x7fc4b30c3576 - <rustc_query_impl[65526abc69128445]::query_structs::optimized_mir::{closure#0}::{closure#0} as core[8ea5df1f30b7f766]::ops::function::FnOnce<(rustc_query_impl[65526abc69128445]::plu
mbing::QueryCtxt, rustc_span[d217bca3c4113f56]::def_id::DefId)>>::call_once
  24:     0x7fc4b2f8ab22 - <rustc_query_system[bfe2dc31ee70e4dd]::query::plumbing::QueryState<rustc_span[d217bca3c4113f56]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[65526abc69128445]::plumb
ing::QueryCtxt>
  25:     0x7fc4b3042a71 - <rustc_query_impl[65526abc69128445]::Queries>::try_collect_active_jobs
  26:     0x7fc4b30b0d88 - rustc_query_system[bfe2dc31ee70e4dd]::query::job::print_query_stack::<rustc_query_impl[65526abc69128445]::plumbing::QueryCtxt>
  27:     0x7fc4b2b4912e - rustc_interface[c45a0c7151d607d6]::interface::try_print_query_stack
  28:     0x55f1c145661a - clippy_driver[ec9c12c8c9bb3289]::ICE_HOOK::{closure#0}::{closure#0}
  29:     0x7fc4af6591dd - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hba54d23f8fd826e2
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/alloc/src/boxed.rs:2032:9
  30:     0x7fc4af6591dd - std::panicking::rust_panic_with_hook::hec8b21fa74280350
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/panicking.rs:692:13
  31:     0x7fc4af658f11 - std::panicking::begin_panic_handler::{{closure}}::hfc7a6bbbe087d67f
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/panicking.rs:577:13
  32:     0x7fc4af6562ec - std::sys_common::backtrace::__rust_end_short_backtrace::h9522cd483b462760
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/sys_common/backtrace.rs:137:18
  33:     0x7fc4af658c72 - rust_begin_unwind
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/panicking.rs:575:5
  34:     0x7fc4af6b4cd3 - core::panicking::panic_fmt::hed336b84331644c6
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/core/src/panicking.rs:65:14
  35:     0x7fc4b1155322 - <rustc_middle[ae550a8702fafb50]::ty::instance::InstanceDef as rustc_serialize[c4be51bf329e549f]::serialize::Decodable<rustc_query_impl[65526abc69128445]::on_disk_cache::CacheDecoder
>>::decode
  36:     0x7fc4b0f9ecac - <rustc_middle[ae550a8702fafb50]::ty::instance::Instance as rustc_serialize[c4be51bf329e549f]::serialize::Decodable<rustc_query_impl[65526abc69128445]::on_disk_cache::CacheDecoder>>:
:decode
  37:     0x7fc4b0f9e730 - <rustc_query_impl[65526abc69128445]::on_disk_cache::CacheDecoder as rustc_type_ir[bbb82a0498668c5c]::codec::TyDecoder>::decode_alloc_id
  38:     0x7fc4b0f9c732 - <rustc_middle[ae550a8702fafb50]::mir::Constant as rustc_serialize[c4be51bf329e549f]::serialize::Decodable<rustc_query_impl[65526abc69128445]::on_disk_cache::CacheDecoder>>::decode
  39:     0x7fc4b0bf3319 - <(rustc_middle[ae550a8702fafb50]::mir::syntax::Place, rustc_middle[ae550a8702fafb50]::mir::syntax::Rvalue) as rustc_serialize[c4be51bf329e549f]::serialize::Decodable<rustc_query_imp
l[65526abc69128445]::on_disk_cache::CacheDecoder>>::decode
  40:     0x7fc4b11305a0 - <alloc[3e9e8460cb2997a]::vec::Vec<rustc_middle[ae550a8702fafb50]::mir::Statement> as rustc_serialize[c4be51bf329e549f]::serialize::Decodable<rustc_query_impl[65526abc69128445]::on_d
isk_cache::CacheDecoder>>::decode
  41:     0x7fc4b112d8b0 - <alloc[3e9e8460cb2997a]::vec::Vec<rustc_middle[ae550a8702fafb50]::mir::BasicBlockData> as rustc_serialize[c4be51bf329e549f]::serialize::Decodable<rustc_query_impl[65526abc69128445]:
:on_disk_cache::CacheDecoder>>::decode
  42:     0x7fc4b1154657 - <rustc_middle[ae550a8702fafb50]::mir::Body as rustc_serialize[c4be51bf329e549f]::serialize::Decodable<rustc_query_impl[65526abc69128445]::on_disk_cache::CacheDecoder>>::decode
  43:     0x7fc4b115440c - <rustc_query_impl[65526abc69128445]::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle[ae550a8702fafb50]::mir::Body>
  44:     0x7fc4b11529bb - rustc_query_system[bfe2dc31ee70e4dd]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[65526abc69128445]::plumbing::QueryCtxt, rustc_span[d217bca3c4113f56]
::def_id::DefId, &rustc_middle[ae550a8702fafb50]::mir::Body>
  45:     0x7fc4b1151c91 - rustc_query_system[bfe2dc31ee70e4dd]::query::plumbing::try_execute_query::<rustc_query_impl[65526abc69128445]::plumbing::QueryCtxt, rustc_query_system[bfe2dc31ee70e4dd]::query::cach
es::DefaultCache<rustc_span[d217bca3c4113f56]::def_id::DefId, &rustc_middle[ae550a8702fafb50]::mir::Body>>
  46:     0x7fc4b221f3cf - <rustc_query_impl[65526abc69128445]::Queries as rustc_middle[ae550a8702fafb50]::ty::query::QueryEngine>::optimized_mir
  47:     0x55f1c1623122 - <clippy_lints[3f45785eba2282d7]::missing_const_for_fn::MissingConstForFn as rustc_lint[e18403ce832d4098]::passes::LateLintPass>::check_fn
  48:     0x7fc4b2b8faef - <rustc_lint[e18403ce832d4098]::late::LateLintPassObjects as rustc_lint[e18403ce832d4098]::passes::LateLintPass>::check_fn
  49:     0x7fc4b2b3a9da - <rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects> as rustc_hir[e85339c1fe06024d]::intravisit::Visitor>::visit_fn
  50:     0x7fc4b2b50067 - rustc_hir[e85339c1fe06024d]::intravisit::walk_impl_item::<rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects>>
  51:     0x7fc4b2b3b2f1 - <rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects> as rustc_hir[e85339c1fe06024d]::intravisit::Visitor>::visit_nested_i
mpl_item
  52:     0x7fc4b2b5140b - rustc_hir[e85339c1fe06024d]::intravisit::walk_item::<rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects>>
  53:     0x7fc4b2b3afff - <rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects> as rustc_hir[e85339c1fe06024d]::intravisit::Visitor>::visit_nested_i
tem
  54:     0x7fc4b2b50bf7 - rustc_hir[e85339c1fe06024d]::intravisit::walk_mod::<rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects>>
  55:     0x7fc4b2b3afff - <rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects> as rustc_hir[e85339c1fe06024d]::intravisit::Visitor>::visit_nested_i
tem
  56:     0x7fc4b2b50bf7 - rustc_hir[e85339c1fe06024d]::intravisit::walk_mod::<rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects>>
  57:     0x7fc4b2b3afff - <rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects> as rustc_hir[e85339c1fe06024d]::intravisit::Visitor>::visit_nested_i
tem
  58:     0x7fc4b2b50bf7 - rustc_hir[e85339c1fe06024d]::intravisit::walk_mod::<rustc_lint[e18403ce832d4098]::late::LateContextAndPass<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects>>
  59:     0x7fc4b2b3a2a3 - rustc_lint[e18403ce832d4098]::late::late_lint_pass_crate::<rustc_lint[e18403ce832d4098]::late::LateLintPassObjects>
  60:     0x7fc4b1eb237e - rustc_lint[e18403ce832d4098]::late::late_lint_crate::<rustc_lint[e18403ce832d4098]::BuiltinCombinedLateLintPass>
  61:     0x7fc4b1eb151c - <rustc_session[137af93febc05af8]::session::Session>::time::<(), rustc_lint[e18403ce832d4098]::late::check_crate<rustc_lint[e18403ce832d4098]::BuiltinCombinedLateLintPass, rustc_inte
rface[c45a0c7151d607d6]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  62:     0x7fc4b1eb138f - <rustc_session[137af93febc05af8]::session::Session>::time::<(), rustc_interface[c45a0c7151d607d6]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  63:     0x7fc4b1ef7bcf - <core[8ea5df1f30b7f766]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[c45a0c7151d607d6]::passes::analysis::{closure#5}::{closure#1}> as core[8ea5df1f30b7f766]::ops::function
::FnOnce<()>>::call_once
  64:     0x7fc4b0c20577 - <rustc_session[137af93febc05af8]::session::Session>::time::<(), rustc_interface[c45a0c7151d607d6]::passes::analysis::{closure#5}>
  65:     0x7fc4b0c1cf06 - rustc_interface[c45a0c7151d607d6]::passes::analysis
  66:     0x7fc4b20371de - <rustc_query_system[bfe2dc31ee70e4dd]::dep_graph::graph::DepGraph<rustc_middle[ae550a8702fafb50]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ae550a8702fafb50]::ty::con
text::TyCtxt, (), core[8ea5df1f30b7f766]::result::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>>
  67:     0x7fc4b2036554 - rustc_query_system[bfe2dc31ee70e4dd]::query::plumbing::try_execute_query::<rustc_query_impl[65526abc69128445]::plumbing::QueryCtxt, rustc_query_system[bfe2dc31ee70e4dd]::query::cach
es::DefaultCache<(), core[8ea5df1f30b7f766]::result::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>>>
  68:     0x7fc4b2035fd7 - rustc_query_system[bfe2dc31ee70e4dd]::query::plumbing::get_query::<rustc_query_impl[65526abc69128445]::queries::analysis, rustc_query_impl[65526abc69128445]::plumbing::QueryCtxt>
  69:     0x7fc4b1b1a5ae - <rustc_interface[c45a0c7151d607d6]::passes::QueryContext>::enter::<rustc_driver[1827d0eac2d62667]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8ea5df1f30b7f766]::resul
t::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>>
  70:     0x7fc4b1b1778f - <rustc_interface[c45a0c7151d607d6]::interface::Compiler>::enter::<rustc_driver[1827d0eac2d62667]::run_compiler::{closure#1}::{closure#2}, core[8ea5df1f30b7f766]::result::Result<core
[8ea5df1f30b7f766]::option::Option<rustc_interface[c45a0c7151d607d6]::queries::Linker>, rustc_errors[858123638ecad7e2]::ErrorGuaranteed>>
  71:     0x7fc4b1b127e2 - rustc_span[d217bca3c4113f56]::with_source_map::<core[8ea5df1f30b7f766]::result::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>, rustc_interface[c45a0c7151d607d6]::inter
face::run_compiler<core[8ea5df1f30b7f766]::result::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>, rustc_driver[1827d0eac2d62667]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  72:     0x7fc4b1b122bc - <scoped_tls[68462eb9b30a08d0]::ScopedKey<rustc_span[d217bca3c4113f56]::SessionGlobals>>::set::<rustc_interface[c45a0c7151d607d6]::interface::run_compiler<core[8ea5df1f30b7f766]::res
ult::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>, rustc_driver[1827d0eac2d62667]::run_compiler::{closure#1}>::{closure#0}, core[8ea5df1f30b7f766]::result::Result<(), rustc_errors[858123638ecad
7e2]::ErrorGuaranteed>>
  73:     0x7fc4b1b118a8 - std[772581ffc1b79bff]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c45a0c7151d607d6]::util::run_in_thread_pool_with_globals<rustc_interface[c45a0c7151d607d
6]::interface::run_compiler<core[8ea5df1f30b7f766]::result::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>, rustc_driver[1827d0eac2d62667]::run_compiler::{closure#1}>::{closure#0}, core[8ea5df1f3
0b7f766]::result::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8ea5df1f30b7f766]::result::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>>
  74:     0x7fc4b1b115cc - <<std[772581ffc1b79bff]::thread::Builder>::spawn_unchecked_<rustc_interface[c45a0c7151d607d6]::util::run_in_thread_pool_with_globals<rustc_interface[c45a0c7151d607d6]::interface::ru
n_compiler<core[8ea5df1f30b7f766]::result::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>, rustc_driver[1827d0eac2d62667]::run_compiler::{closure#1}>::{closure#0}, core[8ea5df1f30b7f766]::result:
:Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8ea5df1f30b7f766]::result::Result<(), rustc_errors[858123638ecad7e2]::ErrorGuaranteed>>::{closure#1} as core[8ea5d
f1f30b7f766]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  75:     0x7fc4b3572153 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb9beccc25c64a001
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/alloc/src/boxed.rs:2000:9
  76:     0x7fc4b3572153 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hde98428e87ac1a81
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/alloc/src/boxed.rs:2000:9
  77:     0x7fc4b3572153 - std::sys::unix::thread::Thread::new::thread_start::h2ef156d73deb51c1
                               at /rustc/e631891f7ad40eac3ef58ec3c2b57ecd81e40615/library/std/src/sys/unix/thread.rs:108:17
  78:     0x7fc4af2d58fd - <unknown>
  79:     0x7fc4af357a60 - <unknown>
  80:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.66 (e631891 2022-11-13)

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: could not compile `common-functions-v2`

Caused by:
  process didn't exit successfully: `/home/cl/.rustup/toolchains/nightly-2022-11-14-x86_64-unknown-linux-gnu/bin/clippy-driver rustc --crate-name common_functions_v2 --edition=2021 src/query/functions-v2/src/
lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C overflow-checks
=off -C metadata=615a3c323fce9709 -C extra-filename=-615a3c323fce9709 --out-dir /home/cl/CLionProjects/databend/target/debug/deps -C incremental=/home/cl/CLionProjects/databend/target/debug/incremental -L dep
endency=/home/cl/CLionProjects/databend/target/debug/deps --extern base64=/home/cl/CLionProjects/databend/target/debug/deps/libbase64-e38d551faf6d3907.rmeta --extern bstr=/home/cl/CLionProjects/databend/targe
t/debug/deps/libbstr-657bd64777e53bd1.rmeta --extern bumpalo=/home/cl/CLionProjects/databend/target/debug/deps/libbumpalo-33016bc2bc879a0a.rmeta --extern bytes=/home/cl/CLionProjects/databend/target/debug/dep
s/libbytes-1e04e98f0a2f5af7.rmeta --extern chrono=/home/cl/CLionProjects/databend/target/debug/deps/libchrono-78abd8a49e759e09.rmeta --extern chrono_tz=/home/cl/CLionProjects/databend/target/debug/deps/libchr
ono_tz-21ca4cdad0ffd067.rmeta --extern common_arrow=/home/cl/CLionProjects/databend/target/debug/deps/libcommon_arrow-cb46578c33cb7c69.rmeta --extern common_exception=/home/cl/CLionProjects/databend/target/de
bug/deps/libcommon_exception-3f0b1e82375a2d0a.rmeta --extern common_expression=/home/cl/CLionProjects/databend/target/debug/deps/libcommon_expression-9e19decb681f9e7f.rmeta --extern common_hashtable=/home/cl/
CLionProjects/databend/target/debug/deps/libcommon_hashtable-91816baf5ba0aad9.rmeta --extern common_io=/home/cl/CLionProjects/databend/target/debug/deps/libcommon_io-ee79fd45a5443c40.rmeta --extern common_jso
nb=/home/cl/CLionProjects/databend/target/debug/deps/libcommon_jsonb-e4324132a2dd0a61.rmeta --extern crc32fast=/home/cl/CLionProjects/databend/target/debug/deps/libcrc32fast-a042aa740831c362.rmeta --extern cr
iterion=/home/cl/CLionProjects/databend/target/debug/deps/libcriterion-7eade58d137b13c1.rmeta --extern hex=/home/cl/CLionProjects/databend/target/debug/deps/libhex-309ba064ea997439.rmeta --extern itertools=/h
ome/cl/CLionProjects/databend/target/debug/deps/libitertools-c5ce1a1b86a5c4ef.rmeta --extern match_template=/home/cl/CLionProjects/databend/target/debug/deps/libmatch_template-012a358e866b9b01.so --extern mem
chr=/home/cl/CLionProjects/databend/target/debug/deps/libmemchr-bd604efcf8399c00.rmeta --extern num_traits=/home/cl/CLionProjects/databend/target/debug/deps/libnum_traits-3a311544538a5962.rmeta --extern once_
cell=/home/cl/CLionProjects/databend/target/debug/deps/libonce_cell-8da6723aa3a86b85.rmeta --extern ordered_float=/home/cl/CLionProjects/databend/target/debug/deps/libordered_float-909cf6d35d1710ce.rmeta --ex
tern rand=/home/cl/CLionProjects/databend/target/debug/deps/librand-ecc9555a5444be87.rmeta --extern regex=/home/cl/CLionProjects/databend/target/debug/deps/libregex-0713249e23b47d37.rmeta --extern serde=/home
/cl/CLionProjects/databend/target/debug/deps/libserde-f80cd46cc7696880.rmeta --extern simdutf8=/home/cl/CLionProjects/databend/target/debug/deps/libsimdutf8-dce636f6acfb3bde.rmeta --extern siphasher=/home/cl/
CLionProjects/databend/target/debug/deps/libsiphasher-670bacc8c7472c2b.rmeta --extern strength_reduce=/home/cl/CLionProjects/databend/target/debug/deps/libstrength_reduce-aa4f997217f1072d.rmeta -C link-arg=-f
use-ld=mold -L native=/home/cl/CLionProjects/databend/target/debug/build/ring-376a514dac689073/out -L native=/home/cl/CLionProjects/databend/target/debug/build/libz-ng-sys-45e675b82dec454d/out/lib -L native=/
home/cl/CLionProjects/databend/target/debug/build/lz4-sys-d5ba9e170e9edf08/out -L native=/home/cl/CLionProjects/databend/target/debug/build/zstd-sys-f0e7dd9d23cca1c2/out -L native=/home/cl/CLionProjects/datab
end/target/debug/build/tikv-jemalloc-sys-2bd0643b3901aeab/out/build/lib` (signal: 6, SIGABRT: process abort signal)
