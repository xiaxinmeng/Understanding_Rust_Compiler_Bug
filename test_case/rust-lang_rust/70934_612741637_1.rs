rust
fn  foo() -> () {
    let mut _0: ();                      // return place in scope 0 at src/lib.rs:3:10: 3:10
    let _1: &(S,);                       // in scope 0 at src/lib.rs:4:5: 4:15
    let mut _2: &(S,);                   // in scope 0 at src/lib.rs:4:5: 4:15

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/lib.rs:4:5: 4:15
        _2 = const foo::promoted[0];     // bb0[1]: scope 0 at src/lib.rs:4:5: 4:15
                                         // ty::Const
                                         // + ty: &(S,)
                                         // + val: Unevaluated(DefId(0:5 ~ playground[4e22]::foo[0]), [], Some(promoted[0]))
                                         // mir::Constant
                                         // + span: src/lib.rs:4:5: 4:15
                                         // + literal: Const { ty: &(S,), val: Unevaluated(DefId(0:5 ~ playground[4e22]::foo[0]), [], Some(promoted[0])) }
        _1 = _2;                         // bb0[2]: scope 0 at src/lib.rs:4:5: 4:15
        StorageDead(_1);                 // bb0[3]: scope 0 at src/lib.rs:4:15: 4:16
        return;                          // bb0[4]: scope 0 at src/lib.rs:5:2: 5:2
    }
}

promoted[0] in  foo: &(S,) = {
    let mut _0: &(S,);                   // return place in scope 0 at src/lib.rs:4:5: 4:15
    let mut _1: (S,);                    // in scope 0 at src/lib.rs:4:6: 4:15
    let mut _2: S;                       // in scope 0 at src/lib.rs:4:7: 4:13
    let mut _3: [S; 1];                  // in scope 0 at src/lib.rs:4:7: 4:10
    let mut _4: S;                       // in scope 0 at src/lib.rs:4:8: 4:9
    let mut _5: usize;                   // in scope 0 at src/lib.rs:4:11: 4:12

    bb0: {
        _3 = [move _4];                  // bb0[0]: scope 0 at src/lib.rs:4:7: 4:10
        _5 = const 0usize;               // bb0[1]: scope 0 at src/lib.rs:4:11: 4:12
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x0000000000000000))
                                         // mir::Constant
                                         // + span: src/lib.rs:4:11: 4:12
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000000)) }
        _2 = move _3[_5];                // bb0[2]: scope 0 at src/lib.rs:4:7: 4:13
        (_1.0: S) = move _2;             // bb0[3]: scope 0 at src/lib.rs:4:6: 4:15
        _0 = &_1;                        // bb0[4]: scope 0 at src/lib.rs:4:5: 4:15
        return;                          // bb0[5]: scope 0 at src/lib.rs:4:5: 4:15
    }
}
