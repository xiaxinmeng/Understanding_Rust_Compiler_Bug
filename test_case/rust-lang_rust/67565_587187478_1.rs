rust
fn  foo() -> () {
    let mut _0: ();                      // return place in scope 0 at foo.rs:5:10: 5:10
    let _1: &mut &mut i32;               // in scope 0 at foo.rs:6:9: 6:28
    let _2: &&mut i32;                   // in scope 0 at foo.rs:6:22: 6:28
    let mut _3: &mut i32;                // in scope 0 at foo.rs:6:31: 6:37
    let mut _4: i32;                     // in scope 0 at foo.rs:6:36: 6:37
    scope 1 {
        debug _z => _1;                  // in scope 1 at foo.rs:6:9: 6:28
        debug _a => _2;                  // in scope 1 at foo.rs:6:22: 6:28
    }

    bb0: {
        StorageLive(_3);                 // bb0[0]: scope 0 at foo.rs:6:31: 6:37
        StorageLive(_4);                 // bb0[1]: scope 0 at foo.rs:6:36: 6:37
        _4 = const 1i32;                 // bb0[2]: scope 0 at foo.rs:6:36: 6:37
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000001))
                                         // mir::Constant
                                         // + span: foo.rs:6:36: 6:37
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }   
        _3 = &mut _4;                    // bb0[3]: scope 0 at foo.rs:6:31: 6:37
        StorageLive(_1);                 // bb0[4]: scope 0 at foo.rs:6:9: 6:28
        _1 = &mut _3;                    // bb0[5]: scope 0 at foo.rs:6:9: 6:28
        StorageLive(_2);                 // bb0[6]: scope 0 at foo.rs:6:22: 6:28
        _2 = &_3;                        // bb0[7]: scope 0 at foo.rs:6:22: 6:28
        (*(*_1)) = const 0i32;           // bb0[8]: scope 1 at foo.rs:7:5: 7:13
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000000))
                                         // mir::Constant
                                         // + span: foo.rs:7:12: 7:13
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }   
        _0 = ();                         // bb0[9]: scope 0 at foo.rs:5:10: 8:2
        StorageDead(_2);                 // bb0[10]: scope 0 at foo.rs:8:1: 8:2
        StorageDead(_1);                 // bb0[11]: scope 0 at foo.rs:8:1: 8:2
        StorageDead(_4);                 // bb0[12]: scope 0 at foo.rs:8:1: 8:2
        StorageDead(_3);                 // bb0[13]: scope 0 at foo.rs:8:1: 8:2
        return;                          // bb0[14]: scope 0 at foo.rs:8:2: 8:2
    }
}
