
error[E0277]: `rustc_span::Span` cannot be shared between threads safely
  --> compiler/rustc_mir/src/transform/coverage/tests.rs:16:5
   |
16 |     static DUMMY_TYS: SyncOnceCell<TyS<'_>> = SyncOnceCell::new();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `rustc_span::Span` cannot be shared between threads safely
   |
   = help: within `VariantDef`, the trait `std::marker::Sync` is not implemented for `rustc_span::Span`
   = note: required because it appears within the type `rustc_span::symbol::Ident`
   = note: required because it appears within the type `VariantDef`
   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<VariantDef>`
   = note: required because it appears within the type `RawVec<VariantDef>`
   = note: required because it appears within the type `Vec<VariantDef>`
   = note: required because it appears within the type `rustc_index::vec::IndexVec<VariantIdx, VariantDef>`
   = note: required because it appears within the type `AdtDef`
   = note: required because of the requirements on the impl of `std::marker::Send` for `&AdtDef`
   = note: required because it appears within the type `rustc_middle::ty::TyKind`
   = note: required because it appears within the type `TyS`
   = note: required because of the requirements on the impl of `std::marker::Sync` for `SyncOnceCell<TyS>`
   = note: shared static variables must have a type that implements `Sync`
