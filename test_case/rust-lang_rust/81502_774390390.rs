plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: expected identifier, found `<<`
   --> src/librustdoc/json/conversions.rs:437:1
    |
437 | <<<<<<< HEAD
    | ^^ expected identifier
error[E0432]: unresolved import `crate::json::conversions::from_def_id`
  --> src/librustdoc/json/mod.rs:27:5
   |
27 | use crate::json::conversions::from_def_id;
27 | use crate::json::conversions::from_def_id;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `from_def_id` in `json::conversions`

error[E0277]: the trait bound `rustdoc_json_types::Trait: From<types::Trait>` is not satisfied
   --> src/librustdoc/json/mod.rs:111:63
    |
111 | ...                   inner: types::ItemEnum::TraitItem(trait_item.clone().into()),
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<types::Trait>` is not implemented for `rustdoc_json_types::Trait`
    |
    = note: required because of the requirements on the impl of `Into<rustdoc_json_types::Trait>` for `types::Trait`

error[E0599]: no method named `convert_item` found for mutable reference `&mut JsonRenderer<'tcx>` in the current scope
   --> src/librustdoc/json/mod.rs:160:42
    |
160 |         if let Some(mut new_item) = self.convert_item(item) {
    |                                          ^^^^^^^^^^^^ method not found in `&mut JsonRenderer<'tcx>`

error[E0277]: the trait bound `rustdoc_json_types::ItemKind: From<ItemType>` is not satisfied
   --> src/librustdoc/json/mod.rs:225:91
    |
225 |                         types::ItemSummary { crate_id: k.krate.as_u32(), path, kind: kind.into() },
    |                                                                                           ^^^^ the trait `From<ItemType>` is not implemented for `rustdoc_json_types::ItemKind`
    |
    = note: required because of the requirements on the impl of `Into<rustdoc_json_types::ItemKind>` for `ItemType`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0432, E0599.
For more information about an error, try `rustc --explain E0277`.
