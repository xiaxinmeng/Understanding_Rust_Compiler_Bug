plain
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0658]: `let` expressions in this position are unstable
    --> compiler/rustc_passes/src/check_attr.rs:1016:12
     |
1016 |         if let Some(mi) = attr.meta() && let Some(list) = mi.meta_item_list() {
     |
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
    --> compiler/rustc_passes/src/check_attr.rs:1016:42
     |
     |
1016 |         if let Some(mi) = attr.meta() && let Some(list) = mi.meta_item_list() {
     |
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
    --> compiler/rustc_passes/src/check_attr.rs:1240:12
     |
     |
1240 |         if let Some(kind) = node.fn_kind() && let rustc_hir::IsAsync::Async = kind.asyncness() {
     |
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
    --> compiler/rustc_passes/src/check_attr.rs:1240:47
     |
     |
1240 |         if let Some(kind) = node.fn_kind() && let rustc_hir::IsAsync::Async = kind.asyncness() {
     |
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
    --> compiler/rustc_passes/src/check_attr.rs:1339:16
     |
     |
1339 |             && let hir::Node::Item(item) = self.tcx.hir().get(hir_id)
     |
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
    --> compiler/rustc_passes/src/check_attr.rs:1340:16
     |
     |
1340 |             && let Item { kind: ItemKind::ForeignMod { abi, .. }, .. } = item
     |
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
    --> compiler/rustc_passes/src/check_attr.rs:2204:18
     |
     |
2204 |             ) && let Some(meta) = attr.meta_item_list()
     |
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
    --> compiler/rustc_passes/src/check_attr.rs:2206:16
     |
     |
2206 |             && let Some(item) = meta[0].meta_item()
     |
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
    --> compiler/rustc_passes/src/check_attr.rs:2207:16
     |
     |
2207 |             && let MetaItemKind::NameValue(_) = &item.kind
     |
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
     = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/dead.rs:183:12
    |
    |
183 |         if let hir::ExprKind::Assign(lhs, rhs, _) = assign.kind
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/dead.rs:250:16
    |
    |
250 |             if let Some(trait_of) = self.tcx.trait_id_of_impl(impl_of)
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/dead.rs:254:20
    |
    |
254 |                 if let ty::Adt(adt_def, _) = trait_ref.self_ty().kind()
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/dead.rs:255:24
    |
    |
255 |                     && let Some(adt_def_id) = adt_def.did().as_local()
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/dead.rs:270:12
    |
    |
270 |         if let Node::ImplItem(hir::ImplItem { def_id, .. }) = node
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/dead.rs:543:16
    |
    |
543 |             if let hir::ItemKind::Struct(ref variant_data, _) = item.kind
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/dead.rs:544:20
    |
    |
544 |                 && let Some(ctor_hir_id) = variant_data.ctor_hir_id()
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
  --> compiler/rustc_passes/src/entry.rs:60:12
   |
   |
60 |         if let Some(name) = ctxt.tcx.opt_item_name(id.def_id.to_def_id())
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
   = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/entry.rs:142:12
    |
    |
142 |         if let Some(main_def) = tcx.resolutions(()).main_def && let Some(def_id) = main_def.opt_fn_def_id() {
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/entry.rs:142:65
    |
    |
142 |         if let Some(main_def) = tcx.resolutions(()).main_def && let Some(def_id) = main_def.opt_fn_def_id() {
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/entry.rs:144:16
    |
    |
144 |             if let Some(def_id) = def_id.as_local() && matches!(tcx.hir().find_by_def_id(def_id), Some(Node::ForeignItem(_))) {
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/entry.rs:215:8
    |
    |
215 |     if let Some(main_def) = tcx.resolutions(()).main_def && main_def.opt_fn_def_id().is_none(){
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_passes/src/liveness.rs:336:12
    |
    |
336 |         if let DefKind::Impl = self.tcx.def_kind(parent)
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
  --> compiler/rustc_passes/src/reachable.rs:96:12
   |
   |
96 |         if let Some(res) = res && let Some(def_id) = res.opt_def_id().and_then(|el| el.as_local()) {
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
   = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0658]: `let` expressions in this position are unstable
  --> compiler/rustc_passes/src/reachable.rs:96:35
   |
   |
96 |         if let Some(res) = res && let Some(def_id) = res.opt_def_id().and_then(|el| el.as_local()) {
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
   = help: add `#![feature(let_chains)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
[RUSTC-TIMING] rustc_passes test:false 0.863
error: could not compile `rustc_passes` due to 24 previous errors
warning: build failed, waiting for other jobs to finish...
