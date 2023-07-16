plain
    Checking semver v0.11.0
    Checking clippy_utils v0.1.54 (/checkout/src/tools/clippy/clippy_utils)
    Checking url v2.2.2
    Checking toml v0.5.7
error[E0658]: or-patterns syntax is experimental
    |
    |
210 |             kind: ItemKind::Const(..) | ItemKind::Static(..),
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
    |
    |
526 |         Res::Def(DefKind::Trait | DefKind::TraitAlias, trait_id) => Some(trait_id),
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
    |
    |
702 | /             Node::Item(Item { ident, .. })
703 | |             | Node::TraitItem(TraitItem { ident, .. })
704 | |             | Node::ImplItem(ImplItem { ident, .. }),
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
     |
     |
1035 |                 def::Res::Def(DefKind::Variant | DefKind::Ctor(..), ..) => true,
     |
     = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
     = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
    |
    |
330 |             Res::Def(DefKind::Const | DefKind::AssocConst, def_id) => {
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
  --> src/tools/clippy/clippy_utils/src/eager_or_lazy.rs:41:43
   |
41 | ...                   res: Res::Def(DefKind::Ctor(..) | DefKind::Variant, ..),
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:147:18
    |
147 |                 (CastTy::Ptr(_) | CastTy::FnPtr, CastTy::Int(_)) => {
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:153:40
    |
153 |         Rvalue::Cast(CastKind::Pointer(PointerCast::MutToConstPointer | PointerCast::ArrayToPointer), operand, _) => {
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:158:17
    |
158 |                 PointerCast::UnsafeFnPointer | PointerCast::ClosureFnPointer(_) | PointerCast::ReifyFnPointer,
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
    |
    |
510 |             } else if matches!(item.res, Res::Def(DefKind::Enum | DefKind::Struct, _)) {
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
     |
     |
1428 |               kind: StmtKind::Expr(_)
     |  ___________________^
1429 | |                 | StmtKind::Semi(_)
1430 | |                 | StmtKind::Local(Local {
1431 | |                     pat: Pat {
1435 | |                     ..
1436 | |                 }),
     | |__________________^
     |
     |
     = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
     = help: add `#![feature(or_patterns)]` to the crate attributes to enable

error[E0658]: or-patterns syntax is experimental
     |
     |
1527 |             (Level::Forbid | Level::Deny | Level::Warn, _)
     |
     = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
     = help: add `#![feature(or_patterns)]` to the crate attributes to enable

