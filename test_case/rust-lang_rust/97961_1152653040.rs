plain
failures:

---- [ui] src/test/ui/async-await/issue-72442.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-72442.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442/issue-72442.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Option<&str>: AsRef<Path>` is not satisfied
   |
LL |             let mut f = File::open(path.to_str())?;
LL |             let mut f = File::open(path.to_str())?;
   |                         ---------- ^^^^^^^^^^^^^ the trait `AsRef<Path>` is not implemented for `Option<&str>`
   |                         required by a bound introduced by this call
   |
note: required by a bound in `File::open`
  --> /checkout/library/std/src/fs.rs:343:20
  --> /checkout/library/std/src/fs.rs:343:20
   |
LL |     pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
   |                    ^^^^^^^^^^^ required by this bound in `File::open`

thread 'rustc' panicked at 'Hashing HIR bodies is forbidden.', /checkout/compiler/rustc_query_system/src/ich/impls_hir.rs:14:40
   0:     0x7f26c011397c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66bc635c24488641
   1:     0x7f26c017a818 - core::fmt::write::hcb19d8130fd1d715
   1:     0x7f26c017a818 - core::fmt::write::hcb19d8130fd1d715
   2:     0x7f26c01037e1 - std::io::Write::write_fmt::hfd879b7f63738318
   3:     0x7f26c011697e - std::panicking::default_hook::{{closure}}::h3cc863080bbf5e98
   4:     0x7f26c0116669 - std::panicking::default_hook::hf60e9668c12215fb
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   5:     0x7f26c0c287e1 - rustc_driver[a4dc5fca8295e0dc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f26c01170e2 - std::panicking::rust_panic_with_hook::h81c95cb920c9905b
   7:     0x7f26c0116ed9 - std::panicking::begin_panic_handler::{{closure}}::h2b88ea4cdbe4a101
   8:     0x7f26c0113e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h33ca4a36f576d747
   9:     0x7f26c0116bf9 - rust_begin_unwind
  10:     0x7f26c00cb013 - core::panicking::panic_fmt::h7dcce2b3cd28e2a9
  11:     0x7f26c3566183 - <rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext as rustc_hir[f0d27074f747bffc]::stable_hash_impls::HashStableContext>::hash_body_id
  12:     0x7f26c3559afc - rustc_data_structures[9723f2609ab46cc2]::stable_hasher::stable_hash_reduce::<rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext, (&rustc_hir[f0d27074f747bffc]::hir::BodyId, &usize), std[b51c605e05435ab]::collections::hash::map::Iter<rustc_hir[f0d27074f747bffc]::hir::BodyId, usize>, <std[b51c605e05435ab]::collections::hash::map::HashMap<rustc_hir[f0d27074f747bffc]::hir::BodyId, usize, core[da774f735824091f]::hash::BuildHasherDefault<rustc_hash[7f469e8a70954446]::FxHasher>> as rustc_data_structures[9723f2609ab46cc2]::stable_hasher::HashStable<rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext>>::hash_stable::{closure#0}>
  13:     0x7f26c36f4197 - <rustc_middle[78b2171944cacd78]::middle::region::ScopeTree as rustc_data_structures[9723f2609ab46cc2]::stable_hasher::HashStable<rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext>>::hash_stable
  14:     0x7f26c25dd17e - rustc_query_system[23b2a77768faa467]::dep_graph::graph::hash_result::<&rustc_middle[78b2171944cacd78]::middle::region::ScopeTree>
  15:     0x7f26c259853f - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::DefId, &rustc_middle[78b2171944cacd78]::middle::region::ScopeTree>
  16:     0x7f26c277d507 - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::DefId, &rustc_middle[78b2171944cacd78]::middle::region::ScopeTree>>
  17:     0x7f26c283177f - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::region_scope_tree, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  18:     0x7f26c23d5f09 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::region_scope_tree
  19:     0x7f26c1481943 - <rustc_typeck[e314516c87591ba6]::check::fn_ctxt::FnCtxt>::resolve_rvalue_scopes
  20:     0x7f26c175c63d - <rustc_infer[192f10f2e4ad9ebd]::infer::InferCtxtBuilder>::enter::<&rustc_middle[78b2171944cacd78]::ty::context::TypeckResults, <rustc_typeck[e314516c87591ba6]::check::inherited::InheritedBuilder>::enter<rustc_typeck[e314516c87591ba6]::check::typeck_with_fallback<rustc_typeck[e314516c87591ba6]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>::{closure#0}>
  21:     0x7f26c158011e - <rustc_typeck[e314516c87591ba6]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[e314516c87591ba6]::check::typeck_with_fallback<rustc_typeck[e314516c87591ba6]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  22:     0x7f26c16195fe - rustc_typeck[e314516c87591ba6]::check::typeck
  23:     0x7f26c247ad3d - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  24:     0x7f26c257ca95 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  25:     0x7f26c2757fbc - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>>
  26:     0x7f26c2873a87 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::typeck, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  27:     0x7f26c23cc364 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::typeck
  28:     0x7f26c1619748 - rustc_typeck[e314516c87591ba6]::check::typeck
  29:     0x7f26c247ad3d - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  30:     0x7f26c257ca95 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  31:     0x7f26c2757fbc - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>>
  32:     0x7f26c2873a87 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::typeck, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  33:     0x7f26c23cc364 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::typeck
  34:     0x7f26c180986b - <rustc_middle[78b2171944cacd78]::hir::map::Map>::par_body_owners::<rustc_typeck[e314516c87591ba6]::check::typeck_item_bodies::{closure#0}>
  35:     0x7f26c161e92d - rustc_typeck[e314516c87591ba6]::check::typeck_item_bodies
  36:     0x7f26c2483b94 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, (), ()>::{closure#0}, ()>
  37:     0x7f26c25c892b - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, (), ()>
  38:     0x7f26c27a06fc - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<(), ()>>
  39:     0x7f26c28376f5 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::typeck_item_bodies, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  40:     0x7f26c23cbe0e - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7f26c1793dca - <rustc_session[87a8455e76c60bce]::session::Session>::time::<(), rustc_typeck[e314516c87591ba6]::check_crate::{closure#7}>
  42:     0x7f26c16dfd8e - rustc_typeck[e314516c87591ba6]::check_crate
  43:     0x7f26c0d46f91 - rustc_interface[a5f7c60448810bbc]::passes::analysis
  44:     0x7f26c2482254 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, (), core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  45:     0x7f26c25b9d2b - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, (), core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  46:     0x7f26c27940df - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<(), core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>>
  47:     0x7f26c2873e62 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::analysis, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  48:     0x7f26c23b027e - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::analysis
  49:     0x7f26c0c99714 - <rustc_interface[a5f7c60448810bbc]::passes::QueryContext>::enter::<rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  50:     0x7f26c0c46095 - <rustc_interface[a5f7c60448810bbc]::interface::Compiler>::enter::<rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}::{closure#2}, core[da774f735824091f]::result::Result<core[da774f735824091f]::option::Option<rustc_interface[a5f7c60448810bbc]::queries::Linker>, rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  51:     0x7f26c0c2a0ab - rustc_span[dbe5b00f8c021e5e]::with_source_map::<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_interface[a5f7c60448810bbc]::interface::create_compiler_and_run<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#1}>
  52:     0x7f26c0c47264 - <scoped_tls[9ccc87b08c856795]::ScopedKey<rustc_span[dbe5b00f8c021e5e]::SessionGlobals>>::set::<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  53:     0x7f26c0ca50a9 - std[b51c605e05435ab]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  54:     0x7f26c0c9d5d9 - <<std[b51c605e05435ab]::thread::Builder>::spawn_unchecked_<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#1} as core[da774f735824091f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  55:     0x7f26c0122473 - std::sys::unix::thread::Thread::new::thread_start::ha8ba8cbd4bd72570
  56:     0x7f26ba675609 - start_thread
  57:     0x7f26bff88133 - clone
  58:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (8e0da9ab1 2022-06-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [region_scope_tree] computing drop scopes for `main`
#1 [typeck] type-checking `main`
#2 [typeck] type-checking `main::{closure#0}`
#3 [typeck_item_bodies] type-checking all item bodies
#4 [analysis] running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/issues/issue-64964.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64964.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964/issue-64964.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Hashing HIR bodies is forbidden.', /checkout/compiler/rustc_query_system/src/ich/impls_hir.rs:14:40
stack backtrace:
   0:     0x7fe5cb89697c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66bc635c24488641
   1:     0x7fe5cb8fd818 - core::fmt::write::hcb19d8130fd1d715
   2:     0x7fe5cb8867e1 - std::io::Write::write_fmt::hfd879b7f63738318
   3:     0x7fe5cb89997e - std::panicking::default_hook::{{closure}}::h3cc863080bbf5e98
   4:     0x7fe5cb899669 - std::panicking::default_hook::hf60e9668c12215fb
   5:     0x7fe5cc3ab7e1 - rustc_driver[a4dc5fca8295e0dc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe5cb89a0e2 - std::panicking::rust_panic_with_hook::h81c95cb920c9905b
   7:     0x7fe5cb899ed9 - std::panicking::begin_panic_handler::{{closure}}::h2b88ea4cdbe4a101
   8:     0x7fe5cb896e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h33ca4a36f576d747
   9:     0x7fe5cb899bf9 - rust_begin_unwind
  10:     0x7fe5cb84e013 - core::panicking::panic_fmt::h7dcce2b3cd28e2a9
  11:     0x7fe5cece9183 - <rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext as rustc_hir[f0d27074f747bffc]::stable_hash_impls::HashStableContext>::hash_body_id
  12:     0x7fe5cecdca2c - rustc_data_structures[9723f2609ab46cc2]::stable_hasher::stable_hash_reduce::<rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext, (&rustc_hir[f0d27074f747bffc]::hir::BodyId, &usize), std[b51c605e05435ab]::collections::hash::map::Iter<rustc_hir[f0d27074f747bffc]::hir::BodyId, usize>, <std[b51c605e05435ab]::collections::hash::map::HashMap<rustc_hir[f0d27074f747bffc]::hir::BodyId, usize, core[da774f735824091f]::hash::BuildHasherDefault<rustc_hash[7f469e8a70954446]::FxHasher>> as rustc_data_structures[9723f2609ab46cc2]::stable_hasher::HashStable<rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext>>::hash_stable::{closure#0}>
  13:     0x7fe5cee77197 - <rustc_middle[78b2171944cacd78]::middle::region::ScopeTree as rustc_data_structures[9723f2609ab46cc2]::stable_hasher::HashStable<rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext>>::hash_stable
  14:     0x7fe5cdd6017e - rustc_query_system[23b2a77768faa467]::dep_graph::graph::hash_result::<&rustc_middle[78b2171944cacd78]::middle::region::ScopeTree>
  15:     0x7fe5cdd1b53f - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::DefId, &rustc_middle[78b2171944cacd78]::middle::region::ScopeTree>
  16:     0x7fe5cdf00507 - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::DefId, &rustc_middle[78b2171944cacd78]::middle::region::ScopeTree>>
  17:     0x7fe5cdfb477f - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::region_scope_tree, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  18:     0x7fe5cdb58f09 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::region_scope_tree
  19:     0x7fe5ccc04943 - <rustc_typeck[e314516c87591ba6]::check::fn_ctxt::FnCtxt>::resolve_rvalue_scopes
  20:     0x7fe5ccedf63d - <rustc_infer[192f10f2e4ad9ebd]::infer::InferCtxtBuilder>::enter::<&rustc_middle[78b2171944cacd78]::ty::context::TypeckResults, <rustc_typeck[e314516c87591ba6]::check::inherited::InheritedBuilder>::enter<rustc_typeck[e314516c87591ba6]::check::typeck_with_fallback<rustc_typeck[e314516c87591ba6]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>::{closure#0}>
  21:     0x7fe5ccd0311e - <rustc_typeck[e314516c87591ba6]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[e314516c87591ba6]::check::typeck_with_fallback<rustc_typeck[e314516c87591ba6]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  22:     0x7fe5ccd9c5fe - rustc_typeck[e314516c87591ba6]::check::typeck
  23:     0x7fe5cdbfdd3d - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  24:     0x7fe5cdcffa95 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  25:     0x7fe5cdedafbc - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>>
  26:     0x7fe5cdff6a87 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::typeck, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  27:     0x7fe5cdb4f364 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::typeck
  28:     0x7fe5ced5a16d - <rustc_middle[78b2171944cacd78]::ty::context::TyCtxt>::typeck_opt_const_arg
  29:     0x7fe5cd32bd37 - rustc_mir_build[f5e089113a49545e]::build::mir_built
  30:     0x7fe5cdbfaaa4 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>::{closure#0}, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>
  31:     0x7fe5cdce8466 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>
  32:     0x7fe5cdecb28e - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>>
  33:     0x7fe5cdff8ed6 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::mir_built, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  34:     0x7fe5cdb38897 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::mir_built
  35:     0x7fe5cca57870 - rustc_mir_transform[bc32d14c384a311b]::check_unsafety::unsafety_check_result
  36:     0x7fe5cca53d06 - <rustc_mir_transform[bc32d14c384a311b]::check_unsafety::provide::{closure#0} as core[da774f735824091f]::ops::function::FnOnce<(rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId)>>::call_once
  37:     0x7fe5cdbfdfdd - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::UnsafetyCheckResult>::{closure#0}, &rustc_middle[78b2171944cacd78]::mir::query::UnsafetyCheckResult>
  38:     0x7fe5cdd01235 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::UnsafetyCheckResult>
  39:     0x7fe5cdedcb3c - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::UnsafetyCheckResult>>
  40:     0x7fe5cdfc7577 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::unsafety_check_result, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  41:     0x7fe5cdb48284 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::unsafety_check_result
  42:     0x7fe5cca181a3 - rustc_mir_transform[bc32d14c384a311b]::mir_const
  43:     0x7fe5cdbfaaa4 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>::{closure#0}, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>
  44:     0x7fe5cdce8466 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>
  45:     0x7fe5cdecb28e - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>>
  46:     0x7fe5cdff9013 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::mir_const, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  47:     0x7fe5cdb38de7 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::mir_const
  48:     0x7fe5cca18cdd - rustc_mir_transform[bc32d14c384a311b]::mir_promoted
  49:     0x7fe5cdbfabd4 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, (&rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_index[39ab802ee44dae62]::vec::IndexVec<rustc_middle[78b2171944cacd78]::mir::Promoted, rustc_middle[78b2171944cacd78]::mir::Body>>)>::{closure#0}, (&rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_index[39ab802ee44dae62]::vec::IndexVec<rustc_middle[78b2171944cacd78]::mir::Promoted, rustc_middle[78b2171944cacd78]::mir::Body>>)>
  50:     0x7fe5cdce8c93 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, (&rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_index[39ab802ee44dae62]::vec::IndexVec<rustc_middle[78b2171944cacd78]::mir::Promoted, rustc_middle[78b2171944cacd78]::mir::Body>>)>
  51:     0x7fe5cdfa308a - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::mir_promoted, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  52:     0x7fe5cdb3b407 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::mir_promoted
  53:     0x7fe5cd61f13f - rustc_borrowck[5ff5be9b41164a53]::mir_borrowck
  54:     0x7fe5cd5e9946 - <rustc_borrowck[5ff5be9b41164a53]::provide::{closure#0} as core[da774f735824091f]::ops::function::FnOnce<(rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId)>>::call_once
  55:     0x7fe5cdbfdefd - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::BorrowCheckResult>::{closure#0}, &rustc_middle[78b2171944cacd78]::mir::query::BorrowCheckResult>
  56:     0x7fe5cdd00a55 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::BorrowCheckResult>
  57:     0x7fe5cdedbd7c - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::BorrowCheckResult>>
  58:     0x7fe5cdfa2308 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::mir_borrowck, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  59:     0x7fe5cdb51394 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::mir_borrowck
  60:     0x7fe5cccd43fc - rustc_typeck[e314516c87591ba6]::collect::type_of::type_of
  61:     0x7fe5cdbff818 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::DefId, rustc_middle[78b2171944cacd78]::ty::Ty>::{closure#0}, rustc_middle[78b2171944cacd78]::ty::Ty>
  62:     0x7fe5cdd0e566 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::DefId, rustc_middle[78b2171944cacd78]::ty::Ty>
  63:     0x7fe5cdef2b21 - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::DefId, rustc_middle[78b2171944cacd78]::ty::Ty>>
  64:     0x7fe5cdff6d41 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::type_of, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  65:     0x7fe5cdb32d29 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::type_of
  66:     0x7fe5cccfc246 - rustc_typeck[e314516c87591ba6]::check::check::check_item_type
  67:     0x7fe5ccd02d47 - rustc_typeck[e314516c87591ba6]::check::check::check_mod_item_types
  68:     0x7fe5cdbfe19d - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, ()>::{closure#0}, ()>
  69:     0x7fe5cdd0220f - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, ()>
  70:     0x7fe5cdede69c - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, ()>>
  71:     0x7fe5cdfbf4c4 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::check_mod_item_types, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  72:     0x7fe5cdb4c314 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::check_mod_item_types
  73:     0x7fe5ccf8c1f6 - <rustc_middle[78b2171944cacd78]::hir::map::Map>::for_each_module::<rustc_typeck[e314516c87591ba6]::check_crate::{closure#6}::{closure#0}>
  74:     0x7fe5ccf16c62 - <rustc_session[87a8455e76c60bce]::session::Session>::time::<(), rustc_typeck[e314516c87591ba6]::check_crate::{closure#6}>
  75:     0x7fe5cce62d6c - rustc_typeck[e314516c87591ba6]::check_crate
  76:     0x7fe5cc4c9f91 - rustc_interface[a5f7c60448810bbc]::passes::analysis
  77:     0x7fe5cdc05254 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, (), core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  78:     0x7fe5cdd3cd2b - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, (), core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  79:     0x7fe5cdf170df - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<(), core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>>
  80:     0x7fe5cdff6e62 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::analysis, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  81:     0x7fe5cdb3327e - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::analysis
  82:     0x7fe5cc41c714 - <rustc_interface[a5f7c60448810bbc]::passes::QueryContext>::enter::<rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  83:     0x7fe5cc3c9095 - <rustc_interface[a5f7c60448810bbc]::interface::Compiler>::enter::<rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}::{closure#2}, core[da774f735824091f]::result::Result<core[da774f735824091f]::option::Option<rustc_interface[a5f7c60448810bbc]::queries::Linker>, rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  84:     0x7fe5cc3ad0ab - rustc_span[dbe5b00f8c021e5e]::with_source_map::<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_interface[a5f7c60448810bbc]::interface::create_compiler_and_run<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#1}>
  85:     0x7fe5cc3ca264 - <scoped_tls[9ccc87b08c856795]::ScopedKey<rustc_span[dbe5b00f8c021e5e]::SessionGlobals>>::set::<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  86:     0x7fe5cc4280a9 - std[b51c605e05435ab]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  87:     0x7fe5cc4205d9 - <<std[b51c605e05435ab]::thread::Builder>::spawn_unchecked_<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#1} as core[da774f735824091f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  88:     0x7fe5cb8a5473 - std::sys::unix::thread::Thread::new::thread_start::ha8ba8cbd4bd72570
  89:     0x7fe5c5df8609 - start_thread
  90:     0x7fe5cb70b133 - clone
  91:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (8e0da9ab1 2022-06-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [region_scope_tree] computing drop scopes for `<impl at /checkout/src/test/ui/async-await/issues/issue-64964.rs:9:1: 13:2>::next`
#1 [typeck] type-checking `<impl at /checkout/src/test/ui/async-await/issues/issue-64964.rs:9:1: 13:2>::next`
#2 [mir_built] building MIR for `<impl at /checkout/src/test/ui/async-await/issues/issue-64964.rs:9:1: 13:2>::next`
#3 [unsafety_check_result] unsafety-checking `<impl at /checkout/src/test/ui/async-await/issues/issue-64964.rs:9:1: 13:2>::next`
#4 [mir_const] processing MIR for `<impl at /checkout/src/test/ui/async-await/issues/issue-64964.rs:9:1: 13:2>::next`
#5 [mir_promoted] processing `<impl at /checkout/src/test/ui/async-await/issues/issue-64964.rs:9:1: 13:2>::next`
#6 [mir_borrowck] borrow-checking `<impl at /checkout/src/test/ui/async-await/issues/issue-64964.rs:9:1: 13:2>::next`
#7 [type_of] computing type of `<impl at /checkout/src/test/ui/async-await/issues/issue-64964.rs:9:1: 13:2>::next::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:7 ~ issue_64964[1770]::{impl#0}::next::{opaque#0}), substs: [ReFree(DefId(0:6 ~ issue_64964[1770]::{impl#0}::next), BrNamed(DefId(0:15 ~ issue_64964[1770]::{impl#0}::next::'_), '_))] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: /checkout/src/test/ui/async-await/issues/issue-64964.rs:10:30: 12:6 (#0), ty: _ }, origin: AsyncFn(DefId(0:6 ~ issue_64964[1770]::{impl#0}::next)) })])
   = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1373:13
stack backtrace:
stack backtrace:
   0:     0x7fe5cb89697c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66bc635c24488641
   1:     0x7fe5cb8fd818 - core::fmt::write::hcb19d8130fd1d715
   2:     0x7fe5cb8867e1 - std::io::Write::write_fmt::hfd879b7f63738318
   3:     0x7fe5cb89997e - std::panicking::default_hook::{{closure}}::h3cc863080bbf5e98
   4:     0x7fe5cb899669 - std::panicking::default_hook::hf60e9668c12215fb
   5:     0x7fe5cc3ab7e1 - rustc_driver[a4dc5fca8295e0dc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe5cb89a0e2 - std::panicking::rust_panic_with_hook::h81c95cb920c9905b
   7:     0x7fe5cf081bd3 - std[b51c605e05435ab]::panicking::begin_panic::<rustc_errors[c1f69eb103452a2b]::ExplicitBug>::{closure#0}
   8:     0x7fe5cf080ba6 - std[b51c605e05435ab]::sys_common::backtrace::__rust_end_short_backtrace::<std[b51c605e05435ab]::panicking::begin_panic<rustc_errors[c1f69eb103452a2b]::ExplicitBug>::{closure#0}, !>
   9:     0x7fe5cc3663f6 - std[b51c605e05435ab]::panicking::begin_panic::<rustc_errors[c1f69eb103452a2b]::ExplicitBug>
  10:     0x7fe5cf08f436 - std[b51c605e05435ab]::panic::panic_any::<rustc_errors[c1f69eb103452a2b]::ExplicitBug>
  11:     0x7fe5cf09499c - <rustc_errors[c1f69eb103452a2b]::HandlerInner as core[da774f735824091f]::ops::drop::Drop>::drop
  12:     0x7fe5cc3c2d32 - core[da774f735824091f]::ptr::drop_in_place::<rustc_session[87a8455e76c60bce]::parse::ParseSess>
  13:     0x7fe5cc3c6ff8 - <alloc[a6556ca02e081799]::rc::Rc<rustc_session[87a8455e76c60bce]::session::Session> as core[da774f735824091f]::ops::drop::Drop>::drop
  14:     0x7fe5cc3b04ec - core[da774f735824091f]::ptr::drop_in_place::<rustc_interface[a5f7c60448810bbc]::interface::Compiler>
  15:     0x7fe5cc3ad55a - rustc_span[dbe5b00f8c021e5e]::with_source_map::<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_interface[a5f7c60448810bbc]::interface::create_compiler_and_run<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fe5cc3ca264 - <scoped_tls[9ccc87b08c856795]::ScopedKey<rustc_span[dbe5b00f8c021e5e]::SessionGlobals>>::set::<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  17:     0x7fe5cc4280a9 - std[b51c605e05435ab]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  18:     0x7fe5cc4205d9 - <<std[b51c605e05435ab]::thread::Builder>::spawn_unchecked_<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#1} as core[da774f735824091f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7fe5cb8a5473 - std::sys::unix::thread::Thread::new::thread_start::ha8ba8cbd4bd72570
  20:     0x7fe5c5df8609 - start_thread
  21:     0x7fe5cb70b133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (8e0da9ab1 2022-06-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/issue-72766.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-72766.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-72766/issue-72766.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-72766" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-72766/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Hashing HIR bodies is forbidden.', /checkout/compiler/rustc_query_system/src/ich/impls_hir.rs:14:40
   0:     0x7f180011197c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66bc635c24488641
   1:     0x7f1800178818 - core::fmt::write::hcb19d8130fd1d715
   1:     0x7f1800178818 - core::fmt::write::hcb19d8130fd1d715
   2:     0x7f18001017e1 - std::io::Write::write_fmt::hfd879b7f63738318
   3:     0x7f180011497e - std::panicking::default_hook::{{closure}}::h3cc863080bbf5e98
   4:     0x7f1800114669 - std::panicking::default_hook::hf60e9668c12215fb
   5:     0x7f1800c267e1 - rustc_driver[a4dc5fca8295e0dc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f18001150e2 - std::panicking::rust_panic_with_hook::h81c95cb920c9905b
   7:     0x7f1800114ed9 - std::panicking::begin_panic_handler::{{closure}}::h2b88ea4cdbe4a101
   8:     0x7f1800111e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h33ca4a36f576d747
   9:     0x7f1800114bf9 - rust_begin_unwind
  10:     0x7f18000c9013 - core::panicking::panic_fmt::h7dcce2b3cd28e2a9
  11:     0x7f1803564183 - <rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext as rustc_hir[f0d27074f747bffc]::stable_hash_impls::HashStableContext>::hash_body_id
  12:     0x7f1803557afc - rustc_data_structures[9723f2609ab46cc2]::stable_hasher::stable_hash_reduce::<rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext, (&rustc_hir[f0d27074f747bffc]::hir::BodyId, &usize), std[b51c605e05435ab]::collections::hash::map::Iter<rustc_hir[f0d27074f747bffc]::hir::BodyId, usize>, <std[b51c605e05435ab]::collections::hash::map::HashMap<rustc_hir[f0d27074f747bffc]::hir::BodyId, usize, core[da774f735824091f]::hash::BuildHasherDefault<rustc_hash[7f469e8a70954446]::FxHasher>> as rustc_data_structures[9723f2609ab46cc2]::stable_hasher::HashStable<rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext>>::hash_stable::{closure#0}>
  13:     0x7f18036f2197 - <rustc_middle[78b2171944cacd78]::middle::region::ScopeTree as rustc_data_structures[9723f2609ab46cc2]::stable_hasher::HashStable<rustc_query_system[23b2a77768faa467]::ich::hcx::StableHashingContext>>::hash_stable
  14:     0x7f18025db17e - rustc_query_system[23b2a77768faa467]::dep_graph::graph::hash_result::<&rustc_middle[78b2171944cacd78]::middle::region::ScopeTree>
  15:     0x7f180259653f - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::DefId, &rustc_middle[78b2171944cacd78]::middle::region::ScopeTree>
  16:     0x7f180277b507 - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::DefId, &rustc_middle[78b2171944cacd78]::middle::region::ScopeTree>>
  17:     0x7f180282f77f - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::region_scope_tree, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  18:     0x7f18023d3f09 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::region_scope_tree
  19:     0x7f180147f943 - <rustc_typeck[e314516c87591ba6]::check::fn_ctxt::FnCtxt>::resolve_rvalue_scopes
  20:     0x7f180175a63d - <rustc_infer[192f10f2e4ad9ebd]::infer::InferCtxtBuilder>::enter::<&rustc_middle[78b2171944cacd78]::ty::context::TypeckResults, <rustc_typeck[e314516c87591ba6]::check::inherited::InheritedBuilder>::enter<rustc_typeck[e314516c87591ba6]::check::typeck_with_fallback<rustc_typeck[e314516c87591ba6]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>::{closure#0}>
  21:     0x7f180157e11e - <rustc_typeck[e314516c87591ba6]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[e314516c87591ba6]::check::typeck_with_fallback<rustc_typeck[e314516c87591ba6]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  22:     0x7f18016175fe - rustc_typeck[e314516c87591ba6]::check::typeck
  23:     0x7f1802478d3d - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  24:     0x7f180257aa95 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>
  25:     0x7f1802755fbc - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::ty::context::TypeckResults>>
  26:     0x7f1802871a87 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::typeck, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  27:     0x7f18023ca364 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::typeck
  28:     0x7f18035d516d - <rustc_middle[78b2171944cacd78]::ty::context::TyCtxt>::typeck_opt_const_arg
  29:     0x7f1801ba6d37 - rustc_mir_build[f5e089113a49545e]::build::mir_built
  30:     0x7f1802475aa4 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>::{closure#0}, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>
  31:     0x7f1802563466 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>
  32:     0x7f180274628e - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>>
  33:     0x7f1802873ed6 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::mir_built, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  34:     0x7f18023b3897 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::mir_built
  35:     0x7f18012d2870 - rustc_mir_transform[bc32d14c384a311b]::check_unsafety::unsafety_check_result
  36:     0x7f18012ced06 - <rustc_mir_transform[bc32d14c384a311b]::check_unsafety::provide::{closure#0} as core[da774f735824091f]::ops::function::FnOnce<(rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId)>>::call_once
  37:     0x7f1802478fdd - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::UnsafetyCheckResult>::{closure#0}, &rustc_middle[78b2171944cacd78]::mir::query::UnsafetyCheckResult>
  38:     0x7f180257c235 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::UnsafetyCheckResult>
  39:     0x7f1802757b3c - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::UnsafetyCheckResult>>
  40:     0x7f1802842577 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::unsafety_check_result, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  41:     0x7f18023c3284 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::unsafety_check_result
  42:     0x7f18012931a3 - rustc_mir_transform[bc32d14c384a311b]::mir_const
  43:     0x7f1802475aa4 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>::{closure#0}, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>
  44:     0x7f1802563466 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>
  45:     0x7f180274628e - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>>>
  46:     0x7f1802874013 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::mir_const, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  47:     0x7f18023b3de7 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::mir_const
  48:     0x7f1801293cdd - rustc_mir_transform[bc32d14c384a311b]::mir_promoted
  49:     0x7f1802475bd4 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, (&rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_index[39ab802ee44dae62]::vec::IndexVec<rustc_middle[78b2171944cacd78]::mir::Promoted, rustc_middle[78b2171944cacd78]::mir::Body>>)>::{closure#0}, (&rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_index[39ab802ee44dae62]::vec::IndexVec<rustc_middle[78b2171944cacd78]::mir::Promoted, rustc_middle[78b2171944cacd78]::mir::Body>>)>
  50:     0x7f1802563c93 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_middle[78b2171944cacd78]::ty::WithOptConstParam<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId>, (&rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_middle[78b2171944cacd78]::mir::Body>, &rustc_data_structures[9723f2609ab46cc2]::steal::Steal<rustc_index[39ab802ee44dae62]::vec::IndexVec<rustc_middle[78b2171944cacd78]::mir::Promoted, rustc_middle[78b2171944cacd78]::mir::Body>>)>
  51:     0x7f180281e08a - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::mir_promoted, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  52:     0x7f18023b6407 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::mir_promoted
  53:     0x7f1801e9a13f - rustc_borrowck[5ff5be9b41164a53]::mir_borrowck
  54:     0x7f1801e64946 - <rustc_borrowck[5ff5be9b41164a53]::provide::{closure#0} as core[da774f735824091f]::ops::function::FnOnce<(rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId)>>::call_once
  55:     0x7f1802478efd - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::BorrowCheckResult>::{closure#0}, &rustc_middle[78b2171944cacd78]::mir::query::BorrowCheckResult>
  56:     0x7f180257ba55 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::BorrowCheckResult>
  57:     0x7f1802756d7c - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, &rustc_middle[78b2171944cacd78]::mir::query::BorrowCheckResult>>
  58:     0x7f180281d308 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::mir_borrowck, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  59:     0x7f18023cc394 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::mir_borrowck
  60:     0x7f180154f3fc - rustc_typeck[e314516c87591ba6]::collect::type_of::type_of
  61:     0x7f180247a818 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::DefId, rustc_middle[78b2171944cacd78]::ty::Ty>::{closure#0}, rustc_middle[78b2171944cacd78]::ty::Ty>
  62:     0x7f1802589566 - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::DefId, rustc_middle[78b2171944cacd78]::ty::Ty>
  63:     0x7f180276db21 - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::DefId, rustc_middle[78b2171944cacd78]::ty::Ty>>
  64:     0x7f1802871d41 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::type_of, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  65:     0x7f18023add29 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::type_of
  66:     0x7f1801577246 - rustc_typeck[e314516c87591ba6]::check::check::check_item_type
  67:     0x7f180157dd47 - rustc_typeck[e314516c87591ba6]::check::check::check_mod_item_types
  68:     0x7f180247919d - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, ()>::{closure#0}, ()>
  69:     0x7f180257d20f - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, ()>
  70:     0x7f180275969c - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<rustc_span[dbe5b00f8c021e5e]::def_id::LocalDefId, ()>>
  71:     0x7f180283a4c4 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::check_mod_item_types, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  72:     0x7f18023c7314 - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::check_mod_item_types
  73:     0x7f18018071f6 - <rustc_middle[78b2171944cacd78]::hir::map::Map>::for_each_module::<rustc_typeck[e314516c87591ba6]::check_crate::{closure#6}::{closure#0}>
  74:     0x7f1801791c62 - <rustc_session[87a8455e76c60bce]::session::Session>::time::<(), rustc_typeck[e314516c87591ba6]::check_crate::{closure#6}>
  75:     0x7f18016ddd6c - rustc_typeck[e314516c87591ba6]::check_crate
  76:     0x7f1800d44f91 - rustc_interface[a5f7c60448810bbc]::passes::analysis
  77:     0x7f1802480254 - <rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind as rustc_query_system[23b2a77768faa467]::dep_graph::DepKind>::with_deps::<<rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, (), core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  78:     0x7f18025b7d2b - <rustc_query_system[23b2a77768faa467]::dep_graph::graph::DepGraph<rustc_middle[78b2171944cacd78]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[78b2171944cacd78]::ty::context::TyCtxt, (), core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  79:     0x7f18027920df - rustc_query_system[23b2a77768faa467]::query::plumbing::try_execute_query::<rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt, rustc_query_system[23b2a77768faa467]::query::caches::DefaultCache<(), core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>>
  80:     0x7f1802871e62 - rustc_query_system[23b2a77768faa467]::query::plumbing::get_query::<rustc_query_impl[c7f7bafe4e0630b2]::queries::analysis, rustc_query_impl[c7f7bafe4e0630b2]::plumbing::QueryCtxt>
  81:     0x7f18023ae27e - <rustc_query_impl[c7f7bafe4e0630b2]::Queries as rustc_middle[78b2171944cacd78]::ty::query::QueryEngine>::analysis
  82:     0x7f1800c97714 - <rustc_interface[a5f7c60448810bbc]::passes::QueryContext>::enter::<rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  83:     0x7f1800c44095 - <rustc_interface[a5f7c60448810bbc]::interface::Compiler>::enter::<rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}::{closure#2}, core[da774f735824091f]::result::Result<core[da774f735824091f]::option::Option<rustc_interface[a5f7c60448810bbc]::queries::Linker>, rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  84:     0x7f1800c280ab - rustc_span[dbe5b00f8c021e5e]::with_source_map::<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_interface[a5f7c60448810bbc]::interface::create_compiler_and_run<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#1}>
  85:     0x7f1800c45264 - <scoped_tls[9ccc87b08c856795]::ScopedKey<rustc_span[dbe5b00f8c021e5e]::SessionGlobals>>::set::<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  86:     0x7f1800ca30a9 - std[b51c605e05435ab]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  87:     0x7f1800c9b5d9 - <<std[b51c605e05435ab]::thread::Builder>::spawn_unchecked_<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#1} as core[da774f735824091f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  88:     0x7f1800120473 - std::sys::unix::thread::Thread::new::thread_start::ha8ba8cbd4bd72570
  89:     0x7f17fa673609 - start_thread
  90:     0x7f17fff86133 - clone
  91:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (8e0da9ab1 2022-06-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [region_scope_tree] computing drop scopes for `<impl at /checkout/src/test/ui/suggestions/issue-72766.rs:6:1: 10:2>::call`
#1 [typeck] type-checking `<impl at /checkout/src/test/ui/suggestions/issue-72766.rs:6:1: 10:2>::call`
#2 [mir_built] building MIR for `<impl at /checkout/src/test/ui/suggestions/issue-72766.rs:6:1: 10:2>::call`
#3 [unsafety_check_result] unsafety-checking `<impl at /checkout/src/test/ui/suggestions/issue-72766.rs:6:1: 10:2>::call`
#4 [mir_const] processing MIR for `<impl at /checkout/src/test/ui/suggestions/issue-72766.rs:6:1: 10:2>::call`
#5 [mir_promoted] processing `<impl at /checkout/src/test/ui/suggestions/issue-72766.rs:6:1: 10:2>::call`
#6 [mir_borrowck] borrow-checking `<impl at /checkout/src/test/ui/suggestions/issue-72766.rs:6:1: 10:2>::call`
#7 [type_of] computing type of `<impl at /checkout/src/test/ui/suggestions/issue-72766.rs:6:1: 10:2>::call::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:7 ~ issue_72766[e611]::{impl#0}::call::{opaque#0}), substs: [ReFree(DefId(0:6 ~ issue_72766[e611]::{impl#0}::call), BrNamed(DefId(0:13 ~ issue_72766[e611]::{impl#0}::call::'_), '_))] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: /checkout/src/test/ui/suggestions/issue-72766.rs:7:48: 9:6 (#0), ty: _ }, origin: AsyncFn(DefId(0:6 ~ issue_72766[e611]::{impl#0}::call)) })])
   = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1373:13
stack backtrace:
stack backtrace:
   0:     0x7f180011197c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h66bc635c24488641
   1:     0x7f1800178818 - core::fmt::write::hcb19d8130fd1d715
   2:     0x7f18001017e1 - std::io::Write::write_fmt::hfd879b7f63738318
   3:     0x7f180011497e - std::panicking::default_hook::{{closure}}::h3cc863080bbf5e98
   4:     0x7f1800114669 - std::panicking::default_hook::hf60e9668c12215fb
   5:     0x7f1800c267e1 - rustc_driver[a4dc5fca8295e0dc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f18001150e2 - std::panicking::rust_panic_with_hook::h81c95cb920c9905b
   7:     0x7f18038fcbd3 - std[b51c605e05435ab]::panicking::begin_panic::<rustc_errors[c1f69eb103452a2b]::ExplicitBug>::{closure#0}
   8:     0x7f18038fbba6 - std[b51c605e05435ab]::sys_common::backtrace::__rust_end_short_backtrace::<std[b51c605e05435ab]::panicking::begin_panic<rustc_errors[c1f69eb103452a2b]::ExplicitBug>::{closure#0}, !>
   9:     0x7f1800be13f6 - std[b51c605e05435ab]::panicking::begin_panic::<rustc_errors[c1f69eb103452a2b]::ExplicitBug>
  10:     0x7f180390a436 - std[b51c605e05435ab]::panic::panic_any::<rustc_errors[c1f69eb103452a2b]::ExplicitBug>
  11:     0x7f180390f99c - <rustc_errors[c1f69eb103452a2b]::HandlerInner as core[da774f735824091f]::ops::drop::Drop>::drop
  12:     0x7f1800c3dd32 - core[da774f735824091f]::ptr::drop_in_place::<rustc_session[87a8455e76c60bce]::parse::ParseSess>
  13:     0x7f1800c41ff8 - <alloc[a6556ca02e081799]::rc::Rc<rustc_session[87a8455e76c60bce]::session::Session> as core[da774f735824091f]::ops::drop::Drop>::drop
  14:     0x7f1800c2b4ec - core[da774f735824091f]::ptr::drop_in_place::<rustc_interface[a5f7c60448810bbc]::interface::Compiler>
  15:     0x7f1800c2855a - rustc_span[dbe5b00f8c021e5e]::with_source_map::<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_interface[a5f7c60448810bbc]::interface::create_compiler_and_run<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f1800c45264 - <scoped_tls[9ccc87b08c856795]::ScopedKey<rustc_span[dbe5b00f8c021e5e]::SessionGlobals>>::set::<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  17:     0x7f1800ca30a9 - std[b51c605e05435ab]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>
  18:     0x7f1800c9b5d9 - <<std[b51c605e05435ab]::thread::Builder>::spawn_unchecked_<rustc_interface[a5f7c60448810bbc]::util::run_in_thread_pool_with_globals<rustc_interface[a5f7c60448810bbc]::interface::run_compiler<core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>, rustc_driver[a4dc5fca8295e0dc]::run_compiler::{closure#1}>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#0}, core[da774f735824091f]::result::Result<(), rustc_errors[c1f69eb103452a2b]::ErrorGuaranteed>>::{closure#1} as core[da774f735824091f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7f1800120473 - std::sys::unix::thread::Thread::new::thread_start::ha8ba8cbd4bd72570
  20:     0x7f17fa673609 - start_thread
  21:     0x7f17fff86133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (8e0da9ab1 2022-06-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
