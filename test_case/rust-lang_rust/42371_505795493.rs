
fn  index_for_loop() -> usize {
    let mut _0: usize;                   // return place in scope 0 at rust.rs:1:28: 1:33
    let mut _1: usize;                   // "sum" in scope 0 at rust.rs:2:9: 2:16
    let mut _3: std::ops::Range<usize>;  // in scope 0 at rust.rs:4:14: 4:24
    let mut _4: std::ops::Range<usize>;  // in scope 0 at rust.rs:4:14: 4:24
    let mut _5: usize;                   // in scope 0 at rust.rs:4:17: 4:24
    let mut _6: &[usize];                // in scope 0 at rust.rs:4:17: 4:18
    let mut _7: &[usize; 4];             // in scope 0 at rust.rs:4:17: 4:18
    let mut _10: std::option::Option<usize>; // in scope 0 at rust.rs:4:14: 4:24
    let mut _11: &mut std::ops::Range<usize>; // in scope 0 at rust.rs:4:14: 4:24
    let mut _12: &mut std::ops::Range<usize>; // in scope 0 at rust.rs:4:14: 4:24
    let mut _13: isize;                  // in scope 0 at rust.rs:4:9: 4:10
    let mut _15: usize;                  // in scope 0 at rust.rs:4:9: 4:10
    let mut _17: usize;                  // in scope 0 at rust.rs:5:16: 5:20
    let mut _18: usize;                  // in scope 0 at rust.rs:5:18: 5:19
    let mut _19: usize;                  // in scope 0 at rust.rs:5:16: 5:20
    let mut _20: bool;                   // in scope 0 at rust.rs:5:16: 5:20
    let mut _21: (usize, bool);          // in scope 0 at rust.rs:5:9: 5:20
    scope 1 {
        let _2: [usize; 4];              // "a" in scope 1 at rust.rs:3:9: 3:10
        scope 2 {
            let mut _8: std::ops::Range<usize>; // "iter" in scope 2 at rust.rs:4:14: 4:24
            scope 3 {
                let mut _9: usize;       // "__next" in scope 3 at rust.rs:4:14: 4:24
                scope 4 {
                    let _14: usize;      // "val" in scope 4 at rust.rs:4:9: 4:10
                    let _16: usize;      // "i" in scope 4 at rust.rs:4:9: 4:10
                    scope 5 {
                    }
                    scope 6 {
                    }
                }
            }
        }
    }

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at rust.rs:2:9: 2:16
        _1 = const 0usize;               // bb0[1]: scope 0 at rust.rs:2:19: 2:20
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Scalar(0x0000000000000000)
                                         // mir::Constant
                                         // + span: rust.rs:2:19: 2:20
                                         // + ty: usize
                                         // + literal: Const { ty: usize, val: Scalar(0x0000000000000000) }
        StorageLive(_2);                 // bb0[2]: scope 1 at rust.rs:3:9: 3:10
        _2 = [const 0usize, const 10usize, const 20usize, const 30usize]; // bb0[3]: scope 1 at rust.rs:3:13: 3:28
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Scalar(0x0000000000000000)
                                         // mir::Constant
                                         // + span: rust.rs:3:14: 3:15
                                         // + ty: usize
                                         // + literal: Const { ty: usize, val: Scalar(0x0000000000000000) }
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Scalar(0x000000000000000a)
                                         // mir::Constant
                                         // + span: rust.rs:3:17: 3:19
                                         // + ty: usize
                                         // + literal: Const { ty: usize, val: Scalar(0x000000000000000a) }
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Scalar(0x0000000000000014)
                                         // mir::Constant
                                         // + span: rust.rs:3:21: 3:23
                                         // + ty: usize
                                         // + literal: Const { ty: usize, val: Scalar(0x0000000000000014) }
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Scalar(0x000000000000001e)
                                         // mir::Constant
                                         // + span: rust.rs:3:25: 3:27
                                         // + ty: usize
                                         // + literal: Const { ty: usize, val: Scalar(0x000000000000001e) }
        StorageLive(_3);                 // bb0[4]: scope 2 at rust.rs:4:14: 4:24
        StorageLive(_4);                 // bb0[5]: scope 2 at rust.rs:4:14: 4:24
        StorageLive(_5);                 // bb0[6]: scope 2 at rust.rs:4:17: 4:24
        StorageLive(_6);                 // bb0[7]: scope 2 at rust.rs:4:17: 4:18
        StorageLive(_7);                 // bb0[8]: scope 2 at rust.rs:4:17: 4:18
        _7 = &_2;                        // bb0[9]: scope 2 at rust.rs:4:17: 4:18
        _6 = move _7 as &[usize] (Pointer(Unsize)); // bb0[10]: scope 2 at rust.rs:4:17: 4:18
        StorageDead(_7);                 // bb0[11]: scope 2 at rust.rs:4:17: 4:18
        _5 = const core::slice::<impl [usize]>::len(move _6) -> bb1; // bb0[12]: scope 2 at rust.rs:4:17: 4:24
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r [usize]) -> usize {core::slice::<impl [usize]>::len}
                                         // + val: Scalar(<ZST>)
                                         // mir::Constant
                                         // + span: rust.rs:4:19: 4:22
                                         // + ty: for<'r> fn(&'r [usize]) -> usize {core::slice::<impl [usize]>::len}
                                         // + literal: Const { ty: for<'r> fn(&'r [usize]) -> usize {core::slice::<impl [usize]>::len}, val: Scalar(<ZST>) }
    }

    bb1: {
        StorageDead(_6);                 // bb1[0]: scope 2 at rust.rs:4:23: 4:24
        (_4.0: usize) = const 0usize;    // bb1[1]: scope 2 at rust.rs:4:14: 4:24
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Scalar(0x0000000000000000)
                                         // mir::Constant
                                         // + span: rust.rs:4:14: 4:15
                                         // + ty: usize
                                         // + literal: Const { ty: usize, val: Scalar(0x0000000000000000) }
        (_4.1: usize) = move _5;         // bb1[2]: scope 2 at rust.rs:4:14: 4:24
        StorageDead(_5);                 // bb1[3]: scope 2 at rust.rs:4:23: 4:24
        _3 = const <std::ops::Range<usize> as std::iter::IntoIterator>::into_iter(move _4) -> bb2; // bb1[4]: scope 2 at rust.rs:4:14: 4:24
                                         // ty::Const
                                         // + ty: fn(std::ops::Range<usize>) -> <std::ops::Range<usize> as std::iter::IntoIterator>::IntoIter {<std::ops::Range<usize> as std::iter::IntoIterator>::into_iter}
                                         // + val: Scalar(<ZST>)
                                         // mir::Constant
                                         // + span: rust.rs:4:14: 4:24
                                         // + ty: fn(std::ops::Range<usize>) -> <std::ops::Range<usize> as std::iter::IntoIterator>::IntoIter {<std::ops::Range<usize> as std::iter::IntoIterator>::into_iter}
                                         // + literal: Const { ty: fn(std::ops::Range<usize>) -> <std::ops::Range<usize> as std::iter::IntoIterator>::IntoIter {<std::ops::Range<usize> as std::iter::IntoIterator>::into_iter}, val: Scalar(<ZST>) }
    }

    bb2: {
        StorageDead(_4);                 // bb2[0]: scope 2 at rust.rs:4:23: 4:24
        StorageLive(_8);                 // bb2[1]: scope 2 at rust.rs:4:14: 4:24
        _8 = move _3;                    // bb2[2]: scope 2 at rust.rs:4:14: 4:24
        goto -> bb3;                     // bb2[3]: scope 3 at rust.rs:4:5: 6:6
    }

    bb3: {
        StorageLive(_9);                 // bb3[0]: scope 3 at rust.rs:4:14: 4:24
        StorageLive(_10);                // bb3[1]: scope 4 at rust.rs:4:14: 4:24
        StorageLive(_11);                // bb3[2]: scope 4 at rust.rs:4:14: 4:24
        StorageLive(_12);                // bb3[3]: scope 4 at rust.rs:4:14: 4:24
        _12 = &mut _8;                   // bb3[4]: scope 4 at rust.rs:4:14: 4:24
        _11 = _12;                       // bb3[5]: scope 4 at rust.rs:4:14: 4:24
        _10 = const <std::ops::Range<usize> as std::iter::Iterator>::next(move _11) -> bb4; // bb3[6]: scope 4 at rust.rs:4:14: 4:24
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut std::ops::Range<usize>) -> std::option::Option<<std::ops::Range<usize> as std::iter::Iterator>::Item> {<std::ops::Range<usize> as std::iter::Iterator>::next}
                                         // + val: Scalar(<ZST>)
                                         // mir::Constant
                                         // + span: rust.rs:4:14: 4:24
                                         // + ty: for<'r> fn(&'r mut std::ops::Range<usize>) -> std::option::Option<<std::ops::Range<usize> as std::iter::Iterator>::Item> {<std::ops::Range<usize> as std::iter::Iterator>::next}
                                         // + literal: Const { ty: for<'r> fn(&'r mut std::ops::Range<usize>) -> std::option::Option<<std::ops::Range<usize> as std::iter::Iterator>::Item> {<std::ops::Range<usize> as std::iter::Iterator>::next}, val: Scalar(<ZST>) }
    }

    bb4: {
        StorageDead(_11);                // bb4[0]: scope 4 at rust.rs:4:23: 4:24
        _13 = discriminant(_10);         // bb4[1]: scope 4 at rust.rs:4:9: 4:10
        switchInt(move _13) -> [0isize: bb5, 1isize: bb7, otherwise: bb6]; // bb4[2]: scope 4 at rust.rs:4:9: 4:10
    }

    bb5: {
        StorageDead(_12);                // bb5[0]: scope 4 at rust.rs:4:23: 4:24
        StorageDead(_10);                // bb5[1]: scope 4 at rust.rs:4:23: 4:24
        StorageDead(_9);                 // bb5[2]: scope 3 at rust.rs:6:5: 6:6
        StorageDead(_8);                 // bb5[3]: scope 2 at rust.rs:6:5: 6:6
        StorageDead(_3);                 // bb5[4]: scope 2 at rust.rs:4:23: 4:24
        _0 = _1;                         // bb5[5]: scope 2 at rust.rs:7:5: 7:8
        StorageDead(_2);                 // bb5[6]: scope 1 at rust.rs:8:1: 8:2
        StorageDead(_1);                 // bb5[7]: scope 0 at rust.rs:8:1: 8:2
        return;                          // bb5[8]: scope 0 at rust.rs:8:2: 8:2
    }

    bb6: {
        unreachable;                     // bb6[0]: scope 4 at rust.rs:4:14: 4:24
    }

    bb7: {
        StorageLive(_14);                // bb7[0]: scope 4 at rust.rs:4:9: 4:10
        _14 = ((_10 as Some).0: usize);  // bb7[1]: scope 4 at rust.rs:4:9: 4:10
        StorageLive(_15);                // bb7[2]: scope 5 at rust.rs:4:9: 4:10
        _15 = _14;                       // bb7[3]: scope 5 at rust.rs:4:9: 4:10
        _9 = move _15;                   // bb7[4]: scope 5 at rust.rs:4:9: 4:10
        StorageDead(_15);                // bb7[5]: scope 5 at rust.rs:4:9: 4:10
        StorageDead(_14);                // bb7[6]: scope 4 at rust.rs:4:9: 4:10
        StorageDead(_12);                // bb7[7]: scope 4 at rust.rs:4:23: 4:24
        StorageDead(_10);                // bb7[8]: scope 4 at rust.rs:4:23: 4:24
        StorageLive(_16);                // bb7[9]: scope 4 at rust.rs:4:9: 4:10
        _16 = _9;                        // bb7[10]: scope 4 at rust.rs:4:14: 4:24
        StorageLive(_17);                // bb7[11]: scope 6 at rust.rs:5:16: 5:20
        StorageLive(_18);                // bb7[12]: scope 6 at rust.rs:5:18: 5:19
        _18 = _16;                       // bb7[13]: scope 6 at rust.rs:5:18: 5:19
        _19 = const 4usize;              // bb7[14]: scope 6 at rust.rs:5:16: 5:20
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Scalar(0x0000000000000004)
                                         // mir::Constant
                                         // + span: rust.rs:5:16: 5:20
                                         // + ty: usize
                                         // + literal: Const { ty: usize, val: Scalar(0x0000000000000004) }
        _20 = Lt(_18, _19);              // bb7[15]: scope 6 at rust.rs:5:16: 5:20
        assert(move _20, "index out of bounds: the len is move _19 but the index is _18") -> bb8; // bb7[16]: scope 6 at rust.rs:5:16: 5:20
    }

    bb8: {
        _17 = _2[_18];                   // bb8[0]: scope 6 at rust.rs:5:16: 5:20
        _21 = CheckedAdd(_1, move _17);  // bb8[1]: scope 6 at rust.rs:5:9: 5:20
        assert(!move (_21.1: bool), "attempt to add with overflow") -> bb9; // bb8[2]: scope 6 at rust.rs:5:9: 5:20
    }

    bb9: {
        _1 = move (_21.0: usize);        // bb9[0]: scope 6 at rust.rs:5:9: 5:20
        StorageDead(_17);                // bb9[1]: scope 6 at rust.rs:5:19: 5:20
        StorageDead(_16);                // bb9[2]: scope 4 at rust.rs:6:5: 6:6
        StorageDead(_9);                 // bb9[3]: scope 3 at rust.rs:6:5: 6:6
        goto -> bb3;                     // bb9[4]: scope 3 at rust.rs:4:5: 6:6
    }
}
