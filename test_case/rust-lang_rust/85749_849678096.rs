plain

error[E0425]: cannot find value `CRATE_DEF_INDEX` in this scope
   --> src/librustdoc/core.rs:327:55
    |
327 |                         LocalDefId { local_def_index: CRATE_DEF_INDEX }.to_def_id(),
    |
help: consider importing one of these items
    |
1   | use crate::core::source_map::def_id::CRATE_DEF_INDEX;
1   | use crate::core::source_map::def_id::CRATE_DEF_INDEX;
    |
1   | use rustc_hir::def_id::CRATE_DEF_INDEX;
    |
1   | use rustc_span::def_id::CRATE_DEF_INDEX;
    |

error: unused import: `early::IntraLinkCrateLoader`
   |
   |
41 | crate use early::IntraLinkCrateLoader;
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`

error[E0599]: no method named `local_def_id` found for mutable reference `&mut Resolver<'_>` in the current scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:55:71
   |
55 |                 self.resolver.borrow_mut().access(|resolver| resolver.local_def_id(item.id));
   |                                                                       ^^^^^^^^^^^^ method not found in `&mut Resolver<'_>`
   = help: items from traits can only be used if the trait is in scope
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
           `use rustc_ast_lowering::ResolverAstLowering;`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0425, E0432, E0599.
For more information about an error, try `rustc --explain E0425`.
