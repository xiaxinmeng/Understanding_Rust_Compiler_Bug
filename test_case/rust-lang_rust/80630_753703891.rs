bb1
    bb1: {
        switchInt(_1) -> [0_i32: bb3, 1_i32: bb5, 2_i32: bb7, otherwise: bb2]; // scope 0 at src/lib.rs:10:13: 10:14
    }
    bb4: {
        StorageDead(_3);                 // scope 0 at src/lib.rs:10:29: 10:30
        StorageDead(_4);                 // scope 0 at src/lib.rs:10:30: 10:31
        StorageDead(_2);                 // scope 0 at src/lib.rs:10:30: 10:31
        _1 = const 2_i32;                // scope 0 at src/lib.rs:10:32: 10:37
        goto -> bb1;                     // scope 0 at src/lib.rs:9:9: 14:10
    }
