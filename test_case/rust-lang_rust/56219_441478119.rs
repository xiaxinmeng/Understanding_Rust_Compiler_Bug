
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds  
    --> src/librustc_typeck/check/mod.rs:2788:12
     |
2788 |         -> impl Iterator<Item=ty::PolyTraitRef<'tcx>> + 'b
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
note: hidden type `std::iter::FilterMap<std::vec::IntoIter<rustc::traits::Obligation<'tcx, rustc::ty::Predicate<'tcx>>>, check::ObligationMapper<'b, 'gcx, 'tcx>>` captures the lifetime 'gcx as defined on the impl at 2786:10

