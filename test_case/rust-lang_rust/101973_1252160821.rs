

    bb0: {
        StorageLive(_2);                 // scope 0 at ../tmp/constprop.rs:10:5: 10:65
        StorageLive(_3);                 // scope 0 at ../tmp/constprop.rs:10:5: 10:58
        StorageLive(_4);                 // scope 0 at ../tmp/constprop.rs:10:5: 10:17
        StorageLive(_5);                 // scope 0 at ../tmp/constprop.rs:10:10: 10:16
        _5 = _1;                         // scope 0 at ../tmp/constprop.rs:10:10: 10:16
        _4 = const 0_u32;                // scope 1 at ../tmp/constprop.rs:3:19: 3:23
        StorageLive(_12);                // scope 2 at ../tmp/constprop.rs:4:12: 4:27
        StorageLive(_13);                // scope 2 at ../tmp/constprop.rs:4:12: 4:20
        StorageLive(_14);                // scope 2 at ../tmp/constprop.rs:4:13: 4:14
        _14 = _5;                        // scope 2 at ../tmp/constprop.rs:4:13: 4:14
        _15 = CheckedShr(_14, const 0_i32); // scope 2 at ../tmp/constprop.rs:4:12: 4:20
        assert(!move (_15.1: bool), "attempt to shift right by `{}`, which would overflow", const 0_i32) -> bb3; // scope 2 at ../tmp/constprop.rs:4:12: 4:20
    }

    bb1: {
        _8 = move (_10.0: u32);          // scope 0 at ../tmp/constprop.rs:10:32: 10:45
        StorageDead(_9);                 // scope 0 at ../tmp/constprop.rs:10:44: 10:45
        _7 = BitAnd(move _8, const 15_u32); // scope 0 at ../tmp/constprop.rs:10:31: 10:52
        StorageDead(_8);                 // scope 0 at ../tmp/constprop.rs:10:51: 10:52
        _11 = CheckedShl(_7, const 1_i32); // scope 0 at ../tmp/constprop.rs:10:31: 10:57
        assert(!move (_11.1: bool), "attempt to shift left by `{}`, which would overflow", const 1_i32) -> bb2; // scope 0 at ../tmp/constprop.rs:10:31: 10:57
    }

    bb2: {
        _6 = move (_11.0: u32);          // scope 0 at ../tmp/constprop.rs:10:31: 10:57
        StorageDead(_7);                 // scope 0 at ../tmp/constprop.rs:10:56: 10:57
        StorageLive(_16);                // scope 3 at /home/r/src/rust/rustc/library/core/src/num/uint_macros.rs:238:38: 238:42
        _16 = _4;                        // scope 3 at /home/r/src/rust/rustc/library/core/src/num/uint_macros.rs:238:38: 238:42
        StorageLive(_17);                // scope 3 at /home/r/src/rust/rustc/library/core/src/num/uint_macros.rs:238:44: 238:45
        _17 = _6;                        // scope 3 at /home/r/src/rust/rustc/library/core/src/num/uint_macros.rs:238:44: 238:45
        _3 = rotate_right::<u32>(move _16, move _17) -> bb4; // scope 3 at /home/r/src/rust/rustc/library/core/src/num/uint_macros.rs:238:13: 238:56
                                         // mir::Constant
                                         // + span: /home/r/src/rust/rustc/library/core/src/num/uint_macros.rs:238:13: 238:37
                                         // + literal: Const { ty: extern "rust-intrinsic" fn(u32, u32) -> u32 {rotate_right::<u32>}, val: Value(<ZST>) }
    }
