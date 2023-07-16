rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn  foo(_1: &mut std::option::Option<u8>) -> Foo {
    let mut _0: Foo;                     // return place in scope 0 at src/lib.rs:7:35: 7:38
    let mut _2: isize;                   // in scope 0 at src/lib.rs:9:9: 9:24
    let _3: &mut u8;                     // "y" in scope 0 at src/lib.rs:9:14: 9:23
    let _4: &&mut u8;                    // "y" in scope 0 at src/lib.rs:9:14: 9:23
    let mut _5: bool;                    // in scope 0 at src/lib.rs:9:28: 9:35
    let mut _6: u8;                      // in scope 0 at src/lib.rs:9:28: 9:30
    let mut _7: &mut u8;                 // in scope 0 at src/lib.rs:9:46: 9:47
    let _8: &mut std::option::Option<u8>; // "z" in scope 0 at src/lib.rs:10:9: 10:18
    let mut _9: &mut std::option::Option<u8>; // in scope 0 at src/lib.rs:10:29: 10:30
    scope 1 {
    }
    scope 2 {
    }

    bb0: {
        _2 = discriminant((*_1));        // bb0[0]: scope 0 at src/lib.rs:9:9: 9:24
        switchInt(move _2) -> [1isize: bb1, otherwise: bb4]; // bb0[1]: scope 0 at src/lib.rs:9:9: 9:24
    }

    bb1: {
        StorageLive(_4);                 // bb1[0]: scope 0 at src/lib.rs:9:14: 9:23
        StorageLive(_3);                 // bb1[1]: scope 0 at src/lib.rs:9:14: 9:23
        _3 = &mut (((*_1) as Some).0: u8); // bb1[2]: scope 0 at src/lib.rs:9:14: 9:23
        _4 = &_3;                        // bb1[3]: scope 0 at src/lib.rs:9:14: 9:23
        StorageLive(_5);                 // bb1[4]: scope 0 at src/lib.rs:9:28: 9:35
        StorageLive(_6);                 // bb1[5]: scope 0 at src/lib.rs:9:28: 9:30
        _6 = (*(*_4));                   // bb1[6]: scope 0 at src/lib.rs:9:28: 9:30
        _5 = Eq(move _6, const 0u8);     // bb1[7]: scope 0 at src/lib.rs:9:28: 9:35
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Scalar(0x00)
                                         // mir::Constant
                                         // + span: src/lib.rs:9:34: 9:35
                                         // + ty: u8
                                         // + literal: Const { ty: u8, val: Scalar(0x00) }
        StorageDead(_6);                 // bb1[8]: scope 0 at src/lib.rs:9:34: 9:35
        switchInt(move _5) -> [false: bb3, otherwise: bb2]; // bb1[9]: scope 0 at src/lib.rs:9:28: 9:35
    }

    bb2: {
        StorageDead(_5);                 // bb2[0]: scope 0 at src/lib.rs:9:48: 9:49
        StorageLive(_7);                 // bb2[1]: scope 1 at src/lib.rs:9:46: 9:47
        _7 = _3;                         // bb2[2]: scope 1 at src/lib.rs:9:46: 9:47
        ((_0 as A).0: &mut u8) = move _7; // bb2[3]: scope 1 at src/lib.rs:9:39: 9:48
        discriminant(_0) = 0;            // bb2[4]: scope 1 at src/lib.rs:9:39: 9:48
        StorageDead(_7);                 // bb2[5]: scope 1 at src/lib.rs:9:47: 9:48
        StorageDead(_3);                 // bb2[6]: scope 0 at src/lib.rs:9:48: 9:49
        StorageDead(_4);                 // bb2[7]: scope 0 at src/lib.rs:9:48: 9:49
        goto -> bb5;                     // bb2[8]: scope 0 at src/lib.rs:8:5: 11:6
    }

    bb3: {
        StorageDead(_5);                 // bb3[0]: scope 0 at src/lib.rs:9:48: 9:49
        StorageDead(_3);                 // bb3[1]: scope 0 at src/lib.rs:9:48: 9:49
        StorageDead(_4);                 // bb3[2]: scope 0 at src/lib.rs:9:48: 9:49
        goto -> bb4;                     // bb3[3]: scope 0 at src/lib.rs:9:9: 9:24
    }

    bb4: {
        StorageLive(_8);                 // bb4[0]: scope 0 at src/lib.rs:10:9: 10:18
        _8 = _1;                         // bb4[1]: scope 0 at src/lib.rs:10:9: 10:18
        StorageLive(_9);                 // bb4[2]: scope 2 at src/lib.rs:10:29: 10:30
        _9 = _8;                         // bb4[3]: scope 2 at src/lib.rs:10:29: 10:30
        ((_0 as B).0: &mut std::option::Option<u8>) = move _9; // bb4[4]: scope 2 at src/lib.rs:10:22: 10:31
        discriminant(_0) = 1;            // bb4[5]: scope 2 at src/lib.rs:10:22: 10:31
        StorageDead(_9);                 // bb4[6]: scope 2 at src/lib.rs:10:30: 10:31
        StorageDead(_8);                 // bb4[7]: scope 0 at src/lib.rs:10:31: 10:32
        goto -> bb5;                     // bb4[8]: scope 0 at src/lib.rs:8:5: 11:6
    }

    bb5: {
        return;                          // bb5[0]: scope 0 at src/lib.rs:12:2: 12:2
    }
}
