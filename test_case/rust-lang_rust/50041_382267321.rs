
// MIR for `foo`
// source = MirSource { def_id: DefId(0/0:10 ~ test[8787]::foo[0]), promoted: None }
// pass_name = mir_map
// disambiguator = 0

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

    bb0: {                              
        StorageLive(_2);                 // bb0[0]: scope 0 at test.rs:26:9: 26:10
        _3 = discriminant(_1);           // bb0[1]: scope 0 at test.rs:26:13: 29:6
        _5 = discriminant(_1);           // bb0[2]: scope 0 at test.rs:27:9: 27:13
        switchInt(move _5) -> [0isize: bb7, 1isize: bb8, otherwise: bb9]; // bb0[3]: scope 0 at test.rs:27:9: 27:13
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
        goto -> bb12;                    // bb2[1]: scope 0 at test.rs:26:13: 29:6
    }

    bb3: {                              
        StorageLive(_6);                 // bb3[0]: scope 3 at test.rs:28:22: 28:26
        _6 = move (*_4);                 // bb3[1]: scope 3 at test.rs:28:22: 28:26
        _2 = move _6;                    // bb3[2]: scope 3 at test.rs:28:22: 28:26
        StorageDead(_6);                 // bb3[3]: scope 3 at test.rs:28:25: 28:26
        goto -> bb12;                    // bb3[4]: scope 0 at test.rs:26:13: 29:6
    }

    bb4: {                              
        falseEdges -> [real: bb10, imaginary: bb5]; // bb4[0]: scope 0 at test.rs:27:9: 27:13
    }

    bb5: {                              
        falseEdges -> [real: bb11, imaginary: bb6]; // bb5[0]: scope 0 at test.rs:28:9: 28:18
    }

    bb6: {                              
        unreachable;                     // bb6[0]: scope 0 at test.rs:26:13: 29:6
    }

    bb7: {                              
        goto -> bb4;                     // bb7[0]: scope 0 at test.rs:27:9: 27:13
    }

    bb8: {                              
        goto -> bb5;                     // bb8[0]: scope 0 at test.rs:28:9: 28:18
    }

    bb9: {                              
        unreachable;                     // bb9[0]: scope 0 at test.rs:31:2: 31:2
    }

    bb10: {                             
        goto -> bb2;                     // bb10[0]: scope 0 at test.rs:27:9: 27:13
    }

    bb11: {                             
        StorageLive(_4);                 // bb11[0]: scope 0 at test.rs:28:14: 28:17
        _4 = move ((_1 as Some).0: Box<Foo<usize>>); // bb11[1]: scope 0 at test.rs:28:14: 28:17
        goto -> bb3;                     // bb11[2]: scope 0 at test.rs:28:9: 28:18
    }

    bb12: {                             
        drop(_4) -> [return: bb24, unwind: bb21]; // bb12[0]: scope 0 at test.rs:29:5: 29:6
    }

    bb13: {                              // cleanup
        goto -> bb1;                     // bb13[0]: scope 0 at test.rs:25:1: 31:2
    }

    bb14: {                              // cleanup
        drop(_1) -> bb13;                // bb14[0]: scope 0 at test.rs:31:1: 31:2
    }

    bb15: {                              // cleanup
        goto -> bb14;                    // bb15[0]: scope 0 at test.rs:25:1: 31:2
    }

    bb16: {                              // cleanup
        goto -> bb15;                    // bb16[0]: scope 0 at test.rs:25:49: 31:2
    }

    bb17: {                              // cleanup
        goto -> bb16;                    // bb17[0]: scope 0 at test.rs:25:49: 31:2
    }

    bb18: {                              // cleanup
        goto -> bb17;                    // bb18[0]: scope 0 at test.rs:25:49: 31:2
    }

    bb19: {                              // cleanup
        goto -> bb18;                    // bb19[0]: scope 0 at test.rs:25:49: 31:2
    }

    bb20: {                              // cleanup
        goto -> bb19;                    // bb20[0]: scope 0 at test.rs:25:49: 31:2
    }

    bb21: {                              // cleanup
        goto -> bb20;                    // bb21[0]: scope 0 at test.rs:25:49: 31:2
    }

    bb22: {                              // cleanup
        drop(_4) -> bb21;                // bb22[0]: scope 0 at test.rs:29:5: 29:6
    }

    bb23: {                              // cleanup
        goto -> bb22;                    // bb23[0]: scope 0 at test.rs:26:13: 29:6
    }

    bb24: {                             
        StorageDead(_4);                 // bb24[0]: scope 0 at test.rs:29:5: 29:6
        StorageLive(_7);                 // bb24[1]: scope 1 at test.rs:30:5: 30:8
        _7 = (_2.0: usize);              // bb24[2]: scope 1 at test.rs:30:5: 30:8
        _0 = move _7;                    // bb24[3]: scope 1 at test.rs:30:5: 30:8
        StorageDead(_7);                 // bb24[4]: scope 1 at test.rs:30:7: 30:8
        StorageDead(_2);                 // bb24[5]: scope 0 at test.rs:31:1: 31:2
        drop(_1) -> [return: bb25, unwind: bb1]; // bb24[6]: scope 0 at test.rs:31:1: 31:2
    }

    bb25: {                             
        goto -> bb26;                    // bb25[0]: scope 0 at test.rs:31:2: 31:2
    }

    bb26: {                             
        return;                          // bb26[0]: scope 0 at test.rs:31:2: 31:2
    }
}
