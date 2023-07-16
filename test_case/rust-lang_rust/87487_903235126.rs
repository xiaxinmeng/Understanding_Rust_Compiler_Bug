plain
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: this file contains an unclosed delimiter
    --> compiler/rustc_privacy/src/lib.rs:2088:3
     |
579  | impl Visitor<'tcx> for EmbargoVisitor<'tcx> {
...
...
590  |         let inherited_item_level = match item.kind {
     |                                                    - this delimiter might not be properly closed...
751  |     }
751  |     }
     |     - ...as it matches this but it has different indentation
2088 | }
     |   ^


error: expected one of `!`, `(`, `+`, `::`, `<`, `>`, or `as`, found keyword `let`
   --> compiler/rustc_privacy/src/lib.rs:590:9
    |
589 | <<<<<<< HEAD
    |             - expected one of 7 possible tokens
590 |         let inherited_item_level = match item.kind {


error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_privacy/src/lib.rs:808:1
    |
808 | impl ReachEverythingInTheInterfaceVisitor<'_, 'tcx> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_privacy/src/lib.rs:847:1
    |
847 | impl DefIdVisitor<'tcx> for ReachEverythingInTheInterfaceVisitor<'_, 'tcx> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
   --> compiler/rustc_privacy/src/lib.rs:875:1
    |
875 | struct NamePrivacyVisitor<'tcx> {
    |
    |
    = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_privacy/src/lib.rs:881:1
    |
881 | impl<'tcx> NamePrivacyVisitor<'tcx> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_privacy/src/lib.rs:930:1
    |
930 | impl<'tcx> Visitor<'tcx> for NamePrivacyVisitor<'tcx> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1011:1
     |
1011 | struct TypePrivacyVisitor<'tcx> {
     |
     |
     = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1018:1
     |
1018 | impl<'tcx> TypePrivacyVisitor<'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1059:1
     |
1059 | impl<'tcx> Visitor<'tcx> for TypePrivacyVisitor<'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1277:1
     |
1277 | impl DefIdVisitor<'tcx> for TypePrivacyVisitor<'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1302:1
     |
1302 | struct ObsoleteVisiblePrivateTypesVisitor<'a, 'tcx> {
     |
     |
     = help: consider moving the struct out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1310:1
     |
1310 | struct ObsoleteCheckTypeForPrivatenessVisitor<'a, 'b, 'tcx> {
     |
     |
     = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1321:1
     |
1321 | impl<'a, 'tcx> ObsoleteVisiblePrivateTypesVisitor<'a, 'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1361:1
     |
1361 | impl<'a, 'b, 'tcx, 'v> Visitor<'v> for ObsoleteCheckTypeForPrivatenessVisitor<'a, 'b, 'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1397:1
     |
1397 | impl<'a, 'tcx> Visitor<'tcx> for ObsoleteVisiblePrivateTypesVisitor<'a, 'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1651:1
     |
1651 | struct SearchInterfaceForPrivateItemsVisitor<'tcx> {
     |
     |
     = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1661:1
     |
1661 | impl SearchInterfaceForPrivateItemsVisitor<'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1783:1
     |
1783 | impl DefIdVisitor<'tcx> for SearchInterfaceForPrivateItemsVisitor<'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1801:1
     |
1801 | struct PrivateItemsInPublicInterfacesVisitor<'tcx> {
     |
     |
     = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1807:1
     |
1807 | impl<'tcx> PrivateItemsInPublicInterfacesVisitor<'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    --> compiler/rustc_privacy/src/lib.rs:1844:1
     |
1844 | impl<'tcx> Visitor<'tcx> for PrivateItemsInPublicInterfacesVisitor<'tcx> {
     |
     |
     = help: consider moving the implementation out to a nearby module scope

error[E0449]: unnecessary visibility qualifier
    --> compiler/rustc_privacy/src/lib.rs:1946:1
1946 | pub fn provide(providers: &mut Providers) {
1946 | pub fn provide(providers: &mut Providers) {
     | ^^^ `pub` not permitted here because it's implied

error[E0407]: method `provide` is not a member of trait `Visitor`
    --> compiler/rustc_privacy/src/lib.rs:1946:1
     |
1946 | / pub fn provide(providers: &mut Providers) {
1947 | |     *providers = Providers {
1948 | |         visibility,
1949 | |         privacy_access_levels,
1953 | |     };
1954 | | }
     | |_^ not a member of trait `Visitor`


error[E0407]: method `visibility` is not a member of trait `Visitor`
    --> compiler/rustc_privacy/src/lib.rs:1956:1
     |
1956 | / fn visibility(tcx: TyCtxt<'_>, def_id: DefId) -> ty::Visibility {
1957 | |     let def_id = def_id.expect_local();
1958 | |     match tcx.resolutions(()).visibilities.get(&def_id) {
1959 | |         Some(vis) => *vis,
2002 | |     }
2003 | | }
     | |_^ not a member of trait `Visitor`


error[E0407]: method `check_mod_privacy` is not a member of trait `Visitor`
    --> compiler/rustc_privacy/src/lib.rs:2005:1
     |
2005 | / fn check_mod_privacy(tcx: TyCtxt<'_>, module_def_id: LocalDefId) {
2006 | |     // Check privacy of names not checked in previous compilation stages.
2007 | |     let mut visitor =
2008 | |         NamePrivacyVisitor { tcx, maybe_typeck_results: None, current_item: module_def_id };
...    |
2017 | |     intravisit::walk_mod(&mut visitor, module, hir_id);
     | |_^ not a member of trait `Visitor`


error[E0407]: method `privacy_access_levels` is not a member of trait `Visitor`
    --> compiler/rustc_privacy/src/lib.rs:2020:1
     |
2020 | / fn privacy_access_levels(tcx: TyCtxt<'_>, (): ()) -> &AccessLevels {
2021 | |     // Build up a set of all exported items in the AST. This is a set of all
2022 | |     // items which are reachable from external crates based on visibility.
2023 | |     let mut visitor = EmbargoVisitor {
...    |
2040 | |     tcx.arena.alloc(visitor.access_levels)
     | |_^ not a member of trait `Visitor`


error[E0407]: method `check_private_in_public` is not a member of trait `Visitor`
    --> compiler/rustc_privacy/src/lib.rs:2043:1
     |
2043 | / fn check_private_in_public(tcx: TyCtxt<'_>, (): ()) {
2044 | |     let access_levels = tcx.privacy_access_levels(());
2045 | |
2046 | |     let krate = tcx.hir().krate();
...    |
2087 | |     krate.visit_all_item_likes(&mut DeepVisitor::new(&mut visitor));
     | |_^ not a member of trait `Visitor`


error[E0425]: cannot find value `visibility` in this scope
    --> compiler/rustc_privacy/src/lib.rs:1948:9
1948 |         visibility,
     |         ^^^^^^^^^^ not found in this scope
     |
help: consider importing one of these items
help: consider importing one of these items
     |
9    | use rustc_middle::dep_graph::DepKind::visibility;
     |
9    | use rustc_middle::dep_graph::label_strs::visibility;


error[E0425]: cannot find value `privacy_access_levels` in this scope
    --> compiler/rustc_privacy/src/lib.rs:1949:9
     |
1949 |         privacy_access_levels,
     |
help: consider importing one of these items
     |
     |
9    | use rustc_middle::dep_graph::DepKind::privacy_access_levels;
     |
9    | use rustc_middle::dep_graph::label_strs::privacy_access_levels;


error[E0425]: cannot find value `check_private_in_public` in this scope
    --> compiler/rustc_privacy/src/lib.rs:1950:9
1950 |         check_private_in_public,
     |         ^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
     |
help: consider importing one of these items
help: consider importing one of these items
     |
9    | use rustc_middle::dep_graph::DepKind::check_private_in_public;
     |
9    | use rustc_middle::dep_graph::label_strs::check_private_in_public;


error[E0425]: cannot find value `check_mod_privacy` in this scope
    --> compiler/rustc_privacy/src/lib.rs:1951:9
1951 |         check_mod_privacy,
     |         ^^^^^^^^^^^^^^^^^ not found in this scope
     |
help: consider importing one of these items
help: consider importing one of these items
     |
9    | use rustc_middle::dep_graph::DepKind::check_mod_privacy;
     |
9    | use rustc_middle::dep_graph::label_strs::check_mod_privacy;


error[E0422]: cannot find struct, variant or union type `NamePrivacyVisitor` in this scope
    --> compiler/rustc_privacy/src/lib.rs:2008:9
     |
2008 |         NamePrivacyVisitor { tcx, maybe_typeck_results: None, current_item: module_def_id };


error[E0422]: cannot find struct, variant or union type `TypePrivacyVisitor` in this scope
    --> compiler/rustc_privacy/src/lib.rs:2016:9
     |
2016 |         TypePrivacyVisitor { tcx, maybe_typeck_results: None, current_item: module_def_id, span };


error[E0422]: cannot find struct, variant or union type `ObsoleteVisiblePrivateTypesVisitor` in this scope
    --> compiler/rustc_privacy/src/lib.rs:2048:23
     |
2048 |     let mut visitor = ObsoleteVisiblePrivateTypesVisitor {


error[E0422]: cannot find struct, variant or union type `PrivateItemsInPublicInterfacesVisitor` in this scope
    --> compiler/rustc_privacy/src/lib.rs:2077:23
     |
2077 |     let mut visitor = PrivateItemsInPublicInterfacesVisitor {

error: unused import: `rustc_attr as attr`
 --> compiler/rustc_privacy/src/lib.rs:9:5
  |
  |
9 | use rustc_attr as attr;
  |     ^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_errors::struct_span_err`
  --> compiler/rustc_privacy/src/lib.rs:11:5
   |
11 | use rustc_errors::struct_span_err;
11 | use rustc_errors::struct_span_err;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `LocalDefIdSet`
  --> compiler/rustc_privacy/src/lib.rs:14:44
   |
14 | use rustc_hir::def_id::{DefId, LocalDefId, LocalDefIdSet};


error: unused imports: `CRATE_DEF_ID`, `CRATE_DEF_INDEX`, `LOCAL_CRATE`
  --> compiler/rustc_privacy/src/lib.rs:15:25
   |
15 | use rustc_hir::def_id::{CRATE_DEF_ID, CRATE_DEF_INDEX, LOCAL_CRATE};


error: unused imports: `AssocItemKind`, `PatKind`
  --> compiler/rustc_privacy/src/lib.rs:17:17
   |
17 | use rustc_hir::{AssocItemKind, HirIdSet, Node, PatKind};
   |                 ^^^^^^^^^^^^^                  ^^^^^^^

error: unused import: `GenericParamDefKind`
  --> compiler/rustc_privacy/src/lib.rs:26:37
   |
26 | use rustc_middle::ty::{self, Const, GenericParamDefKind, TraitRef, Ty, TyCtxt, TypeFoldable};

error: unused import: `rustc_session::lint`
  --> compiler/rustc_privacy/src/lib.rs:27:5
   |
   |
27 | use rustc_session::lint;
   |     ^^^^^^^^^^^^^^^^^^^

error: unused import: `rustc_span::hygiene::Transparency`
  --> compiler/rustc_privacy/src/lib.rs:28:5
   |
28 | use rustc_span::hygiene::Transparency;


error: unused imports: `Ident`, `kw`
  --> compiler/rustc_privacy/src/lib.rs:29:26
   |
29 | use rustc_span::symbol::{kw, Ident};
   |                          ^^  ^^^^^
error: unused import: `rustc_span::Span`
  --> compiler/rustc_privacy/src/lib.rs:30:5
   |
30 | use rustc_span::Span;
30 | use rustc_span::Span;
   |     ^^^^^^^^^^^^^^^^

error: unused import: `mem`
  --> compiler/rustc_privacy/src/lib.rs:35:21
   |
35 | use std::{cmp, fmt, mem};


error[E0599]: no method named `ty` found for struct `ReachEverythingInTheInterfaceVisitor` in the current scope
   --> compiler/rustc_privacy/src/lib.rs:542:59
    |
425 | struct ReachEverythingInTheInterfaceVisitor<'a, 'tcx> {
    | ----------------------------------------------------- method `ty` not found for this
...
542 |                                 self.reach(def_id, level).ty();
    |                                                           ^^ method not found in `ReachEverythingInTheInterfaceVisitor<'_, '_>`
Some errors have detailed explanations: E0407, E0422, E0425, E0449, E0599.
For more information about an error, try `rustc --explain E0407`.
error: could not compile `rustc_privacy` due to 48 previous errors
warning: build failed, waiting for other jobs to finish...
