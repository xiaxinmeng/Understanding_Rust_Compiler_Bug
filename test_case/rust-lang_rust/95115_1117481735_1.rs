rust
    bb1: {
        StorageDead(_3);                 // scope 2 at test.rs:11:12: 11:13
        StorageLive(_4);                 // scope 2 at test.rs:13:9: 13:12
        _4 = (_1.0: i32);                // scope 2 at test.rs:13:9: 13:12
        _2 = move _4;                    // scope 2 at test.rs:13:5: 13:12
        StorageDead(_4);                 // scope 2 at test.rs:13:11: 13:12
        StorageLive(_5);                 // scope 2 at test.rs:15:5: 15:12
        _5 = touch() -> [return: bb2, unwind: bb4]; // scope 2 at test.rs:15:5: 15:12
                                         // mir::Constant
                                         // + span: test.rs:15:5: 15:10
                                         // + literal: Const { ty: fn() {touch}, val: Value(Scalar(<ZST>)) }
    }
