
99.50% (286,155,244B) (heap allocation functions) malloc/new/new[], --alloc-fns, etc.
->57.95% (166,652,149B) 0x5352FDF: __rdl_alloc (lib.rs:131)
| ->51.10% (146,960,384B) 0x92C8978: _$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$::allocate_in::h79ee2a1009556b6d (heap.rs:84)
| | ->51.10% (146,960,384B) 0x8DCB78E: _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$::clone::h9c3d151fd71233a3 (raw_vec.rs:141)
| | | ->40.89% (117,600,896B) 0x92607AC: rustc::traits::project::ProjectionCache::insert_ty::h0cf47cfd76f17eb9 (project.rs:340)
| | | | ->40.89% (117,600,896B) 0x925B678: rustc::traits::project::opt_normalize_projection_type::h1383880954193dab (project.rs:530)
| | | |   ->40.89% (117,600,896B) 0x925A277: rustc::traits::project::normalize_projection_type::hf37038bf35e92756 (project.rs:365)
| | | |   | ->40.89% (117,600,896B) 0x9259FF6: _$LT$rustc..traits..project..AssociatedTypeNormalizer$LT$$u27$a$C$$u20$$u27$b$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$u20$as$u20$rustc..ty..fold..TypeFolder$LT$$
| | | |   |   ->30.69% (88,251,264B) 0x923A220: rustc::ty::structural_impls::_$LT$impl$u20$rustc..ty..fold..TypeFoldable$LT$$u27$tcx$GT$$u20$for$u20$$RF$$u27$tcx$u20$rustc..ty..TyS$LT$$u27$tcx$GT$$GT$::fo
| | | |   |   | ->30.69% (88,251,136B) 0x8F41703: core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$$u27$a$u20$mut$u20$F$GT$::call_once::h6574094d91ee375f (str
| | | |   |   | | ->30.69% (88,251,136B) 0x8E9412E: _$LT$rustc_data_structures..array_vec..ArrayVec$LT$A$GT$$u20$as$u20$core..iter..traits..Extend$LT$$LT$A$u20$as$u20$rustc_data_structures..array_vec..Arr
| | | |   |   | |   ->30.69% (88,251,136B) 0x9128645: _$LT$rustc_data_structures..accumulate_vec..AccumulateVec$LT$A$GT$$u20$as$u20$core..iter..traits..FromIterator$LT$$LT$A$u20$as$u20$rustc_data_structur
| | | |   |   | |     ->30.69% (88,251,136B) 0x922A1AD: rustc::ty::fold::TypeFoldable::fold_with::he61f397c4c6fc301 (iterator.rs:1302)
| | | |   |   | |       ->20.48% (58,891,648B) 0x9239AB6: rustc::ty::structural_impls::_$LT$impl$u20$rustc..ty..fold..TypeFoldable$LT$$u27$tcx$GT$$u20$for$u20$$RF$$u27$tcx$u20$rustc..ty..TyS$LT$$u27$tcx$G
| | | |   |   | |       | ->20.48% (58,891,648B) 0x9259E9B: _$LT$rustc..traits..project..AssociatedTypeNormalizer$LT$$u27$a$C$$u20$$u27$b$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$u20$as$u20$rustc..ty..fold..Ty
| | | |   |   | |       |   ->20.48% (58,891,648B) 0x923A220: rustc::ty::structural_impls::_$LT$impl$u20$rustc..ty..fold..TypeFoldable$LT$$u27$tcx$GT$$u20$for$u20$$RF$$u27$tcx$u20$rustc..ty..TyS$LT$$u27$t
| | | |   |   | |       |     ->10.26% (29,508,352B) 0x925B47B: rustc::traits::project::opt_normalize_projection_type::h1383880954193dab (project.rs:266)
| | | |   |   | |       |     | ->10.26% (29,508,352B) 0x925A277: rustc::traits::project::normalize_projection_type::hf37038bf35e92756 (project.rs:365)
| | | |   |   | |       |     | | ->10.26% (29,508,352B) 0x9259FF6: _$LT$rustc..traits..project..AssociatedTypeNormalizer$LT$$u27$a$C$$u20$$u27$b$C$$u20$$u27$gcx$C$$u20$$u27$tcx$GT$$u20$as$u20$rustc..ty..
| | | |   |   | |       |     | |   ->10.26% (29,508,352B) 0x923A220: rustc::ty::structural_impls::_$LT$impl$u20$rustc..ty..fold..TypeFoldable$LT$$u27$tcx$GT$$u20$for$u20$$RF$$u27$tcx$u20$rustc..ty..TyS$L
