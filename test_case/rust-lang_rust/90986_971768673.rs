plain
    Checking getopts v0.2.21
    Checking unicode-normalization v0.1.13
   Compiling clippy v0.1.58 (/checkout/src/tools/clippy)
    Checking bstr v0.2.13
error[E0432]: unresolved import `rustc_hir::intravisit::NestedVisitorMap`
  --> src/tools/clippy/clippy_utils/src/eager_or_lazy.rs:17:29
   |
17 | use rustc_hir::intravisit::{NestedVisitorMap, Visitor};
   |                             ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`

error[E0432]: unresolved import `rustc_hir::intravisit::NestedVisitorMap`
  |
  |
3 | use rustc_hir::intravisit::{walk_expr, NestedVisitorMap, Visitor};
  |                                        ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`

error[E0432]: unresolved import `rustc_hir::intravisit::NestedVisitorMap`
  |
  |
4 | use rustc_hir::intravisit::{NestedVisitorMap, Visitor};
  |                             ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`

error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
  |
  |
3 | use rustc_hir::intravisit::{self, walk_expr, ErasedMap, NestedVisitorMap, Visitor};
  |                                              ^^^^^^^^^  ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
  |                                              |
  |                                              no `ErasedMap` in `intravisit`

error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
   |
   |
71 | use rustc_hir::intravisit::{self, walk_expr, ErasedMap, FnKind, NestedVisitorMap, Visitor};
   |                                              ^^^^^^^^^          ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
   |                                              |
   |                                              no `ErasedMap` in `intravisit`

error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
    |
    |
109 |         intravisit::NestedVisitorMap::None
    |                     ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
    |
    |
147 |         intravisit::NestedVisitorMap::OnlyBodies(self.cx.tcx.hir())
    |                     ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
   |
   |
19 |             intravisit::NestedVisitorMap::None
   |                         ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
   |
   |
87 |             intravisit::NestedVisitorMap::None
   |                         ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0433]: failed to resolve: could not find `NestedVisitorMap` in `intravisit`
     |
     |
1135 |             hir::intravisit::NestedVisitorMap::None
     |                              ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
108 |     fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                   ^^^^^^^^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
146 |     fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                   ^^^^^^^^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
   |
   |
18 |         fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
   |                                                       ^^^^^^^^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
   |
   |
86 |         fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
   |                                                       ^^^^^^^^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `hir::intravisit`
     |
     |
1134 |         fn nested_visit_map(&mut self) -> hir::intravisit::NestedVisitorMap<Self::Map> {
     |                                                            ^^^^^^^^^^^^^^^^ not found in `hir::intravisit`
    Checking quote v1.0.7
   Compiling libz-sys v1.1.3
    Checking idna v0.2.0
    Checking getrandom v0.2.0
