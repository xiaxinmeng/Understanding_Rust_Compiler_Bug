
    bb5: {                              
        StorageDead(_4);                 // bb5[0]: scope 0 at src/main.rs:2:37: 2:38
        StorageLive(_6);                 // bb5[1]: scope 1 at src/main.rs:3:5: 3:9
        StorageLive(_7);                 // bb5[2]: scope 1 at src/main.rs:3:7: 3:8
        _7 = const 1usize;               // bb5[3]: scope 1 at src/main.rs:3:7: 3:8
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(Bits { defined: 64, bits: 1 }))
                                         // mir::Constant
                                         // + span: src/main.rs:3:7: 3:8
                                         // + ty: usize
                                         // + literal: const 1usize
        _8 = const 1usize;               // bb5[4]: scope 1 at src/main.rs:3:5: 3:9
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(Bits { defined: 64, bits: 1 }))
                                         // mir::Constant
                                         // + span: src/main.rs:3:5: 3:9
                                         // + ty: usize
                                         // + literal: const 1usize
        _9 = Lt(_7, _8);                 // bb5[5]: scope 1 at src/main.rs:3:5: 3:9
        assert(move _9, "index out of bounds: the len is move _8 but the index is _7") -> bb6; // bb5[6]: scope 1 at src/main.rs:3:5: 3:9
    }
