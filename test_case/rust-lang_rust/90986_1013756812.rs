plain
   Compiling unicode-normalization v0.1.13
   Compiling clippy v0.1.60 (/checkout/src/tools/clippy)
   Compiling clippy_utils v0.1.60 (/checkout/src/tools/clippy/clippy_utils)
   Compiling unicase v2.6.0
error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
  --> src/tools/clippy/clippy_utils/src/eager_or_lazy.rs:15:40
   |
15 | use rustc_hir::intravisit::{walk_expr, ErasedMap, NestedVisitorMap, Visitor};
   |                                        ^^^^^^^^^  ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
   |                                        |
   |                                        no `ErasedMap` in `intravisit`

error[E0432]: unresolved import `rustc_hir::intravisit::NestedVisitorMap`
  |
  |
4 | use rustc_hir::intravisit::{self, walk_block, walk_expr, NestedVisitorMap, Visitor};
  |                                                          ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`

error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
   |
   |
75 | use rustc_hir::intravisit::{walk_expr, ErasedMap, FnKind, NestedVisitorMap, Visitor};
   |                                        ^^^^^^^^^          ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
   |                                        |
   |                                        no `ErasedMap` in `intravisit`

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
119 |             intravisit::NestedVisitorMap::None
    |                         ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

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

error[E0412]: cannot find type `ErasedMap` in module `intravisit`
   |
   |
43 |         type Map = intravisit::ErasedMap<'tcx>;
   |                                ^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
118 |         fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                       ^^^^^^^^^^^^^^^^ not found in `intravisit`
   Compiling quote v1.0.7
   Compiling idna v0.2.0
Some errors have detailed explanations: E0412, E0432, E0433.
For more information about an error, try `rustc --explain E0412`.
---
   Compiling pest v2.1.3
   Compiling clippy_utils v0.1.60 (/checkout/src/tools/clippy/clippy_utils)
   Compiling vte v0.3.3
   Compiling heck v0.3.1
error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
  --> src/tools/clippy/clippy_utils/src/eager_or_lazy.rs:15:40
   |
15 | use rustc_hir::intravisit::{walk_expr, ErasedMap, NestedVisitorMap, Visitor};
   |                                        ^^^^^^^^^  ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
   |                                        |
   |                                        no `ErasedMap` in `intravisit`

error[E0432]: unresolved import `rustc_hir::intravisit::NestedVisitorMap`
  |
  |
4 | use rustc_hir::intravisit::{self, walk_block, walk_expr, NestedVisitorMap, Visitor};
  |                                                          ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`

error[E0432]: unresolved imports `rustc_hir::intravisit::ErasedMap`, `rustc_hir::intravisit::NestedVisitorMap`
   |
   |
75 | use rustc_hir::intravisit::{walk_expr, ErasedMap, FnKind, NestedVisitorMap, Visitor};
   |                                        ^^^^^^^^^          ^^^^^^^^^^^^^^^^ no `NestedVisitorMap` in `intravisit`
   |                                        |
   |                                        no `ErasedMap` in `intravisit`

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
119 |             intravisit::NestedVisitorMap::None
    |                         ^^^^^^^^^^^^^^^^ could not find `NestedVisitorMap` in `intravisit`

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

error[E0412]: cannot find type `ErasedMap` in module `intravisit`
   |
   |
43 |         type Map = intravisit::ErasedMap<'tcx>;
   |                                ^^^^^^^^^ not found in `intravisit`

error[E0412]: cannot find type `NestedVisitorMap` in module `intravisit`
    |
    |
118 |         fn nested_visit_map(&mut self) -> intravisit::NestedVisitorMap<Self::Map> {
    |                                                       ^^^^^^^^^^^^^^^^ not found in `intravisit`
   Compiling rls v1.41.0 (/checkout/src/tools/rls)
   Compiling idna v0.1.5
Some errors have detailed explanations: E0412, E0432, E0433.
For more information about an error, try `rustc --explain E0412`.
---
Cloning into 'rust-toolstate'...
Build completed successfully in 0:00:01
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.20s
thread 'main' panicked at 'error: no rules matched src/tools/clippy', src/bootstrap/builder.rs:236:17
Build completed unsuccessfully in 0:00:00
