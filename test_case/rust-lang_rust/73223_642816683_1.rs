
    bb3: {
        _6 = move _2;                    // scope 1 at bad_arm.rs:7:17: 7:28
        StorageDead(_2);                 // scope 0 at bad_arm.rs:5:6: 5:7
        StorageLive(_6);                 // scope 1 at bad_arm.rs:7:9: 7:14
