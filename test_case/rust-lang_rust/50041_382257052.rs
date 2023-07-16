
// MIR for `foo`
// source = MirSource { def_id: DefId(0/0:10 ~ test[8787]::foo[0]), promoted: None }
// pass_name = PreTrans
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
    scope 4 {
    }
    let mut _3: isize;
    let mut _5: isize;
    let mut _6: Foo<usize>;
    let mut _7: usize;
    let mut _8: bool;
    let mut _9: bool;
    let mut _10: ();
    let mut _11: isize;
    let mut _12: &mut Foo<usize>;
    let mut _13: *mut Foo<usize>;

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
        switchInt(move _5) -> [0isize: bb1, 1isize: bb3, otherwise: bb2]; // bb0[6]: scope 0 at test.rs:27:9: 27:13
    }

    bb1: {                              
        (_2.0: usize) = const 0usize;    // bb1[0]: scope 0 at test.rs:27:17: 27:23
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:27:21: 27:22
                                         // + ty: usize
                                         // + literal: const 0usize
        goto -> bb6;                     // bb1[1]: scope 0 at test.rs:26:13: 29:6
    }

    bb2: {                              
        unreachable;                     // bb2[0]: scope 0 at test.rs:31:2: 31:2
    }

    bb3: {                              
        StorageLive(_4);                 // bb3[0]: scope 0 at test.rs:28:14: 28:17
        _9 = const false;                // bb3[1]: scope 0 at test.rs:28:14: 28:17
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:28:14: 28:17
                                         // + ty: bool
                                         // + literal: const false
        _8 = const true;                 // bb3[2]: scope 0 at test.rs:28:14: 28:17
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(1)))
                                         // mir::Constant
                                         // + span: test.rs:28:14: 28:17
                                         // + ty: bool
                                         // + literal: const true
        _4 = move ((_1 as Some).0: Box<Foo<usize>>); // bb3[3]: scope 0 at test.rs:28:14: 28:17
        StorageLive(_6);                 // bb3[4]: scope 3 at test.rs:28:22: 28:26
        _6 = move (*_4);                 // bb3[5]: scope 3 at test.rs:28:22: 28:26
        _2 = move _6;                    // bb3[6]: scope 3 at test.rs:28:22: 28:26
        StorageDead(_6);                 // bb3[7]: scope 3 at test.rs:28:25: 28:26
        goto -> bb6;                     // bb3[8]: scope 0 at test.rs:26:13: 29:6
    }

    bb4: {                              
        _8 = const false;                // bb4[0]: scope 0 at test.rs:29:5: 29:6
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:29:5: 29:6
                                         // + ty: bool
                                         // + literal: const false
        StorageDead(_4);                 // bb4[1]: scope 0 at test.rs:29:5: 29:6
        StorageLive(_7);                 // bb4[2]: scope 1 at test.rs:30:5: 30:8
        _7 = (_2.0: usize);              // bb4[3]: scope 1 at test.rs:30:5: 30:8
        _0 = move _7;                    // bb4[4]: scope 1 at test.rs:30:5: 30:8
        StorageDead(_7);                 // bb4[5]: scope 1 at test.rs:30:7: 30:8
        StorageDead(_2);                 // bb4[6]: scope 0 at test.rs:31:1: 31:2
        _11 = discriminant(_1);          // bb4[7]: scope 0 at test.rs:31:1: 31:2
        switchInt(move _11) -> [1isize: bb8, otherwise: bb10]; // bb4[8]: scope 0 at test.rs:31:1: 31:2
    }

    bb5: {                              
        _8 = const false;                // bb5[0]: scope 0 at test.rs:29:5: 29:6
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:29:5: 29:6
                                         // + ty: bool
                                         // + literal: const false
        _12 = &mut (*_4);                // bb5[1]: scope 0 at test.rs:29:5: 29:6
        _13 = move _12 as *mut Foo<usize> (Misc); // bb5[2]: scope 0 at test.rs:29:5: 29:6
        _10 = const dealloc(move _13) -> bb4; // bb5[3]: scope 4 at test.rs:16:5: 16:17
                                         // ty::Const
                                         // + ty: fn(*mut Foo<usize>) {dealloc::<Foo<usize>>}
                                         // + val: Value(ByVal(Undef))
                                         // mir::Constant
                                         // + span: test.rs:16:5: 16:12
                                         // + ty: fn(*mut Foo<usize>) {dealloc::<Foo<usize>>}
                                         // + literal: const dealloc
    }

    bb6: {                              
        switchInt(_8) -> [false: bb4, otherwise: bb5]; // bb6[0]: scope 0 at test.rs:29:5: 29:6
    }

    bb7: {                              
        return;                          // bb7[0]: scope 0 at test.rs:31:2: 31:2
    }

    bb8: {                              
        switchInt(_9) -> [false: bb7, otherwise: bb9]; // bb8[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb9: {                              
        _9 = const false;                // bb9[0]: scope 0 at test.rs:31:1: 31:2
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:31:1: 31:2
                                         // + ty: bool
                                         // + literal: const false
        drop(((_1 as Some).0: Box<Foo<usize>>)) -> bb7; // bb9[1]: scope 0 at test.rs:31:1: 31:2
    }

    bb10: {                             
        drop(_1) -> bb7;                 // bb10[0]: scope 0 at test.rs:31:1: 31:2
    }
}
