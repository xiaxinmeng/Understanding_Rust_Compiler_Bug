
fn f(_1: &Option<i32>, _2: &Option<i32>) -> i32 {
    debug x => _1;                       // in scope 0 at t.rs:22:12: 22:13
    debug y => _2;                       // in scope 0 at t.rs:22:19: 22:20
    let mut _0: i32;                     // return place in scope 0 at t.rs:22:29: 22:32
    let _3: ();                          // in scope 0 at t.rs:23:5: 27:6
    let mut _4: isize;                   // in scope 0 at t.rs:23:13: 23:20
    let mut _5: isize;                   // in scope 0 at t.rs:24:17: 24:24
    let mut _6: !;                       // in scope 0 at t.rs:25:13: 25:21

    bb0: {
        StorageLive(_3);                 // scope 0 at t.rs:23:5: 27:6
        _4 = discriminant((*_1));        // scope 0 at t.rs:23:13: 23:20
        switchInt(move _4) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at t.rs:23:13: 23:20
    }

    bb1: {
        _3 = const ();                   // scope 0 at t.rs:27:6: 27:6
        goto -> bb5;                     // scope 0 at t.rs:23:5: 27:6
    }

    bb2: {
        _5 = discriminant((*_2));        // scope 0 at t.rs:24:17: 24:24
        switchInt(move _5) -> [0_isize: bb4, otherwise: bb3]; // scope 0 at t.rs:24:17: 24:24
    }

    bb3: {
        _3 = const ();                   // scope 0 at t.rs:26:10: 26:10
        goto -> bb5;                     // scope 0 at t.rs:24:9: 26:10
    }

    bb4: {
        _0 = const 1_i32;                // scope 0 at t.rs:25:20: 25:21
        StorageDead(_3);                 // scope 0 at t.rs:27:5: 27:6
        goto -> bb6;                     // scope 0 at t.rs:29:2: 29:2
    }

    bb5: {
        StorageDead(_3);                 // scope 0 at t.rs:27:5: 27:6
        _0 = const 42_i32;               // scope 0 at t.rs:28:5: 28:7
        goto -> bb6;                     // scope 0 at t.rs:29:2: 29:2
    }

    bb6: {
        return;                          // scope 0 at t.rs:29:2: 29:2
    }
}
