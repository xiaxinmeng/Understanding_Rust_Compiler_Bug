
    let mut _0: usize;                   // return place
    scope 1 {
        let _1: u32;                     // "x" in scope 1 at src/main.rs:4:9: 4:10
        scope 3 {
            let mut _2: *const u32;      // "x" in scope 3 at src/main.rs:5:9: 5:14
            scope 5 {
                let _6: u32;             // "y" in scope 5 at src/main.rs:7:13: 7:14
            }
            scope 6 {
            }
        }
        scope 4 {
        }
    }
    scope 2 {
    }
    let mut _3: *const u32;
    let mut _4: &u32;
    let mut _5: &u32;
    let mut _7: *const u32;
    let mut _8: &u32;
    let mut _9: &u32;

    bb0: {                              
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:4:9: 4:10
        _1 = const 5u32;                 // bb0[1]: scope 0 at src/main.rs:4:13: 4:14
                                         // ty::Const
                                         // + ty: u32
                                         // + val: Value(ByVal(Bytes(5)))
                                         // mir::Constant
                                         // + span: src/main.rs:4:13: 4:14
                                         // + ty: u32
                                         // + literal: const 5u32
        StorageLive(_2);                 // bb0[2]: scope 1 at src/main.rs:5:9: 5:14
        StorageLive(_3);                 // bb0[3]: scope 1 at src/main.rs:5:29: 5:31
        StorageLive(_4);                 // bb0[4]: scope 1 at src/main.rs:5:29: 5:31
        StorageLive(_5);                 // bb0[5]: scope 1 at src/main.rs:5:29: 5:31
        _5 = &_1;                        // bb0[6]: scope 1 at src/main.rs:5:29: 5:31
        _4 = _5;                         // bb0[7]: scope 1 at src/main.rs:5:29: 5:31
        _3 = move _4 as *const u32 (Misc); // bb0[8]: scope 1 at src/main.rs:5:29: 5:31
        _2 = move _3;                    // bb0[9]: scope 1 at src/main.rs:5:29: 5:31
        StorageDead(_3);                 // bb0[10]: scope 1 at src/main.rs:5:30: 5:31
        StorageDead(_4);                 // bb0[11]: scope 1 at src/main.rs:5:30: 5:31
        StorageDead(_5);                 // bb0[12]: scope 1 at src/main.rs:5:31: 5:32
        goto -> bb1;                     // bb0[13]: scope 3 at src/main.rs:6:5: 9:6
    }

    bb1: {                              
        StorageLive(_6);                 // bb1[0]: scope 3 at src/main.rs:7:13: 7:14
        _6 = const 5u32;                 // bb1[1]: scope 3 at src/main.rs:7:17: 7:18
                                         // ty::Const
                                         // + ty: u32
                                         // + val: Value(ByVal(Bytes(5)))
                                         // mir::Constant
                                         // + span: src/main.rs:7:17: 7:18
                                         // + ty: u32
                                         // + literal: const 5u32
        StorageLive(_7);                 // bb1[2]: scope 5 at src/main.rs:8:13: 8:15
        StorageLive(_8);                 // bb1[3]: scope 5 at src/main.rs:8:13: 8:15
        StorageLive(_9);                 // bb1[4]: scope 5 at src/main.rs:8:13: 8:15
        _9 = &_6;                        // bb1[5]: scope 5 at src/main.rs:8:13: 8:15
        _8 = _9;                         // bb1[6]: scope 5 at src/main.rs:8:13: 8:15
        _7 = move _8 as *const u32 (Misc); // bb1[7]: scope 5 at src/main.rs:8:13: 8:15
        StorageDead(_8);                 // bb1[8]: scope 5 at src/main.rs:8:14: 8:15
        _2 = move _7;                    // bb1[9]: scope 5 at src/main.rs:8:9: 8:15
        StorageDead(_7);                 // bb1[10]: scope 5 at src/main.rs:8:14: 8:15
        StorageDead(_9);                 // bb1[11]: scope 5 at src/main.rs:8:15: 8:16
        StorageDead(_6);                 // bb1[12]: scope 3 at src/main.rs:9:5: 9:6
        goto -> bb1;                     // bb1[13]: scope 3 at src/main.rs:6:5: 9:6
    }
