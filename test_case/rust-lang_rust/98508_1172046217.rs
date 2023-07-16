plain
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: this file contains an unclosed delimiter
   --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:824:3
    |
232 | impl<'tcx> AbstractConst<'tcx> {
...
...
267 |                     && tcx.def_kind(uv.def.did) == DefKind::AnonConst {
    |                                                                       - this delimiter might not be properly closed...
270 |               }
270 |               }
    |               - ...as it matches this but it has different indentation
824 | }
    |   ^


error: expected one of `=` or `|`, found `{`
    |
    |
263 |               if let ty::ConstKind::Unevaluated(uv) {
    |                                                     ^ expected one of `=` or `|`

error: struct is not supported in `trait`s or `impl`s
    |
    |
282 | struct AbstractConstBuilder<'a, 'tcx> {
    |
    |
    = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    |
    |
290 | impl<'a, 'tcx> AbstractConstBuilder<'a, 'tcx> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
    |
    |
713 | struct ConstUnifyCtxt<'tcx> {
    |
    |
    = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
    |
    |
718 | impl<'tcx> ConstUnifyCtxt<'tcx> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope
error[E0433]: failed to resolve: use of undeclared type `AbstractConstBuilder`
   --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:637:9
    |
    |
637 |         AbstractConstBuilder::new(tcx, (&*body.0.borrow(), body.1))?
    |         ^^^^^^^^^^^^^^^^^^^^ use of undeclared type `AbstractConstBuilder`
error[E0433]: failed to resolve: use of undeclared type `AbstractConstBuilder`
   --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:638:18
    |
    |
638 |             .map(AbstractConstBuilder::build)
    |                  ^^^^^^^^^^^^^^^^^^^^ use of undeclared type `AbstractConstBuilder`
error[E0425]: cannot find function `walk_abstract_const` in this scope
  --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:61:13
   |
   |
61 |             walk_abstract_const::<!, _>(tcx, ct, |node| match node.root(tcx) {


error[E0422]: cannot find struct, variant or union type `ConstUnifyCtxt` in this scope
    |
    |
195 |                     let const_unify_ctxt = ConstUnifyCtxt { tcx, param_env };

error[E0425]: cannot find function `walk_abstract_const` in this scope
   --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:200:34
    |
    |
200 |                     let result = walk_abstract_const(tcx, b_ct, |b_ct| {


error[E0422]: cannot find struct, variant or union type `ConstUnifyCtxt` in this scope
    |
    |
654 |                 let const_unify_ctxt = ConstUnifyCtxt { tcx, param_env };

error[E0425]: cannot find function `walk_abstract_const` in module `const_evaluatable`
   --> compiler/rustc_trait_selection/src/traits/object_safety.rs:846:36
    |
    |
846 |                 const_evaluatable::walk_abstract_const(self.tcx, ct, |node| {


error[E0425]: cannot find function `thir_abstract_const` in module `const_evaluatable`
    |
    |
864 |                 const_evaluatable::thir_abstract_const(tcx, ty::WithOptConstParam::unknown(def_id))
    |
help: consider importing one of these items
    |
    |
25  | use rustc_middle::dep_graph::DepKind::thir_abstract_const;
    |
25  | use rustc_middle::dep_graph::label_strs::thir_abstract_const;
    |
help: if you import `thir_abstract_const`, refer to it directly
    |
864 -                 const_evaluatable::thir_abstract_const(tcx, ty::WithOptConstParam::unknown(def_id))
864 +                 thir_abstract_const(tcx, ty::WithOptConstParam::unknown(def_id))


error[E0425]: cannot find function `thir_abstract_const` in module `const_evaluatable`
    |
    |
868 |             const_evaluatable::thir_abstract_const(
    |
help: consider importing one of these items
    |
    |
25  | use rustc_middle::dep_graph::DepKind::thir_abstract_const;
    |
25  | use rustc_middle::dep_graph::label_strs::thir_abstract_const;
    |
help: if you import `thir_abstract_const`, refer to it directly
    |
868 -             const_evaluatable::thir_abstract_const(
868 +             thir_abstract_const(

error[E0425]: cannot find function `try_unify_abstract_consts` in module `const_evaluatable`
   --> compiler/rustc_trait_selection/src/traits/mod.rs:875:32
    |
    |
875 |             const_evaluatable::try_unify_abstract_consts(tcx, (a, b), param_env)
    |
help: consider importing one of these items
    |
25  | use rustc_middle::dep_graph::DepKind::try_unify_abstract_consts;
25  | use rustc_middle::dep_graph::DepKind::try_unify_abstract_consts;
    |
25  | use rustc_middle::dep_graph::label_strs::try_unify_abstract_consts;
    |
help: if you import `try_unify_abstract_consts`, refer to it directly
    |
875 -             const_evaluatable::try_unify_abstract_consts(tcx, (a, b), param_env)
875 +             try_unify_abstract_consts(tcx, (a, b), param_env)

error: unused import: `rustc_index::vec::IndexVec`
  --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:13:5
   |
   |
13 | use rustc_index::vec::IndexVec;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_middle::mir`
  --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:15:5
   |
15 | use rustc_middle::mir;
15 | use rustc_middle::mir;
   |     ^^^^^^^^^^^^^^^^^

error: unused imports: `LitToConstError`, `LitToConstInput`
   |
   |
16 | use rustc_middle::mir::interpret::{ErrorHandled, LitToConstError, LitToConstInput};

error: unused import: `self`
  --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:18:42
   |
   |
18 | use rustc_middle::thir::abstract_const::{self, Node, NodeId, NotConstEvaluatable};

error: unused import: `Subst`
  --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:19:31
   |
   |
19 | use rustc_middle::ty::subst::{Subst, SubstsRef};


error: unused import: `EarlyBinder`
   |
   |
20 | use rustc_middle::ty::{self, DelaySpanBugEmitted, EarlyBinder, TyCtxt, TypeFoldable};

error: unused import: `std::iter`
  --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:26:5
   |
   |
26 | use std::iter;
   |     ^^^^^^^^^

error[E0496]: lifetime name `'tcx` shadows a lifetime name that is already in scope
    |
    |
232 | impl<'tcx> AbstractConst<'tcx> {
...
...
620 | pub(super) fn thir_abstract_const<'tcx>(
    |                                   ^^^^ lifetime `'tcx` already in scope

error[E0496]: lifetime name `'tcx` shadows a lifetime name that is already in scope
    |
    |
232 | impl<'tcx> AbstractConst<'tcx> {
...
646 | pub(super) fn try_unify_abstract_consts<'tcx>(
    |                                         ^^^^ lifetime `'tcx` already in scope


error[E0496]: lifetime name `'tcx` shadows a lifetime name that is already in scope
    |
    |
232 | impl<'tcx> AbstractConst<'tcx> {
...
...
668 | pub fn walk_abstract_const<'tcx, R, F>(
    |                            ^^^^ lifetime `'tcx` already in scope
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
Some errors have detailed explanations: E0422, E0425, E0433, E0496.
For more information about an error, try `rustc --explain E0422`.
error: could not compile `rustc_trait_selection` due to 26 previous errors
