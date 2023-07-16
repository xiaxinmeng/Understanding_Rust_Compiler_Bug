
// MIR for `foo2` after PreCodegen

fn foo2(_1: u64) -> u64 {
    debug n => _1;                       // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:2:9: 2:10
    let mut _0: u64;                     // return place in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:3:9: 3:18
    let mut _2: std::ops::Range<u64>;    // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
    let mut _3: std::ops::Range<u64>;    // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
    let mut _4: u64;                     // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
    let mut _7: std::option::Option<u64>; // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
    let mut _8: &mut std::ops::Range<u64>; // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
    let mut _9: &mut std::ops::Range<u64>; // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
    let mut _10: isize;                  // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:9: 4:10
    let mut _12: std::iter::Rev<std::ops::RangeInclusive<u64>>; // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
    let mut _13: std::ops::RangeInclusive<u64>; // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:25
    let mut _14: u64;                    // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:23: 5:24
    let mut _16: std::option::Option<u64>; // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
    let mut _17: &mut std::iter::Rev<std::ops::RangeInclusive<u64>>; // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
    let mut _18: &mut std::iter::Rev<std::ops::RangeInclusive<u64>>; // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
    let mut _19: isize;                  // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:13: 5:14
    let mut _21: u64;                    // in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:25
    scope 1 {
        debug count => _0;               // in scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:3:9: 3:18
        let mut _5: std::ops::Range<u64>; // in scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        scope 2 {
            debug iter => _5;            // in scope 2 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
            let mut _6: u64;             // in scope 2 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
            scope 3 {
                debug __next => _6;      // in scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
                let _11: u64;            // in scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:9: 4:10
                scope 4 {
                    debug val => _11;    // in scope 4 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:9: 4:10
                }
                scope 5 {
                    let mut _15: std::iter::Rev<std::ops::RangeInclusive<u64>>; // in scope 5 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
                    scope 6 {
                        debug iter => _15; // in scope 6 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
                        scope 7 {
                            debug __next => _20; // in scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
                            let _20: u64; // in scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:13: 5:14
                            scope 8 {
                                debug val => _20; // in scope 8 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:13: 5:14
                            }
                            scope 9 {
                                debug j => _20; // in scope 9 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:13: 5:14
                            }
                            scope 14 {
                                debug self => _17; // in scope 14 at /home/alarsyo/work/rust/library/core/src/iter/adapters/mod.rs:49:13: 49:22
                                let mut _23: &mut std::ops::RangeInclusive<u64>; // in scope 14 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
                            }
                        }
                    }
                    scope 11 {
                        debug start => _21; // in scope 11 at /home/alarsyo/work/rust/library/core/src/ops/range.rs:366:22: 366:27
                        debug end => _14; // in scope 11 at /home/alarsyo/work/rust/library/core/src/ops/range.rs:366:34: 366:37
                    }
                    scope 12 {
                        debug self => _13; // in scope 12 at /home/alarsyo/work/rust/library/core/src/iter/traits/iterator.rs:2622:12: 2622:16
                        let mut _22: std::ops::RangeInclusive<u64>; // in scope 12 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
                        scope 15 {
                            debug iter => _22; // in scope 15 at /home/alarsyo/work/rust/library/core/src/iter/adapters/mod.rs:36:23: 36:27
                        }
                    }
                    scope 13 {
                        debug self => _12; // in scope 13 at /home/alarsyo/work/rust/library/core/src/iter/traits/collect.rs:248:18: 248:22
                    }
                }
            }
        }
        scope 10 {
            debug self => _3;            // in scope 10 at /home/alarsyo/work/rust/library/core/src/iter/traits/collect.rs:248:18: 248:22
        }
    }

    bb0: {
        _0 = const 0_u64;                // scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:3:21: 3:22
        StorageLive(_2);                 // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        StorageLive(_3);                 // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        StorageLive(_4);                 // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        _4 = _1;                         // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        (_3.0: u64) = const 0_u64;       // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        (_3.1: u64) = move _4;           // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        StorageDead(_4);                 // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        _2 = move _3;                    // scope 10 at /home/alarsyo/work/rust/library/core/src/iter/traits/collect.rs:249:9: 249:13
        StorageDead(_3);                 // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        StorageLive(_5);                 // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        _5 = move _2;                    // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        goto -> bb1;                     // scope 2 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:5: 8:6
    }

    bb1: {
        StorageLive(_6);                 // scope 2 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        StorageLive(_7);                 // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        StorageLive(_8);                 // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        StorageLive(_9);                 // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        _9 = &mut _5;                    // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        _8 = &mut (*_9);                 // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
        _7 = <std::ops::Range<u64> as std::iter::Iterator>::next(move _8) -> bb2; // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
                                         // mir::Constant
                                         // + span: /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:14: 4:18
                                         // + literal: Const { ty: for<'r> fn(&'r mut std::ops::Range<u64>) -> std::option::Option<<std::ops::Range<u64> as std::iter::Iterator>::Item> {<std::ops::Range<u64> as std::iter::Iterator>::next}, val: Value(Scalar(<ZST>)) }
    }

    bb2: {
        StorageDead(_8);                 // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        _10 = discriminant(_7);          // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:9: 4:10
        switchInt(move _10) -> [0_isize: bb3, otherwise: bb4]; // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:9: 4:10
    }

    bb3: {
        StorageDead(_9);                 // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        StorageDead(_7);                 // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        StorageDead(_6);                 // scope 2 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:8:5: 8:6
        StorageDead(_5);                 // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:8:5: 8:6
        StorageDead(_2);                 // scope 1 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        return;                          // scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:10:2: 10:2
    }

    bb4: {
        _11 = ((_7 as Some).0: u64);     // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:9: 4:10
        _6 = move _11;                   // scope 4 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:9: 4:10
        StorageDead(_9);                 // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        StorageDead(_7);                 // scope 3 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:17: 4:18
        StorageLive(_13);                // scope 5 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:25
        _14 = _1;                        // scope 5 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:23: 5:24
        _21 = const 0_u64;               // scope 5 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:25
        (_13.0: u64) = const 0_u64;      // scope 11 at /home/alarsyo/work/rust/library/core/src/ops/range.rs:367:9: 367:46
        (_13.1: u64) = move _14;         // scope 11 at /home/alarsyo/work/rust/library/core/src/ops/range.rs:367:9: 367:46
        (_13.2: bool) = const false;     // scope 11 at /home/alarsyo/work/rust/library/core/src/ops/range.rs:367:9: 367:46
        _22 = move _13;                  // scope 12 at /home/alarsyo/work/rust/library/core/src/iter/traits/iterator.rs:2626:18: 2626:22
        (_12.0: std::ops::RangeInclusive<u64>) = move _22; // scope 15 at /home/alarsyo/work/rust/library/core/src/iter/adapters/mod.rs:37:9: 37:21
        StorageDead(_13);                // scope 5 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:30: 5:31
        StorageLive(_15);                // scope 5 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
        _15 = move _12;                  // scope 5 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
        goto -> bb5;                     // scope 6 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:9: 7:10
    }

    bb5: {
        StorageLive(_16);                // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
        StorageLive(_17);                // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
        StorageLive(_18);                // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
        _18 = &mut _15;                  // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
        _17 = &mut (*_18);               // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:18: 5:31
        StorageLive(_23);                // scope 14 at /home/alarsyo/work/rust/library/core/src/iter/adapters/mod.rs:50:9: 50:18
        _23 = &mut ((*_17).0: std::ops::RangeInclusive<u64>); // scope 14 at /home/alarsyo/work/rust/library/core/src/iter/adapters/mod.rs:50:9: 50:18
        _16 = <std::ops::RangeInclusive<u64> as std::iter::DoubleEndedIterator>::next_back(move _23) -> bb8; // scope 14 at /home/alarsyo/work/rust/library/core/src/iter/adapters/mod.rs:50:9: 50:30
                                         // mir::Constant
                                         // + span: /home/alarsyo/work/rust/library/core/src/iter/adapters/mod.rs:50:19: 50:28
                                         // + literal: Const { ty: for<'r> fn(&'r mut std::ops::RangeInclusive<u64>) -> std::option::Option<<std::ops::RangeInclusive<u64> as std::iter::Iterator>::Item> {<std::ops::RangeInclusive<u64> as std::iter::DoubleEndedIterator>::next_back}, val: Value(Scalar(<ZST>)) }
    }

    bb6: {
        StorageDead(_18);                // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:30: 5:31
        StorageDead(_16);                // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:30: 5:31
        StorageDead(_15);                // scope 5 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:7:9: 7:10
        StorageDead(_6);                 // scope 2 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:8:5: 8:6
        goto -> bb1;                     // scope 2 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:4:5: 8:6
    }

    bb7: {
        _20 = ((_16 as Some).0: u64);    // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:13: 5:14
        StorageDead(_18);                // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:30: 5:31
        StorageDead(_16);                // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:30: 5:31
        _0 = Add(_0, move _20);          // scope 9 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:6:13: 6:23
        goto -> bb5;                     // scope 6 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:9: 7:10
    }

    bb8: {
        StorageDead(_23);                // scope 14 at /home/alarsyo/work/rust/library/core/src/iter/adapters/mod.rs:50:29: 50:30
        StorageDead(_17);                // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:30: 5:31
        _19 = discriminant(_16);         // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:13: 5:14
        switchInt(move _19) -> [0_isize: bb6, otherwise: bb7]; // scope 7 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:5:13: 5:14
    }
}
