
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src/main.rs:4:5
  |
3 | fn main() {
  |    ---- this is not `async`
4 |     async { let (); }.await
  |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks

thread 'rustc' panicked at 'src/librustc/hir/map/hir_id_validator.rs:26: 
ItemLocalIds not assigned densely in ::main[0]. Max ItemLocalId = 5, missing IDs = ["[local_id: 1, node:unknown node (hir_id=HirId { owner: DefIndex(12), local_id: 1 })]"]; seens IDs = ["(HirId { owner: DefIndex(12), local_id: 0 } fn main (hir_id=HirId { owner: DefIndex(12), local_id: 0 }))", "(HirId { owner: DefIndex(12), local_id: 5 } unknown node (hir_id=HirId { owner: DefIndex(12), local_id: 5 }))", "(HirId { owner: DefIndex(12), local_id: 2 } expr (/*ERROR*/) (hir_id=HirId { owner: DefIndex(12), local_id: 2 }))", "(HirId { owner: DefIndex(12), local_id: 4 } expr { (/*ERROR*/) } (hir_id=HirId { owner: DefIndex(12), local_id: 4 }))", "(HirId { owner: DefIndex(12), local_id: 3 } block { (/*ERROR*/) } (hir_id=HirId { owner: DefIndex(12), local_id: 3 }))"]', src/librustc/util/bug.rs:37:26
