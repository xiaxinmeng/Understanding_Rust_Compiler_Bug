
Error: internal compiler error: src\librustc_typeck\check/mod.rs:1708: escaping regions in predicate Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { trait_ref: <_ as std::iter::Iterator>, item_name: Item(747) }, &foldertree::FolderPath)),depth=0)
  --> src\foldertree.rs:121:48
    |
121 |   fn walk_paths(&self, parent: &FolderPath) -> impl Iterator<Item=&FolderPath> {
    |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

