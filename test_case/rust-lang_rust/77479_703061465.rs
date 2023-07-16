
warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_data_structures/src/fx.rs:3:22
  |
3 | pub use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
  |                      ^^^^^^^^^
  |
  = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_data_structures/src/fx.rs:3:33
  |
3 | pub use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
  |                                 ^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_data_structures/src/fx.rs:3:44
  |
3 | pub use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
  |                                            ^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_data_structures/src/jobserver.rs:1:1
  |
1 | pub use jobserver_crate::Client;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/owning_ref/mod.rs:248:5
    |
248 |     CloneStableDeref as CloneStableAddress, StableDeref as StableAddress,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/owning_ref/mod.rs:248:45
    |
248 |     CloneStableDeref as CloneStableAddress, StableDeref as StableAddress,
    |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_data_structures/src/snapshot_map/mod.rs:8:1
  |
8 | pub use crate::undo_log::Snapshot;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_data_structures/src/stable_map.rs:1:1
  |
1 | pub use rustc_hash::FxHashMap;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_data_structures/src/lib.rs:85:1
   |
85 | pub use ena::snapshot_vec;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_data_structures/src/stable_set.rs:1:1
  |
1 | pub use rustc_hash::FxHashSet;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_data_structures/src/sync.rs:25:1
   |
25 | pub use std::sync::atomic::Ordering;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_data_structures/src/sync.rs:26:1
   |
26 | pub use std::sync::atomic::Ordering::SeqCst;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_data_structures/src/sync.rs:26:9
   |
26 | pub use std::sync::atomic::Ordering::SeqCst;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:199:9
    |
199 |         pub use std::iter::Iterator as ParallelIterator;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:223:9
    |
223 |         pub use std::rc::Rc as Lrc;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:224:9
    |
224 |         pub use std::rc::Weak as Weak;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:225:9
    |
225 |         pub use std::cell::Ref as ReadGuard;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:226:9
    |
226 |         pub use std::cell::Ref as MappedReadGuard;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:227:9
    |
227 |         pub use std::cell::RefMut as WriteGuard;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:228:9
    |
228 |         pub use std::cell::RefMut as MappedWriteGuard;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:229:9
    |
229 |         pub use std::cell::RefMut as LockGuard;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:230:9
    |
230 |         pub use std::cell::RefMut as MappedLockGuard;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/sync.rs:232:9
    |
232 |         pub use std::lazy::OnceCell;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/lib.rs:109:1
    |
109 | pub use ena::undo_log;
    | ^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_data_structures/src/lib.rs:110:1
    |
110 | pub use ena::unify;
    | ^^^^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead
warning: 25 warnings emitted
warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_errors/src/lib.rs:264:35
    |
264 | pub use rustc_span::fatal_error::{FatalError, FatalErrorMarker};
    |                                   ^^^^^^^^^^
    |
    = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_errors/src/lib.rs:264:35
    |
264 | pub use rustc_span::fatal_error::{FatalError, FatalErrorMarker};
    |                                   ^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
   --> compiler/rustc_errors/src/lib.rs:264:47
    |
264 | pub use rustc_span::fatal_error::{FatalError, FatalErrorMarker};
    |                                               ^^^^^^^^^^^^^^^^
    |
    = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_session/src/session.rs:10:1
   |
10 | pub use rustc_ast::attr::MarkedAttrs;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_session/src/session.rs:11:1
   |
11 | pub use rustc_ast::crate_disambiguator::CrateDisambiguator;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_session/src/session.rs:12:1
   |
12 | pub use rustc_ast::Attribute;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_session/src/lib.rs:28:1
   |
28 | pub use getopts;
   | ^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_attr/src/lib.rs:19:1
   |
19 | pub use rustc_ast::attr::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_hir/src/definitions.rs:7:1
  |
7 | pub use crate::def_id::DefPathHash;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_hir/src/definitions.rs:7:9
  |
7 | pub use crate::def_id::DefPathHash;
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_hir/src/lib.rs:23:1
   |
23 | pub use rustc_span::def_id;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_hir/src/hir.rs:10:21
   |
10 | pub use rustc_ast::{BorrowKind, ImplPolarity, IsAuto};
   |                     ^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_hir/src/hir.rs:10:33
   |
10 | pub use rustc_ast::{BorrowKind, ImplPolarity, IsAuto};
   |                                 ^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_hir/src/hir.rs:10:47
   |
10 | pub use rustc_ast::{BorrowKind, ImplPolarity, IsAuto};
   |                                               ^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_hir/src/hir.rs:11:21
   |
11 | pub use rustc_ast::{CaptureBy, Movability, Mutability};
   |                     ^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_hir/src/hir.rs:11:32
   |
11 | pub use rustc_ast::{CaptureBy, Movability, Mutability};
   |                                ^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_hir/src/hir.rs:11:44
   |
11 | pub use rustc_ast::{CaptureBy, Movability, Mutability};
   |                                            ^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/dep_node.rs:71:41
   |
71 | pub use rustc_query_system::dep_graph::{DepContext, DepNodeParams};
   |                                         ^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/dep_node.rs:71:53
   |
71 | pub use rustc_query_system::dep_graph::{DepContext, DepNodeParams};
   |                                                     ^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/mod.rs:14:5
   |
14 |     debug, hash_result, DepContext, DepNodeColor, DepNodeIndex, SerializedDepNodeIndex,
   |     ^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/mod.rs:14:12
   |
14 |     debug, hash_result, DepContext, DepNodeColor, DepNodeIndex, SerializedDepNodeIndex,
   |            ^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/mod.rs:14:25
   |
14 |     debug, hash_result, DepContext, DepNodeColor, DepNodeIndex, SerializedDepNodeIndex,
   |                         ^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/mod.rs:14:37
   |
14 |     debug, hash_result, DepContext, DepNodeColor, DepNodeIndex, SerializedDepNodeIndex,
   |                                     ^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/mod.rs:14:51
   |
14 |     debug, hash_result, DepContext, DepNodeColor, DepNodeIndex, SerializedDepNodeIndex,
   |                                                   ^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/mod.rs:14:65
   |
14 |     debug, hash_result, DepContext, DepNodeColor, DepNodeIndex, SerializedDepNodeIndex,
   |                                                                 ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/mod.rs:15:5
   |
15 |     WorkProduct, WorkProductId,
   |     ^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/dep_graph/mod.rs:15:18
   |
15 |     WorkProduct, WorkProductId,
   |                  ^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/mir/mod.rs:23:1
   |
23 | pub use rustc_ast::Mutability;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_middle/src/mir/terminator/mod.rs:9:1
  |
9 | pub use rustc_ast::Mutability;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/ty/query/mod.rs:75:37
   |
75 | pub use rustc_query_system::query::{QueryInfo, QueryJob, QueryJobId};
   |                                     ^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/ty/query/mod.rs:75:48
   |
75 | pub use rustc_query_system::query::{QueryInfo, QueryJob, QueryJobId};
   |                                                ^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/ty/query/mod.rs:75:58
   |
75 | pub use rustc_query_system::query::{QueryInfo, QueryJob, QueryJobId};
   |                                                          ^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_middle/src/ty/query/mod.rs:84:1
   |
84 | pub use rustc_query_system::query::QueryConfig;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/infer/mod.rs:30:1
   |
30 | pub use rustc_middle::ty::IntVarValue;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/infer/canonical/mod.rs:32:1
   |
32 | pub use rustc_middle::infer::canonical::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/infer/region_constraints/mod.rs:30:1
   |
30 | pub use rustc_middle::infer::MemberConstraint;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/infer/mod.rs:73:1
   |
73 | pub use rustc_middle::infer::unify_key;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/traits/project.rs:13:1
   |
13 | pub use rustc_middle::traits::Reveal;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/traits/mod.rs:17:1
   |
17 | pub use self::ImplSource::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/traits/mod.rs:18:1
   |
18 | pub use self::ObligationCauseCode::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/traits/mod.rs:19:1
   |
19 | pub use self::SelectionError::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/traits/mod.rs:26:29
   |
26 |     ProjectionCacheStorage, Reveal,
   |                             ^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_infer/src/traits/mod.rs:28:1
   |
28 | pub use rustc_middle::traits::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_codegen_ssa/src/traits/backend.rs:20:1
   |
20 | pub use rustc_data_structures::sync::MetadataRef;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_resolve/src/lib.rs:18:26
   |
18 | pub use rustc_hir::def::{Namespace, PerNS};
   |                          ^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_resolve/src/lib.rs:18:37
   |
18 | pub use rustc_hir::def::{Namespace, PerNS};
   |                                     ^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: 1 warning emitted

warning: 2 warnings emitted

warning: 10 warnings emitted

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_codegen_llvm/src/abi.rs:13:36
   |
13 | pub use rustc_middle::ty::layout::{FAT_PTR_ADDR, FAT_PTR_EXTRA};
   |                                    ^^^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_codegen_llvm/src/abi.rs:13:50
   |
13 | pub use rustc_middle::ty::layout::{FAT_PTR_ADDR, FAT_PTR_EXTRA};
   |                                                  ^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_codegen_llvm/src/abi.rs:16:1
   |
16 | pub use rustc_target::abi::call::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_codegen_llvm/src/abi.rs:18:1
   |
18 | pub use rustc_target::spec::abi::Abi;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_codegen_llvm/src/attributes.rs:21:22
   |
21 | pub use rustc_attr::{InlineAttr, OptimizeAttr};
   |                      ^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_codegen_llvm/src/attributes.rs:21:34
   |
21 | pub use rustc_attr::{InlineAttr, OptimizeAttr};
   |                                  ^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_codegen_llvm/src/metadata.rs:16:1
   |
16 | pub use rustc_data_structures::sync::MetadataRef;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_codegen_llvm/src/mono_item.rs:9:1
  |
9 | pub use rustc_middle::mir::mono::MonoItem;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/infer.rs:16:1
   |
16 | pub use rustc_infer::infer::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:37:1
   |
37 | pub use rustc_infer::traits::error_reporting::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/object_safety.rs:29:25
   |
29 | pub use crate::traits::{MethodViolationCode, ObjectSafetyViolation};
   |                         ^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/object_safety.rs:29:46
   |
29 | pub use crate::traits::{MethodViolationCode, ObjectSafetyViolation};
   |                                              ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/project.rs:34:1
   |
34 | pub use rustc_middle::traits::Reveal;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs:8:39
  |
8 | pub use rustc_middle::traits::query::{DropckOutlivesResult, DtorckConstraint};
  |                                       ^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs:8:61
  |
8 | pub use rustc_middle::traits::query::{DropckOutlivesResult, DtorckConstraint};
  |                                                             ^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs:2:5
  |
2 |     CandidateStep, MethodAutoderefBadTy, MethodAutoderefStepsResult,
  |     ^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs:2:20
  |
2 |     CandidateStep, MethodAutoderefBadTy, MethodAutoderefStepsResult,
  |                    ^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs:2:42
  |
2 |     CandidateStep, MethodAutoderefBadTy, MethodAutoderefStepsResult,
  |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/query/normalize.rs:19:1
   |
19 | pub use rustc_middle::traits::query::NormalizationResult;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/query/outlives_bounds.rs:10:1
   |
10 | pub use rustc_middle::traits::query::OutlivesBound;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/type_op/ascribe_user_type.rs:5:1
  |
5 | pub use rustc_middle::traits::query::type_op::AscribeUserType;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/type_op/eq.rs:5:1
  |
5 | pub use rustc_middle::traits::query::type_op::Eq;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/type_op/normalize.rs:7:1
  |
7 | pub use rustc_middle::traits::query::type_op::Normalize;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/type_op/prove_predicate.rs:5:1
  |
5 | pub use rustc_middle::traits::query::type_op::ProvePredicate;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/query/type_op/subtype.rs:5:1
  |
5 | pub use rustc_middle::traits::query::type_op::Subtype;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/query/type_op/mod.rs:22:1
   |
22 | pub use rustc_middle::traits::query::type_op::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/query/mod.rs:15:1
   |
15 | pub use rustc_middle::traits::query::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/select/mod.rs:50:1
   |
50 | pub use rustc_middle::traits::select::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs:9:1
  |
9 | pub use rustc_middle::traits::specialization_graph::*;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/util.rs:12:1
   |
12 | pub use rustc_infer::traits::util::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:40:1
   |
40 | pub use self::FulfillmentErrorCode::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:41:1
   |
41 | pub use self::ImplSource::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:42:1
   |
42 | pub use self::ObligationCauseCode::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:43:1
   |
43 | pub use self::SelectionError::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:51:1
   |
51 | pub use self::object_safety::MethodViolationCode;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:52:1
   |
52 | pub use self::object_safety::ObjectSafetyViolation;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:55:24
   |
55 | pub use self::select::{EvaluationCache, SelectionCache, SelectionContext};
   |                        ^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:55:41
   |
55 | pub use self::select::{EvaluationCache, SelectionCache, SelectionContext};
   |                                         ^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:56:24
   |
56 | pub use self::select::{EvaluationResult, IntercrateAmbiguityCause, OverflowError};
   |                        ^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:56:68
   |
56 | pub use self::select::{EvaluationResult, IntercrateAmbiguityCause, OverflowError};
   |                                                                    ^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:62:22
   |
62 | pub use self::util::{elaborate_predicates, elaborate_trait_ref, elaborate_trait_refs};
   |                      ^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:62:44
   |
62 | pub use self::util::{elaborate_predicates, elaborate_trait_ref, elaborate_trait_refs};
   |                                            ^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:62:65
   |
62 | pub use self::util::{elaborate_predicates, elaborate_trait_ref, elaborate_trait_refs};
   |                                                                 ^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:68:25
   |
68 |     supertrait_def_ids, supertraits, transitive_bounds, SupertraitDefIds, Supertraits,
   |                         ^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:68:38
   |
68 |     supertrait_def_ids, supertraits, transitive_bounds, SupertraitDefIds, Supertraits,
   |                                      ^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:68:75
   |
68 |     supertrait_def_ids, supertraits, transitive_bounds, SupertraitDefIds, Supertraits,
   |                                                                           ^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_trait_selection/src/traits/mod.rs:73:1
   |
73 | pub use rustc_infer::traits::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_lint/src/builtin.rs:62:1
   |
62 | pub use rustc_session::lint::builtin::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_lint/src/lib.rs:87:38
   |
87 | pub use rustc_session::lint::Level::{self, *};
   |                                      ^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_lint/src/lib.rs:87:44
   |
87 | pub use rustc_session::lint::Level::{self, *};
   |                                            ^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_lint/src/lib.rs:88:31
   |
88 | pub use rustc_session::lint::{BufferedEarlyLint, FutureIncompatibleInfo, Lint, LintId};
   |                               ^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_lint/src/lib.rs:88:50
   |
88 | pub use rustc_session::lint::{BufferedEarlyLint, FutureIncompatibleInfo, Lint, LintId};
   |                                                  ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_lint/src/lib.rs:88:74
   |
88 | pub use rustc_session::lint::{BufferedEarlyLint, FutureIncompatibleInfo, Lint, LintId};
   |                                                                          ^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_lint/src/lib.rs:88:80
   |
88 | pub use rustc_session::lint::{BufferedEarlyLint, FutureIncompatibleInfo, Lint, LintId};
   |                                                                                ^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_lint/src/lib.rs:89:31
   |
89 | pub use rustc_session::lint::{LintArray, LintPass};
   |                               ^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_lint/src/lib.rs:89:42
   |
89 | pub use rustc_session::lint::{LintArray, LintPass};
   |                                          ^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_typeck/src/expr_use_visitor.rs:8:36
  |
8 | pub use rustc_middle::hir::place::{PlaceBase, PlaceWithHirId, Projection};
  |                                    ^^^^^^^^^
  |
  = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_typeck/src/expr_use_visitor.rs:8:47
  |
8 | pub use rustc_middle::hir::place::{PlaceBase, PlaceWithHirId, Projection};
  |                                               ^^^^^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
 --> compiler/rustc_typeck/src/expr_use_visitor.rs:8:63
  |
8 | pub use rustc_middle::hir::place::{PlaceBase, PlaceWithHirId, Projection};
  |                                                               ^^^^^^^^^^
  |
  = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_mir/src/interpret/mod.rs:19:1
   |
19 | pub use rustc_middle::mir::interpret::*; // have all the `interpret` symbols in one place: here
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_driver/src/pretty.rs:22:1
   |
22 | pub use self::PpMode::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-W rustc::pub-cross-crate-reexport` implied by `-W rustc::internal`
   = note: facade crates are discouraged; import from the original crate instead

warning: publicly re-exporting an item from a different crate
  --> compiler/rustc_driver/src/pretty.rs:23:1
   |
23 | pub use self::PpSourceMode::*;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: facade crates are discouraged; import from the original crate instead
