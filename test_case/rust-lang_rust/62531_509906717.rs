rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn  compare() -> bool {
    let mut _0: bool;                    // return place in scope 0 at src/lib.rs:2:21: 2:25
    let _1: [u8; 4];                     // "bytes" in scope 0 at src/lib.rs:3:9: 3:14
    let mut _2: &[u8; 4];                // in scope 0 at src/lib.rs:4:5: 4:10
    let mut _3: &[u8; 4];                // in scope 0 at src/lib.rs:4:14: 8:6
    let _4: [u8; 4];                     // in scope 0 at src/lib.rs:4:14: 8:6
    let mut _5: bool;                    // in scope 0 at src/lib.rs:4:17: 4:44
    scope 1 {
    }

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/lib.rs:3:9: 3:14
        _1 = const core::f32::<impl f32>::to_ne_bytes(const 12.5f32) -> bb1; // bb0[1]: scope 0 at src/lib.rs:3:17: 3:38
                                         // ty::Const
                                         // + ty: fn(f32) -> [u8; _] {core::f32::<impl f32>::to_ne_bytes}
                                         // + val: Scalar(<ZST>)
                                         // mir::Constant
                                         // + span: src/lib.rs:3:25: 3:36
                                         // + ty: fn(f32) -> [u8; _] {core::f32::<impl f32>::to_ne_bytes}
                                         // + literal: Const { ty: fn(f32) -> [u8; _] {core::f32::<impl f32>::to_ne_bytes}, val: Scalar(<ZST>) }
                                         // ty::Const
                                         // + ty: f32
                                         // + val: Scalar(0x41480000)
                                         // mir::Constant
                                         // + span: src/lib.rs:3:17: 3:24
                                         // + ty: f32
                                         // + literal: Const { ty: f32, val: Scalar(0x41480000) }
    }

    bb1: {
        StorageLive(_2);                 // bb1[0]: scope 1 at src/lib.rs:4:5: 4:10
        _2 = &_1;                        // bb1[1]: scope 1 at src/lib.rs:4:5: 4:10
        StorageLive(_3);                 // bb1[2]: scope 1 at src/lib.rs:4:14: 8:6
        StorageLive(_4);                 // bb1[3]: scope 1 at src/lib.rs:4:14: 8:6
        StorageLive(_5);                 // bb1[4]: scope 1 at src/lib.rs:4:17: 4:44
        _5 = const false;                // bb1[5]: scope 1 at src/lib.rs:4:17: 4:44
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Scalar(0x00)
                                         // mir::Constant
                                         // + span: src/lib.rs:4:17: 4:44
                                         // + ty: bool
                                         // + literal: Const { ty: bool, val: Scalar(0x00) }
        switchInt(_5) -> [false: bb2, otherwise: bb3]; // bb1[6]: scope 1 at src/lib.rs:4:14: 8:6
    }

    bb2: {
        _4 = [const 0u8, const 0u8, const 72u8, const 65u8]; // bb2[0]: scope 1 at src/lib.rs:7:9: 7:33
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Scalar(0x00)
                                         // mir::Constant
                                         // + span: src/lib.rs:7:10: 7:14
                                         // + ty: u8
                                         // + literal: Const { ty: u8, val: Scalar(0x00) }
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Scalar(0x00)
                                         // mir::Constant
                                         // + span: src/lib.rs:7:16: 7:20
                                         // + ty: u8
                                         // + literal: Const { ty: u8, val: Scalar(0x00) }
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Scalar(0x48)
                                         // mir::Constant
                                         // + span: src/lib.rs:7:22: 7:26
                                         // + ty: u8
                                         // + literal: Const { ty: u8, val: Scalar(0x48) }
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Scalar(0x41)
                                         // mir::Constant
                                         // + span: src/lib.rs:7:28: 7:32
                                         // + ty: u8
                                         // + literal: Const { ty: u8, val: Scalar(0x41) }
        goto -> bb4;                     // bb2[1]: scope 1 at src/lib.rs:4:14: 8:6
    }

    bb3: {
        _4 = [const 65u8, const 72u8, const 0u8, const 0u8]; // bb3[0]: scope 1 at src/lib.rs:5:9: 5:33
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Scalar(0x41)
                                         // mir::Constant
                                         // + span: src/lib.rs:5:10: 5:14
                                         // + ty: u8
                                         // + literal: Const { ty: u8, val: Scalar(0x41) }
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Scalar(0x48)
                                         // mir::Constant
                                         // + span: src/lib.rs:5:16: 5:20
                                         // + ty: u8
                                         // + literal: Const { ty: u8, val: Scalar(0x48) }
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Scalar(0x00)
                                         // mir::Constant
                                         // + span: src/lib.rs:5:22: 5:26
                                         // + ty: u8
                                         // + literal: Const { ty: u8, val: Scalar(0x00) }
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Scalar(0x00)
                                         // mir::Constant
                                         // + span: src/lib.rs:5:28: 5:32
                                         // + ty: u8
                                         // + literal: Const { ty: u8, val: Scalar(0x00) }
        goto -> bb4;                     // bb3[1]: scope 1 at src/lib.rs:4:14: 8:6
    }

    bb4: {
        _3 = &_4;                        // bb4[0]: scope 1 at src/lib.rs:4:14: 8:6
        _0 = const <[u8; 4] as std::cmp::PartialEq>::eq(move _2, move _3) -> bb5; // bb4[1]: scope 1 at src/lib.rs:4:5: 8:6
                                         // ty::Const
                                         // + ty: for<'r, 's> fn(&'r [u8; 4], &'s [u8; 4]) -> bool {<[u8; 4] as std::cmp::PartialEq>::eq}
                                         // + val: Scalar(<ZST>)
                                         // mir::Constant
                                         // + span: src/lib.rs:4:5: 8:6
                                         // + ty: for<'r, 's> fn(&'r [u8; 4], &'s [u8; 4]) -> bool {<[u8; 4] as std::cmp::PartialEq>::eq}
                                         // + literal: Const { ty: for<'r, 's> fn(&'r [u8; 4], &'s [u8; 4]) -> bool {<[u8; 4] as std::cmp::PartialEq>::eq}, val: Scalar(<ZST>) }
    }

    bb5: {
        StorageDead(_3);                 // bb5[0]: scope 1 at src/lib.rs:8:5: 8:6
        StorageDead(_2);                 // bb5[1]: scope 1 at src/lib.rs:8:5: 8:6
        StorageDead(_1);                 // bb5[2]: scope 0 at src/lib.rs:9:1: 9:2
        StorageDead(_5);                 // bb5[3]: scope 0 at src/lib.rs:9:1: 9:2
        StorageDead(_4);                 // bb5[4]: scope 0 at src/lib.rs:9:1: 9:2
        return;                          // bb5[5]: scope 0 at src/lib.rs:9:2: 9:2
    }
}
