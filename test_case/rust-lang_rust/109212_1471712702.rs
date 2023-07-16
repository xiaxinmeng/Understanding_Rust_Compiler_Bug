plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Successfully built ba13c1767e94
Successfully tagged rust-ci:latest
Built container sha256:ba13c1767e949fc1359ab92b43d7c46885647c185962320910cb9de0e4c0cb96
Uploading finished image to https://ci-caches.rust-lang.org/docker/fa16b54262d00a378eee0b4414848c5f43b18d4834e4bed4c4343f19aa40bf2d28e70ed404889b52718c7269e7e82882fdc9e90fcf2e71af8c5d6ce646f6a1f8
upload failed: - to s3://rust-lang-ci-sccache2/docker/fa16b54262d00a378eee0b4414848c5f43b18d4834e4bed4c4343f19aa40bf2d28e70ed404889b52718c7269e7e82882fdc9e90fcf2e71af8c5d6ce646f6a1f8 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-14]
---
running 14646 tests
..........................................ii............................................ 88/14646
.................................................................................iiiiiii 176/14646
iiiiiiii.....................i..................i....................................... 264/14646
..............................................F.....................................F... 352/14646
..................................................................F..................... 440/14646
........................................................................................ 528/14646
................F..F.................................................................... 616/14646
........................................................................................ 792/14646
........................................................................................ 880/14646
.......................................................................i................ 968/14646
........................................................................................ 1056/14646
---
...............................................................F........................ 4224/14646
.............F......................................i..........i..........i............. 4312/14646
........................................................................................ 4400/14646
.................................................iii.................................... 4488/14646
.......................................F.F.............................................. 4576/14646
........................................................................................ 4752/14646
........................................................................................ 4840/14646
........................................................................................ 4928/14646
........................................................................................ 5016/14646
........................................................................................ 5016/14646
.......................................i................................................ 5104/14646
........................................................................................ 5192/14646
.........................................F......................................F....... 5280/14646
........................................................................................ 5456/14646
........................................................................................ 5544/14646
.............................................F.......................................... 5632/14646
........................................................................................ 5720/14646
---
........................................................................................ 6336/14646
........................................................................................ 6424/14646
........................................................................................ 6512/14646
........................................................................................ 6600/14646
..................FF...........F........................................................ 6688/14646
........................................................................................ 6864/14646
........................................F...............................i............... 6952/14646
........................................................................................ 7040/14646
..............................................i......................................... 7128/14646
---
.................................................F...................................... 7920/14646
........................................................................................ 8008/14646
........................................................................................ 8096/14646
...................ii...............................................ii.................. 8184/14646
....................................F...........i....................................... 8272/14646
.........................F.................................................F............ 8360/14646
........................................................................................ 8536/14646
........................................................................................ 8624/14646
........................................................................................ 8712/14646
.....................ii............F....i......i..ii.................................... 8800/14646
---
........................................................................................ 10208/14646
........................................................................................ 10296/14646
.............................................................................FF......... 10384/14646
........................................................................................ 10472/14646
..............................................................F....FF................... 10560/14646
............................F...................................F....................... 10648/14646
......................F.........................................................F....ii. 10736/14646
........F.....i.....iii.......................F.............................F........... 10824/14646
........................................................F............................... 11000/14646
........................................................................................ 11088/14646
........................................................................................ 11176/14646
........................................................................................ 11176/14646
...................................F............................F..F.......F......F..... 11264/14646
.............F..........F........................F...F..F.FF............................ 11352/14646
.................................................................................iiiii.. 11528/14646
.i....i.i............................................................................... 11616/14646
........................................................................................ 11704/14646
......i................................................................................. 11792/14646
---
........................................................................................ 12760/14646
..................................................................................i..... 12848/14646
......i........i.....i......................i........................................... 12936/14646
........................................................................................ 13024/14646
........................................................F............................... 13112/14646
............................F..............................................F............ 13200/14646
...................F.....................F.............................................. 13288/14646
........................................................................................ 13464/14646
........................................................................................ 13552/14646
........................................................................................ 13640/14646
..................................................................i..................... 13728/14646
---
failures:

---- [ui] tests/ui/associated-consts/issue-93835.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-consts/issue-93835.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-93835" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-93835/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/job.rs:141:44
   0:     0x7f114cb34af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   1:     0x7f114cba1cc8 - core::fmt::write::h5b4b993aacaf08ed
   2:     0x7f114cb29271 - std::io::Write::write_fmt::hf420647c1429c197
   3:     0x7f114cb34901 - std::sys_common::backtrace::print::h39b64acc765d9915
   3:     0x7f114cb34901 - std::sys_common::backtrace::print::h39b64acc765d9915
   4:     0x7f114cb37ad4 - std::panicking::default_hook::{{closure}}::h984ccf8c48efd9dd
   5:     0x7f114cb377ba - std::panicking::default_hook::he22f59932909c7ec
   6:     0x7f114d638f75 - rustc_driver_impl[f2eb435b2e98c66d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f114cb381f1 - std::panicking::rust_panic_with_hook::h53b87e485107618d
   8:     0x7f114cb37f2a - std::panicking::begin_panic_handler::{{closure}}::h4e1d7611ee576041
   9:     0x7f114cb34fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ea49fc2484af23b
  10:     0x7f114cb37c47 - rust_begin_unwind
  11:     0x7f114caee2c3 - core::panicking::panic_fmt::hb860c5b6d0410325
  12:     0x7f114caee35d - core::panicking::panic::h1309072cc4b1b0e7
  13:     0x7f114d4baa4c - <rustc_query_system[5fe272afb00b13fa]::query::job::QueryJobId>::find_cycle_in_stack::<rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>
  14:     0x7f114f4a4b82 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  15:     0x7f114f256874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  16:     0x7f11502c5bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  17:     0x7f11502b9774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  18:     0x7f115043e7b7 - rustc_middle[b78252b658aef7b]::query::descs::lookup_stability
  19:     0x7f114f10cabe - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_span[eaad1e142fe170d9]::def_id::DefId>
  20:     0x7f114f03f139 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::lookup_stability::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  21:     0x7f114f2ec8c4 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_span[eaad1e142fe170d9]::def_id::DefId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  22:     0x7f114f1afc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  23:     0x7f114f4a4b37 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  24:     0x7f114f256874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  25:     0x7f11502c5bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  26:     0x7f11502b9774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  27:     0x7f11504373d1 - rustc_middle[b78252b658aef7b]::query::descs::hir_attrs
  28:     0x7f114f10c498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  29:     0x7f114f041fe3 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_attrs::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  30:     0x7f114f2ec540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  31:     0x7f114f1afc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  32:     0x7f114f4b68a4 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  33:     0x7f114f1b8c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  34:     0x7f11504cd3d6 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#5} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  35:     0x7f114f4b461e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_attrs, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  36:     0x7f114f1bca89 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_attrs
  37:     0x7f115041abb8 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::attrs
  38:     0x7f1150334054 - <rustc_middle[b78252b658aef7b]::middle::limits::provide::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, ())>>::call_once
  39:     0x7f114f4a3e1e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  40:     0x7f114f256874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  41:     0x7f11502c5bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  42:     0x7f11502b9774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  43:     0x7f1150436fd1 - rustc_middle[b78252b658aef7b]::query::descs::hir_owner
  44:     0x7f114f10c498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  45:     0x7f114f042053 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_owner::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  46:     0x7f114f2ec540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  47:     0x7f114f1afc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  48:     0x7f114f40e941 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  49:     0x7f114f1b7aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  50:     0x7f114d6f58e7 - rustc_interface[ab8a3f336018d81b]::passes::output_filenames
  51:     0x7f114f39a9dd - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::output_filenames, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  52:     0x7f114f247c73 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::output_filenames
  53:     0x7f114e2cd043 - rustc_ast_lowering[3d1fce858f29a9f9]::lower_to_hir
  54:     0x7f114f4b6632 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  55:     0x7f114f1b8c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  56:     0x7f11504cdd46 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  57:     0x7f114f4b71ac - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_owner, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  58:     0x7f114f1ba695 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_owner
  59:     0x7f1150417030 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::find
  60:     0x7f115041ae63 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::opt_span
  61:     0x7f115041adbd - <rustc_middle[b78252b658aef7b]::hir::map::Map>::span
  62:     0x7f114e62e928 - rustc_passes[e839ac6b29e77bf7]::stability::stability_index
  63:     0x7f114f37b4d2 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::stability_index, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  64:     0x7f114f241b13 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::stability_index
  65:     0x7f11502c2bd7 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::stability
  66:     0x7f114e621228 - <rustc_passes[e839ac6b29e77bf7]::stability::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  67:     0x7f114f394071 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::lookup_stability, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  68:     0x7f114f206b46 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::lookup_stability
  69:     0x7f11502b51fd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability_allow_unstable
  70:     0x7f11502b4e17 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability
  71:     0x7f114e562f81 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::add_typo_suggestion
  72:     0x7f114e47e114 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_report_errors
  73:     0x7f114e4cfb08 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment::{closure#0}
  74:     0x7f114e4cc91e - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  75:     0x7f114e4c02e6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path
  76:     0x7f114e4c2a2c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_expr
  77:     0x7f114e5d6354 - rustc_ast[2b4c606609dd0b28]::visit::walk_expr::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  78:     0x7f114e4c2873 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_expr
  79:     0x7f114e489bef - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_block
  80:     0x7f114e498d8f - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_fn
  81:     0x7f114e5da707 - rustc_ast[2b4c606609dd0b28]::visit::walk_item::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  82:     0x7f114e4aab70 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_item
  83:     0x7f114e48926c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_item
  84:     0x7f114e5c00fd - rustc_ast[2b4c606609dd0b28]::visit::walk_crate::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  85:     0x7f114e57128e - <rustc_resolve[f303ed5d69cd7988]::Resolver>::late_resolve_crate
  86:     0x7f114e4e4e02 - <rustc_session[c2c6e323d951d754]::session::Session>::time::<(), <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate::{closure#0}>
  87:     0x7f114e57fc0d - <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate
  88:     0x7f114d6f4e30 - rustc_interface[ab8a3f336018d81b]::passes::resolver_for_lowering
  89:     0x7f114f40e809 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  90:     0x7f114f1b7aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  91:     0x7f114d63b210 - <rustc_middle[b78252b658aef7b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a1b30124f59aa5b6]::steal::Steal<(rustc_middle[b78252b658aef7b]::ty::ResolverAstLowering, alloc[6af766aba38ed60]::rc::Rc<rustc_ast[2b4c606609dd0b28]::ast::Crate>)>>
  92:     0x7f114d68017d - <rustc_interface[ab8a3f336018d81b]::interface::Compiler>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}, core[3c115420e38098da]::result::Result<core[3c115420e38098da]::option::Option<rustc_interface[ab8a3f336018d81b]::queries::Linker>, rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  93:     0x7f114d63a238 - rustc_span[eaad1e142fe170d9]::with_source_map::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  94:     0x7f114d674a97 - <scoped_tls[e8a8b4df10b4528b]::ScopedKey<rustc_span[eaad1e142fe170d9]::SessionGlobals>>::set::<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  95:     0x7f114d6530b6 - std[c748adf407831b33]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  96:     0x7f114d622bb6 - std[c748adf407831b33]::panicking::try::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, core[3c115420e38098da]::panic::unwind_safe::AssertUnwindSafe<<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  97:     0x7f114d648c95 - <<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  98:     0x7f114cb444de - std::sys::unix::thread::Thread::new::thread_start::h3446ed5cb7c819ff
  99:     0x7f114c8deb43 - <unknown>
 100:     0x7f114c970a00 - <unknown>
 101:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (0e53f3007 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/associated-item/issue-87638.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-item/issue-87638.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/issue-87638" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/issue-87638/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/job.rs:141:44
   0:     0x7fc5c87a4af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   0:     0x7fc5c87a4af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   1:     0x7fc5c8811cc8 - core::fmt::write::h5b4b993aacaf08ed
   2:     0x7fc5c8799271 - std::io::Write::write_fmt::hf420647c1429c197
   3:     0x7fc5c87a4901 - std::sys_common::backtrace::print::h39b64acc765d9915
   4:     0x7fc5c87a7ad4 - std::panicking::default_hook::{{closure}}::h984ccf8c48efd9dd
   5:     0x7fc5c87a77ba - std::panicking::default_hook::he22f59932909c7ec
   6:     0x7fc5c92a8f75 - rustc_driver_impl[f2eb435b2e98c66d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc5c87a81f1 - std::panicking::rust_panic_with_hook::h53b87e485107618d
   8:     0x7fc5c87a7f2a - std::panicking::begin_panic_handler::{{closure}}::h4e1d7611ee576041
   9:     0x7fc5c87a4fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ea49fc2484af23b
  10:     0x7fc5c87a7c47 - rust_begin_unwind
  11:     0x7fc5c875e2c3 - core::panicking::panic_fmt::hb860c5b6d0410325
  12:     0x7fc5c875e35d - core::panicking::panic::h1309072cc4b1b0e7
  13:     0x7fc5c912aa4c - <rustc_query_system[5fe272afb00b13fa]::query::job::QueryJobId>::find_cycle_in_stack::<rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>
  14:     0x7fc5cb114b82 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  15:     0x7fc5caec6874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  16:     0x7fc5cbf35bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  17:     0x7fc5cbf29774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  18:     0x7fc5cc0ae7b7 - rustc_middle[b78252b658aef7b]::query::descs::lookup_stability
  19:     0x7fc5cad7cabe - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_span[eaad1e142fe170d9]::def_id::DefId>
  20:     0x7fc5cacaf139 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::lookup_stability::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  21:     0x7fc5caf5c8c4 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_span[eaad1e142fe170d9]::def_id::DefId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  22:     0x7fc5cae1fc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  23:     0x7fc5cb114b37 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  24:     0x7fc5caec6874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  25:     0x7fc5cbf35bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  26:     0x7fc5cbf29774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  27:     0x7fc5cc0a73d1 - rustc_middle[b78252b658aef7b]::query::descs::hir_attrs
  28:     0x7fc5cad7c498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  29:     0x7fc5cacb1fe3 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_attrs::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  30:     0x7fc5caf5c540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  31:     0x7fc5cae1fc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  32:     0x7fc5cb1268a4 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  33:     0x7fc5cae28c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  34:     0x7fc5cc13d3d6 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#5} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  35:     0x7fc5cb12461e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_attrs, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  36:     0x7fc5cae2ca89 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_attrs
  37:     0x7fc5cc08abb8 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::attrs
  38:     0x7fc5cbfa4054 - <rustc_middle[b78252b658aef7b]::middle::limits::provide::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, ())>>::call_once
  39:     0x7fc5cb113e1e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  40:     0x7fc5caec6874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  41:     0x7fc5cbf35bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  42:     0x7fc5cbf29774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  43:     0x7fc5cc0a6fd1 - rustc_middle[b78252b658aef7b]::query::descs::hir_owner
  44:     0x7fc5cad7c498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  45:     0x7fc5cacb2053 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_owner::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  46:     0x7fc5caf5c540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  47:     0x7fc5cae1fc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  48:     0x7fc5cb07e941 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  49:     0x7fc5cae27aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  50:     0x7fc5c93658e7 - rustc_interface[ab8a3f336018d81b]::passes::output_filenames
  51:     0x7fc5cb00a9dd - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::output_filenames, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  52:     0x7fc5caeb7c73 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::output_filenames
  53:     0x7fc5c9f3d043 - rustc_ast_lowering[3d1fce858f29a9f9]::lower_to_hir
  54:     0x7fc5cb126632 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  55:     0x7fc5cae28c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  56:     0x7fc5cc13dd46 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  57:     0x7fc5cb1271ac - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_owner, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  58:     0x7fc5cae2a695 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_owner
  59:     0x7fc5cc087030 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::find
  60:     0x7fc5cc08ae63 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::opt_span
  61:     0x7fc5cc08adbd - <rustc_middle[b78252b658aef7b]::hir::map::Map>::span
  62:     0x7fc5ca29e928 - rustc_passes[e839ac6b29e77bf7]::stability::stability_index
  63:     0x7fc5cafeb4d2 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::stability_index, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  64:     0x7fc5caeb1b13 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::stability_index
  65:     0x7fc5cbf32bd7 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::stability
  66:     0x7fc5ca291228 - <rustc_passes[e839ac6b29e77bf7]::stability::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  67:     0x7fc5cb004071 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::lookup_stability, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  68:     0x7fc5cae76b46 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::lookup_stability
  69:     0x7fc5cbf251fd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability_allow_unstable
  70:     0x7fc5cbf24e17 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability
  71:     0x7fc5ca1d2f81 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::add_typo_suggestion
  72:     0x7fc5ca0ee41f - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_report_errors
  73:     0x7fc5ca13fb08 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment::{closure#0}
  74:     0x7fc5ca13c91e - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  75:     0x7fc5ca13bdf2 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  76:     0x7fc5ca1302e6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path
  77:     0x7fc5ca0fb948 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_ty
  78:     0x7fc5ca0fb3f6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_local
  79:     0x7fc5ca0f9bef - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_block
  80:     0x7fc5ca108d8f - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_fn
  81:     0x7fc5ca24a707 - rustc_ast[2b4c606609dd0b28]::visit::walk_item::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  82:     0x7fc5ca11ab70 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_item
  83:     0x7fc5ca0f926c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_item
  84:     0x7fc5ca2300fd - rustc_ast[2b4c606609dd0b28]::visit::walk_crate::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  85:     0x7fc5ca1e128e - <rustc_resolve[f303ed5d69cd7988]::Resolver>::late_resolve_crate
  86:     0x7fc5ca154e02 - <rustc_session[c2c6e323d951d754]::session::Session>::time::<(), <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate::{closure#0}>
  87:     0x7fc5ca1efc0d - <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate
  88:     0x7fc5c9364e30 - rustc_interface[ab8a3f336018d81b]::passes::resolver_for_lowering
  89:     0x7fc5cb07e809 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  90:     0x7fc5cae27aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  91:     0x7fc5c92ab210 - <rustc_middle[b78252b658aef7b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a1b30124f59aa5b6]::steal::Steal<(rustc_middle[b78252b658aef7b]::ty::ResolverAstLowering, alloc[6af766aba38ed60]::rc::Rc<rustc_ast[2b4c606609dd0b28]::ast::Crate>)>>
  92:     0x7fc5c92f017d - <rustc_interface[ab8a3f336018d81b]::interface::Compiler>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}, core[3c115420e38098da]::result::Result<core[3c115420e38098da]::option::Option<rustc_interface[ab8a3f336018d81b]::queries::Linker>, rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  93:     0x7fc5c92aa238 - rustc_span[eaad1e142fe170d9]::with_source_map::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  94:     0x7fc5c92e4a97 - <scoped_tls[e8a8b4df10b4528b]::ScopedKey<rustc_span[eaad1e142fe170d9]::SessionGlobals>>::set::<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  95:     0x7fc5c92c30b6 - std[c748adf407831b33]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  96:     0x7fc5c9292bb6 - std[c748adf407831b33]::panicking::try::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, core[3c115420e38098da]::panic::unwind_safe::AssertUnwindSafe<<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  97:     0x7fc5c92b8c95 - <<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  98:     0x7fc5c87b44de - std::sys::unix::thread::Thread::new::thread_start::h3446ed5cb7c819ff
  99:     0x7fc5c854eb43 - <unknown>
 100:     0x7fc5c85e0a00 - <unknown>
 101:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (0e53f3007 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/associated-types/associated-types-eq-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-types/associated-types-eq-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-1/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/job.rs:141:44
   0:     0x7f810e5d0af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   1:     0x7f810e63dcc8 - core::fmt::write::h5b4b993aacaf08ed
   2:     0x7f810e5c5271 - std::io::Write::write_fmt::hf420647c1429c197
   3:     0x7f810e5d0901 - std::sys_common::backtrace::print::h39b64acc765d9915
   3:     0x7f810e5d0901 - std::sys_common::backtrace::print::h39b64acc765d9915
   4:     0x7f810e5d3ad4 - std::panicking::default_hook::{{closure}}::h984ccf8c48efd9dd
   5:     0x7f810e5d37ba - std::panicking::default_hook::he22f59932909c7ec
   6:     0x7f810f0d4f75 - rustc_driver_impl[f2eb435b2e98c66d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f810e5d41f1 - std::panicking::rust_panic_with_hook::h53b87e485107618d
   8:     0x7f810e5d3f2a - std::panicking::begin_panic_handler::{{closure}}::h4e1d7611ee576041
   9:     0x7f810e5d0fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ea49fc2484af23b
  10:     0x7f810e5d3c47 - rust_begin_unwind
  11:     0x7f810e58a2c3 - core::panicking::panic_fmt::hb860c5b6d0410325
  12:     0x7f810e58a35d - core::panicking::panic::h1309072cc4b1b0e7
  13:     0x7f810ef56a4c - <rustc_query_system[5fe272afb00b13fa]::query::job::QueryJobId>::find_cycle_in_stack::<rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>
  14:     0x7f8110f40b82 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  15:     0x7f8110cf2874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  16:     0x7f8111d61bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  17:     0x7f8111d55774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  18:     0x7f8111eda7b7 - rustc_middle[b78252b658aef7b]::query::descs::lookup_stability
  19:     0x7f8110ba8abe - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_span[eaad1e142fe170d9]::def_id::DefId>
  20:     0x7f8110adb139 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::lookup_stability::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  21:     0x7f8110d888c4 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_span[eaad1e142fe170d9]::def_id::DefId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  22:     0x7f8110c4bc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  23:     0x7f8110f40b37 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  24:     0x7f8110cf2874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  25:     0x7f8111d61bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  26:     0x7f8111d55774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  27:     0x7f8111ed33d1 - rustc_middle[b78252b658aef7b]::query::descs::hir_attrs
  28:     0x7f8110ba8498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  29:     0x7f8110addfe3 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_attrs::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  30:     0x7f8110d88540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  31:     0x7f8110c4bc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  32:     0x7f8110f528a4 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  33:     0x7f8110c54c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  34:     0x7f8111f693d6 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#5} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  35:     0x7f8110f5061e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_attrs, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  36:     0x7f8110c58a89 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_attrs
  37:     0x7f8111eb6bb8 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::attrs
  38:     0x7f8111dd0054 - <rustc_middle[b78252b658aef7b]::middle::limits::provide::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, ())>>::call_once
  39:     0x7f8110f3fe1e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  40:     0x7f8110cf2874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  41:     0x7f8111d61bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  42:     0x7f8111d55774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  43:     0x7f8111ed2fd1 - rustc_middle[b78252b658aef7b]::query::descs::hir_owner
  44:     0x7f8110ba8498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  45:     0x7f8110ade053 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_owner::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  46:     0x7f8110d88540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  47:     0x7f8110c4bc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  48:     0x7f8110eaa941 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  49:     0x7f8110c53aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  50:     0x7f810f1918e7 - rustc_interface[ab8a3f336018d81b]::passes::output_filenames
  51:     0x7f8110e369dd - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::output_filenames, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  52:     0x7f8110ce3c73 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::output_filenames
  53:     0x7f810fd69043 - rustc_ast_lowering[3d1fce858f29a9f9]::lower_to_hir
  54:     0x7f8110f52632 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  55:     0x7f8110c54c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  56:     0x7f8111f69d46 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  57:     0x7f8110f531ac - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_owner, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  58:     0x7f8110c56695 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_owner
  59:     0x7f8111eb3030 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::find
  60:     0x7f8111eb6e63 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::opt_span
  61:     0x7f8111eb6dbd - <rustc_middle[b78252b658aef7b]::hir::map::Map>::span
  62:     0x7f81100ca928 - rustc_passes[e839ac6b29e77bf7]::stability::stability_index
  63:     0x7f8110e174d2 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::stability_index, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  64:     0x7f8110cddb13 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::stability_index
  65:     0x7f8111d5ebd7 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::stability
  66:     0x7f81100bd228 - <rustc_passes[e839ac6b29e77bf7]::stability::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  67:     0x7f8110e30071 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::lookup_stability, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  68:     0x7f8110ca2b46 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::lookup_stability
  69:     0x7f8111d511fd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability_allow_unstable
  70:     0x7f8111d50e17 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability
  71:     0x7f810fffef81 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::add_typo_suggestion
  72:     0x7f810ff1a114 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_report_errors
  73:     0x7f810ff6bb08 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment::{closure#0}
  74:     0x7f810ff6891e - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  75:     0x7f810ff5c2e6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path
  76:     0x7f810ff27948 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_ty
  77:     0x7f810ff273f6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_local
  78:     0x7f810ff25bef - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_block
  79:     0x7f810ff34d8f - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_fn
  80:     0x7f8110076707 - rustc_ast[2b4c606609dd0b28]::visit::walk_item::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  81:     0x7f810ff46b70 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_item
  82:     0x7f810ff2526c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_item
  83:     0x7f811005c0fd - rustc_ast[2b4c606609dd0b28]::visit::walk_crate::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  84:     0x7f811000d28e - <rustc_resolve[f303ed5d69cd7988]::Resolver>::late_resolve_crate
  85:     0x7f810ff80e02 - <rustc_session[c2c6e323d951d754]::session::Session>::time::<(), <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate::{closure#0}>
  86:     0x7f811001bc0d - <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate
  87:     0x7f810f190e30 - rustc_interface[ab8a3f336018d81b]::passes::resolver_for_lowering
  88:     0x7f8110eaa809 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  89:     0x7f8110c53aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  90:     0x7f810f0d7210 - <rustc_middle[b78252b658aef7b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a1b30124f59aa5b6]::steal::Steal<(rustc_middle[b78252b658aef7b]::ty::ResolverAstLowering, alloc[6af766aba38ed60]::rc::Rc<rustc_ast[2b4c606609dd0b28]::ast::Crate>)>>
  91:     0x7f810f11c17d - <rustc_interface[ab8a3f336018d81b]::interface::Compiler>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}, core[3c115420e38098da]::result::Result<core[3c115420e38098da]::option::Option<rustc_interface[ab8a3f336018d81b]::queries::Linker>, rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  92:     0x7f810f0d6238 - rustc_span[eaad1e142fe170d9]::with_source_map::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  93:     0x7f810f110a97 - <scoped_tls[e8a8b4df10b4528b]::ScopedKey<rustc_span[eaad1e142fe170d9]::SessionGlobals>>::set::<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  94:     0x7f810f0ef0b6 - std[c748adf407831b33]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  95:     0x7f810f0bebb6 - std[c748adf407831b33]::panicking::try::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, core[3c115420e38098da]::panic::unwind_safe::AssertUnwindSafe<<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  96:     0x7f810f0e4c95 - <<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  97:     0x7f810e5e04de - std::sys::unix::thread::Thread::new::thread_start::h3446ed5cb7c819ff
  98:     0x7f810e37ab43 - <unknown>
  99:     0x7f810e40ca00 - <unknown>
 100:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (0e53f3007 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/associated-types/issue-19883.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-types/issue-19883.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-19883" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-19883/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/job.rs:141:44
   0:     0x7fb11f802af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   1:     0x7fb11f86fcc8 - core::fmt::write::h5b4b993aacaf08ed
   2:     0x7fb11f7f7271 - std::io::Write::write_fmt::hf420647c1429c197
   3:     0x7fb11f802901 - std::sys_common::backtrace::print::h39b64acc765d9915
   3:     0x7fb11f802901 - std::sys_common::backtrace::print::h39b64acc765d9915
   4:     0x7fb11f805ad4 - std::panicking::default_hook::{{closure}}::h984ccf8c48efd9dd
   5:     0x7fb11f8057ba - std::panicking::default_hook::he22f59932909c7ec
   6:     0x7fb120306f75 - rustc_driver_impl[f2eb435b2e98c66d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb11f8061f1 - std::panicking::rust_panic_with_hook::h53b87e485107618d
   8:     0x7fb11f805f2a - std::panicking::begin_panic_handler::{{closure}}::h4e1d7611ee576041
   9:     0x7fb11f802fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ea49fc2484af23b
  10:     0x7fb11f805c47 - rust_begin_unwind
  11:     0x7fb11f7bc2c3 - core::panicking::panic_fmt::hb860c5b6d0410325
  12:     0x7fb11f7bc35d - core::panicking::panic::h1309072cc4b1b0e7
  13:     0x7fb120188a4c - <rustc_query_system[5fe272afb00b13fa]::query::job::QueryJobId>::find_cycle_in_stack::<rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>
  14:     0x7fb122172b82 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  15:     0x7fb121f24874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  16:     0x7fb122f93bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  17:     0x7fb122f87774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  18:     0x7fb12310c7b7 - rustc_middle[b78252b658aef7b]::query::descs::lookup_stability
  19:     0x7fb121ddaabe - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_span[eaad1e142fe170d9]::def_id::DefId>
  20:     0x7fb121d0d139 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::lookup_stability::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  21:     0x7fb121fba8c4 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_span[eaad1e142fe170d9]::def_id::DefId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  22:     0x7fb121e7dc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  23:     0x7fb122172b37 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  24:     0x7fb121f24874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  25:     0x7fb122f93bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  26:     0x7fb122f87774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  27:     0x7fb1231053d1 - rustc_middle[b78252b658aef7b]::query::descs::hir_attrs
  28:     0x7fb121dda498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  29:     0x7fb121d0ffe3 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_attrs::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  30:     0x7fb121fba540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  31:     0x7fb121e7dc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  32:     0x7fb1221848a4 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  33:     0x7fb121e86c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  34:     0x7fb12319b3d6 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#5} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  35:     0x7fb12218261e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_attrs, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  36:     0x7fb121e8aa89 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_attrs
  37:     0x7fb1230e8bb8 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::attrs
  38:     0x7fb123002054 - <rustc_middle[b78252b658aef7b]::middle::limits::provide::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, ())>>::call_once
  39:     0x7fb122171e1e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  40:     0x7fb121f24874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  41:     0x7fb122f93bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  42:     0x7fb122f87774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  43:     0x7fb123104fd1 - rustc_middle[b78252b658aef7b]::query::descs::hir_owner
  44:     0x7fb121dda498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  45:     0x7fb121d10053 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_owner::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  46:     0x7fb121fba540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  47:     0x7fb121e7dc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  48:     0x7fb1220dc941 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  49:     0x7fb121e85aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  50:     0x7fb1203c38e7 - rustc_interface[ab8a3f336018d81b]::passes::output_filenames
  51:     0x7fb1220689dd - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::output_filenames, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  52:     0x7fb121f15c73 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::output_filenames
  53:     0x7fb120f9b043 - rustc_ast_lowering[3d1fce858f29a9f9]::lower_to_hir
  54:     0x7fb122184632 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  55:     0x7fb121e86c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  56:     0x7fb12319bd46 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  57:     0x7fb1221851ac - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_owner, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  58:     0x7fb121e88695 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_owner
  59:     0x7fb1230e5030 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::find
  60:     0x7fb1230e8e63 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::opt_span
  61:     0x7fb1230e8dbd - <rustc_middle[b78252b658aef7b]::hir::map::Map>::span
  62:     0x7fb1212fc928 - rustc_passes[e839ac6b29e77bf7]::stability::stability_index
  63:     0x7fb1220494d2 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::stability_index, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  64:     0x7fb121f0fb13 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::stability_index
  65:     0x7fb122f90bd7 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::stability
  66:     0x7fb1212ef228 - <rustc_passes[e839ac6b29e77bf7]::stability::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  67:     0x7fb122062071 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::lookup_stability, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  68:     0x7fb121ed4b46 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::lookup_stability
  69:     0x7fb122f831fd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability_allow_unstable
  70:     0x7fb122f82e17 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability
  71:     0x7fb121230f81 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::add_typo_suggestion
  72:     0x7fb12114c41f - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_report_errors
  73:     0x7fb12119db08 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment::{closure#0}
  74:     0x7fb12119a91e - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  75:     0x7fb121199df2 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  76:     0x7fb12118e2e6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path
  77:     0x7fb121159948 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_ty
  78:     0x7fb121165250 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_fn
  79:     0x7fb12129409d - rustc_ast[2b4c606609dd0b28]::visit::walk_assoc_item::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  80:     0x7fb121184890 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_trait_items::{closure#0}
  81:     0x7fb121180a48 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::with_generic_param_rib::<<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_item::{closure#2}>
  82:     0x7fb121176705 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_item
  83:     0x7fb12115726c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_item
  84:     0x7fb12128e0fd - rustc_ast[2b4c606609dd0b28]::visit::walk_crate::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  85:     0x7fb12123f28e - <rustc_resolve[f303ed5d69cd7988]::Resolver>::late_resolve_crate
  86:     0x7fb1211b2e02 - <rustc_session[c2c6e323d951d754]::session::Session>::time::<(), <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate::{closure#0}>
  87:     0x7fb12124dc0d - <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate
  88:     0x7fb1203c2e30 - rustc_interface[ab8a3f336018d81b]::passes::resolver_for_lowering
  89:     0x7fb1220dc809 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  90:     0x7fb121e85aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  91:     0x7fb120309210 - <rustc_middle[b78252b658aef7b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a1b30124f59aa5b6]::steal::Steal<(rustc_middle[b78252b658aef7b]::ty::ResolverAstLowering, alloc[6af766aba38ed60]::rc::Rc<rustc_ast[2b4c606609dd0b28]::ast::Crate>)>>
  92:     0x7fb12034e17d - <rustc_interface[ab8a3f336018d81b]::interface::Compiler>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}, core[3c115420e38098da]::result::Result<core[3c115420e38098da]::option::Option<rustc_interface[ab8a3f336018d81b]::queries::Linker>, rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  93:     0x7fb120308238 - rustc_span[eaad1e142fe170d9]::with_source_map::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  94:     0x7fb120342a97 - <scoped_tls[e8a8b4df10b4528b]::ScopedKey<rustc_span[eaad1e142fe170d9]::SessionGlobals>>::set::<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  95:     0x7fb1203210b6 - std[c748adf407831b33]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  96:     0x7fb1202f0bb6 - std[c748adf407831b33]::panicking::try::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, core[3c115420e38098da]::panic::unwind_safe::AssertUnwindSafe<<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  97:     0x7fb120316c95 - <<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  98:     0x7fb11f8124de - std::sys::unix::thread::Thread::new::thread_start::h3446ed5cb7c819ff
  99:     0x7fb11f5acb43 - <unknown>
 100:     0x7fb11f63ea00 - <unknown>
 101:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (0e53f3007 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------


---
To only update this specific test, also pass `--test-args issues/issue-32655.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-32655.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32655" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32655/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot find attribute `derive_Clone` in this scope
  --> fake-test-src-base/issues/issue-32655.rs:3:11
   |
LL |         #[derive_Clone] //~ ERROR cannot find attribute `derive_Clone` in this scope
...
...
LL | foo!();
   |
   = note: this error originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find attribute `derive_Clone` in this scope
error: cannot find attribute `derive_Clone` in this scope
  --> fake-test-src-base/issues/issue-32655.rs:15:7
   |
LL |     #[derive_Clone] //~ ERROR cannot find attribute `derive_Clone` in this scope

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] tests/ui/issues/issue-46332.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-46332.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46332" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46332/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/job.rs:141:44
   0:     0x7f95d76b2af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   1:     0x7f95d771fcc8 - core::fmt::write::h5b4b993aacaf08ed
   2:     0x7f95d76a7271 - std::io::Write::write_fmt::hf420647c1429c197
   3:     0x7f95d76b2901 - std::sys_common::backtrace::print::h39b64acc765d9915
   3:     0x7f95d76b2901 - std::sys_common::backtrace::print::h39b64acc765d9915
   4:     0x7f95d76b5ad4 - std::panicking::default_hook::{{closure}}::h984ccf8c48efd9dd
   5:     0x7f95d76b57ba - std::panicking::default_hook::he22f59932909c7ec
   6:     0x7f95d81b6f75 - rustc_driver_impl[f2eb435b2e98c66d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f95d76b61f1 - std::panicking::rust_panic_with_hook::h53b87e485107618d
   8:     0x7f95d76b5f2a - std::panicking::begin_panic_handler::{{closure}}::h4e1d7611ee576041
   9:     0x7f95d76b2fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ea49fc2484af23b
  10:     0x7f95d76b5c47 - rust_begin_unwind
  11:     0x7f95d766c2c3 - core::panicking::panic_fmt::hb860c5b6d0410325
  12:     0x7f95d766c35d - core::panicking::panic::h1309072cc4b1b0e7
  13:     0x7f95d8038a4c - <rustc_query_system[5fe272afb00b13fa]::query::job::QueryJobId>::find_cycle_in_stack::<rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>
  14:     0x7f95da022b82 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  15:     0x7f95d9dd4874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  16:     0x7f95dae43bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  17:     0x7f95dae37774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  18:     0x7f95dafbc7b7 - rustc_middle[b78252b658aef7b]::query::descs::lookup_stability
  19:     0x7f95d9c8aabe - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_span[eaad1e142fe170d9]::def_id::DefId>
  20:     0x7f95d9bbd139 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::lookup_stability::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  21:     0x7f95d9e6a8c4 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_span[eaad1e142fe170d9]::def_id::DefId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  22:     0x7f95d9d2dc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  23:     0x7f95da022b37 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  24:     0x7f95d9dd4874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  25:     0x7f95dae43bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  26:     0x7f95dae37774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  27:     0x7f95dafb53d1 - rustc_middle[b78252b658aef7b]::query::descs::hir_attrs
  28:     0x7f95d9c8a498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  29:     0x7f95d9bbffe3 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_attrs::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  30:     0x7f95d9e6a540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  31:     0x7f95d9d2dc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  32:     0x7f95da0348a4 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  33:     0x7f95d9d36c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  34:     0x7f95db04b3d6 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#5} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  35:     0x7f95da03261e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_attrs, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  36:     0x7f95d9d3aa89 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_attrs
  37:     0x7f95daf98bb8 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::attrs
  38:     0x7f95daeb2054 - <rustc_middle[b78252b658aef7b]::middle::limits::provide::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, ())>>::call_once
  39:     0x7f95da021e1e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  40:     0x7f95d9dd4874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  41:     0x7f95dae43bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  42:     0x7f95dae37774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  43:     0x7f95dafb4fd1 - rustc_middle[b78252b658aef7b]::query::descs::hir_owner
  44:     0x7f95d9c8a498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  45:     0x7f95d9bc0053 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_owner::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  46:     0x7f95d9e6a540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  47:     0x7f95d9d2dc95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  48:     0x7f95d9f8c941 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  49:     0x7f95d9d35aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  50:     0x7f95d82738e7 - rustc_interface[ab8a3f336018d81b]::passes::output_filenames
  51:     0x7f95d9f189dd - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::output_filenames, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  52:     0x7f95d9dc5c73 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::output_filenames
  53:     0x7f95d8e4b043 - rustc_ast_lowering[3d1fce858f29a9f9]::lower_to_hir
  54:     0x7f95da034632 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  55:     0x7f95d9d36c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  56:     0x7f95db04bd46 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  57:     0x7f95da0351ac - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_owner, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  58:     0x7f95d9d38695 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_owner
  59:     0x7f95daf95030 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::find
  60:     0x7f95daf98e63 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::opt_span
  61:     0x7f95daf98dbd - <rustc_middle[b78252b658aef7b]::hir::map::Map>::span
  62:     0x7f95d91ac928 - rustc_passes[e839ac6b29e77bf7]::stability::stability_index
  63:     0x7f95d9ef94d2 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::stability_index, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  64:     0x7f95d9dbfb13 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::stability_index
  65:     0x7f95dae40bd7 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::stability
  66:     0x7f95d919f228 - <rustc_passes[e839ac6b29e77bf7]::stability::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  67:     0x7f95d9f12071 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::lookup_stability, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  68:     0x7f95d9d84b46 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::lookup_stability
  69:     0x7f95dae331fd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability_allow_unstable
  70:     0x7f95dae32e17 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability
  71:     0x7f95d90e0f81 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::add_typo_suggestion
  72:     0x7f95d8ffc114 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_report_errors
  73:     0x7f95d904db08 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment::{closure#0}
  74:     0x7f95d904a91e - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  75:     0x7f95d903e2e6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path
  76:     0x7f95d9040a2c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_expr
  77:     0x7f95d9007bef - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_block
  78:     0x7f95d9016d8f - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_fn
  79:     0x7f95d9158707 - rustc_ast[2b4c606609dd0b28]::visit::walk_item::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  80:     0x7f95d9028b70 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_item
  81:     0x7f95d900726c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_item
  82:     0x7f95d913e0fd - rustc_ast[2b4c606609dd0b28]::visit::walk_crate::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  83:     0x7f95d90ef28e - <rustc_resolve[f303ed5d69cd7988]::Resolver>::late_resolve_crate
  84:     0x7f95d9062e02 - <rustc_session[c2c6e323d951d754]::session::Session>::time::<(), <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate::{closure#0}>
  85:     0x7f95d90fdc0d - <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate
  86:     0x7f95d8272e30 - rustc_interface[ab8a3f336018d81b]::passes::resolver_for_lowering
  87:     0x7f95d9f8c809 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  88:     0x7f95d9d35aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  89:     0x7f95d81b9210 - <rustc_middle[b78252b658aef7b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a1b30124f59aa5b6]::steal::Steal<(rustc_middle[b78252b658aef7b]::ty::ResolverAstLowering, alloc[6af766aba38ed60]::rc::Rc<rustc_ast[2b4c606609dd0b28]::ast::Crate>)>>
  90:     0x7f95d81fe17d - <rustc_interface[ab8a3f336018d81b]::interface::Compiler>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}, core[3c115420e38098da]::result::Result<core[3c115420e38098da]::option::Option<rustc_interface[ab8a3f336018d81b]::queries::Linker>, rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  91:     0x7f95d81b8238 - rustc_span[eaad1e142fe170d9]::with_source_map::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  92:     0x7f95d81f2a97 - <scoped_tls[e8a8b4df10b4528b]::ScopedKey<rustc_span[eaad1e142fe170d9]::SessionGlobals>>::set::<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  93:     0x7f95d81d10b6 - std[c748adf407831b33]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  94:     0x7f95d81a0bb6 - std[c748adf407831b33]::panicking::try::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, core[3c115420e38098da]::panic::unwind_safe::AssertUnwindSafe<<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  95:     0x7f95d81c6c95 - <<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  96:     0x7f95d76c24de - std::sys::unix::thread::Thread::new::thread_start::h3446ed5cb7c819ff
  97:     0x7f95d745cb43 - <unknown>
  98:     0x7f95d74eea00 - <unknown>
  99:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (0e53f3007 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/issues/issue-64792-bad-unicode-ctor.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-64792-bad-unicode-ctor.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-64792-bad-unicode-ctor" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-64792-bad-unicode-ctor/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/job.rs:141:44
   0:     0x7f4013ac3af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   1:     0x7f4013b30cc8 - core::fmt::write::h5b4b993aacaf08ed
   2:     0x7f4013ab8271 - std::io::Write::write_fmt::hf420647c1429c197
   3:     0x7f4013ac3901 - std::sys_common::backtrace::print::h39b64acc765d9915
   3:     0x7f4013ac3901 - std::sys_common::backtrace::print::h39b64acc765d9915
   4:     0x7f4013ac6ad4 - std::panicking::default_hook::{{closure}}::h984ccf8c48efd9dd
   5:     0x7f4013ac67ba - std::panicking::default_hook::he22f59932909c7ec
   6:     0x7f40145c7f75 - rustc_driver_impl[f2eb435b2e98c66d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4013ac71f1 - std::panicking::rust_panic_with_hook::h53b87e485107618d
   8:     0x7f4013ac6f2a - std::panicking::begin_panic_handler::{{closure}}::h4e1d7611ee576041
   9:     0x7f4013ac3fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ea49fc2484af23b
  10:     0x7f4013ac6c47 - rust_begin_unwind
  11:     0x7f4013a7d2c3 - core::panicking::panic_fmt::hb860c5b6d0410325
  12:     0x7f4013a7d35d - core::panicking::panic::h1309072cc4b1b0e7
  13:     0x7f4014449a4c - <rustc_query_system[5fe272afb00b13fa]::query::job::QueryJobId>::find_cycle_in_stack::<rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>
  14:     0x7f4016433b82 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  15:     0x7f40161e5874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  16:     0x7f4017254bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  17:     0x7f4017248774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  18:     0x7f40173cd7b7 - rustc_middle[b78252b658aef7b]::query::descs::lookup_stability
  19:     0x7f401609babe - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_span[eaad1e142fe170d9]::def_id::DefId>
  20:     0x7f4015fce139 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::lookup_stability::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  21:     0x7f401627b8c4 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_span[eaad1e142fe170d9]::def_id::DefId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  22:     0x7f401613ec95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  23:     0x7f4016433b37 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  24:     0x7f40161e5874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  25:     0x7f4017254bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  26:     0x7f4017248774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  27:     0x7f40173c63d1 - rustc_middle[b78252b658aef7b]::query::descs::hir_attrs
  28:     0x7f401609b498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  29:     0x7f4015fd0fe3 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_attrs::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  30:     0x7f401627b540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  31:     0x7f401613ec95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  32:     0x7f40164458a4 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  33:     0x7f4016147c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  34:     0x7f401745c3d6 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#5} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  35:     0x7f401644361e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_attrs, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  36:     0x7f401614ba89 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_attrs
  37:     0x7f40173a9bb8 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::attrs
  38:     0x7f40172c3054 - <rustc_middle[b78252b658aef7b]::middle::limits::provide::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, ())>>::call_once
  39:     0x7f4016432e1e - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::limits, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  40:     0x7f40161e5874 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::limits
  41:     0x7f4017254bfd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::type_length_limit
  42:     0x7f4017248774 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::def_path_str_with_substs
  43:     0x7f40173c5fd1 - rustc_middle[b78252b658aef7b]::query::descs::hir_owner
  44:     0x7f401609b498 - rustc_query_impl[10ce0422fe936d3f]::plumbing::create_query_frame::<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId>
  45:     0x7f4015fd1053 - <rustc_query_impl[10ce0422fe936d3f]::query_structs::hir_owner::{closure#0}::{closure#0} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  46:     0x7f401627b540 - <rustc_query_system[5fe272afb00b13fa]::query::plumbing::QueryState<rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId, rustc_middle[b78252b658aef7b]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  47:     0x7f401613ec95 - <rustc_query_impl[10ce0422fe936d3f]::Queries>::try_collect_active_jobs
  48:     0x7f401639d941 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  49:     0x7f4016146aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  50:     0x7f40146848e7 - rustc_interface[ab8a3f336018d81b]::passes::output_filenames
  51:     0x7f40163299dd - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::output_filenames, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  52:     0x7f40161d6c73 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::output_filenames
  53:     0x7f401525c043 - rustc_ast_lowering[3d1fce858f29a9f9]::lower_to_hir
  54:     0x7f4016445632 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_crate, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  55:     0x7f4016147c83 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_crate
  56:     0x7f401745cd46 - <rustc_middle[b78252b658aef7b]::hir::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_hir[dd8ef900a03d120a]::hir_id::OwnerId)>>::call_once
  57:     0x7f40164461ac - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::hir_owner, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  58:     0x7f4016149695 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::hir_owner
  59:     0x7f40173a6030 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::find
  60:     0x7f40173a9e63 - <rustc_middle[b78252b658aef7b]::hir::map::Map>::opt_span
  61:     0x7f40173a9dbd - <rustc_middle[b78252b658aef7b]::hir::map::Map>::span
  62:     0x7f40155bd928 - rustc_passes[e839ac6b29e77bf7]::stability::stability_index
  63:     0x7f401630a4d2 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::stability_index, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  64:     0x7f40161d0b13 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::stability_index
  65:     0x7f4017251bd7 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::stability
  66:     0x7f40155b0228 - <rustc_passes[e839ac6b29e77bf7]::stability::provide::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<(rustc_middle[b78252b658aef7b]::ty::context::TyCtxt, rustc_span[eaad1e142fe170d9]::def_id::DefId)>>::call_once
  67:     0x7f4016323071 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::lookup_stability, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  68:     0x7f4016195b46 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::lookup_stability
  69:     0x7f40172441fd - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability_allow_unstable
  70:     0x7f4017243e17 - <rustc_middle[b78252b658aef7b]::ty::context::TyCtxt>::eval_stability
  71:     0x7f40154f1f81 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::add_typo_suggestion
  72:     0x7f4015410d4b - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::try_lookup_name_relaxed
  73:     0x7f401540a7eb - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_report_errors
  74:     0x7f401545eb08 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment::{closure#0}
  75:     0x7f401545cd99 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  76:     0x7f401544f2e6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path
  77:     0x7f4015451a2c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_expr
  78:     0x7f40154514fe - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_expr
  79:     0x7f401543adb3 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_item
  80:     0x7f401541826c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_item
  81:     0x7f401554f0fd - rustc_ast[2b4c606609dd0b28]::visit::walk_crate::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  82:     0x7f401550028e - <rustc_resolve[f303ed5d69cd7988]::Resolver>::late_resolve_crate
  83:     0x7f4015473e02 - <rustc_session[c2c6e323d951d754]::session::Session>::time::<(), <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate::{closure#0}>
  84:     0x7f401550ec0d - <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate
  85:     0x7f4014683e30 - rustc_interface[ab8a3f336018d81b]::passes::resolver_for_lowering
  86:     0x7f401639d809 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  87:     0x7f4016146aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  88:     0x7f40145ca210 - <rustc_middle[b78252b658aef7b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a1b30124f59aa5b6]::steal::Steal<(rustc_middle[b78252b658aef7b]::ty::ResolverAstLowering, alloc[6af766aba38ed60]::rc::Rc<rustc_ast[2b4c606609dd0b28]::ast::Crate>)>>
  89:     0x7f401460f17d - <rustc_interface[ab8a3f336018d81b]::interface::Compiler>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}, core[3c115420e38098da]::result::Result<core[3c115420e38098da]::option::Option<rustc_interface[ab8a3f336018d81b]::queries::Linker>, rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  90:     0x7f40145c9238 - rustc_span[eaad1e142fe170d9]::with_source_map::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  91:     0x7f4014603a97 - <scoped_tls[e8a8b4df10b4528b]::ScopedKey<rustc_span[eaad1e142fe170d9]::SessionGlobals>>::set::<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  92:     0x7f40145e20b6 - std[c748adf407831b33]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  93:     0x7f40145b1bb6 - std[c748adf407831b33]::panicking::try::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, core[3c115420e38098da]::panic::unwind_safe::AssertUnwindSafe<<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  94:     0x7f40145d7c95 - <<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  95:     0x7f4013ad34de - std::sys::unix::thread::Thread::new::thread_start::h3446ed5cb7c819ff
  96:     0x7f401386db43 - <unknown>
  97:     0x7f40138ffa00 - <unknown>
  98:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (0e53f3007 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/lint/recommend-literal.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/recommend-literal.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/recommend-literal" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/recommend-literal/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempted .def_id() on invalid res: PrimTy(Bool)', /checkout/compiler/rustc_hir/src/def.rs:623:45
   0:     0x7f34461bdaf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   1:     0x7f344622acc8 - core::fmt::write::h5b4b993aacaf08ed
   2:     0x7f34461b2271 - std::io::Write::write_fmt::hf420647c1429c197
   3:     0x7f34461bd901 - std::sys_common::backtrace::print::h39b64acc765d9915
   3:     0x7f34461bd901 - std::sys_common::backtrace::print::h39b64acc765d9915
   4:     0x7f34461c0ad4 - std::panicking::default_hook::{{closure}}::h984ccf8c48efd9dd
   5:     0x7f34461c07ba - std::panicking::default_hook::he22f59932909c7ec
   6:     0x7f3446cc1f75 - rustc_driver_impl[f2eb435b2e98c66d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f34461c11f1 - std::panicking::rust_panic_with_hook::h53b87e485107618d
   8:     0x7f34461c0f69 - std::panicking::begin_panic_handler::{{closure}}::h4e1d7611ee576041
   9:     0x7f34461bdfc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ea49fc2484af23b
  10:     0x7f34461c0c47 - rust_begin_unwind
  11:     0x7f34461772c3 - core::panicking::panic_fmt::hb860c5b6d0410325
  12:     0x7f3447b8abf6 - <rustc_hir[dd8ef900a03d120a]::def::Res<rustc_ast[2b4c606609dd0b28]::node_id::NodeId>>::def_id
  13:     0x7f3447bebf56 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::add_typo_suggestion
  14:     0x7f3447b07114 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_report_errors
  15:     0x7f3447b58b08 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment::{closure#0}
  16:     0x7f3447b5591e - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  17:     0x7f3447b492e6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path
  18:     0x7f3447b14948 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_ty
  19:     0x7f3447b143f6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_local
  20:     0x7f3447b12bef - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_block
  21:     0x7f3447b21d8f - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_fn
  22:     0x7f3447c63707 - rustc_ast[2b4c606609dd0b28]::visit::walk_item::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  23:     0x7f3447b33b70 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_item
  24:     0x7f3447b1226c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_item
  25:     0x7f3447c490fd - rustc_ast[2b4c606609dd0b28]::visit::walk_crate::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  26:     0x7f3447bfa28e - <rustc_resolve[f303ed5d69cd7988]::Resolver>::late_resolve_crate
  27:     0x7f3447b6de02 - <rustc_session[c2c6e323d951d754]::session::Session>::time::<(), <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate::{closure#0}>
  28:     0x7f3447c08c0d - <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate
  29:     0x7f3446d7de30 - rustc_interface[ab8a3f336018d81b]::passes::resolver_for_lowering
  30:     0x7f3448a97809 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  31:     0x7f3448840aa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  32:     0x7f3446cc4210 - <rustc_middle[b78252b658aef7b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a1b30124f59aa5b6]::steal::Steal<(rustc_middle[b78252b658aef7b]::ty::ResolverAstLowering, alloc[6af766aba38ed60]::rc::Rc<rustc_ast[2b4c606609dd0b28]::ast::Crate>)>>
  33:     0x7f3446d0917d - <rustc_interface[ab8a3f336018d81b]::interface::Compiler>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}, core[3c115420e38098da]::result::Result<core[3c115420e38098da]::option::Option<rustc_interface[ab8a3f336018d81b]::queries::Linker>, rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  34:     0x7f3446cc3238 - rustc_span[eaad1e142fe170d9]::with_source_map::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x7f3446cfda97 - <scoped_tls[e8a8b4df10b4528b]::ScopedKey<rustc_span[eaad1e142fe170d9]::SessionGlobals>>::set::<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  36:     0x7f3446cdc0b6 - std[c748adf407831b33]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  37:     0x7f3446cabbb6 - std[c748adf407831b33]::panicking::try::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, core[3c115420e38098da]::panic::unwind_safe::AssertUnwindSafe<<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  38:     0x7f3446cd1c95 - <<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f34461cd4de - std::sys::unix::thread::Thread::new::thread_start::h3446ed5cb7c819ff
  40:     0x7f3445f67b43 - <unknown>
  41:     0x7f3445ff9a00 - <unknown>
  42:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (0e53f3007 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
------------------------------------------
------------------------------------------


---- [ui] tests/ui/macros/macro-context.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/macro-context.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-context" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-context/auxiliary"
stdout: none
--- stderr -------------------------------
error: macro expansion ignores token `;` and any following
  --> fake-test-src-base/macros/macro-context.rs:3:15
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
...
...
LL |     let a: m!();
   |            ---- caused by the macro expansion here
   |
   = note: the usage of `m!` is likely invalid in type context

error: macro expansion ignores token `typeof` and any following
  --> fake-test-src-base/macros/macro-context.rs:3:17
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
...
LL |     let i = m!();
   |             ---- caused by the macro expansion here
   |
   |
   = note: the usage of `m!` is likely invalid in expression context

error: macro expansion ignores token `;` and any following
  --> fake-test-src-base/macros/macro-context.rs:3:15
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
...
...
LL |         m!() => {}
   |         ---- caused by the macro expansion here
   |
   = note: the usage of `m!` is likely invalid in pattern context
error: expected expression, found reserved keyword `typeof`
  --> fake-test-src-base/macros/macro-context.rs:3:17
   |
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
...
LL |     m!();
   |     ---- in this macro invocation
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at 'attempted .def_id() on invalid res: PrimTy(Int(I8))', /checkout/compiler/rustc_hir/src/def.rs:623:45
   0:     0x7fd612a68af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   1:     0x7fd612ad5cc8 - core::fmt::write::h5b4b993aacaf08ed
   2:     0x7fd612a5d271 - std::io::Write::write_fmt::hf420647c1429c197
   3:     0x7fd612a68901 - std::sys_common::backtrace::print::h39b64acc765d9915
   3:     0x7fd612a68901 - std::sys_common::backtrace::print::h39b64acc765d9915
   4:     0x7fd612a6bad4 - std::panicking::default_hook::{{closure}}::h984ccf8c48efd9dd
   5:     0x7fd612a6b7ba - std::panicking::default_hook::he22f59932909c7ec
   6:     0x7fd61356cf75 - rustc_driver_impl[f2eb435b2e98c66d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd612a6c1f1 - std::panicking::rust_panic_with_hook::h53b87e485107618d
   8:     0x7fd612a6bf69 - std::panicking::begin_panic_handler::{{closure}}::h4e1d7611ee576041
   9:     0x7fd612a68fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ea49fc2484af23b
  10:     0x7fd612a6bc47 - rust_begin_unwind
  11:     0x7fd612a222c3 - core::panicking::panic_fmt::hb860c5b6d0410325
  12:     0x7fd614435bf6 - <rustc_hir[dd8ef900a03d120a]::def::Res<rustc_ast[2b4c606609dd0b28]::node_id::NodeId>>::def_id
  13:     0x7fd614496f56 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::add_typo_suggestion
  14:     0x7fd6143b2114 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_report_errors
  15:     0x7fd614403b08 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment::{closure#0}
  16:     0x7fd61440091e - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  17:     0x7fd6143f42e6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::smart_resolve_path
  18:     0x7fd6143bf948 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_ty
  19:     0x7fd6143bf3f6 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_local
  20:     0x7fd6143bdbef - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_block
  21:     0x7fd6143ccd8f - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_fn
  22:     0x7fd61450e707 - rustc_ast[2b4c606609dd0b28]::visit::walk_item::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  23:     0x7fd6143deb70 - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>::resolve_item
  24:     0x7fd6143bd26c - <rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor as rustc_ast[2b4c606609dd0b28]::visit::Visitor>::visit_item
  25:     0x7fd6144f40fd - rustc_ast[2b4c606609dd0b28]::visit::walk_crate::<rustc_resolve[f303ed5d69cd7988]::late::LateResolutionVisitor>
  26:     0x7fd6144a528e - <rustc_resolve[f303ed5d69cd7988]::Resolver>::late_resolve_crate
  27:     0x7fd614418e02 - <rustc_session[c2c6e323d951d754]::session::Session>::time::<(), <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate::{closure#0}>
  28:     0x7fd6144b3c0d - <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate
  29:     0x7fd613628e30 - rustc_interface[ab8a3f336018d81b]::passes::resolver_for_lowering
  30:     0x7fd615342809 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  31:     0x7fd6150ebaa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  32:     0x7fd61356f210 - <rustc_middle[b78252b658aef7b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a1b30124f59aa5b6]::steal::Steal<(rustc_middle[b78252b658aef7b]::ty::ResolverAstLowering, alloc[6af766aba38ed60]::rc::Rc<rustc_ast[2b4c606609dd0b28]::ast::Crate>)>>
  33:     0x7fd6135b417d - <rustc_interface[ab8a3f336018d81b]::interface::Compiler>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}, core[3c115420e38098da]::result::Result<core[3c115420e38098da]::option::Option<rustc_interface[ab8a3f336018d81b]::queries::Linker>, rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  34:     0x7fd61356e238 - rustc_span[eaad1e142fe170d9]::with_source_map::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x7fd6135a8a97 - <scoped_tls[e8a8b4df10b4528b]::ScopedKey<rustc_span[eaad1e142fe170d9]::SessionGlobals>>::set::<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  36:     0x7fd6135870b6 - std[c748adf407831b33]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  37:     0x7fd613556bb6 - std[c748adf407831b33]::panicking::try::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, core[3c115420e38098da]::panic::unwind_safe::AssertUnwindSafe<<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  38:     0x7fd61357cc95 - <<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7fd612a784de - std::sys::unix::thread::Thread::new::thread_start::h3446ed5cb7c819ff
  40:     0x7fd612812b43 - <unknown>
  41:     0x7fd6128a4a00 - <unknown>
  42:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (0e53f3007 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
error: aborting due to 4 previous errors
error: aborting due to 4 previous errors
------------------------------------------


---- [ui] tests/ui/macros/macro-reexport-removed.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/macro-reexport-removed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-reexport-removed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-reexport-removed/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/macros/macro-reexport-removed.rs:3:12
   |
   |
LL | #![feature(macro_reexport)] //~ ERROR feature has been removed
   |
   = note: subsumed by `pub use`


thread 'rustc' panicked at 'attempted .def_id() on invalid res: NonMacroAttr(Builtin(""))', /checkout/compiler/rustc_hir/src/def.rs:623:45
   0:     0x7fcae9638af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha5a4fc39ddef0fcd
   1:     0x7fcae96a5cc8 - core::fmt::write::h5b4b993aacaf08ed
   2:     0x7fcae962d271 - std::io::Write::write_fmt::hf420647c1429c197
   3:     0x7fcae9638901 - std::sys_common::backtrace::print::h39b64acc765d9915
   3:     0x7fcae9638901 - std::sys_common::backtrace::print::h39b64acc765d9915
   4:     0x7fcae963bad4 - std::panicking::default_hook::{{closure}}::h984ccf8c48efd9dd
   5:     0x7fcae963b7ba - std::panicking::default_hook::he22f59932909c7ec
   6:     0x7fcaea13cf75 - rustc_driver_impl[f2eb435b2e98c66d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fcae963c1f1 - std::panicking::rust_panic_with_hook::h53b87e485107618d
   8:     0x7fcae963bf69 - std::panicking::begin_panic_handler::{{closure}}::h4e1d7611ee576041
   9:     0x7fcae9638fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h7ea49fc2484af23b
  10:     0x7fcae963bc47 - rust_begin_unwind
  11:     0x7fcae95f22c3 - core::panicking::panic_fmt::hb860c5b6d0410325
  12:     0x7fcaeb005bf6 - <rustc_hir[dd8ef900a03d120a]::def::Res<rustc_ast[2b4c606609dd0b28]::node_id::NodeId>>::def_id
  13:     0x7fcaeb066f56 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::add_typo_suggestion
  14:     0x7fcaeb07c277 - <rustc_resolve[f303ed5d69cd7988]::Resolver>::finalize_macro_resolutions
  15:     0x7fcaeafe8d57 - <rustc_session[c2c6e323d951d754]::session::Session>::time::<(), <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate::{closure#0}>
  16:     0x7fcaeb083c0d - <rustc_resolve[f303ed5d69cd7988]::Resolver>::resolve_crate
  17:     0x7fcaea1f8e30 - rustc_interface[ab8a3f336018d81b]::passes::resolver_for_lowering
  18:     0x7fcaebf12809 - rustc_query_system[5fe272afb00b13fa]::query::plumbing::try_execute_query::<rustc_query_impl[10ce0422fe936d3f]::queries::resolver_for_lowering, rustc_query_impl[10ce0422fe936d3f]::plumbing::QueryCtxt>
  19:     0x7fcaebcbbaa3 - <rustc_query_impl[10ce0422fe936d3f]::Queries as rustc_middle[b78252b658aef7b]::ty::query::QueryEngine>::resolver_for_lowering
  20:     0x7fcaea13f210 - <rustc_middle[b78252b658aef7b]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a1b30124f59aa5b6]::steal::Steal<(rustc_middle[b78252b658aef7b]::ty::ResolverAstLowering, alloc[6af766aba38ed60]::rc::Rc<rustc_ast[2b4c606609dd0b28]::ast::Crate>)>>
  21:     0x7fcaea18417d - <rustc_interface[ab8a3f336018d81b]::interface::Compiler>::enter::<rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}::{closure#2}, core[3c115420e38098da]::result::Result<core[3c115420e38098da]::option::Option<rustc_interface[ab8a3f336018d81b]::queries::Linker>, rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  22:     0x7fcaea13e238 - rustc_span[eaad1e142fe170d9]::with_source_map::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  23:     0x7fcaea178a97 - <scoped_tls[e8a8b4df10b4528b]::ScopedKey<rustc_span[eaad1e142fe170d9]::SessionGlobals>>::set::<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  24:     0x7fcaea1570b6 - std[c748adf407831b33]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>
  25:     0x7fcaea126bb6 - std[c748adf407831b33]::panicking::try::<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, core[3c115420e38098da]::panic::unwind_safe::AssertUnwindSafe<<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  26:     0x7fcaea14cc95 - <<std[c748adf407831b33]::thread::Builder>::spawn_unchecked_<rustc_interface[ab8a3f336018d81b]::util::run_in_thread_pool_with_globals<rustc_interface[ab8a3f336018d81b]::interface::run_compiler<core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>, rustc_driver_impl[f2eb435b2e98c66d]::run_compiler::{closure#1}>::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3c115420e38098da]::result::Result<(), rustc_span[eaad1e142fe170d9]::ErrorGuaranteed>>::{closure#1} as core[3c115420e38098da]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7fcae96484de - std::sys::unix::thread::Thread::new::thread_start::h3446ed5cb7c819ff
  28:     0x7fcae93e2b43 - <unknown>
  29:     0x7fcae9474a00 - <unknown>
  30:                0x0 - <unknown>
