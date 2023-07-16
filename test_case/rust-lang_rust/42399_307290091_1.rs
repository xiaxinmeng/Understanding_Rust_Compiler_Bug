
rustc 1.19.0-nightly (f062832b2 2017-06-07)
error: internal compiler error: /checkout/src/librustc_typeck/check/mod.rs:578: escaping regions in predicate Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { trait_ref: <_ as std::iter::Iterator>, item_def_id: DefId { krate: CrateNum(2), node: DefIndex(1495) => core/ce488d1::iter[0]::iterator[0]::Iterator[0]::Item[0] } }, (&'b &[T], &'b &[T]))),depth=0)
  --> <anon>:10:32
   |
10 |     pub fn z<'b: 'a>(&self) -> impl Iterator<Item = (&'b &[T], &'b &[T])> {
   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
