plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0599]: no method named `push` found for struct `HashMap` in the current scope
   --> compiler/rustc_hir/src/definitions.rs:359:36
    |
359 |         let _root = def_id_to_span.push(crate_span);
    |                                    ^^^^ method not found in `HashMap<_, _, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `push` found for struct `HashMap` in the current scope
   --> compiler/rustc_hir/src/definitions.rs:411:39
    |
411 |         let _id = self.def_id_to_span.push(span);
    |                                       ^^^^ method not found in `HashMap<LocalDefId, rustc_span::Span, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `iter_enumerated` found for struct `HashMap` in the current scope
   --> compiler/rustc_hir/src/definitions.rs:430:14
430 |             .iter_enumerated()
430 |             .iter_enumerated()
    |              ^^^^^^^^^^^^^^^ method not found in `HashMap<LocalDefId, Option<hir_id::HirId>, BuildHasherDefault<FxHasher>>`
error[E0308]: mismatched types
   --> compiler/rustc_hir/src/definitions.rs:444:29
    |
    |
444 |         self.def_id_to_span[def_id]
    |                             |
    |                             |
    |                             expected `&LocalDefId`, found struct `LocalDefId`
    |                             help: consider borrowing here: `&def_id`

error[E0277]: the trait bound `LocalDefId: Idx` is not satisfied
  --> compiler/rustc_hir/src/hir_id.rs:37:39
   |
37 |         (rustc_index::vec::Idx::index(self.owner), rustc_index::vec::Idx::index(self.local_id))
   |          ---------------------------- ^^^^^^^^^^ the trait `Idx` is not implemented for `LocalDefId`
   |          required by a bound introduced by this call

Some errors have detailed explanations: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
