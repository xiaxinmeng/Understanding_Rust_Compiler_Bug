plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0432]: unresolved import `hir::intravisit::NestedVisitorMap`
 --> compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs:6:24
  |
6 |     intravisit::{self, NestedVisitorMap, Visitor},
  |                        ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`

error[E0412]: cannot find type `ErasedMap` in module `intravisit`
   --> compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs:216:28
    |
216 |     type Map = intravisit::ErasedMap<'tcx>;
    |                            ^^^^^^^^^ not found in `intravisit`

error[E0284]: type annotations needed: cannot satisfy `<DropRangeVisitor<'a, 'tcx> as rustc_hir::intravisit::Visitor<'tcx>>::Map == _`
   --> compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs:216:5
    |
216 |     type Map = intravisit::ErasedMap<'tcx>;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot satisfy `<DropRangeVisitor<'a, 'tcx> as rustc_hir::intravisit::Visitor<'tcx>>::Map == _`

error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
   --> compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs:192:25
    |
192 |             | ExprKind::LlvmInlineAsm(..)
    |                         |
    |                         variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                         help: there is a variant with a similar name: `InlineAsm`


error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
   --> compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs:381:25
    |
381 |             | ExprKind::LlvmInlineAsm(..)
    |                         |
    |                         variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                         help: there is a variant with a similar name: `InlineAsm`

