rust
fn opt_char(_1: char) -> u32 {
    debug x => _1;                       // in scope 0 at ./example.rs:1:17: 1:18
    let mut _0: u32;                     // return place in scope 0 at ./example.rs:1:29: 1:32
    let mut _2: bool;                    // in scope 0 at ./example.rs:2:8: 2:28
    let mut _3: char;                    // in scope 0 at ./example.rs:2:8: 2:9
    let mut _4: bool;                    // in scope 0 at ./example.rs:2:20: 2:28
    let mut _5: char;                    // in scope 0 at ./example.rs:2:20: 2:21

    bb0: {
        StorageLive(_2);                 // scope 0 at ./example.rs:2:8: 2:28
        StorageLive(_3);                 // scope 0 at ./example.rs:2:8: 2:9
        _3 = _1;                         // scope 0 at ./example.rs:2:8: 2:9
        switchInt(move _3) -> ['b': bb1, otherwise: bb2]; // scope 0 at ./example.rs:2:8: 2:28
    }

    bb1: {
        StorageDead(_3);                 // scope 0 at ./example.rs:2:8: 2:28
        _2 = const true;                 // scope 0 at ./example.rs:2:8: 2:28
        goto -> bb3;                     // scope 0 at ./example.rs:2:8: 2:28
    }

    bb2: {
        StorageDead(_3);                 // scope 0 at ./example.rs:2:8: 2:28
        StorageLive(_4);                 // scope 0 at ./example.rs:2:20: 2:28
        StorageLive(_5);                 // scope 0 at ./example.rs:2:20: 2:21
        _5 = _1;                         // scope 0 at ./example.rs:2:20: 2:21
        _4 = Eq(move _5, const 'a');     // scope 0 at ./example.rs:2:20: 2:28
        StorageDead(_5);                 // scope 0 at ./example.rs:2:27: 2:28
        _2 = Ne(move _4, const false);        // scope 0 at ./example.rs:2:8: 2:28
        goto -> bb3;                     // scope 0 at ./example.rs:2:8: 2:28
    }

    bb3: {
        StorageDead(_4);                 // scope 0 at ./example.rs:2:27: 2:28
        switchInt(move _2) -> [false: bb4, otherwise: bb5]; // scope 0 at ./example.rs:2:5: 2:45
    }

    bb4: {
        _0 = const 1_u32;                // scope 0 at ./example.rs:2:42: 2:43
        goto -> bb6;                     // scope 0 at ./example.rs:2:5: 2:45
    }

    bb5: {
        _0 = const 0_u32;                // scope 0 at ./example.rs:2:31: 2:32
        goto -> bb6;                     // scope 0 at ./example.rs:2:5: 2:45
    }

    bb6: {
        StorageDead(_2);                 // scope 0 at ./example.rs:3:1: 3:2
        return;                          // scope 0 at ./example.rs:3:2: 3:2
    }
}
