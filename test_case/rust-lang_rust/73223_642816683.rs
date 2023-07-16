
    bb3: {
        StorageLive(_4);                 // scope 0 at bad_arm.rs:3:14: 3:15
        _4 = ((_2 as Some).0: i32);      // scope 0 at bad_arm.rs:3:14: 3:15
        _1 = _4;                         // scope 2 at bad_arm.rs:3:20: 3:21
        StorageDead(_4);                 // scope 0 at bad_arm.rs:3:21: 3:22
        StorageDead(_2);                 // scope 0 at bad_arm.rs:5:6: 5:7
        StorageLive(_6);                 // scope 1 at bad_arm.rs:7:9: 7:14
        StorageLive(_7);                 // scope 1 at bad_arm.rs:7:22: 7:27
        _7 = _1;                         // scope 1 at bad_arm.rs:7:22: 7:27
        ((_6 as Some).0: i32) = move _7; // scope 1 at bad_arm.rs:7:17: 7:28
        discriminant(_6) = 1;            // scope 1 at bad_arm.rs:7:17: 7:28
        StorageDead(_7);                 // scope 1 at bad_arm.rs:7:27: 7:28
