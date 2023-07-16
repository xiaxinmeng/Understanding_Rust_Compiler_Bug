rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
const  FOO: &*const i32 = {
    let mut _0: &*const i32;             // return place in scope 0 at src/lib.rs:1:17: 1:36
    let _1: &*const i32;                 // in scope 0 at src/lib.rs:1:39: 1:49

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/lib.rs:1:39: 1:49
        _1 = &(promoted[0]: *const i32); // bb0[1]: scope 0 at src/lib.rs:1:39: 1:49
        _0 = _1;                         // bb0[2]: scope 0 at src/lib.rs:1:39: 1:49
        StorageDead(_1);                 // bb0[3]: scope 0 at src/lib.rs:1:48: 1:49
        return;                          // bb0[4]: scope 0 at src/lib.rs:1:1: 1:50
    }
}

promoted[0] in  FOO: *const i32 = {
    let mut _0: *const i32;              // return place in scope 0 at src/lib.rs:1:39: 1:49
    let mut _1: *const i32;              // in scope 0 at src/lib.rs:1:40: 1:49
    let mut _2: &i32;                    // in scope 0 at src/lib.rs:1:41: 1:43
    let mut _3: i32;                     // in scope 0 at src/lib.rs:1:42: 1:43

    bb0: {
        _3 = const 0i32;                 // bb0[0]: scope 0 at src/lib.rs:1:42: 1:43
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000000))
                                         // mir::Constant
                                         // + span: src/lib.rs:1:42: 1:43
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
        _2 = &_3;                        // bb0[1]: scope 0 at src/lib.rs:1:41: 1:43
        _1 = &raw const (*_2);           // bb0[2]: scope 0 at src/lib.rs:1:41: 1:43
        _0 = move _1;                    // bb0[3]: scope 0 at src/lib.rs:1:39: 1:49
        return;                          // bb0[4]: scope 0 at src/lib.rs:1:39: 1:49
    }
}
