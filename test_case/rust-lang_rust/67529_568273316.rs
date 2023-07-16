rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
 main::{{constant}}#0: usize = {
    let mut _0: usize;                   // return place in scope 0 at example/dst-field-align.rs:6:23: 6:24

    bb0: {
        _0 = const 4usize;               // bb0[0]: scope 0 at example/dst-field-align.rs:6:23: 6:24
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x0000000000000004))
                                         // mir::Constant
                                         // + span: example/dst-field-align.rs:6:23: 6:24
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000004)) }
        return;                          // bb0[1]: scope 0 at example/dst-field-align.rs:6:23: 6:24
    }
}

fn  main() -> () {
    let mut _0: ();                      // return place in scope 0 at example/dst-field-align.rs:5:11: 5:11
    let _1: Baz<[i32; 4]> as UserTypeProjection { base: UserType(0), projs: [] }; // in scope 0 at example/dst-field-align.rs:6:9: 6:10
    let mut _2: [i32; 4];                // in scope 0 at example/dst-field-align.rs:6:38: 6:47
    let _3: ();                          // in scope 0 at <::std::macros::println macros>:2:4: 2:71
    let mut _4: std::fmt::Arguments;     // in scope 0 at <::std::macros::println macros>:2:29: 2:70
    let mut _5: &[&str];                 // in scope 0 at example/dst-field-align.rs:7:14: 7:20
    let _6: &[&str; 2];                  // in scope 0 at example/dst-field-align.rs:7:14: 7:20
    let mut _7: &[std::fmt::ArgumentV1]; // in scope 0 at <::std::macros::println macros>:2:29: 2:70
    let _8: &[std::fmt::ArgumentV1; 1];  // in scope 0 at <::std::macros::println macros>:2:29: 2:70
    let _9: [std::fmt::ArgumentV1; 1];   // in scope 0 at <::std::macros::println macros>:2:29: 2:70
    let mut _10: (&[i32; 4],);           // in scope 0 at <::std::macros::println macros>:2:29: 2:70
    let mut _11: &[i32; 4];              // in scope 0 at example/dst-field-align.rs:7:22: 7:25
    let mut _13: std::fmt::ArgumentV1;   // in scope 0 at <::std::macros::println macros>:2:29: 2:70
    let mut _14: &[i32; 4];              // in scope 0 at example/dst-field-align.rs:7:22: 7:25
    let mut _15: for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 0 at example/dst-field-align.rs:7:22: 7:25
    scope 1 {
        debug d => _1;                   // in scope 1 at example/dst-field-align.rs:6:9: 6:10
        let _12: &[i32; 4];              // in scope 1 at example/dst-field-align.rs:7:22: 7:25
        scope 2 {
            debug arg0 => _12;           // in scope 2 at example/dst-field-align.rs:7:22: 7:25
            scope 3 {
                debug x => _14;          // in scope 3 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:277:23: 277:24
                debug f => _15;          // in scope 3 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:277:33: 277:34
                let mut _16: for<'r, 's, 't0> fn(&'r core::fmt::Void, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 3 at <::std::macros::println macros>:2:29: 2:70
                let mut _17: for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 3 at <::std::macros::println macros>:2:29: 2:70
                let mut _18: &core::fmt::Void; // in scope 3 at <::std::macros::println macros>:2:29: 2:70
                let mut _19: &[i32; 4];  // in scope 3 at <::std::macros::println macros>:2:29: 2:70
            }
        }
        scope 5 {
            debug pieces => _5;          // in scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:313:19: 313:25
            debug args => _7;            // in scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:313:42: 313:46
            let mut _20: &[&str];        // in scope 5 at <::std::macros::println macros>:2:29: 2:70
            let mut _21: std::option::Option<&[std::fmt::rt::v1::Argument]>; // in scope 5 at <::std::macros::println macros>:2:29: 2:70
            let mut _22: &[std::fmt::ArgumentV1]; // in scope 5 at <::std::macros::println macros>:2:29: 2:70
        }
    }
    scope 4 {
    }

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at example/dst-field-align.rs:6:9: 6:10
        StorageLive(_2);                 // bb0[1]: scope 0 at example/dst-field-align.rs:6:38: 6:47
        _2 = [const 1i32, const 2i32, const 3i32, const 4i32]; // bb0[2]: scope 0 at example/dst-field-align.rs:6:38: 6:47
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000001))
                                         // mir::Constant
                                         // + span: example/dst-field-align.rs:6:39: 6:40
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000002))
                                         // mir::Constant
                                         // + span: example/dst-field-align.rs:6:41: 6:42
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000002)) }
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000003))
                                         // mir::Constant
                                         // + span: example/dst-field-align.rs:6:43: 6:44
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000003)) }
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000004))
                                         // mir::Constant
                                         // + span: example/dst-field-align.rs:6:45: 6:46
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000004)) }
        (_1.0: [i32; 4]) = move _2;      // bb0[3]: scope 0 at example/dst-field-align.rs:6:29: 6:49
        StorageDead(_2);                 // bb0[4]: scope 0 at example/dst-field-align.rs:6:48: 6:49
        StorageLive(_3);                 // bb0[5]: scope 1 at <::std::macros::println macros>:2:4: 2:71
        StorageLive(_4);                 // bb0[6]: scope 1 at <::std::macros::println macros>:2:29: 2:70
        StorageLive(_5);                 // bb0[7]: scope 1 at example/dst-field-align.rs:7:14: 7:20
        _6 = &(promoted[0]: [&str; 2]);  // bb0[8]: scope 1 at example/dst-field-align.rs:7:14: 7:20
        _5 = move _6 as &[&str] (Pointer(Unsize)); // bb0[9]: scope 1 at example/dst-field-align.rs:7:14: 7:20
        StorageLive(_7);                 // bb0[10]: scope 1 at <::std::macros::println macros>:2:29: 2:70
        StorageLive(_9);                 // bb0[11]: scope 1 at <::std::macros::println macros>:2:29: 2:70
        StorageLive(_11);                // bb0[12]: scope 1 at example/dst-field-align.rs:7:22: 7:25
        _11 = &(_1.0: [i32; 4]);         // bb0[13]: scope 1 at example/dst-field-align.rs:7:22: 7:25
        _10 = const Scalar(AllocId(40).0x0) : (&[i32; 4],); // bb0[14]: scope 1 at <::std::macros::println macros>:2:29: 2:70
                                         // ty::Const
                                         // + ty: (&[i32; 4],)
                                         // + val: Value(Scalar(AllocId(40).0x0))
                                         // mir::Constant
                                         // + span: <::std::macros::println macros>:2:29: 2:70
                                         // + literal: Const { ty: (&[i32; 4],), val: Value(Scalar(AllocId(40).0x0)) }
        StorageDead(_11);                // bb0[15]: scope 1 at <::std::macros::println macros>:2:69: 2:70
        StorageLive(_12);                // bb0[16]: scope 1 at example/dst-field-align.rs:7:22: 7:25
        _12 = (_10.0: &[i32; 4]);        // bb0[17]: scope 1 at example/dst-field-align.rs:7:22: 7:25
        StorageLive(_13);                // bb0[18]: scope 2 at <::std::macros::println macros>:2:29: 2:70
        StorageLive(_14);                // bb0[19]: scope 2 at example/dst-field-align.rs:7:22: 7:25
        _14 = _12;                       // bb0[20]: scope 2 at example/dst-field-align.rs:7:22: 7:25
        StorageLive(_15);                // bb0[21]: scope 2 at example/dst-field-align.rs:7:22: 7:25
        _15 = const <[i32; 4] as std::fmt::Debug>::fmt as for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> (Pointer(ReifyFnPointer)); // bb0[22]: scope 2 at example/dst-field-align.rs:7:22: 7:25
                                         // ty::Const
                                         // + ty: for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> {<[i32; 4] as std::fmt::Debug>::fmt}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: example/dst-field-align.rs:7:22: 7:25
                                         // + literal: Const { ty: for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> {<[i32; 4] as std::fmt::Debug>::fmt}, val: Value(Scalar(<ZST>)) }
        StorageLive(_16);                // bb0[23]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:42: 278:59
        StorageLive(_17);                // bb0[24]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:57: 278:58
        _17 = _15;                       // bb0[25]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:57: 278:58
        _16 = const std::intrinsics::transmute::<for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Void, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>>(move _17) -> bb2; // bb0[26]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:42: 278:59
                                         // ty::Const
                                         // + ty: unsafe extern "rust-intrinsic" fn(for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) -> for<'r, 's, 't0> fn(&'r core::fmt::Void, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> {std::intrinsics::transmute::<for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Void, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:42: 278:56
                                         // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) -> for<'r, 's, 't0> fn(&'r core::fmt::Void, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> {std::intrinsics::transmute::<for<'r, 's, 't0> fn(&'r [i32; 4], &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Void, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>>}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        StorageDead(_4);                 // bb1[0]: scope 1 at <::std::macros::println macros>:2:70: 2:71
        StorageDead(_9);                 // bb1[1]: scope 1 at <::std::macros::println macros>:2:72: 2:73
        StorageDead(_3);                 // bb1[2]: scope 1 at <::std::macros::println macros>:2:72: 2:73
        StorageDead(_1);                 // bb1[3]: scope 0 at example/dst-field-align.rs:8:1: 8:2
        return;                          // bb1[4]: scope 0 at example/dst-field-align.rs:8:2: 8:2
    }

    bb2: {
        StorageDead(_17);                // bb2[0]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:58: 278:59
        StorageLive(_18);                // bb2[1]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:68: 278:85
        StorageLive(_19);                // bb2[2]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:83: 278:84
        _19 = _14;                       // bb2[3]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:83: 278:84
        _18 = const std::intrinsics::transmute::<&[i32; 4], &core::fmt::Void>(move _19) -> bb3; // bb2[4]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:68: 278:85
                                         // ty::Const
                                         // + ty: unsafe extern "rust-intrinsic" fn(&[i32; 4]) -> &core::fmt::Void {std::intrinsics::transmute::<&[i32; 4], &core::fmt::Void>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:68: 278:82
                                         // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(&[i32; 4]) -> &core::fmt::Void {std::intrinsics::transmute::<&[i32; 4], &core::fmt::Void>}, val: Value(Scalar(<ZST>)) }
    }

    bb3: {
        StorageDead(_19);                // bb3[0]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:84: 278:85
        (_13.0: &core::fmt::Void) = move _18; // bb3[1]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:18: 278:87
        (_13.1: for<'r, 's, 't0> fn(&'r core::fmt::Void, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) = move _16; // bb3[2]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:18: 278:87
        StorageDead(_18);                // bb3[3]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:86: 278:87
        StorageDead(_16);                // bb3[4]: scope 4 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:278:86: 278:87
        StorageDead(_15);                // bb3[5]: scope 2 at <::std::macros::println macros>:2:69: 2:70
        StorageDead(_14);                // bb3[6]: scope 2 at <::std::macros::println macros>:2:69: 2:70
        _9 = [move _13];                 // bb3[7]: scope 2 at <::std::macros::println macros>:2:29: 2:70
        StorageDead(_13);                // bb3[8]: scope 2 at <::std::macros::println macros>:2:69: 2:70
        StorageDead(_12);                // bb3[9]: scope 1 at <::std::macros::println macros>:2:69: 2:70
        _8 = &_9;                        // bb3[10]: scope 1 at <::std::macros::println macros>:2:29: 2:70
        _7 = move _8 as &[std::fmt::ArgumentV1] (Pointer(Unsize)); // bb3[11]: scope 1 at <::std::macros::println macros>:2:29: 2:70
        StorageLive(_20);                // bb3[12]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:21: 314:27
        _20 = _5;                        // bb3[13]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:21: 314:27
        StorageLive(_21);                // bb3[14]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:34: 314:38
        discriminant(_21) = 0;           // bb3[15]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:34: 314:38
        StorageLive(_22);                // bb3[16]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:40: 314:44
        _22 = _7;                        // bb3[17]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:40: 314:44
        (_4.0: &[&str]) = move _20;      // bb3[18]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:9: 314:46
        (_4.1: std::option::Option<&[std::fmt::rt::v1::Argument]>) = move _21; // bb3[19]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:9: 314:46
        (_4.2: &[std::fmt::ArgumentV1]) = move _22; // bb3[20]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:9: 314:46
        StorageDead(_22);                // bb3[21]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:45: 314:46
        StorageDead(_21);                // bb3[22]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:45: 314:46
        StorageDead(_20);                // bb3[23]: scope 5 at /rustc/fc5deca2143a448d10a1241a777275e59448c94d/src/libcore/fmt/mod.rs:314:45: 314:46
        StorageDead(_7);                 // bb3[24]: scope 1 at <::std::macros::println macros>:2:69: 2:70
        StorageDead(_5);                 // bb3[25]: scope 1 at <::std::macros::println macros>:2:69: 2:70
        _3 = const std::io::_print(move _4) -> bb1; // bb3[26]: scope 1 at <::std::macros::println macros>:2:4: 2:71
                                         // ty::Const
                                         // + ty: for<'r> fn(std::fmt::Arguments<'r>) {std::io::_print}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: <::std::macros::println macros>:2:4: 2:27
                                         // + literal: Const { ty: for<'r> fn(std::fmt::Arguments<'r>) {std::io::_print}, val: Value(Scalar(<ZST>)) }
    }
}

promoted[0] in  main: [&str; 2] = {
    let mut _0: [&str; 2];               // return place in scope 0 at example/dst-field-align.rs:7:14: 7:20
    let mut _1: [&str; 2];               // in scope 0 at example/dst-field-align.rs:7:14: 7:20
    scope 1 {
        scope 2 {
        }
    }

    bb0: {
        _1 = [const "", const "\n"];     // bb0[0]: scope 0 at example/dst-field-align.rs:7:14: 7:20
                                         // ty::Const
                                         // + ty: &str
                                         // + val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 })
                                         // mir::Constant
                                         // + span: example/dst-field-align.rs:7:14: 7:20
                                         // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
                                         // ty::Const
                                         // + ty: &str
                                         // + val: Value(Slice { data: Allocation { bytes: [10], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
                                         // mir::Constant
                                         // + span: example/dst-field-align.rs:7:14: 7:20
                                         // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [10], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
        _0 = move _1;                    // bb0[1]: scope 0 at example/dst-field-align.rs:7:14: 7:20
        return;                          // bb0[2]: scope 0 at example/dst-field-align.rs:7:14: 7:20
    }
}
