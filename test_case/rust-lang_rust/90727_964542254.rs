
notriddle:rust$ rg full_generic
src/librustdoc/clean/types.rs
1520:    crate fn is_full_generic(&self) -> bool {

src/librustdoc/html/render/cache.rs
259:        let is_full_generic = ty.is_full_generic();
261:        if is_full_generic && generics.len() == 1 {
305:        if is_full_generic {
