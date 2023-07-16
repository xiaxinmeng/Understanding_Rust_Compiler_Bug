
// MIR for `test` after SimplifyCfg-promote-consts

fn test() -> bool {
    let mut _0: bool;                    // return place in scope 0 at test.rs:8:18: 8:22
    let mut _1: Test;                    // in scope 0 at test.rs:9:11: 9:18
    let mut _2: isize;                   // in scope 0 at test.rs:10:9: 10:16
    let mut _3: &Test;                   // in scope 0 at test.rs:9:11: 9:18
    let mut _4: bool;                    // in scope 0 at test.rs:10:38: 10:43
    let _5: bool;                        // in scope 0 at test.rs:10:34: 10:35
    let mut _6: bool;                    // in scope 0 at test.rs:10:38: 10:43
    let _7: bool;                        // in scope 0 at test.rs:10:34: 10:35
    scope 1 {
        debug x => _5;                   // in scope 1 at test.rs:10:34: 10:35
    }
    scope 2 {
        debug x => _7;                   // in scope 2 at test.rs:10:34: 10:35
    }

    bb0: {
        StorageLive(_1);                 // scope 0 at test.rs:9:11: 9:18
        _1 = Test::A;                    // scope 0 at test.rs:9:11: 9:18
        FakeRead(ForMatchedPlace(None), _1); // scope 0 at test.rs:9:11: 9:18
        _2 = discriminant(_1);           // scope 0 at test.rs:10:9: 10:16
        switchInt(move _2) -> [0_isize: bb1, 1_isize: bb3, otherwise: bb2]; // scope 0 at test.rs:10:9: 10:16
    }

    bb1: {
        falseEdge -> [real: bb5, imaginary: bb3]; // scope 0 at test.rs:10:9: 10:16
    }

    bb2: {
        _0 = const true;                 // scope 0 at test.rs:12:14: 12:18
        goto -> bb11;                    // scope 0 at test.rs:9:5: 13:6
    }

    bb3: {
        falseEdge -> [real: bb8, imaginary: bb2]; // scope 0 at test.rs:10:19: 10:26
    }

    bb4: {
        _0 = _7;                         // scope 0 at test.rs:11:13: 11:14
        StorageDead(_7);                 // scope 0 at test.rs:11:13: 11:14
        goto -> bb11;                    // scope 0 at test.rs:9:5: 13:6
    }

    bb5: {
        _3 = &shallow _1;                // scope 0 at test.rs:9:11: 9:18
        StorageLive(_4);                 // scope 0 at test.rs:10:38: 10:43
        _4 = const false;                // scope 0 at test.rs:10:38: 10:43
        FakeRead(ForMatchedPlace(None), _4); // scope 0 at test.rs:10:38: 10:43
        falseEdge -> [real: bb7, imaginary: bb6]; // scope 0 at test.rs:10:34: 10:35
    }

    bb6: {
        StorageDead(_5);                 // scope 0 at test.rs:11:13: 11:14
        falseEdge -> [real: bb2, imaginary: bb3]; // scope 0 at test.rs:10:38: 10:43
    }

    bb7: {
        StorageLive(_5);                 // scope 0 at test.rs:10:34: 10:35
        _5 = _4;                         // scope 0 at test.rs:10:34: 10:35
        FakeRead(ForMatchGuard, _3);     // scope 0 at test.rs:10:42: 10:43
        goto -> bb4;                     // scope 0 at test.rs:9:5: 13:6
    }

    bb8: {
        _3 = &shallow _1;                // scope 0 at test.rs:9:11: 9:18
        StorageLive(_6);                 // scope 0 at test.rs:10:38: 10:43
        _6 = const false;                // scope 0 at test.rs:10:38: 10:43
        FakeRead(ForMatchedPlace(None), _6); // scope 0 at test.rs:10:38: 10:43
        falseEdge -> [real: bb10, imaginary: bb9]; // scope 0 at test.rs:10:34: 10:35
    }

    bb9: {
        StorageDead(_7);                 // scope 0 at test.rs:11:13: 11:14
        falseEdge -> [real: bb2, imaginary: bb2]; // scope 0 at test.rs:10:38: 10:43
    }

    bb10: {
        StorageLive(_7);                 // scope 0 at test.rs:10:34: 10:35
        _7 = _6;                         // scope 0 at test.rs:10:34: 10:35
        FakeRead(ForMatchGuard, _3);     // scope 0 at test.rs:10:42: 10:43
        goto -> bb4;                     // scope 0 at test.rs:9:5: 13:6
    }

    bb11: {
        StorageDead(_6);                 // scope 0 at test.rs:14:1: 14:2
        StorageDead(_4);                 // scope 0 at test.rs:14:1: 14:2
        StorageDead(_1);                 // scope 0 at test.rs:14:1: 14:2
        return;                          // scope 0 at test.rs:14:2: 14:2
    }
}
