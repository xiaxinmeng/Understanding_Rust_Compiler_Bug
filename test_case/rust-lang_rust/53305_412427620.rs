rust
// MIR for `foo`
// source = MirSource { def_id: DefId(0/0:9 ~ borrowck_suggest_extraction_of_subcomputation[317d]::foo[0]), promoted: None }
// pass_name = SimplifyCfg-initial
// disambiguator = after

fn foo(_1: B) -> (){
    let mut _0: ();                      // return place
    scope 1 {
    }
    scope 2 {
        let _2: u32;                     // "key" in scope 2 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:24:9: 24:12
    }
    let mut _3: &mut [u32];
    let mut _4: &mut B;
    let mut _5: usize;
    let mut _6: usize;
    let mut _7: usize;
    let mut _8: u32;
    let mut _9: usize;
    let mut _10: &B;
    let mut _11: usize;
    let mut _12: bool;

    bb0: {                              
        StorageLive(_2);                 // bb0[0]: scope 0 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:24:9: 24:12
        _2 = const 0u32;                 // bb0[1]: scope 0 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:24:15: 24:19
                                         // ty::Const
                                         // + ty: u32
                                         // + val: Scalar(Bits { size: 4, bits: 0 })
                                         // mir::Constant
                                         // + span: .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:24:15: 24:19
                                         // + ty: u32
                                         // + literal: Const { ty: u32, val: Scalar(Bits { size: 4, bits: 0 }) }
        StorageLive(_3);                 // bb0[2]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:24
        StorageLive(_4);                 // bb0[3]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:12
        _4 = &mut _1;                    // bb0[4]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:12
        _3 = const SliceWrapperMut::slice_mut(move _4) -> [return: bb2, unwind: bb3]; // bb0[5]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:24
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut B) -> &'r mut [u32] {<B as SliceWrapperMut<u32>>::slice_mut}
                                         // + val: Scalar(Bits { size: 0, bits: 0 })
                                         // mir::Constant
                                         // + span: .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:24
                                         // + ty: for<'r> fn(&'r mut B) -> &'r mut [u32] {<B as SliceWrapperMut<u32>>::slice_mut}
                                         // + literal: Const { ty: for<'r> fn(&'r mut B) -> &'r mut [u32] {<B as SliceWrapperMut<u32>>::slice_mut}, val: Scalar(Bits { size: 0, bits: 0 }) }
    }

    bb1: {                               // cleanup
        resume;                          // bb1[0]: scope 0 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:23:1: 27:2
    }

    bb2: {                              
        StorageDead(_4);                 // bb2[0]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:23: 25:24
        StorageLive(_5);                 // bb2[1]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:25: 25:86
        StorageLive(_6);                 // bb2[2]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:25: 25:56
        StorageLive(_7);                 // bb2[3]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:25: 25:39
        StorageLive(_8);                 // bb2[4]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:26: 25:29
        _8 = _2;                         // bb2[5]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:26: 25:29
        _7 = move _8 as usize (Misc);    // bb2[6]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:25: 25:39
        StorageDead(_8);                 // bb2[7]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:38: 25:39
        _6 = const core::num::<impl usize>::wrapping_add(move _7, const 22usize) -> [return: bb4, unwind: bb3]; // bb2[8]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:25: 25:56
                                         // ty::Const
                                         // + ty: fn(usize, usize) -> usize {core::num::<impl usize>::wrapping_add}
                                         // + val: Scalar(Bits { size: 0, bits: 0 })
                                         // mir::Constant
                                         // + span: .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:25: 25:56
                                         // + ty: fn(usize, usize) -> usize {core::num::<impl usize>::wrapping_add}
                                         // + literal: Const { ty: fn(usize, usize) -> usize {core::num::<impl usize>::wrapping_add}, val: Scalar(Bits { size: 0, bits: 0 }) }
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Scalar(Bits { size: 8, bits: 22 })
                                         // mir::Constant
                                         // + span: .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:53: 25:55
                                         // + ty: usize
                                         // + literal: Const { ty: usize, val: Scalar(Bits { size: 8, bits: 22 }) }
    }

    bb3: {                               // cleanup
        EndRegion();                     // bb3[0]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:87
        drop(_1) -> bb1;                 // bb3[1]: scope 0 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:27:1: 27:2
    }

    bb4: {                              
        StorageDead(_7);                 // bb4[0]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:55: 25:56
        StorageLive(_9);                 // bb4[1]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:70: 25:85
        StorageLive(_10);                // bb4[2]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:70: 25:77
        _10 = &_1;                       // bb4[3]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:70: 25:77
        _9 = const Sweep::sweep(move _10) -> [return: bb5, unwind: bb6]; // bb4[4]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:70: 25:85
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r B) -> usize {<B as Sweep>::sweep}
                                         // + val: Scalar(Bits { size: 0, bits: 0 })
                                         // mir::Constant
                                         // + span: .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:70: 25:85
                                         // + ty: for<'r> fn(&'r B) -> usize {<B as Sweep>::sweep}
                                         // + literal: Const { ty: for<'r> fn(&'r B) -> usize {<B as Sweep>::sweep}, val: Scalar(Bits { size: 0, bits: 0 }) }
    }

    bb5: {                              
        EndRegion();                     // bb5[0]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:70: 25:85
        StorageDead(_10);                // bb5[1]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:84: 25:85
        _5 = const core::num::<impl usize>::wrapping_rem(move _6, move _9) -> [return: bb7, unwind: bb3]; // bb5[2]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:25: 25:86
                                         // ty::Const
                                         // + ty: fn(usize, usize) -> usize {core::num::<impl usize>::wrapping_rem}
                                         // + val: Scalar(Bits { size: 0, bits: 0 })
                                         // mir::Constant
                                         // + span: .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:25: 25:86
                                         // + ty: fn(usize, usize) -> usize {core::num::<impl usize>::wrapping_rem}
                                         // + literal: Const { ty: fn(usize, usize) -> usize {core::num::<impl usize>::wrapping_rem}, val: Scalar(Bits { size: 0, bits: 0 }) }
    }

    bb6: {                               // cleanup
        EndRegion();                     // bb6[0]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:70: 25:85
        goto -> bb3;                     // bb6[1]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:70: 25:85
    }

    bb7: {                              
        StorageDead(_9);                 // bb7[0]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:85: 25:86
        StorageDead(_6);                 // bb7[1]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:85: 25:86
        _11 = Len((*_3));                // bb7[2]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:87
        _12 = Lt(_5, _11);               // bb7[3]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:87
        assert(move _12, "index out of bounds: the len is move _11 but the index is _5") -> [success: bb8, unwind: bb3]; // bb7[4]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:87
    }

    bb8: {                              
        EndRegion();                     // bb8[0]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:87
        (*_3)[_5] = const 22u32;         // bb8[1]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:5: 25:92
                                         // ty::Const
                                         // + ty: u32
                                         // + val: Scalar(Bits { size: 4, bits: 22 })
                                         // mir::Constant
                                         // + span: .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:90: 25:92
                                         // + ty: u32
                                         // + literal: Const { ty: u32, val: Scalar(Bits { size: 4, bits: 22 }) }
        StorageDead(_3);                 // bb8[2]: scope 1 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:25:92: 25:93
        _0 = ();                         // bb8[3]: scope 0 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:23:77: 27:2
        StorageDead(_2);                 // bb8[4]: scope 0 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:27:1: 27:2
        drop(_1) -> [return: bb9, unwind: bb1]; // bb8[5]: scope 0 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:27:1: 27:2
    }

    bb9: {                              
        return;                          // bb9[0]: scope 0 at .\src\test\ui\borrowck\borrowck-suggest-extraction-of-subcomputation.rs:27:2: 27:2
    }
}
