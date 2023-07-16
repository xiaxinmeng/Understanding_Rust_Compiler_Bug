
// MIR for `foo`
// source = MirSource { def_id: DefId(0/0:10 ~ test[8787]::foo[0]), promoted: None }
// pass_name = ElaborateDrops
// disambiguator = after

fn foo(_1: core::option::Option<Box<Foo<usize>>>) -> usize{
    let mut _0: usize;                   // return place
    scope 1 {
        let _2: Foo<usize>;              // "f" in scope 1 at test.rs:26:9: 26:10
    }
    scope 2 {
    }
    scope 3 {
        let _4: Box<Foo<usize>>;         // "vec" in scope 3 at test.rs:28:14: 28:17
    }
    let mut _3: isize;
    let mut _5: isize;
    let mut _6: Foo<usize>;
    let mut _7: usize;
    let mut _8: bool;
    let mut _9: bool;
    let mut _10: ();
    let mut _11: ();
    let mut _12: isize;
    let mut _13: isize;
    let mut _14: isize;

    bb0: {                              
        _8 = const false;                // bb0[0]: scope 0 at test.rs:26:9: 26:10
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:26:9: 26:10
                                         // + ty: bool
                                         // + literal: const false
        _9 = const false;                // bb0[1]: scope 0 at test.rs:26:9: 26:10
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:26:9: 26:10
                                         // + ty: bool
                                         // + literal: const false
        _9 = const true;                 // bb0[2]: scope 0 at test.rs:26:9: 26:10
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(1)))
                                         // mir::Constant
                                         // + span: test.rs:26:9: 26:10
                                         // + ty: bool
                                         // + literal: const true
        StorageLive(_2);                 // bb0[3]: scope 0 at test.rs:26:9: 26:10
        _3 = discriminant(_1);           // bb0[4]: scope 0 at test.rs:26:13: 29:6
        _5 = discriminant(_1);           // bb0[5]: scope 0 at test.rs:27:9: 27:13
        switchInt(move _5) -> [0isize: bb2, 1isize: bb4, otherwise: bb3]; // bb0[6]: scope 0 at test.rs:27:9: 27:13
    }

    bb1: {                               // cleanup
        resume;                          // bb1[0]: scope 0 at test.rs:25:1: 31:2
    }

    bb2: {                              
        _2 = Foo<usize>::{{constructor}}(const 0usize,); // bb2[0]: scope 0 at test.rs:27:17: 27:23
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:27:21: 27:22
                                         // + ty: usize
                                         // + literal: const 0usize
        goto -> bb5;                     // bb2[1]: scope 0 at test.rs:26:13: 29:6
    }

    bb3: {                              
        unreachable;                     // bb3[0]: scope 0 at test.rs:31:2: 31:2
    }

    bb4: {                              
        StorageLive(_4);                 // bb4[0]: scope 0 at test.rs:28:14: 28:17
        _9 = const false;                // bb4[1]: scope 0 at test.rs:28:14: 28:17
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:28:14: 28:17
                                         // + ty: bool
                                         // + literal: const false
        _8 = const true;                 // bb4[2]: scope 0 at test.rs:28:14: 28:17
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(1)))
                                         // mir::Constant
                                         // + span: test.rs:28:14: 28:17
                                         // + ty: bool
                                         // + literal: const true
        _4 = move ((_1 as Some).0: Box<Foo<usize>>); // bb4[3]: scope 0 at test.rs:28:14: 28:17
        StorageLive(_6);                 // bb4[4]: scope 3 at test.rs:28:22: 28:26
        _6 = move (*_4);                 // bb4[5]: scope 3 at test.rs:28:22: 28:26
        _2 = move _6;                    // bb4[6]: scope 3 at test.rs:28:22: 28:26
        StorageDead(_6);                 // bb4[7]: scope 3 at test.rs:28:25: 28:26
        goto -> bb5;                     // bb4[8]: scope 0 at test.rs:26:13: 29:6
    }

    bb5: {                              
        goto -> bb13;                    // bb5[0]: scope 0 at test.rs:29:5: 29:6
    }

    bb6: {                               // cleanup
        goto -> bb20;                    // bb6[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb7: {                              
        _8 = const false;                // bb7[0]: scope 0 at test.rs:29:5: 29:6
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:29:5: 29:6
                                         // + ty: bool
                                         // + literal: const false
        StorageDead(_4);                 // bb7[1]: scope 0 at test.rs:29:5: 29:6
        StorageLive(_7);                 // bb7[2]: scope 1 at test.rs:30:5: 30:8
        _7 = (_2.0: usize);              // bb7[3]: scope 1 at test.rs:30:5: 30:8
        _0 = move _7;                    // bb7[4]: scope 1 at test.rs:30:5: 30:8
        StorageDead(_7);                 // bb7[5]: scope 1 at test.rs:30:7: 30:8
        StorageDead(_2);                 // bb7[6]: scope 0 at test.rs:31:1: 31:2
        goto -> bb37;                    // bb7[7]: scope 0 at test.rs:31:1: 31:2
    }

    bb8: {                              
        return;                          // bb8[0]: scope 0 at test.rs:31:2: 31:2
    }

    bb9: {                              
        _8 = const false;                // bb9[0]: scope 0 at test.rs:29:5: 29:6
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:29:5: 29:6
                                         // + ty: bool
                                         // + literal: const false
        _10 = const box_free(move _4) -> bb7; // bb9[1]: scope 0 at test.rs:29:5: 29:6
                                         // ty::Const
                                         // + ty: unsafe fn(*mut Foo<usize>) {box_free::<Foo<usize>>}
                                         // + val: Value(ByVal(Undef))
                                         // mir::Constant
                                         // + span: test.rs:29:5: 29:6
                                         // + ty: unsafe fn(*mut Foo<usize>) {box_free::<Foo<usize>>}
                                         // + literal: const box_free
    }

    bb10: {                             
        switchInt(_8) -> [false: bb7, otherwise: bb9]; // bb10[0]: scope 0 at test.rs:29:5: 29:6
    }

    bb11: {                              // cleanup
        _8 = const false;                // bb11[0]: scope 0 at test.rs:29:5: 29:6
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:29:5: 29:6
                                         // + ty: bool
                                         // + literal: const false
        _11 = const box_free(move _4) -> bb6; // bb11[1]: scope 0 at test.rs:29:5: 29:6
                                         // ty::Const
                                         // + ty: unsafe fn(*mut Foo<usize>) {box_free::<Foo<usize>>}
                                         // + val: Value(ByVal(Undef))
                                         // mir::Constant
                                         // + span: test.rs:29:5: 29:6
                                         // + ty: unsafe fn(*mut Foo<usize>) {box_free::<Foo<usize>>}
                                         // + literal: const box_free
    }

    bb12: {                              // cleanup
        switchInt(_8) -> [false: bb6, otherwise: bb11]; // bb12[0]: scope 0 at test.rs:29:5: 29:6
    }

    bb13: {                             
        goto -> bb10;                    // bb13[0]: scope 0 at test.rs:29:5: 29:6
    }

    bb14: {                              // cleanup
        goto -> bb1;                     // bb14[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb15: {                              // cleanup
        goto -> bb18;                    // bb15[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb16: {                              // cleanup
        drop(((_1 as Some).0: Box<Foo<usize>>)) -> bb14; // bb16[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb17: {                              // cleanup
        _9 = const false;                // bb17[0]: scope 0 at test.rs:31:1: 31:2
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:31:1: 31:2
                                         // + ty: bool
                                         // + literal: const false
        goto -> bb16;                    // bb17[1]: scope 0 at test.rs:31:1: 31:2
    }

    bb18: {                              // cleanup
        switchInt(_9) -> [false: bb14, otherwise: bb17]; // bb18[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb19: {                              // cleanup
        drop(_1) -> bb14;                // bb19[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb20: {                              // cleanup
        _12 = discriminant(_1);          // bb20[0]: scope 0 at test.rs:31:1: 31:2
        switchInt(move _12) -> [1isize: bb15, otherwise: bb19]; // bb20[1]: scope 0 at test.rs:31:1: 31:2
    }

    bb21: {                             
        goto -> bb8;                     // bb21[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb22: {                              // cleanup
        goto -> bb1;                     // bb22[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb23: {                              // cleanup
        goto -> bb26;                    // bb23[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb24: {                              // cleanup
        drop(((_1 as Some).0: Box<Foo<usize>>)) -> bb22; // bb24[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb25: {                              // cleanup
        _9 = const false;                // bb25[0]: scope 0 at test.rs:31:1: 31:2
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:31:1: 31:2
                                         // + ty: bool
                                         // + literal: const false
        goto -> bb24;                    // bb25[1]: scope 0 at test.rs:31:1: 31:2
    }

    bb26: {                              // cleanup
        switchInt(_9) -> [false: bb22, otherwise: bb25]; // bb26[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb27: {                              // cleanup
        goto -> bb30;                    // bb27[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb28: {                              // cleanup
        drop(((_1 as Some).0: Box<Foo<usize>>)) -> bb22; // bb28[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb29: {                              // cleanup
        _9 = const false;                // bb29[0]: scope 0 at test.rs:31:1: 31:2
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:31:1: 31:2
                                         // + ty: bool
                                         // + literal: const false
        goto -> bb28;                    // bb29[1]: scope 0 at test.rs:31:1: 31:2
    }

    bb30: {                              // cleanup
        switchInt(_9) -> [false: bb22, otherwise: bb29]; // bb30[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb31: {                             
        goto -> bb34;                    // bb31[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb32: {                             
        drop(((_1 as Some).0: Box<Foo<usize>>)) -> [return: bb21, unwind: bb22]; // bb32[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb33: {                             
        _9 = const false;                // bb33[0]: scope 0 at test.rs:31:1: 31:2
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:31:1: 31:2
                                         // + ty: bool
                                         // + literal: const false
        goto -> bb32;                    // bb33[1]: scope 0 at test.rs:31:1: 31:2
    }

    bb34: {                             
        switchInt(_9) -> [false: bb21, otherwise: bb33]; // bb34[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb35: {                             
        drop(_1) -> [return: bb21, unwind: bb22]; // bb35[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb36: {                              // cleanup
        drop(_1) -> bb22;                // bb36[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb37: {                             
        _13 = discriminant(_1);          // bb37[0]: scope 0 at test.rs:31:1: 31:2
        switchInt(move _13) -> [1isize: bb31, otherwise: bb35]; // bb37[1]: scope 0 at test.rs:31:1: 31:2
    }

    bb38: {                              // cleanup
        _14 = discriminant(_1);          // bb38[0]: scope 0 at test.rs:31:1: 31:2
        switchInt(move _14) -> [1isize: bb23, otherwise: bb36]; // bb38[1]: scope 0 at test.rs:31:1: 31:2
    }
}
