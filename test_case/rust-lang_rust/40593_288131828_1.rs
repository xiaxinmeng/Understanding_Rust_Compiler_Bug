
src/librustc/ty/contents.rs:222
  272b70:       ebf7edab        bl      6e224 <_ZN5rustc2ty6AdtDef8has_dtor17hfb2c4f039e32f892E@plt>
src/librustc/ty/contents.rs:226
  272b74:       e59d1014        ldr     r1, [sp, #20]
_ZN11collections3vec8{{impl}}54deref<core::option::Option<rustc::hir::def_id::DefId>>E():
src/libcollections/vec.rs:1546
  272b78:       e5912404        ldr     r2, [r1, #1028] ; 0x404
_ZN11collections3vec8{{impl}}54index<core::option::Option<rustc::hir::def_id::DefId>>E():
src/libcollections/vec.rs:1426
  272b7c:       e3520032        cmp     r2, #50 ; 0x32
  272b80:       9a0000a1        bls     272e0c <_ZN5rustc2ty8contents48_$LT$impl$u20$rustc..ty..TyS$LT$$u27$tcx$GT$$GT$13type_contents5tc_ty17h8a564cc8cbe4b939E+0x59c>
_ZN5rustc2ty8contents8{{impl}}13type_contents5tc_tyE():
src/librustc/ty/contents.rs:222
  272b84:       e7c56290        bfi     r6, r0, #5, #1
