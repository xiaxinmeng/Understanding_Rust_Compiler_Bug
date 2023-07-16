rust
    bb2: {
        StorageDead(_5);                 // scope 2 at test.rs:15:12: 15:13
        StorageLive(_6);                 // scope 2 at test.rs:17:5: 17:21
        StorageLive(_7);                 // scope 2 at test.rs:17:14: 17:15
        _7 = (_1.0: i32);                // scope 2 at test.rs:17:14: 17:15
        StorageLive(_8);                 // scope 3 at test.rs:17:14: 17:15
        _8 = _7;                         // scope 3 at test.rs:17:14: 17:15
        _2 = move _8;                    // scope 3 at test.rs:17:14: 17:15
        StorageDead(_8);                 // scope 3 at test.rs:17:14: 17:15
        _6 = const ();                   // scope 2 at test.rs:17:5: 17:21
        StorageDead(_7);                 // scope 2 at test.rs:17:20: 17:21
        StorageDead(_6);                 // scope 2 at test.rs:17:21: 17:22
        StorageLive(_9);                 // scope 2 at test.rs:19:5: 19:12
        _9 = touch() -> [return: bb3, unwind: bb4]; // scope 2 at test.rs:19:5: 19:12
                                         // mir::Constant
                                         // + span: test.rs:19:5: 19:10
                                         // + literal: Const { ty: fn() {touch}, val: Value(Scalar(<ZST>)) }
    }
