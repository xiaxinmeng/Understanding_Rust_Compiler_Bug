
// MIR for `main`
// source = MirSource { def_id: DefId(0/0:8 ~ issue_48070[317d]::main[0]), promoted: None }
// pass_name = QualifyAndPromoteConstants
// disambiguator = before

fn main() -> (){
    let mut _0: ();                      // return place
    scope 1 {
        let mut _1: Foo;                 // "foo" in scope 1 at .\issue-48070.rs:14:9: 14:16
    }
    scope 2 {
    }
    let mut _2: ();
    let mut _3: &mut Foo;
    let mut _4: &mut Foo;
    let mut _5: i32;
    let mut _6: &mut Foo;
    let mut _7: &mut Foo;
    let mut _8: &mut Foo;
    let mut _9: &mut Foo;

    bb0: {                              
        StorageLive(_1);                 // bb0[0]: scope 0 at .\issue-48070.rs:14:9: 14:16
        _1 = Foo { x: const 0u32 };      // bb0[1]: scope 0 at .\issue-48070.rs:14:19: 14:31
                                         // ty::Const
                                         // + ty: u32
                                         // + val: Integral(U32(0))
                                         // mir::Constant
                                         // + span: .\issue-48070.rs:14:28: 14:29
                                         // + ty: u32
                                         // + literal: const 0u32
        StorageLive(_3);                 // bb0[2]: scope 1 at .\issue-48070.rs:15:5: 19:6
        StorageLive(_4);                 // bb0[3]: scope 1 at .\issue-48070.rs:15:5: 19:6
        StorageLive(_5);                 // bb0[4]: scope 1 at .\issue-48070.rs:15:11: 15:13
        _5 = const 22i32;                // bb0[5]: scope 1 at .\issue-48070.rs:15:11: 15:13
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Integral(I32(22))
                                         // mir::Constant
                                         // + span: .\issue-48070.rs:15:11: 15:13
                                         // + ty: i32
                                         // + literal: const 22i32
        switchInt(_5) -> [22i32: bb5, 44i32: bb6, otherwise: bb7]; // bb0[6]: scope 1 at .\issue-48070.rs:16:9: 16:11
    }

    bb1: {                               // cleanup
        resume;                          // bb1[0]: scope 0 at .\issue-48070.rs:13:1: 20:2
    }

    bb2: {                              
        _4 = &mut _1;                    // bb2[0]: scope 1 at .\issue-48070.rs:16:15: 16:23
        goto -> bb9;                     // bb2[1]: scope 1 at .\issue-48070.rs:15:5: 19:6
    }

    bb3: {                              
        StorageLive(_6);                 // bb3[0]: scope 1 at .\issue-48070.rs:17:15: 17:28
        StorageLive(_7);                 // bb3[1]: scope 1 at .\issue-48070.rs:17:15: 17:18
        _7 = &mut _1;                    // bb3[2]: scope 1 at .\issue-48070.rs:17:15: 17:18
        _6 = const Foo::twiddle(move _7) -> [return: bb10, unwind: bb1]; // bb3[3]: scope 1 at .\issue-48070.rs:17:15: 17:28
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut Foo) -> &'r mut Foo {Foo::twiddle}
                                         // + val: Function(DefId(0/0:5 ~ issue_48070[317d]::{{impl}}[0]::twiddle[0]), Slice([]))
                                         // mir::Constant
                                         // + span: .\issue-48070.rs:17:15: 17:28
                                         // + ty: for<'r> fn(&'r mut Foo) -> &'r mut Foo {Foo::twiddle}
                                         // + literal: const Foo::twiddle
    }

    bb4: {                              
        StorageLive(_8);                 // bb4[0]: scope 1 at .\issue-48070.rs:18:14: 18:27
        StorageLive(_9);                 // bb4[1]: scope 1 at .\issue-48070.rs:18:14: 18:17
        _9 = &mut _1;                    // bb4[2]: scope 1 at .\issue-48070.rs:18:14: 18:17
        _8 = const Foo::twaddle(move _9) -> [return: bb11, unwind: bb1]; // bb4[3]: scope 1 at .\issue-48070.rs:18:14: 18:27
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut Foo) -> &'r mut Foo {Foo::twaddle}
                                         // + val: Function(DefId(0/0:6 ~ issue_48070[317d]::{{impl}}[0]::twaddle[0]), Slice([]))
                                         // mir::Constant
                                         // + span: .\issue-48070.rs:18:14: 18:27
                                         // + ty: for<'r> fn(&'r mut Foo) -> &'r mut Foo {Foo::twaddle}
                                         // + literal: const Foo::twaddle
    }

    bb5: {                              
        falseEdges -> [real: bb2, imaginary: bb6]; // bb5[0]: scope 1 at .\issue-48070.rs:16:9: 16:11
    }

    bb6: {                              
        falseEdges -> [real: bb3, imaginary: bb7]; // bb6[0]: scope 1 at .\issue-48070.rs:17:9: 17:11
    }

    bb7: {                              
        falseEdges -> [real: bb4, imaginary: bb8]; // bb7[0]: scope 1 at .\issue-48070.rs:18:9: 18:10
    }

    bb8: {                              
        unreachable;                     // bb8[0]: scope 1 at .\issue-48070.rs:15:5: 19:6
    }

    bb9: {                              
        _3 = &mut (*_4);                 // bb9[0]: scope 1 at .\issue-48070.rs:15:5: 19:6
        _2 = const Foo::emit(move _3) -> [return: bb12, unwind: bb1]; // bb9[1]: scope 1 at .\issue-48070.rs:15:5: 19:13
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut Foo) {Foo::emit}
                                         // + val: Function(DefId(0/0:7 ~ issue_48070[317d]::{{impl}}[0]::emit[0]), Slice([]))
                                         // mir::Constant
                                         // + span: .\issue-48070.rs:15:5: 19:13
                                         // + ty: for<'r> fn(&'r mut Foo) {Foo::emit}
                                         // + literal: const Foo::emit
    }

    bb10: {                             
        _4 = &mut (*_6);                 // bb10[0]: scope 1 at .\issue-48070.rs:17:15: 17:28
        StorageDead(_6);                 // bb10[1]: scope 1 at .\issue-48070.rs:17:27: 17:28
        StorageDead(_7);                 // bb10[2]: scope 1 at .\issue-48070.rs:17:27: 17:28
        goto -> bb9;                     // bb10[3]: scope 1 at .\issue-48070.rs:15:5: 19:6
    }

    bb11: {                             
        _4 = &mut (*_8);                 // bb11[0]: scope 1 at .\issue-48070.rs:18:14: 18:27
        StorageDead(_8);                 // bb11[1]: scope 1 at .\issue-48070.rs:18:26: 18:27
        StorageDead(_9);                 // bb11[2]: scope 1 at .\issue-48070.rs:18:26: 18:27
        goto -> bb9;                     // bb11[3]: scope 1 at .\issue-48070.rs:15:5: 19:6
    }

    bb12: {                             
        EndRegion();                     // bb12[0]: scope 1 at .\issue-48070.rs:15:5: 19:13
        StorageDead(_3);                 // bb12[1]: scope 1 at .\issue-48070.rs:19:12: 19:13
        StorageDead(_4);                 // bb12[2]: scope 1 at .\issue-48070.rs:19:13: 19:14
        StorageDead(_5);                 // bb12[3]: scope 1 at .\issue-48070.rs:19:13: 19:14
        _0 = ();                         // bb12[4]: scope 0 at .\issue-48070.rs:13:11: 20:2
        StorageDead(_1);                 // bb12[5]: scope 0 at .\issue-48070.rs:20:1: 20:2
        return;                          // bb12[6]: scope 0 at .\issue-48070.rs:20:2: 20:2
    }
}
