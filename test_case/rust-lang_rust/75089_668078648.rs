
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn rust_fn(_1: i32, _2: i32) -> i32 {
    debug x => _1;                       // in scope 0 at src/lib.rs:51:27: 51:28
    debug y => _2;                       // in scope 0 at src/lib.rs:51:35: 51:36
    let mut _0: i32;                     // return place in scope 0 at src/lib.rs:51:46: 51:49
    let mut _3: [u8; 100];               // in scope 0 at src/lib.rs:52:9: 52:16
    let _4: ();                          // in scope 0 at src/lib.rs:53:5: 53:73
    let mut _5: core::result::Result<(), core::fmt::Error>; // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:34: 418:80
    let mut _6: &mut Wrapper;            // in scope 0 at src/lib.rs:53:12: 53:34
    let mut _7: Wrapper;                 // in scope 0 at src/lib.rs:53:12: 53:34
    let mut _8: &mut [u8];               // in scope 0 at src/lib.rs:53:25: 53:33
    let mut _9: &mut [u8; 100];          // in scope 0 at src/lib.rs:53:25: 53:33
    let mut _10: &mut [u8; 100];         // in scope 0 at src/lib.rs:53:25: 53:33
    let mut _11: core::fmt::Arguments;   // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
    let mut _12: &[&str];                // in scope 0 at src/lib.rs:53:36: 53:50
    let mut _13: &[&str; 3];             // in scope 0 at src/lib.rs:53:36: 53:50
    let _14: &[&str; 3];                 // in scope 0 at src/lib.rs:53:36: 53:50
    let mut _15: &[core::fmt::ArgumentV1]; // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
    let mut _16: &[core::fmt::ArgumentV1; 3]; // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
    let _17: &[core::fmt::ArgumentV1; 3]; // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
    let _18: [core::fmt::ArgumentV1; 3]; // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
    let mut _19: (&i32, &i32, &i32);     // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
    let mut _20: &i32;                   // in scope 0 at src/lib.rs:53:52: 53:53
    let mut _21: &i32;                   // in scope 0 at src/lib.rs:53:55: 53:56
    let mut _22: &i32;                   // in scope 0 at src/lib.rs:53:58: 53:63
    let _23: i32;                        // in scope 0 at src/lib.rs:53:58: 53:63
    let mut _24: i32;                    // in scope 0 at src/lib.rs:53:58: 53:59
    let mut _25: i32;                    // in scope 0 at src/lib.rs:53:62: 53:63
    let mut _29: core::fmt::ArgumentV1;  // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
    let mut _30: &i32;                   // in scope 0 at src/lib.rs:53:52: 53:53
    let mut _31: for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error>; // in scope 0 at src/lib.rs:53:52: 53:53
    let mut _32: core::fmt::ArgumentV1;  // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
    let mut _33: &i32;                   // in scope 0 at src/lib.rs:53:55: 53:56
    let mut _34: for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error>; // in scope 0 at src/lib.rs:53:55: 53:56
    let mut _35: core::fmt::ArgumentV1;  // in scope 0 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
    let mut _36: &i32;                   // in scope 0 at src/lib.rs:53:58: 53:63
    let mut _37: for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error>; // in scope 0 at src/lib.rs:53:58: 53:63
    scope 1 {
        debug buf => _3;                 // in scope 1 at src/lib.rs:52:9: 52:16
        let _26: &i32;                   // in scope 1 at src/lib.rs:53:52: 53:53
        let _27: &i32;                   // in scope 1 at src/lib.rs:53:55: 53:56
        let _28: &i32;                   // in scope 1 at src/lib.rs:53:58: 53:63
        let mut _38: &[&str; 3];         // in scope 1 at src/lib.rs:53:36: 53:50
        scope 2 {
            debug arg0 => _26;           // in scope 2 at src/lib.rs:53:52: 53:53
            debug arg1 => _27;           // in scope 2 at src/lib.rs:53:55: 53:56
            debug arg2 => _28;           // in scope 2 at src/lib.rs:53:58: 53:63
        }
    }

    bb0: {
        StorageLive(_3);                 // scope 0 at src/lib.rs:52:9: 52:16
        _3 = [const 0_u8; 100];          // scope 0 at src/lib.rs:52:19: 52:29
                                         // ty::Const
                                         // + ty: u8
                                         // + val: Value(Scalar(0x00))
                                         // mir::Constant
                                         // + span: src/lib.rs:52:20: 52:23
                                         // + literal: Const { ty: u8, val: Value(Scalar(0x00)) }
        StorageLive(_4);                 // scope 1 at src/lib.rs:53:5: 53:73
        StorageLive(_5);                 // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:34: 418:80
        StorageLive(_6);                 // scope 1 at src/lib.rs:53:12: 53:34
        StorageLive(_7);                 // scope 1 at src/lib.rs:53:12: 53:34
        StorageLive(_8);                 // scope 1 at src/lib.rs:53:25: 53:33
        StorageLive(_9);                 // scope 1 at src/lib.rs:53:25: 53:33
        StorageLive(_10);                // scope 1 at src/lib.rs:53:25: 53:33
        _10 = &mut _3;                   // scope 1 at src/lib.rs:53:25: 53:33
        _9 = &mut (*_10);                // scope 1 at src/lib.rs:53:25: 53:33
        _8 = move _9 as &mut [u8] (Pointer(Unsize)); // scope 1 at src/lib.rs:53:25: 53:33
        StorageDead(_9);                 // scope 1 at src/lib.rs:53:32: 53:33
        _7 = const Wrapper::new(move _8) -> bb1; // scope 1 at src/lib.rs:53:12: 53:34
                                         // ty::Const
                                         // + ty: fn(&mut [u8]) -> Wrapper {Wrapper::new}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:53:12: 53:24
                                         // + user_ty: UserType(0)
                                         // + literal: Const { ty: fn(&mut [u8]) -> Wrapper {Wrapper::new}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        _6 = &mut _7;                    // scope 1 at src/lib.rs:53:12: 53:34
        StorageDead(_8);                 // scope 1 at src/lib.rs:53:33: 53:34
        StorageLive(_11);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageLive(_12);                // scope 1 at src/lib.rs:53:36: 53:50
        StorageLive(_13);                // scope 1 at src/lib.rs:53:36: 53:50
        StorageLive(_14);                // scope 1 at src/lib.rs:53:36: 53:50
        _38 = const rust_fn::promoted[0]; // scope 1 at src/lib.rs:53:36: 53:50
                                         // ty::Const
                                         // + ty: &[&str; 3]
                                         // + val: Unevaluated(WithOptConstParam { did: DefId(0:18 ~ rs_nostd_core_lib_1[b3f4]::rust_fn[0]), const_param_did: None }, [], Some(promoted[0]))
                                         // mir::Constant
                                         // + span: src/lib.rs:53:36: 53:50
                                         // + literal: Const { ty: &[&str; 3], val: Unevaluated(WithOptConstParam { did: DefId(0:18 ~ rs_nostd_core_lib_1[b3f4]::rust_fn[0]), const_param_did: None }, [], Some(promoted[0])) }
        _14 = _38;                       // scope 1 at src/lib.rs:53:36: 53:50
        _13 = _14;                       // scope 1 at src/lib.rs:53:36: 53:50
        _12 = move _13 as &[&str] (Pointer(Unsize)); // scope 1 at src/lib.rs:53:36: 53:50
        StorageDead(_13);                // scope 1 at src/lib.rs:53:49: 53:50
        StorageLive(_15);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageLive(_16);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageLive(_17);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageLive(_18);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageLive(_19);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageLive(_20);                // scope 1 at src/lib.rs:53:52: 53:53
        _20 = &_1;                       // scope 1 at src/lib.rs:53:52: 53:53
        StorageLive(_21);                // scope 1 at src/lib.rs:53:55: 53:56
        _21 = &_2;                       // scope 1 at src/lib.rs:53:55: 53:56
        StorageLive(_22);                // scope 1 at src/lib.rs:53:58: 53:63
        StorageLive(_23);                // scope 1 at src/lib.rs:53:58: 53:63
        StorageLive(_24);                // scope 1 at src/lib.rs:53:58: 53:59
        _24 = _1;                        // scope 1 at src/lib.rs:53:58: 53:59
        StorageLive(_25);                // scope 1 at src/lib.rs:53:62: 53:63
        _25 = _2;                        // scope 1 at src/lib.rs:53:62: 53:63
        _23 = Add(move _24, move _25);   // scope 1 at src/lib.rs:53:58: 53:63
        StorageDead(_25);                // scope 1 at src/lib.rs:53:62: 53:63
        StorageDead(_24);                // scope 1 at src/lib.rs:53:62: 53:63
        _22 = &_23;                      // scope 1 at src/lib.rs:53:58: 53:63
        (_19.0: &i32) = move _20;        // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        (_19.1: &i32) = move _21;        // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        (_19.2: &i32) = move _22;        // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageDead(_22);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_21);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_20);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageLive(_26);                // scope 1 at src/lib.rs:53:52: 53:53
        _26 = (_19.0: &i32);             // scope 1 at src/lib.rs:53:52: 53:53
        StorageLive(_27);                // scope 1 at src/lib.rs:53:55: 53:56
        _27 = (_19.1: &i32);             // scope 1 at src/lib.rs:53:55: 53:56
        StorageLive(_28);                // scope 1 at src/lib.rs:53:58: 53:63
        _28 = (_19.2: &i32);             // scope 1 at src/lib.rs:53:58: 53:63
        StorageLive(_29);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageLive(_30);                // scope 2 at src/lib.rs:53:52: 53:53
        _30 = _26;                       // scope 2 at src/lib.rs:53:52: 53:53
        StorageLive(_31);                // scope 2 at src/lib.rs:53:52: 53:53
        _31 = const <i32 as core::fmt::Display>::fmt as for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error> (Pointer(ReifyFnPointer)); // scope 2 at src/lib.rs:53:52: 53:53
                                         // ty::Const
                                         // + ty: for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error> {<i32 as core::fmt::Display>::fmt}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:53:52: 53:53
                                         // + literal: Const { ty: for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error> {<i32 as core::fmt::Display>::fmt}, val: Value(Scalar(<ZST>)) }
        _29 = const core::fmt::ArgumentV1::new::<i32>(move _30, move _31) -> bb2; // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
                                         // ty::Const
                                         // + ty: for<'b> fn(&'b i32, for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error>) -> core::fmt::ArgumentV1<'b> {core::fmt::ArgumentV1::new::<i32>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
                                         // + user_ty: UserType(2)
                                         // + literal: Const { ty: for<'b> fn(&'b i32, for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error>) -> core::fmt::ArgumentV1<'b> {core::fmt::ArgumentV1::new::<i32>}, val: Value(Scalar(<ZST>)) }
    }

    bb2: {
        StorageDead(_31);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_30);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageLive(_32);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageLive(_33);                // scope 2 at src/lib.rs:53:55: 53:56
        _33 = _27;                       // scope 2 at src/lib.rs:53:55: 53:56
        StorageLive(_34);                // scope 2 at src/lib.rs:53:55: 53:56
        _34 = const <i32 as core::fmt::Display>::fmt as for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error> (Pointer(ReifyFnPointer)); // scope 2 at src/lib.rs:53:55: 53:56
                                         // ty::Const
                                         // + ty: for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error> {<i32 as core::fmt::Display>::fmt}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:53:55: 53:56
                                         // + literal: Const { ty: for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error> {<i32 as core::fmt::Display>::fmt}, val: Value(Scalar(<ZST>)) }
        _32 = const core::fmt::ArgumentV1::new::<i32>(move _33, move _34) -> bb3; // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
                                         // ty::Const
                                         // + ty: for<'b> fn(&'b i32, for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error>) -> core::fmt::ArgumentV1<'b> {core::fmt::ArgumentV1::new::<i32>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
                                         // + user_ty: UserType(3)
                                         // + literal: Const { ty: for<'b> fn(&'b i32, for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error>) -> core::fmt::ArgumentV1<'b> {core::fmt::ArgumentV1::new::<i32>}, val: Value(Scalar(<ZST>)) }
    }

    bb3: {
        StorageDead(_34);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_33);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageLive(_35);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageLive(_36);                // scope 2 at src/lib.rs:53:58: 53:63
        _36 = _28;                       // scope 2 at src/lib.rs:53:58: 53:63
        StorageLive(_37);                // scope 2 at src/lib.rs:53:58: 53:63
        _37 = const <i32 as core::fmt::Display>::fmt as for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error> (Pointer(ReifyFnPointer)); // scope 2 at src/lib.rs:53:58: 53:63
                                         // ty::Const
                                         // + ty: for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error> {<i32 as core::fmt::Display>::fmt}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:53:58: 53:63
                                         // + literal: Const { ty: for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error> {<i32 as core::fmt::Display>::fmt}, val: Value(Scalar(<ZST>)) }
        _35 = const core::fmt::ArgumentV1::new::<i32>(move _36, move _37) -> bb4; // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
                                         // ty::Const
                                         // + ty: for<'b> fn(&'b i32, for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error>) -> core::fmt::ArgumentV1<'b> {core::fmt::ArgumentV1::new::<i32>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
                                         // + user_ty: UserType(4)
                                         // + literal: Const { ty: for<'b> fn(&'b i32, for<'r, 's, 't0> fn(&'r i32, &'s mut core::fmt::Formatter<'t0>) -> core::result::Result<(), core::fmt::Error>) -> core::fmt::ArgumentV1<'b> {core::fmt::ArgumentV1::new::<i32>}, val: Value(Scalar(<ZST>)) }
    }

    bb4: {
        StorageDead(_37);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_36);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        _18 = [move _29, move _32, move _35]; // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageDead(_35);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_32);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_29);                // scope 2 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_28);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_27);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_26);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        _17 = &_18;                      // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        _16 = _17;                       // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        _15 = move _16 as &[core::fmt::ArgumentV1] (Pointer(Unsize)); // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
        StorageDead(_16);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        _11 = const core::fmt::Arguments::new_v1(move _12, move _15) -> bb5; // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
                                         // ty::Const
                                         // + ty: fn(&[&'static str], &[core::fmt::ArgumentV1]) -> core::fmt::Arguments {core::fmt::Arguments::new_v1}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: /home/omer/rust/rust/library/core/src/macros/mod.rs:418:49: 418:79
                                         // + user_ty: UserType(1)
                                         // + literal: Const { ty: fn(&[&'static str], &[core::fmt::ArgumentV1]) -> core::fmt::Arguments {core::fmt::Arguments::new_v1}, val: Value(Scalar(<ZST>)) }
    }

    bb5: {
        StorageDead(_15);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        StorageDead(_12);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:78: 418:79
        _5 = const <Wrapper as core::fmt::Write>::write_fmt(move _6, move _11) -> bb6; // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:34: 418:80
                                         // ty::Const
                                         // + ty: for<'r, 's> fn(&'r mut Wrapper, core::fmt::Arguments<'s>) -> core::result::Result<(), core::fmt::Error> {<Wrapper as core::fmt::Write>::write_fmt}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: /home/omer/rust/rust/library/core/src/macros/mod.rs:418:39: 418:48
                                         // + literal: Const { ty: for<'r, 's> fn(&'r mut Wrapper, core::fmt::Arguments<'s>) -> core::result::Result<(), core::fmt::Error> {<Wrapper as core::fmt::Write>::write_fmt}, val: Value(Scalar(<ZST>)) }
    }

    bb6: {
        StorageDead(_11);                // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:79: 418:80
        StorageDead(_6);                 // scope 1 at /home/omer/rust/rust/library/core/src/macros/mod.rs:418:79: 418:80
        _4 = const core::result::Result::<(), core::fmt::Error>::unwrap(move _5) -> bb7; // scope 1 at src/lib.rs:53:5: 53:73
                                         // ty::Const
                                         // + ty: fn(core::result::Result<(), core::fmt::Error>) {core::result::Result::<(), core::fmt::Error>::unwrap}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:53:65: 53:71
                                         // + literal: Const { ty: fn(core::result::Result<(), core::fmt::Error>) {core::result::Result::<(), core::fmt::Error>::unwrap}, val: Value(Scalar(<ZST>)) }
    }

    bb7: {
        StorageDead(_5);                 // scope 1 at src/lib.rs:53:72: 53:73
        StorageDead(_23);                // scope 1 at src/lib.rs:53:73: 53:74
        StorageDead(_19);                // scope 1 at src/lib.rs:53:73: 53:74
        StorageDead(_18);                // scope 1 at src/lib.rs:53:73: 53:74
        StorageDead(_17);                // scope 1 at src/lib.rs:53:73: 53:74
        StorageDead(_14);                // scope 1 at src/lib.rs:53:73: 53:74
        StorageDead(_10);                // scope 1 at src/lib.rs:53:73: 53:74
        StorageDead(_7);                 // scope 1 at src/lib.rs:53:73: 53:74
        StorageDead(_4);                 // scope 1 at src/lib.rs:53:73: 53:74
        _0 = const 0_i32;                // scope 1 at src/lib.rs:54:5: 54:6
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000000))
                                         // mir::Constant
                                         // + span: src/lib.rs:54:5: 54:6
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
        StorageDead(_3);                 // scope 0 at src/lib.rs:55:1: 55:2
        return;                          // scope 0 at src/lib.rs:55:2: 55:2
    }
}

promoted[0] in rust_fn: &[&str; 3] = {
    let mut _0: &[&str; 3];              // return place in scope 0 at src/lib.rs:53:36: 53:50
    let mut _1: [&str; 3];               // in scope 0 at src/lib.rs:53:36: 53:50
    scope 1 {
        scope 2 {
        }
    }

    bb0: {
        _1 = [const "", const " + ", const " = "]; // scope 0 at src/lib.rs:53:36: 53:50
                                         // ty::Const
                                         // + ty: &str
                                         // + val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 })
                                         // mir::Constant
                                         // + span: src/lib.rs:53:36: 53:50
                                         // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
                                         // ty::Const
                                         // + ty: &str
                                         // + val: Value(Slice { data: Allocation { bytes: [32, 43, 32], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [7], len: Size { raw: 3 } }, size: Size { raw: 3 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 3 })
                                         // mir::Constant
                                         // + span: src/lib.rs:53:36: 53:50
                                         // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [32, 43, 32], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [7], len: Size { raw: 3 } }, size: Size { raw: 3 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 3 }) }
                                         // ty::Const
                                         // + ty: &str
                                         // + val: Value(Slice { data: Allocation { bytes: [32, 61, 32], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [7], len: Size { raw: 3 } }, size: Size { raw: 3 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 3 })
                                         // mir::Constant
                                         // + span: src/lib.rs:53:36: 53:50
                                         // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [32, 61, 32], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [7], len: Size { raw: 3 } }, size: Size { raw: 3 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 3 }) }
        _0 = &_1;                        // scope 0 at src/lib.rs:53:36: 53:50
        return;                          // scope 0 at src/lib.rs:53:36: 53:50
    }
}

fn panic(_1: &core::panic::PanicInfo) -> ! {
    debug _info => _1;                   // in scope 0 at src/lib.rs:8:10: 8:15
    let mut _0: !;                       // return place in scope 0 at src/lib.rs:8:32: 8:33

    bb0: {
        goto -> bb1;                     // scope 0 at src/lib.rs:9:5: 9:12
    }

    bb1: {
        goto -> bb1;                     // scope 0 at src/lib.rs:9:5: 9:12
    }
}

fn <impl at src/lib.rs:28:1: 48:2>::write_str(_1: &mut Wrapper, _2: &str) -> core::result::Result<(), core::fmt::Error> {
    debug self => _1;                    // in scope 0 at src/lib.rs:29:18: 29:27
    debug s => _2;                       // in scope 0 at src/lib.rs:29:29: 29:30
    let mut _0: core::result::Result<(), core::fmt::Error>; // return place in scope 0 at src/lib.rs:29:41: 29:52
    let _3: &[u8];                       // in scope 0 at src/lib.rs:30:13: 30:18
    let mut _4: &str;                    // in scope 0 at src/lib.rs:30:21: 30:22
    let mut _6: &mut [u8];               // in scope 0 at src/lib.rs:33:30: 33:53
    let mut _7: &mut [u8];               // in scope 0 at src/lib.rs:33:30: 33:38
    let mut _8: core::ops::RangeFrom<usize>; // in scope 0 at src/lib.rs:33:39: 33:52
    let mut _9: usize;                   // in scope 0 at src/lib.rs:33:39: 33:50
    let mut _10: bool;                   // in scope 0 at src/lib.rs:35:12: 35:41
    let mut _11: usize;                  // in scope 0 at src/lib.rs:35:12: 35:27
    let mut _12: &[u8];                  // in scope 0 at src/lib.rs:35:12: 35:21
    let mut _13: usize;                  // in scope 0 at src/lib.rs:35:30: 35:41
    let mut _14: &[u8];                  // in scope 0 at src/lib.rs:35:30: 35:35
    let mut _16: &mut [u8];              // in scope 0 at src/lib.rs:39:30: 39:54
    let mut _17: &mut [u8];              // in scope 0 at src/lib.rs:39:30: 39:39
    let mut _18: core::ops::RangeTo<usize>; // in scope 0 at src/lib.rs:39:40: 39:53
    let mut _19: usize;                  // in scope 0 at src/lib.rs:39:42: 39:53
    let mut _20: &[u8];                  // in scope 0 at src/lib.rs:39:42: 39:47
    let _21: ();                         // in scope 0 at src/lib.rs:41:9: 41:41
    let mut _22: &mut [u8];              // in scope 0 at src/lib.rs:41:9: 41:18
    let mut _23: &[u8];                  // in scope 0 at src/lib.rs:41:35: 41:40
    let mut _24: usize;                  // in scope 0 at src/lib.rs:44:24: 44:35
    let mut _25: &[u8];                  // in scope 0 at src/lib.rs:44:24: 44:29
    scope 1 {
        debug bytes => _3;               // in scope 1 at src/lib.rs:30:13: 30:18
        let _5: &mut [u8];               // in scope 1 at src/lib.rs:33:13: 33:22
        scope 2 {
            debug remainder => _5;       // in scope 2 at src/lib.rs:33:13: 33:22
            let _15: &mut [u8];          // in scope 2 at src/lib.rs:39:13: 39:22
            scope 3 {
                debug remainder => _15;  // in scope 3 at src/lib.rs:39:13: 39:22
            }
        }
    }

    bb0: {
        StorageLive(_3);                 // scope 0 at src/lib.rs:30:13: 30:18
        StorageLive(_4);                 // scope 0 at src/lib.rs:30:21: 30:22
        _4 = _2;                         // scope 0 at src/lib.rs:30:21: 30:22
        _3 = const core::str::<impl str>::as_bytes(move _4) -> bb1; // scope 0 at src/lib.rs:30:21: 30:33
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r str) -> &'r [u8] {core::str::<impl str>::as_bytes}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:30:23: 30:31
                                         // + literal: Const { ty: for<'r> fn(&'r str) -> &'r [u8] {core::str::<impl str>::as_bytes}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        StorageDead(_4);                 // scope 0 at src/lib.rs:30:32: 30:33
        StorageLive(_5);                 // scope 1 at src/lib.rs:33:13: 33:22
        StorageLive(_6);                 // scope 1 at src/lib.rs:33:30: 33:53
        StorageLive(_7);                 // scope 1 at src/lib.rs:33:30: 33:38
        _7 = &mut (*((*_1).0: &mut [u8])); // scope 1 at src/lib.rs:33:30: 33:38
        StorageLive(_8);                 // scope 1 at src/lib.rs:33:39: 33:52
        StorageLive(_9);                 // scope 1 at src/lib.rs:33:39: 33:50
        _9 = ((*_1).1: usize);           // scope 1 at src/lib.rs:33:39: 33:50
        (_8.0: usize) = move _9;         // scope 1 at src/lib.rs:33:39: 33:52
        StorageDead(_9);                 // scope 1 at src/lib.rs:33:51: 33:52
        _6 = const <[u8] as core::ops::IndexMut<core::ops::RangeFrom<usize>>>::index_mut(move _7, move _8) -> bb2; // scope 1 at src/lib.rs:33:30: 33:53
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut [u8], core::ops::RangeFrom<usize>) -> &'r mut <[u8] as core::ops::Index<core::ops::RangeFrom<usize>>>::Output {<[u8] as core::ops::IndexMut<core::ops::RangeFrom<usize>>>::index_mut}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:33:30: 33:53
                                         // + literal: Const { ty: for<'r> fn(&'r mut [u8], core::ops::RangeFrom<usize>) -> &'r mut <[u8] as core::ops::Index<core::ops::RangeFrom<usize>>>::Output {<[u8] as core::ops::IndexMut<core::ops::RangeFrom<usize>>>::index_mut}, val: Value(Scalar(<ZST>)) }
    }

    bb2: {
        StorageDead(_8);                 // scope 1 at src/lib.rs:33:52: 33:53
        StorageDead(_7);                 // scope 1 at src/lib.rs:33:52: 33:53
        _5 = &mut (*_6);                 // scope 1 at src/lib.rs:33:25: 33:53
        StorageLive(_10);                // scope 2 at src/lib.rs:35:12: 35:41
        StorageLive(_11);                // scope 2 at src/lib.rs:35:12: 35:27
        StorageLive(_12);                // scope 2 at src/lib.rs:35:12: 35:21
        _12 = &(*_5);                    // scope 2 at src/lib.rs:35:12: 35:21
        _11 = const core::slice::<impl [u8]>::len(move _12) -> bb3; // scope 2 at src/lib.rs:35:12: 35:27
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r [u8]) -> usize {core::slice::<impl [u8]>::len}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:35:22: 35:25
                                         // + literal: Const { ty: for<'r> fn(&'r [u8]) -> usize {core::slice::<impl [u8]>::len}, val: Value(Scalar(<ZST>)) }
    }

    bb3: {
        StorageDead(_12);                // scope 2 at src/lib.rs:35:26: 35:27
        StorageLive(_13);                // scope 2 at src/lib.rs:35:30: 35:41
        StorageLive(_14);                // scope 2 at src/lib.rs:35:30: 35:35
        _14 = _3;                        // scope 2 at src/lib.rs:35:30: 35:35
        _13 = const core::slice::<impl [u8]>::len(move _14) -> bb4; // scope 2 at src/lib.rs:35:30: 35:41
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r [u8]) -> usize {core::slice::<impl [u8]>::len}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:35:36: 35:39
                                         // + literal: Const { ty: for<'r> fn(&'r [u8]) -> usize {core::slice::<impl [u8]>::len}, val: Value(Scalar(<ZST>)) }
    }

    bb4: {
        StorageDead(_14);                // scope 2 at src/lib.rs:35:40: 35:41
        _10 = Lt(move _11, move _13);    // scope 2 at src/lib.rs:35:12: 35:41
        StorageDead(_13);                // scope 2 at src/lib.rs:35:40: 35:41
        StorageDead(_11);                // scope 2 at src/lib.rs:35:40: 35:41
        switchInt(_10) -> [false: bb5, otherwise: bb6]; // scope 2 at src/lib.rs:35:9: 37:10
    }

    bb5: {
        StorageDead(_10);                // scope 2 at src/lib.rs:37:9: 37:10
        StorageLive(_15);                // scope 2 at src/lib.rs:39:13: 39:22
        StorageLive(_16);                // scope 2 at src/lib.rs:39:30: 39:54
        StorageLive(_17);                // scope 2 at src/lib.rs:39:30: 39:39
        _17 = &mut (*_5);                // scope 2 at src/lib.rs:39:30: 39:39
        StorageLive(_18);                // scope 2 at src/lib.rs:39:40: 39:53
        StorageLive(_19);                // scope 2 at src/lib.rs:39:42: 39:53
        StorageLive(_20);                // scope 2 at src/lib.rs:39:42: 39:47
        _20 = _3;                        // scope 2 at src/lib.rs:39:42: 39:47
        _19 = const core::slice::<impl [u8]>::len(move _20) -> bb8; // scope 2 at src/lib.rs:39:42: 39:53
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r [u8]) -> usize {core::slice::<impl [u8]>::len}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:39:48: 39:51
                                         // + literal: Const { ty: for<'r> fn(&'r [u8]) -> usize {core::slice::<impl [u8]>::len}, val: Value(Scalar(<ZST>)) }
    }

    bb6: {
        _0 = const core::result::Result::<(), core::fmt::Error>::Err(core::fmt::Error); // scope 2 at src/lib.rs:36:20: 36:41
                                         // ty::Const
                                         // + ty: core::result::Result<(), core::fmt::Error>
                                         // + val: Value(Scalar(0x01))
                                         // mir::Constant
                                         // + span: src/lib.rs:36:20: 36:41
                                         // + literal: Const { ty: core::result::Result<(), core::fmt::Error>, val: Value(Scalar(0x01)) }
        StorageDead(_10);                // scope 2 at src/lib.rs:37:9: 37:10
        StorageDead(_6);                 // scope 1 at src/lib.rs:47:5: 47:6
        StorageDead(_5);                 // scope 1 at src/lib.rs:47:5: 47:6
        StorageDead(_3);                 // scope 0 at src/lib.rs:47:5: 47:6
        goto -> bb7;                     // scope 0 at src/lib.rs:36:13: 36:41
    }

    bb7: {
        return;                          // scope 0 at src/lib.rs:47:6: 47:6
    }

    bb8: {
        StorageDead(_20);                // scope 2 at src/lib.rs:39:52: 39:53
        (_18.0: usize) = move _19;       // scope 2 at src/lib.rs:39:40: 39:53
        StorageDead(_19);                // scope 2 at src/lib.rs:39:52: 39:53
        _16 = const <[u8] as core::ops::IndexMut<core::ops::RangeTo<usize>>>::index_mut(move _17, move _18) -> bb9; // scope 2 at src/lib.rs:39:30: 39:54
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut [u8], core::ops::RangeTo<usize>) -> &'r mut <[u8] as core::ops::Index<core::ops::RangeTo<usize>>>::Output {<[u8] as core::ops::IndexMut<core::ops::RangeTo<usize>>>::index_mut}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:39:30: 39:54
                                         // + literal: Const { ty: for<'r> fn(&'r mut [u8], core::ops::RangeTo<usize>) -> &'r mut <[u8] as core::ops::Index<core::ops::RangeTo<usize>>>::Output {<[u8] as core::ops::IndexMut<core::ops::RangeTo<usize>>>::index_mut}, val: Value(Scalar(<ZST>)) }
    }

    bb9: {
        StorageDead(_18);                // scope 2 at src/lib.rs:39:53: 39:54
        StorageDead(_17);                // scope 2 at src/lib.rs:39:53: 39:54
        _15 = &mut (*_16);               // scope 2 at src/lib.rs:39:25: 39:54
        StorageLive(_21);                // scope 3 at src/lib.rs:41:9: 41:41
        StorageLive(_22);                // scope 3 at src/lib.rs:41:9: 41:18
        _22 = &mut (*_15);               // scope 3 at src/lib.rs:41:9: 41:18
        StorageLive(_23);                // scope 3 at src/lib.rs:41:35: 41:40
        _23 = _3;                        // scope 3 at src/lib.rs:41:35: 41:40
        _21 = const core::slice::<impl [u8]>::copy_from_slice(move _22, move _23) -> bb10; // scope 3 at src/lib.rs:41:9: 41:41
                                         // ty::Const
                                         // + ty: for<'r, 's> fn(&'r mut [u8], &'s [u8]) {core::slice::<impl [u8]>::copy_from_slice}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:41:19: 41:34
                                         // + literal: Const { ty: for<'r, 's> fn(&'r mut [u8], &'s [u8]) {core::slice::<impl [u8]>::copy_from_slice}, val: Value(Scalar(<ZST>)) }
    }

    bb10: {
        StorageDead(_23);                // scope 3 at src/lib.rs:41:40: 41:41
        StorageDead(_22);                // scope 3 at src/lib.rs:41:40: 41:41
        StorageDead(_21);                // scope 3 at src/lib.rs:41:41: 41:42
        StorageLive(_24);                // scope 3 at src/lib.rs:44:24: 44:35
        StorageLive(_25);                // scope 3 at src/lib.rs:44:24: 44:29
        _25 = _3;                        // scope 3 at src/lib.rs:44:24: 44:29
        _24 = const core::slice::<impl [u8]>::len(move _25) -> bb11; // scope 3 at src/lib.rs:44:24: 44:35
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r [u8]) -> usize {core::slice::<impl [u8]>::len}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:44:30: 44:33
                                         // + literal: Const { ty: for<'r> fn(&'r [u8]) -> usize {core::slice::<impl [u8]>::len}, val: Value(Scalar(<ZST>)) }
    }

    bb11: {
        StorageDead(_25);                // scope 3 at src/lib.rs:44:34: 44:35
        ((*_1).1: usize) = Add(((*_1).1: usize), move _24); // scope 3 at src/lib.rs:44:9: 44:35
        StorageDead(_24);                // scope 3 at src/lib.rs:44:34: 44:35
        _0 = const core::result::Result::<(), core::fmt::Error>::Ok(()); // scope 3 at src/lib.rs:46:9: 46:15
                                         // ty::Const
                                         // + ty: core::result::Result<(), core::fmt::Error>
                                         // + val: Value(Scalar(0x00))
                                         // mir::Constant
                                         // + span: src/lib.rs:46:9: 46:15
                                         // + literal: Const { ty: core::result::Result<(), core::fmt::Error>, val: Value(Scalar(0x00)) }
        StorageDead(_16);                // scope 2 at src/lib.rs:47:5: 47:6
        StorageDead(_15);                // scope 2 at src/lib.rs:47:5: 47:6
        StorageDead(_6);                 // scope 1 at src/lib.rs:47:5: 47:6
        StorageDead(_5);                 // scope 1 at src/lib.rs:47:5: 47:6
        StorageDead(_3);                 // scope 0 at src/lib.rs:47:5: 47:6
        goto -> bb7;                     // scope 0 at src/lib.rs:47:6: 47:6
    }
}

fn <impl at src/lib.rs:19:1: 26:2>::new(_1: &mut [u8]) -> Wrapper {
    debug buf => _1;                     // in scope 0 at src/lib.rs:20:16: 20:19
    let mut _0: Wrapper;                 // return place in scope 0 at src/lib.rs:20:38: 20:42
    let mut _2: &mut [u8];               // in scope 0 at src/lib.rs:22:18: 22:21

    bb0: {
        StorageLive(_2);                 // scope 0 at src/lib.rs:22:18: 22:21
        _2 = &mut (*_1);                 // scope 0 at src/lib.rs:22:18: 22:21
        (_0.0: &mut [u8]) = move _2;     // scope 0 at src/lib.rs:21:9: 24:10
        (_0.1: usize) = const 0_usize;   // scope 0 at src/lib.rs:21:9: 24:10
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x00000000))
                                         // mir::Constant
                                         // + span: src/lib.rs:23:21: 23:22
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x00000000)) }
        StorageDead(_2);                 // scope 0 at src/lib.rs:24:9: 24:10
        return;                          // scope 0 at src/lib.rs:25:6: 25:6
    }
}

rust_fn::{{constant}}#0: usize = {
    let mut _0: usize;                   // return place in scope 0 at src/lib.rs:52:25: 52:28

    bb0: {
        _0 = const 100_usize;            // scope 0 at src/lib.rs:52:25: 52:28
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x00000064))
                                         // mir::Constant
                                         // + span: src/lib.rs:52:25: 52:28
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x00000064)) }
        return;                          // scope 0 at src/lib.rs:52:25: 52:28
    }
}
