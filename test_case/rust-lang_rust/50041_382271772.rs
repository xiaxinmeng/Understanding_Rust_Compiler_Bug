
    bb5: {                              
        _8 = const false;                // bb5[0]: scope 0 at test.rs:29:5: 29:6
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Value(ByVal(Bytes(0)))
                                         // mir::Constant
                                         // + span: test.rs:29:5: 29:6
                                         // + ty: bool
                                         // + literal: const false
        _10 = const box_free(move _4) -> bb4; // bb5[1]: scope 0 at test.rs:29:5: 29:6
                                         // ty::Const
                                         // + ty: unsafe fn(*mut Foo<usize>) {box_free::<Foo<usize>>}
                                         // + val: Value(ByVal(Undef))
                                         // mir::Constant
                                         // + span: test.rs:29:5: 29:6
                                         // + ty: unsafe fn(*mut Foo<usize>) {box_free::<Foo<usize>>}
                                         // + literal: const box_free
    }
