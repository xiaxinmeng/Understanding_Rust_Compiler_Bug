plain
travis_time:end:00d4db6a:start=1552652248282691615,finish=1552652249204306350,duration=921614735
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:16]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:27]    Compiling synstructure v0.10.1
[00:06:48]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:39]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:41] error: no rules expected the token `,`
[00:07:41]    --> src/librustc/ty/query/mod.rs:110:77
[00:07:41]     |
[00:07:41] 110 |         [no_hash] fn hir_map: HirMap(CrateNum) -> &'tcx hir::map::Map<'tcx>,,
[00:07:41]     |                                                                             ^ no rules expected this token in macro call
[00:07:41]     | 
[00:07:41]    ::: src/librustc/ty/query/plumbing.rs:683:1
[00:07:41]     |
[00:07:41] 683 | macro_rules! define_queries {
[00:07:41]     | --------------------------- when calling this macro
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]   --> src/librustc/hir/mod.rs:33:5
[00:07:41]    |
[00:07:41] 33 | use crate::ty::query::Providers;
[00:07:41] 33 | use crate::ty::query::Providers;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]  --> src/librustc/hir/check_attr.rs:9:5
[00:07:41]   |
[00:07:41] 9 | use crate::ty::query::Providers;
[00:07:41] 9 | use crate::ty::query::Providers;
[00:07:41]   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]   --> src/librustc/lint/mod.rs:33:5
[00:07:41]    |
[00:07:41] 33 | use crate::ty::query::Providers;
[00:07:41] 33 | use crate::ty::query::Providers;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]   --> src/librustc/middle/entry.rs:11:5
[00:07:41]    |
[00:07:41] 11 | use crate::ty::query::Providers;
[00:07:41] 11 | use crate::ty::query::Providers;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]  --> src/librustc/middle/intrinsicck.rs:5:5
[00:07:41] 5 | use crate::ty::query::Providers;
[00:07:41] 5 | use crate::ty::query::Providers;
[00:07:41]   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]    --> src/librustc/middle/liveness.rs:103:5
[00:07:41]     |
[00:07:41] 103 | use crate::ty::query::Providers;
[00:07:41] 103 | use crate::ty::query::Providers;
[00:07:41]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]   --> src/librustc/middle/reachable.rs:14:5
[00:07:41]    |
[00:07:41] 14 | use crate::ty::query::Providers;
[00:07:41] 14 | use crate::ty::query::Providers;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]   --> src/librustc/middle/region.rs:21:5
[00:07:41]    |
[00:07:41] 21 | use crate::ty::query::Providers;
[00:07:41] 21 | use crate::ty::query::Providers;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]   --> src/librustc/middle/stability.rs:11:5
[00:07:41]    |
[00:07:41] 11 | use crate::ty::query::Providers;
[00:07:41] 11 | use crate::ty::query::Providers;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] 
[00:07:41] error[E0432]: unresolved import `crate::ty::query::TyCtxtAt`
[00:07:41]   --> src/librustc/mir/interpret/error.rs:15:5
[00:07:41]    |
[00:07:41] 15 | use crate::ty::query::TyCtxtAt;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TyCtxtAt` in `ty::query`. Did you mean to use `TyCtxt`?
[00:07:41] 
[00:07:41] error[E0432]: unresolved import `self::query::queries`
[00:07:41]    |
[00:07:41]    |
[00:07:41] 85 | pub use self::query::queries;
[00:07:41]    |         ^^^^^^^^^^^^^^^^^^^^ no `queries` in `ty::query`
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Providers`
[00:07:41]  --> src/librustc/ty/constness.rs:1:5
[00:07:41]   |
[00:07:41] 1 | use crate::ty::query::Providers;
[00:07:41] 1 | use crate::ty::query::Providers;
[00:07:41]   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Providers` in `ty::query`
[00:07:41] 
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Query`
[00:07:41]  --> src/librustc/ty/query/plumbing.rs:8:5
[00:07:41]   |
[00:07:41] 8 | use crate::ty::query::Query;
[00:07:41]   |     ^^^^^^^^^^^^^^^^^^^^^^^ no `Query` in `ty::query`
[00:07:41] 
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Query`
[00:07:41]   --> src/librustc/ty/query/job.rs:14:5
[00:07:41]    |
[00:07:41] 14 | use crate::ty::query::Query;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^ no `Query` in `ty::query`
[00:07:41] 
[00:07:41] error[E0432]: unresolved import `crate::ty::query::queries`
[00:07:41]   --> src/librustc/ty/query/config.rs:13:5
[00:07:41]    |
[00:07:41] 13 | use crate::ty::query::queries;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ no `queries` in `ty::query`
[00:07:41] 
[00:07:41] error[E0432]: unresolved import `crate::ty::query::Query`
[00:07:41]   --> src/librustc/ty/query/config.rs:14:5
[00:07:41]    |
[00:07:41] 14 | use crate::ty::query::Query;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^ no `Query` in `ty::query`
[00:07:41] 
[00:07:41] error[E0432]: unresolved import `crate::ty::query::queries`
[00:07:41]    --> src/librustc/ty/query/on_disk_cache.rs:206:39
[00:07:41]     |
[00:07:41] 206 |                 use crate::ty::query::queries::*;
[00:07:41]     |                                       ^^^^^^^ could not find `queries` in `query`
[00:07:41] 
[00:07:41] error[E0432]: unresolved import `crate::ty::query::TyCtxtAt`
[00:07:41]    |
[00:07:41]    |
[00:07:41] 12 | use crate::ty::query::TyCtxtAt;
[00:07:41]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `TyCtxtAt` in `ty::query`. Did you mean to use `TyCtxt`?
[00:07:41] 
[00:07:41] error[E0432]: unresolved import `crate::ty::query::queries`
[00:07:41]     --> src/librustc/ty/query/plumbing.rs:1462:21
[00:07:41]      |
[00:07:41] 1456 | / macro_rules! impl_load_from_cache {
[00:07:41] 1457 | |     ($($dep_kind:ident => $query_name:ident,)*) => {
[00:07:41] 1458 | |         impl DepNode {
[00:07:41] 1459 | |             // Check whether the query invocation corresponding to the given
[00:07:41] ...    |
[00:07:41] 1462 | |                 use crate::ty::query::queries;
[00:07:41]      | |                     ^^^^^^^^^^^^^^^^^^^^^^^^^ no `queries` in `ty::query`
[00:07:41] 1496 | |     }
[00:07:41] 1497 | | }
[00:07:41] 1497 | | }
[00:07:41]      | |_- in this expansion of `impl_load_from_cache!`
[00:07:41] 1498 | 
[00:07:41] 1499 | / impl_load_from_cache!(
[00:07:41] 1500 | |     TypeckTables => typeck_tables_of,
[00:07:41] 1501 | |     MirOptimized => optimized_mir,
[00:07:41] 1502 | |     UnsafetyCheckResult => unsafety_check_result,
[00:07:41] ...    |
[00:07:41] 1514 | |     SpecializationGraph => specialization_graph_of,
[00:07:41] 1515 | | );
[00:07:41]      | |__- in this macro invocation
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1193 |               force!(hir_map, LOCAL_CRATE);
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1197 |               force!(hir_map, krate!());
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1257 |           DepKind::RegionScopeTree => { force!(region_scope_tree, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1259 |           DepKind::Coherence => { force!(crate_inherent_impls, LOCAL_CRATE); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1261 |               force!(crate_inherent_impls_overlap_check, LOCAL_CRATE)
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1263 |           DepKind::PrivacyAccessLevels => { force!(privacy_access_levels, LOCAL_CRATE); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1264 |           DepKind::CheckPrivateInPublic => { force!(check_private_in_public, LOCAL_CRATE); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1265 |           DepKind::MirBuilt => { force!(mir_built, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1266 |           DepKind::MirConstQualif => { force!(mir_const_qualif, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1267 |           DepKind::MirConst => { force!(mir_const, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1268 |           DepKind::MirValidated => { force!(mir_validated, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1269 |           DepKind::MirOptimized => { force!(optimized_mir, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1271 |           DepKind::BorrowCheck => { force!(borrowck, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1272 |           DepKind::MirBorrowCheck => { force!(mir_borrowck, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1273 |           DepKind::UnsafetyCheckResult => { force!(unsafety_check_result, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1274 |           DepKind::UnsafeDeriveOnReprPacked => { force!(unsafe_derive_on_repr_packed, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1275 |           DepKind::CheckModAttrs => { force!(check_mod_attrs, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1276 |           DepKind::CheckModLoops => { force!(check_mod_loops, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1277 |           DepKind::CheckModUnstableApiUsage => { force!(check_mod_unstable_api_usage, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1278 |           DepKind::CheckModItemTypes => { force!(check_mod_item_types, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1279 |           DepKind::CheckModPrivacy => { force!(check_mod_privacy, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45]     --> src/librustc/ty/query/plumbing.rs:1182:53
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1179 | /     macro_rules! force {
[00:07:45] 1180 | |         ($query:ident, $key:expr) => {
[00:07:45] 1181 | |             {
[00:07:45] 1182 | |                 tcx.force_query::<crate::ty::query::queries::$query<'_>>($key, DUMMY_SP, *dep_node);
[00:07:45]      | |                                                     ^^^^^^^ could not find `queries` in `query`
[00:07:45] 1184 | |         }
[00:07:45] 1185 | |     };
[00:07:45]      | |_____- in this expansion of `force!`
[00:07:45] ...
[00:07:45] ...
[00:07:45] 1280 |           DepKind::CheckModIntrinsics => { force!(check_mod_intrinsics, def_id!()); }
[00:07:45] 
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
[00:07:45] error[E0433]: failed to resolve: could not find `queries` in `query`
---
[00:07:58]    |
[00:07:58] 12 | use crate::middle::region;
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 
[00:07:58] error: unused imports: `ObjectLifetimeDefault`, `Region`, `ResolveLifetimes`
[00:07:58]   --> src/librustc/ty/query/mod.rs:13:39
[00:07:58]    |
[00:07:58] 13 | use crate::middle::resolve_lifetime::{ResolveLifetimes, Region, ObjectLifetimeDefault};
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused imports: `DeprecationEntry`, `self`
[00:07:58]   --> src/librustc/ty/query/mod.rs:14:32
[00:07:58] 14 | use crate::middle::stability::{self, DeprecationEntry};
[00:07:58]    |                                ^^^^  ^^^^^^^^^^^^^^^^
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::middle::lib_features::LibFeatures`
[00:07:58]   --> src/librustc/ty/query/mod.rs:15:5
[00:07:58]    |
[00:07:58] 15 | use crate::middle::lib_features::LibFeatures;
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused imports: `LangItem`, `LanguageItems`
[00:07:58]   --> src/librustc/ty/query/mod.rs:16:33
[00:07:58]    |
[00:07:58] 16 | use crate::middle::lang_items::{LanguageItems, LangItem};
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused imports: `ExportedSymbol`, `SymbolExportLevel`
[00:07:58]   --> src/librustc/ty/query/mod.rs:17:39
[00:07:58]    |
[00:07:58] 17 | use crate::middle::exported_symbols::{SymbolExportLevel, ExportedSymbol};
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused imports: `ConstEvalRawResult`, `ConstEvalResult`
[00:07:58]   --> src/librustc/ty/query/mod.rs:18:29
[00:07:58]    |
[00:07:58] 18 | use crate::mir::interpret::{ConstEvalRawResult, ConstEvalResult};
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::mir::mono::CodegenUnit`
[00:07:58]   --> src/librustc/ty/query/mod.rs:19:5
[00:07:58]    |
[00:07:58] 19 | use crate::mir::mono::CodegenUnit;
[00:07:58] 
[00:07:58] error: unused import: `crate::mir`
[00:07:58]   --> src/librustc/ty/query/mod.rs:20:5
[00:07:58]    |
---
[00:07:58]    |
[00:07:58] 22 | use crate::session::CrateDisambiguator;
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 
[00:07:58] error: unused imports: `EntryFnType`, `OptLevel`, `OutputFilenames`
[00:07:58]   --> src/librustc/ty/query/mod.rs:23:30
[00:07:58]    |
[00:07:58] 23 | use crate::session::config::{EntryFnType, OutputFilenames, OptLevel};
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused imports: `Vtable`, `self`
[00:07:58]   --> src/librustc/ty/query/mod.rs:24:21
[00:07:58]    |
[00:07:58] 24 | use crate::traits::{self, Vtable};
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused imports: `CanonicalPredicateGoal`, `CanonicalProjectionGoal`, `CanonicalTyGoal`, `CanonicalTypeOpAscribeUserTypeGoal`, `CanonicalTypeOpEqGoal`, `CanonicalTypeOpNormalizeGoal`, `CanonicalTypeOpProvePredicateGoal`, `CanonicalTypeOpSubtypeGoal`, `NoSolution`
[00:07:58]   --> src/librustc/ty/query/mod.rs:26:5
[00:07:58]    |
[00:07:58] 26 |     CanonicalPredicateGoal, CanonicalProjectionGoal,
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 27 |     CanonicalTyGoal, CanonicalTypeOpAscribeUserTypeGoal,
[00:07:58]    |     ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 28 |     CanonicalTypeOpEqGoal, CanonicalTypeOpSubtypeGoal, CanonicalTypeOpProvePredicateGoal,
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 29 |     CanonicalTypeOpNormalizeGoal, NoSolution,
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::traits::query::method_autoderef::MethodAutoderefStepsResult`
[00:07:58]   --> src/librustc/ty/query/mod.rs:31:5
[00:07:58]    |
[00:07:58] 31 | use crate::traits::query::method_autoderef::MethodAutoderefStepsResult;
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused imports: `DropckOutlivesResult`, `DtorckConstraint`
[00:07:58]   --> src/librustc/ty/query/mod.rs:32:45
[00:07:58]    |
[00:07:58] 32 | use crate::traits::query::dropck_outlives::{DtorckConstraint, DropckOutlivesResult};
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::traits::query::normalize::NormalizationResult`
[00:07:58]   --> src/librustc/ty/query/mod.rs:33:5
[00:07:58]    |
[00:07:58] 33 | use crate::traits::query::normalize::NormalizationResult;
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::traits::query::outlives_bounds::OutlivesBound`
[00:07:58]   --> src/librustc/ty/query/mod.rs:34:5
[00:07:58]    |
[00:07:58] 34 | use crate::traits::query::outlives_bounds::OutlivesBound;
[00:07:58] 
[00:07:58] error: unused import: `crate::traits::specialization_graph`
[00:07:58]   --> src/librustc/ty/query/mod.rs:35:5
[00:07:58]    |
---
[00:07:58]    |
[00:07:58] 36 | use crate::traits::Clauses;
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 
[00:07:58] error: unused imports: `AdtSizedConstraint`, `CrateInherentImpls`, `ParamEnvAnd`, `TyCtxt`
[00:07:58]   --> src/librustc/ty/query/mod.rs:37:23
[00:07:58]    |
[00:07:58] 37 | use crate::ty::{self, CrateInherentImpls, ParamEnvAnd, Ty, TyCtxt, AdtSizedConstraint};
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::ty::steal::Steal`
[00:07:58]   --> src/librustc/ty/query/mod.rs:38:5
[00:07:58]    |
[00:07:58] 38 | use crate::ty::steal::Steal;
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::ty::util::NeedsDrop`
[00:07:58]   --> src/librustc/ty/query/mod.rs:39:5
[00:07:58]    |
[00:07:58] 39 | use crate::ty::util::NeedsDrop;
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused imports: `DefIdMap`, `DefIdSet`, `ItemLocalSet`
[00:07:58]   --> src/librustc/ty/query/mod.rs:41:28
[00:07:58]    |
[00:07:58] 41 | use crate::util::nodemap::{DefIdSet, DefIdMap, ItemLocalSet};
[00:07:58] 
[00:07:58] error: unused import: `ErrorReported`
[00:07:58]   --> src/librustc/ty/query/mod.rs:42:27
[00:07:58]    |
[00:07:58]    |
[00:07:58] 42 | use crate::util::common::{ErrorReported};
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::util::profiling::ProfileCategory::*`
[00:07:58]   --> src/librustc/ty/query/mod.rs:43:5
[00:07:58]    |
[00:07:58] 43 | use crate::util::profiling::ProfileCategory::*;
[00:07:58] 
[00:07:58] error: unused import: `crate::session::Session`
[00:07:58]   --> src/librustc/ty/query/mod.rs:44:5
[00:07:58]    |
[00:07:58]    |
[00:07:58] 44 | use crate::session::Session;
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 
[00:07:58] error: unused import: `rustc_data_structures::svh::Svh`
[00:07:58]   --> src/librustc/ty/query/mod.rs:46:5
[00:07:58]    |
[00:07:58] 46 | use rustc_data_structures::svh::Svh;
[00:07:58] 
[00:07:58] error: unused import: `rustc_data_structures::bit_set::BitSet`
[00:07:58]   --> src/librustc/ty/query/mod.rs:47:5
[00:07:58]    |
---
[00:07:58]    |
[00:07:58] 48 | use rustc_data_structures::indexed_vec::IndexVec;
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 
[00:07:58] error: unused imports: `FxHashMap`, `FxHashSet`
[00:07:58]   --> src/librustc/ty/query/mod.rs:49:33
[00:07:58]    |
[00:07:58] 49 | use rustc_data_structures::fx::{FxHashMap, FxHashSet};
[00:07:58] 
[00:07:58] error: unused import: `rustc_data_structures::stable_hasher::StableVec`
[00:07:58]   --> src/librustc/ty/query/mod.rs:50:5
[00:07:58]    |
---
[00:07:58]    |
[00:07:58] 58 | use std::intrinsics::type_name;
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 
[00:07:58] error: unused imports: `DUMMY_SP`, `Span`
[00:07:58]   --> src/librustc/ty/query/mod.rs:59:18
[00:07:58] 59 | use syntax_pos::{Span, DUMMY_SP};
[00:07:58]    |                  ^^^^  ^^^^^^^^
[00:07:58] 
[00:07:58] error: unused import: `syntax_pos::symbol::InternedString`
---
[00:07:58] 64 | use syntax::symbol::Symbol;
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^^^^
[00:07:58] 
[00:07:58] error: unused macro definition
[00:07:58]    --> src/librustc/ty/query/plumbing.rs:652:1
[00:07:58] 652 | / macro_rules! handle_cycle_error {
[00:07:58] 652 | / macro_rules! handle_cycle_error {
[00:07:58] 653 | |     ([][$tcx: expr, $error:expr]) => {{
[00:07:58] 654 | |         $tcx.report_cycle($error).emit();
[00:07:58] 655 | |         Value::from_cycle_error($tcx.global_tcx())
[00:07:58] 668 | |     };
[00:07:58] 669 | | }
[00:07:58]     | |_^
[00:07:58]     |
[00:07:58]     |
[00:07:58]     = note: `-D unused-macros` implied by `-D warnings`
[00:07:58] 
[00:07:58] error: unused macro definition
[00:07:58]    --> src/librustc/ty/query/plumbing.rs:671:1
[00:07:58] 671 | / macro_rules! hash_result {
[00:07:58] 671 | / macro_rules! hash_result {
[00:07:58] 672 | |     ([][$hcx:expr, $result:expr]) => {{
[00:07:58] 673 | |         dep_graph::hash_result($hcx, &$result)
[00:07:58] 674 | |     }};
[00:07:58] 680 | |     };
[00:07:58] 681 | | }
[00:07:58]     | |_^
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused macro definition
[00:07:58]     --> src/librustc/ty/query/plumbing.rs:693:1
[00:07:58]      |
[00:07:58] 693  | / macro_rules! define_queries_inner {
[00:07:58] 694  | |     (<$tcx:tt>
[00:07:58] 695  | |      $($(#[$attr:meta])* category<$category:tt>
[00:07:58] 696  | |         [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
[00:07:58] 1057 | |     }
[00:07:58] 1058 | | }
[00:07:58]      | |_^
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused macro definition
[00:07:58]     --> src/librustc/ty/query/plumbing.rs:1060:1
[00:07:58]      |
[00:07:58] 1060 | / macro_rules! define_queries_struct {
[00:07:58] 1061 | |     (tcx: $tcx:tt,
[00:07:58] 1062 | |      input: ($(([$($modifiers:tt)*] [$($attr:tt)*] [$name:ident]))*)) => {
[00:07:58] 1063 | |         pub struct Queries<$tcx> {
[00:07:58] 1074 | |     };
[00:07:58] 1075 | | }
[00:07:58]      | |_^
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused macro definition
[00:07:58]     --> src/librustc/ty/query/plumbing.rs:1077:1
[00:07:58] 1077 | / macro_rules! define_provider_struct {
[00:07:58] 1077 | / macro_rules! define_provider_struct {
[00:07:58] 1078 | |     (tcx: $tcx:tt,
[00:07:58] 1079 | |      input: ($(([$($modifiers:tt)*] [$name:ident] [$K:ty] [$R:ty]))*)) => {
[00:07:58] 1080 | |         pub struct Providers<$tcx> {
[00:07:58] 1093 | |     };
[00:07:58] 1094 | | }
[00:07:58]      | |_^
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::ty::query::QueryDescription`
[00:07:58]     --> src/librustc/ty/query/plumbing.rs:1463:21
[00:07:58]      |
[00:07:58] 1456 | / macro_rules! impl_load_from_cache {
[00:07:58] 1457 | |     ($($dep_kind:ident => $query_name:ident,)*) => {
[00:07:58] 1458 | |         impl DepNode {
[00:07:58] 1459 | |             // Check whether the query invocation corresponding to the given
[00:07:58] ...    |
[00:07:58] 1463 | |                 use crate::ty::query::QueryDescription;
[00:07:58] ...    |
[00:07:58] 1496 | |     }
[00:07:58] 1497 | | }
[00:07:58] 1497 | | }
[00:07:58]      | |_- in this expansion of `impl_load_from_cache!`
[00:07:58] 1498 | 
[00:07:58] 1499 | / impl_load_from_cache!(
[00:07:58] 1500 | |     TypeckTables => typeck_tables_of,
[00:07:58] 1501 | |     MirOptimized => optimized_mir,
[00:07:58] 1502 | |     UnsafetyCheckResult => unsafety_check_result,
[00:07:58] ...    |
[00:07:58] 1514 | |     SpecializationGraph => specialization_graph_of,
[00:07:58] 1515 | | );
[00:07:58]      | |__- in this macro invocation
[00:07:58] 
[00:07:58] error: unused import: `self::keys::Key`
[00:07:58]   --> src/librustc/ty/query/mod.rs:77:5
[00:07:58]    |
[00:07:58] 77 | use self::keys::Key;
[00:07:58] 
[00:07:58] error: unused import: `self::values::Value`
[00:07:58]   --> src/librustc/ty/query/mod.rs:80:5
[00:07:58]    |
[00:07:58]    |
[00:07:58] 80 | use self::values::Value;
[00:07:58]    |     ^^^^^^^^^^^^^^^^^^^
[00:07:58] 
[00:07:58] error: unused import: `crate::ty::query::queries::*`
[00:07:58]    --> src/librustc/ty/query/on_disk_cache.rs:206:21
[00:07:58]     |
[00:07:58] 206 |                 use crate::ty::query::queries::*;
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::ty::query::QueryAccessors`
[00:07:58]    --> src/librustc/ty/query/on_disk_cache.rs:229:21
[00:07:58]     |
[00:07:58] 229 |                 use crate::ty::query::QueryAccessors;
[00:07:58] 
[00:07:58] 
[00:07:58] error: unused import: `crate::ty::query::config::QueryDescription`
[00:07:58]    --> src/librustc/ty/query/on_disk_cache.rs:233:25
[00:07:58]     |
[00:07:58] 233 |                     use crate::ty::query::config::QueryDescription;
[00:07:58] 
[00:07:58] 
[00:08:01] error[E0581]: return type references lifetime `'tcx`, which is not constrained by the fn input types
[00:08:01]    --> src/librustc/mir/interpret/error.rs:164:6
[00:08:01]     |
[00:08:01] 164 | ) -> DiagnosticBuilder<'tcx> {
[00:08:01] 
[00:08:01] error: aborting due to 276 previous errors
[00:08:01] 
[00:08:01] Some errors occurred: E0412, E0422, E0432, E0433, E0581.
---
travis_time:end:005a44ae:start=1552652747712727337,finish=1552652747717475815,duration=4748478
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:239e89c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02cd55bc
travis_time:start:02cd55bc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/n
