rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn front(_1: &mut VecDeque<usize>) -> () {
    debug v => _1;                       // in scope 0 at src/lib.rs:3:14: 3:15
    let mut _0: ();                      // return place in scope 0 at src/lib.rs:3:39: 3:39
    let mut _2: bool;                    // in scope 0 at src/lib.rs:4:8: 4:21
    let mut _3: bool;                    // in scope 0 at src/lib.rs:4:9: 4:21
    let mut _4: &std::collections::VecDeque<usize>; // in scope 0 at src/lib.rs:4:9: 4:10
    let _5: &usize;                      // in scope 0 at src/lib.rs:5:9: 5:31
    let mut _6: std::option::Option<&usize>; // in scope 0 at src/lib.rs:5:9: 5:17
    let mut _7: &std::collections::VecDeque<usize>; // in scope 0 at src/lib.rs:5:9: 5:10
    let mut _8: &str;                    // in scope 0 at src/lib.rs:5:25: 5:30
    let _9: &str;                        // in scope 0 at src/lib.rs:5:25: 5:30

    bb0: {
        StorageLive(_2);                 // scope 0 at src/lib.rs:4:8: 4:21
        StorageLive(_3);                 // scope 0 at src/lib.rs:4:9: 4:21
        StorageLive(_4);                 // scope 0 at src/lib.rs:4:9: 4:10
        _4 = &(*_1);                     // scope 0 at src/lib.rs:4:9: 4:10
        _3 = VecDeque::<usize>::is_empty(move _4) -> bb1; // scope 0 at src/lib.rs:4:9: 4:21
                                         // mir::Constant
                                         // + span: src/lib.rs:4:11: 4:19
                                         // + literal: Const { ty: for<'r> fn(&'r std::collections::VecDeque<usize>) -> bool {std::collections::VecDeque::<usize>::is_empty}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        StorageDead(_4);                 // scope 0 at src/lib.rs:4:20: 4:21
        _2 = Not(move _3);               // scope 0 at src/lib.rs:4:8: 4:21
        StorageDead(_3);                 // scope 0 at src/lib.rs:4:20: 4:21
        switchInt(_2) -> [false: bb2, otherwise: bb3]; // scope 0 at src/lib.rs:4:5: 6:6
    }

    bb2: {
        _0 = const ();                   // scope 0 at src/lib.rs:4:5: 6:6
        goto -> bb6;                     // scope 0 at src/lib.rs:4:5: 6:6
    }

    bb3: {
        StorageLive(_5);                 // scope 0 at src/lib.rs:5:9: 5:31
        StorageLive(_6);                 // scope 0 at src/lib.rs:5:9: 5:17
        StorageLive(_7);                 // scope 0 at src/lib.rs:5:9: 5:10
        _7 = &(*_1);                     // scope 0 at src/lib.rs:5:9: 5:10
        _6 = VecDeque::<usize>::get(move _7, const 0_usize) -> bb4; // scope 0 at src/lib.rs:5:9: 5:17
                                         // mir::Constant
                                         // + span: src/lib.rs:5:11: 5:14
                                         // + literal: Const { ty: for<'r> fn(&'r std::collections::VecDeque<usize>, usize) -> std::option::Option<&'r usize> {std::collections::VecDeque::<usize>::get}, val: Value(Scalar(<ZST>)) }
    }

    bb4: {
        StorageDead(_7);                 // scope 0 at src/lib.rs:5:16: 5:17
        StorageLive(_8);                 // scope 0 at src/lib.rs:5:25: 5:30
        StorageLive(_9);                 // scope 0 at src/lib.rs:5:25: 5:30
        _9 = const "foo";                // scope 0 at src/lib.rs:5:25: 5:30
                                         // ty::Const
                                         // + ty: &str
                                         // + val: Value(Slice { data: Allocation { bytes: [102, 111, 111], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [7], len: Size { raw: 3 } }, size: Size { raw: 3 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 3 })
                                         // mir::Constant
                                         // + span: src/lib.rs:5:25: 5:30
                                         // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [102, 111, 111], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [7], len: Size { raw: 3 } }, size: Size { raw: 3 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 3 }) }
        _8 = _9;                         // scope 0 at src/lib.rs:5:25: 5:30
        _5 = Option::<&usize>::expect(move _6, move _8) -> bb5; // scope 0 at src/lib.rs:5:9: 5:31
                                         // mir::Constant
                                         // + span: src/lib.rs:5:18: 5:24
                                         // + literal: Const { ty: for<'r> fn(std::option::Option<&usize>, &'r str) -> &usize {std::option::Option::<&usize>::expect}, val: Value(Scalar(<ZST>)) }
    }

    bb5: {
        StorageDead(_8);                 // scope 0 at src/lib.rs:5:30: 5:31
        StorageDead(_6);                 // scope 0 at src/lib.rs:5:30: 5:31
        StorageDead(_9);                 // scope 0 at src/lib.rs:5:31: 5:32
        StorageDead(_5);                 // scope 0 at src/lib.rs:5:31: 5:32
        _0 = const ();                   // scope 0 at src/lib.rs:4:22: 6:6
        goto -> bb6;                     // scope 0 at src/lib.rs:4:5: 6:6
    }

    bb6: {
        StorageDead(_2);                 // scope 0 at src/lib.rs:7:1: 7:2
        return;                          // scope 0 at src/lib.rs:7:2: 7:2
    }
}
