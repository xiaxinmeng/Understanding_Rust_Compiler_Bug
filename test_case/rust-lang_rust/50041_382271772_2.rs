
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
