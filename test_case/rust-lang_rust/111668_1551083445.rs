plain
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0252]: the name `OnceCell` is defined multiple times
  --> compiler/rustc_interface/src/queries.rs:25:34
   |
10 | use rustc_data_structures::sync::{AppendOnlyIndexVec, Lrc, OnceCell, RwLock, WorkerLocal};
   |                                                            -------- previous import of the type `OnceCell` here
...
25 | use std::cell::{RefCell, RefMut, OnceCell};
   |                                  ^^^^^^^^ `OnceCell` reimported here
   |
   = note: `OnceCell` must be defined only once in the type namespace of this module
error: unused import: `OnceCell`
  --> compiler/rustc_interface/src/queries.rs:25:34
   |
   |
25 | use std::cell::{RefCell, RefMut, OnceCell};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error[E0308]: mismatched types
  --> compiler/rustc_interface/src/queries.rs:48:9
   |
   |
47 |       fn compute<F: FnOnce() -> Result<T>>(&self, f: F) -> Result<QueryResult<'_, T>> {
   |                                                            -------------------------- expected `std::result::Result<QueryResult<'_, T>, rustc_errors::ErrorGuaranteed>` because of return type
48 | /         self.result.get_or_init(|| f().map(|t| RefCell::new(Steal::new(t)))).as_ref()
49 | |             .map(|rst| QueryResult(rst.borrow_mut()))
   | |_____________________________________________________^ expected `Result<QueryResult<'_, T>, ...>`, found `Result<QueryResult<'_, T>, &...>`
   |
   = note: expected enum `std::result::Result<QueryResult<'_, _>, rustc_errors::ErrorGuaranteed>`
              found enum `std::result::Result<QueryResult<'_, _>, &rustc_errors::ErrorGuaranteed>`

error[E0599]: no method named `borrow_mut` found for struct `OnceCell` in the current scope
    |
    |
401 |         if let Some(Ok(gcx)) = &mut *queries.gcx.result.borrow_mut() {
    |                                                         ^^^^^^^^^^ method not found in `OnceCell<Result<RefCell<Steal<&GlobalCtxt<'_>>>, ErrorGuaranteed>>`
   ::: /checkout/library/core/src/borrow.rs:204:8
    |
204 |     fn borrow_mut(&mut self) -> &mut Borrowed;
204 |     fn borrow_mut(&mut self) -> &mut Borrowed;
    |        ---------- the method is available for `OnceCell<std::result::Result<RefCell<Steal<&GlobalCtxt<'_>>>, rustc_errors::ErrorGuaranteed>>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
1   + use std::borrow::BorrowMut;
